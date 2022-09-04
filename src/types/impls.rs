use serde_json::{json, Map, Value};

use super::*;
use crate::error::{Cause, Error};
use crate::utils;

impl From<Message> for Value {
    fn from(message: Message) -> Self {
        let Message {
            id,
            format,
            variables,
            send_telemetry,
            show_user,
            url,
            url_label,
        } = message;

        let id = utils::attribute_u64("id", id);
        let format = utils::attribute_string("format", format);
        let variables = utils::attribute_map_optional("variables", variables);
        let send_telemetry = utils::attribute_bool_optional("sendTelemetry", send_telemetry);
        let show_user = utils::attribute_bool_optional("showUser", show_user);
        let url = utils::attribute_string_optional("url", url);
        let url_label = utils::attribute_string_optional("urlLabel", url_label);

        utils::finalize_object(
            id.chain(format)
                .chain(variables)
                .chain(send_telemetry)
                .chain(show_user)
                .chain(url)
                .chain(url_label),
        )
    }
}

impl TryFrom<&Map<String, Value>> for Message {
    type Error = Error;

    fn try_from(map: &Map<String, Value>) -> Result<Self, Self::Error> {
        let id = utils::get_u64(map, "id")?;
        let format = utils::get_string(map, "format")?;
        let variables = utils::get_map_to_string_optional(map, "variables")?;
        let send_telemetry = utils::get_bool_optional(map, "sendTelemetry")?;
        let show_user = utils::get_bool_optional(map, "showUser")?;
        let url = utils::get_string_optional(map, "url")?;
        let url_label = utils::get_string_optional(map, "urlLabel")?;

        Ok(Self {
            id,
            format,
            variables,
            send_telemetry,
            show_user,
            url,
            url_label,
        })
    }
}

impl From<ChecksumAlgorithm> for &'static str {
    fn from(a: ChecksumAlgorithm) -> Self {
        match a {
            ChecksumAlgorithm::Md5 => "MD5",
            ChecksumAlgorithm::Sha1 => "SHA1",
            ChecksumAlgorithm::Sha256 => "SHA256",
            ChecksumAlgorithm::Timestamp => "timestamp",
        }
    }
}

impl TryFrom<&str> for ChecksumAlgorithm {
    type Error = Error;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        match s {
            "MD5" => Ok(ChecksumAlgorithm::Md5),
            "SHA1" => Ok(ChecksumAlgorithm::Sha1),
            "SHA256" => Ok(ChecksumAlgorithm::Sha256),
            "timestamp" => Ok(ChecksumAlgorithm::Timestamp),
            _ => Err(Error::new("checksumAlgorithm", Cause::ExpectsEnum)),
        }
    }
}

impl From<Checksum> for Value {
    fn from(c: Checksum) -> Self {
        json!({
            "algorithm": Value::String(<&'static str>::from(c.algorithm).into()),
            "checksum": Value::String(c.checksum),
        })
    }
}

impl TryFrom<&Map<String, Value>> for Checksum {
    type Error = Error;

    fn try_from(map: &Map<String, Value>) -> Result<Self, Self::Error> {
        let algorithm = utils::get_str(map, "algorithm").and_then(ChecksumAlgorithm::try_from)?;
        let checksum = utils::get_string(map, "checksum")?;

        Ok(Self {
            algorithm,
            checksum,
        })
    }
}

impl From<SourcePresentationHint> for &'static str {
    fn from(p: SourcePresentationHint) -> Self {
        match p {
            SourcePresentationHint::Normal => "normal",
            SourcePresentationHint::Emphasize => "emphasize",
            SourcePresentationHint::Deemphasize => "deemphasize",
        }
    }
}

impl From<SourcePresentationHint> for String {
    fn from(s: SourcePresentationHint) -> Self {
        <&'static str>::from(s).into()
    }
}

impl TryFrom<&str> for SourcePresentationHint {
    type Error = Error;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        match s {
            "normal" => Ok(SourcePresentationHint::Normal),
            "emphasize" => Ok(SourcePresentationHint::Emphasize),
            "deemphasize" => Ok(SourcePresentationHint::Deemphasize),
            _ => Err(Error::new("sourcePresentationHint", Cause::ExpectsEnum)),
        }
    }
}

impl From<Source> for Value {
    fn from(source: Source) -> Self {
        let Source {
            name,
            source_reference,
            presentation_hint,
            origin,
            sources,
            adapter_data,
            checksums,
        } = source;

        let (path, source_reference) = source_reference
            .map(|r| match r {
                SourceReference::Path(path) => (Some(path), None),
                SourceReference::Reference(n) if n == 0 => (None, None),
                SourceReference::Reference(n) => (None, Some(n)),
            })
            .unwrap_or((None, None));

        let name = utils::attribute_string_optional("name", name);
        let path = utils::attribute_string_optional("path", path);
        let source_reference = utils::attribute_u32_optional("sourceReference", source_reference);
        let presentation_hint =
            utils::attribute_string_optional("presentationHint", presentation_hint);
        let origin = utils::attribute_string_optional("origin", origin);
        let sources = utils::attribute_array_optional("sources", sources);
        let adapter_data = utils::attribute_optional("adapterData", adapter_data);
        let checksums = utils::attribute_array_optional("checksums", checksums);

        utils::finalize_object(
            name.chain(path)
                .chain(source_reference)
                .chain(presentation_hint)
                .chain(origin)
                .chain(sources)
                .chain(adapter_data)
                .chain(checksums),
        )
    }
}

impl TryFrom<&Map<String, Value>> for Source {
    type Error = Error;

    fn try_from(map: &Map<String, Value>) -> Result<Self, Self::Error> {
        let name = utils::get_string_optional(map, "name")?;

        let path = utils::get_string_optional(map, "path")?.map(SourceReference::Path);
        let source_reference = utils::get_u32_optional(map, "sourceReference")?
            .filter(|n| n > &0)
            .map(SourceReference::Reference);

        let source_reference = source_reference.or(path);

        let presentation_hint = utils::get_str_optional(map, "presentationHint")?
            .map(SourcePresentationHint::try_from)
            .transpose()?;

        let origin = utils::get_string_optional(map, "origin")?;
        let sources = utils::get_array_optional(map, "sources")?;
        let adapter_data = map.get("adapterData").cloned();
        let checksums = utils::get_array_optional(map, "checksums")?;

        Ok(Self {
            name,
            source_reference,
            presentation_hint,
            origin,
            sources,
            adapter_data,
            checksums,
        })
    }
}

impl From<Breakpoint> for Value {
    fn from(breakpoint: Breakpoint) -> Self {
        let Breakpoint {
            id,
            verified,
            message,
            source,
            line,
            column,
            end_line,
            end_column,
            instruction_reference,
            offset,
        } = breakpoint;

        let id = utils::attribute_u64_optional("id", id);
        let verified = utils::attribute_bool("verified", verified);
        let message = utils::attribute_string_optional("message", message);
        let source = utils::attribute_optional("source", source);
        let line = utils::attribute_u64_optional("line", line);
        let column = utils::attribute_u64_optional("column", column);
        let end_line = utils::attribute_u64_optional("endLine", end_line);
        let end_column = utils::attribute_u64_optional("endColumn", end_column);
        let instruction_reference =
            utils::attribute_string_optional("instructionReference", instruction_reference);
        let offset = utils::attribute_i64_optional("offset", offset);

        utils::finalize_object(
            id.chain(verified)
                .chain(message)
                .chain(source)
                .chain(line)
                .chain(column)
                .chain(end_line)
                .chain(end_column)
                .chain(instruction_reference)
                .chain(offset),
        )
    }
}

impl TryFrom<&Map<String, Value>> for Breakpoint {
    type Error = Error;

    fn try_from(map: &Map<String, Value>) -> Result<Self, Self::Error> {
        let id = utils::get_u64_optional(map, "id")?;
        let verified = utils::get_bool(map, "verified")?;
        let message = utils::get_string_optional(map, "message")?;
        let source = utils::get_object_optional(map, "source")?;
        let line = utils::get_u64_optional(map, "line")?;
        let column = utils::get_u64_optional(map, "column")?;
        let end_line = utils::get_u64_optional(map, "endLine")?;
        let end_column = utils::get_u64_optional(map, "endColumn")?;
        let instruction_reference = utils::get_string_optional(map, "instructionReference")?;
        let offset = utils::get_i64_optional(map, "endColumn")?;

        Ok(Self {
            id,
            verified,
            message,
            source,
            line,
            column,
            end_line,
            end_column,
            instruction_reference,
            offset,
        })
    }
}
