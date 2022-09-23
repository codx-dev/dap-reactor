use core::{fmt, str};
use std::io;

pub use serde_json::{json, Map, Value};

use super::*;
use crate::error::{Cause, Error};
use crate::utils;

impl From<ProtocolRequest> for Value {
    fn from(request: ProtocolRequest) -> Self {
        let ProtocolRequest {
            seq,
            command,
            arguments,
        } = request;

        let seq = utils::attribute_u64("seq", seq);
        let ty = utils::attribute_string("type", "request");
        let command = utils::attribute_string("command", command);
        let arguments = utils::attribute_optional("arguments", arguments);

        utils::finalize_object(seq.chain(ty).chain(command).chain(arguments))
    }
}

impl From<ProtocolEvent> for Value {
    fn from(event: ProtocolEvent) -> Self {
        let ProtocolEvent { seq, event, body } = event;

        let seq = utils::attribute_u64("seq", seq);
        let ty = utils::attribute_string("type", "event");
        let event = utils::attribute_string("event", event);
        let body = utils::attribute_optional("body", body);

        utils::finalize_object(seq.chain(ty).chain(event).chain(body))
    }
}

impl fmt::Display for ProtocolResponseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        <ProtocolResponseError as fmt::Debug>::fmt(self, f)
    }
}

impl From<ProtocolResponse> for Value {
    fn from(response: ProtocolResponse) -> Self {
        let ProtocolResponse {
            seq,
            request_seq,
            command,
            result,
        } = response;

        let success;
        let error;
        let body_result;

        match result {
            Ok(body) => {
                success = true;
                error = None;
                body_result = body;
            }

            Err(ProtocolResponseError { message, body }) => {
                success = false;
                error = message;
                body_result = body.map(Value::from);
            }
        };

        let seq = utils::attribute_u64("seq", seq);
        let ty = utils::attribute_string("type", "response");
        let request_seq = utils::attribute_u64("request_seq", request_seq);
        let success = utils::attribute_bool("success", success);
        let command = utils::attribute_string("command", command);
        let message = utils::attribute_string_optional("message", error);
        let body = utils::attribute_optional("body", body_result);

        utils::finalize_object(
            seq.chain(ty)
                .chain(request_seq)
                .chain(success)
                .chain(command)
                .chain(message)
                .chain(body),
        )
    }
}

impl From<ProtocolMessage> for Value {
    fn from(m: ProtocolMessage) -> Self {
        match m {
            ProtocolMessage::Request(r) => r.into(),
            ProtocolMessage::Response(r) => r.into(),
            ProtocolMessage::Event(e) => e.into(),
        }
    }
}

impl TryFrom<&Value> for ProtocolMessage {
    type Error = Error;

    fn try_from(value: &Value) -> Result<Self, Self::Error> {
        let map = value
            .as_object()
            .ok_or(Error::new("protocolMessage", Cause::ExpectsObject))?;

        let seq = utils::get_u64(map, "seq")?;
        let ty = utils::get_str(map, "type")?;

        match ty {
            "request" => {
                let command = utils::get_string(map, "command")?;
                let arguments = utils::get_optional(map, "arguments");

                Ok(Self::Request(ProtocolRequest {
                    seq,
                    command,
                    arguments,
                }))
            }

            "response" => {
                let request_seq = utils::get_u64(map, "request_seq")?;
                let success = utils::get_bool(map, "success")?;
                let command = utils::get_string(map, "command")?;
                let message = utils::get_string_optional(map, "message")?;
                let body = utils::get_optional(map, "body");

                let result = if success {
                    Ok(body)
                } else {
                    Err(ProtocolResponseError {
                        message,
                        body: body
                            .and_then(|b| b.as_object().and_then(|b| Message::try_from(b).ok())),
                    })
                };

                Ok(Self::Response(ProtocolResponse {
                    seq,
                    request_seq,
                    command,
                    result,
                }))
            }

            "event" => {
                let event = utils::get_string(map, "event")?;
                let body = map.get("body").cloned();

                Ok(Self::Event(ProtocolEvent { seq, event, body }))
            }

            _ => Err(Error::new("protocolMessage", Cause::ExpectsEnum)),
        }
    }
}

impl ProtocolMessage {
    pub fn into_adapter_message(self) -> String {
        let payload = Value::from(self).to_string();
        let payload = format!("Content-Length: {}\r\n\r\n{}", payload.len(), payload);

        payload
    }

    pub fn try_from_json_bytes<B>(bytes: B) -> Result<Self, Error>
    where
        B: AsRef<[u8]>,
    {
        serde_json::from_slice(bytes.as_ref())
            .map_err(|_| Error::new("protocolMessage", Cause::IsInvalid))
            .and_then(|m| Self::try_from(&m))
    }

    pub fn try_from_bytes<B>(bytes: B) -> Result<(usize, Self), Error>
    where
        B: AsRef<[u8]>,
    {
        let (header, body) = str::from_utf8(bytes.as_ref())
            .map_err(|_| Error::new("protocolMessage", Cause::InvalidUtf8))?
            .split_once("\r\n\r\n")
            .ok_or(Error::new("protocolMessage", Cause::IsMandatory))?;

        // split header + separator
        let consumed = header.len() + 4;

        let len = header
            .lines()
            .filter_map(|h| h.split_once(": "))
            .filter_map(|(key, value)| match key.to_lowercase().as_str() {
                "content-length" => value.parse::<usize>().ok(),
                _ => None,
            })
            .next()
            .ok_or(Error::new("protocolMessage", Cause::IsMandatory))?;

        if body.len() < len {
            return Err(Error::new("protocolMessage", Cause::UnexpectedEof));
        }

        let message = serde_json::from_str(&body[..len])
            .map_err(|_| Error::new("protocolMessage", Cause::IsInvalid))
            .and_then(|m| Self::try_from(&m))?;

        let consumed = consumed + len;

        Ok((consumed, message))
    }

    pub fn try_from_reader<R>(reader: R) -> io::Result<(usize, Self)>
    where
        R: io::Read,
    {
        let mut bytes = reader.bytes();

        let mut consumed = 0;
        let content_len;

        // find content-length header
        loop {
            let line = bytes
                .by_ref()
                // forward the error to collect
                .take_while(|b| b.as_ref().map(|b| b != &b'\n').unwrap_or(true))
                .collect::<io::Result<Vec<_>>>()?;

            if line.is_empty() {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidInput,
                    "content-length is mandatory",
                ));
            }

            // include the line break
            consumed += line.len() + 1;

            let line = line.strip_suffix(&[b'\r']).unwrap_or(&line);

            let header = match str::from_utf8(line) {
                Ok(h) => h.to_ascii_lowercase(),
                Err(e) => {
                    #[cfg(not(feature = "tracing"))]
                    let _ = e;

                    #[cfg(feature = "tracing")]
                    tracing::warn!("discarding invalid utf-8 header: {}", e);

                    continue;
                }
            };

            let len = match header.split_once(": ") {
                Some(("content-length", len)) => len,
                _ => continue,
            };

            content_len = len
                .parse::<usize>()
                .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;

            break;
        }

        // skip header items until empty line
        loop {
            let line = bytes
                .by_ref()
                // forward the error to collect
                .take_while(|b| b.as_ref().map(|b| b != &b'\n').unwrap_or(true))
                .collect::<io::Result<Vec<_>>>()?;

            // include the line break
            consumed += line.len() + 1;

            if line.is_empty() || line.len() == 1 && line[0] == b'\r' {
                break;
            }
        }

        let content = bytes.take(content_len).collect::<io::Result<Vec<_>>>()?;

        if content.len() != content_len {
            return Err(io::Error::new(
                io::ErrorKind::UnexpectedEof,
                "the provided content is not big enough",
            ));
        }

        consumed += content_len;

        serde_json::from_slice(&content)
            .map_err(|_| Error::new("protocolMessage", Cause::IsInvalid))
            .and_then(|m| Self::try_from(&m))
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))
            .map(|m| (consumed, m))
    }
}
