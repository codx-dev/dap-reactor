use std::io;

pub use serde_json::Value;

use crate::event::Event;
use crate::protocol::ProtocolMessage;
use crate::request::Request;
use crate::response::Response;

pub trait Backend {
    fn request(&mut self, request: Request) -> Response;
    fn event(&mut self, event: Event);
}

#[async_trait::async_trait]
pub trait AsyncBackend {
    async fn request(&mut self, request: Request) -> Response;
    async fn event(&mut self, event: Event);
}

#[derive(Debug)]
pub struct Reactor<B, O> {
    backend: B,
    output: O,
    seq: u64,
}

impl<B, O> Reactor<B, O> {
    fn get_seq(&mut self) -> u64 {
        let seq = self.seq;

        self.seq = seq.wrapping_add(1).min(1);

        seq
    }
}

impl<B, O> Reactor<B, O>
where
    B: Backend,
    O: io::Write,
{
    pub fn consume<T>(&mut self, bytes: T) -> io::Result<usize>
    where
        T: AsRef<[u8]>,
    {
        let (consumed, message) = ProtocolMessage::try_from_bytes(bytes)?;

        match message {
            ProtocolMessage::Request(request) => {
                let request_seq = request.seq;
                let response = Request::try_from(&request).map(|r| self.backend.request(r))?;
                let response = response.into_protocol(self.get_seq(), request_seq);
                let response = ProtocolMessage::Response(response);

                let _ = self
                    .output
                    .write(response.into_adapter_message().as_bytes())?;
            }

            ProtocolMessage::Event(event) => {
                Event::try_from(&event).map(|e| self.backend.event(e))?;
            }

            ProtocolMessage::Response(_) => {
                return Err(io::Error::new(io::ErrorKind::Other, "not implemented"))
            }
        }

        Ok(consumed)
    }
}

impl<B, O> Reactor<B, O>
where
    B: AsyncBackend,
    O: io::Write,
{
    pub async fn consume_async<T>(&mut self, bytes: T) -> io::Result<usize>
    where
        T: AsRef<[u8]>,
    {
        let (consumed, message) = ProtocolMessage::try_from_bytes(bytes)?;

        match message {
            ProtocolMessage::Request(request) => {
                let request_seq = request.seq;
                let request = Request::try_from(&request)?;
                let response = self.backend.request(request).await;
                let response = response.into_protocol(self.get_seq(), request_seq);
                let response = ProtocolMessage::Response(response);

                // TODO consider using some async I/O write instead
                //
                // might not be desirable since most of the space uses the std::io
                let _ = self
                    .output
                    .write(response.into_adapter_message().as_bytes())?;
            }

            ProtocolMessage::Event(event) => {
                let event = Event::try_from(&event)?;

                self.backend.event(event).await;
            }

            ProtocolMessage::Response(_) => {
                return Err(io::Error::new(io::ErrorKind::Other, "not implemented"))
            }
        }

        Ok(consumed)
    }
}
