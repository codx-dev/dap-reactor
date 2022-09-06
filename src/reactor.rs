use std::io;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;

pub use serde_json::Value;
use tokio::io::{AsyncBufReadExt, AsyncReadExt as _, AsyncWriteExt as _};
use tokio::{net, sync};

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

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReactorOutput {
    pub reply: Option<ProtocolMessage>,
    pub consumed_bytes: usize,
}

#[derive(Debug)]
pub struct Reactor<B> {
    backend: B,
    seq: Arc<AtomicUsize>,
}

impl<B> Reactor<B> {
    pub fn new(backend: B) -> Self {
        Self {
            backend,
            seq: Arc::new(AtomicUsize::new(1)),
        }
    }

    fn get_seq(&self) -> usize {
        // not checking wrapping here because `usize::MAX` is far beyond the domain of normal DAP
        // usage
        self.seq.fetch_add(1, Ordering::SeqCst)
    }
}

impl<B> Default for Reactor<B>
where
    B: Default,
{
    fn default() -> Self {
        Self::new(B::default())
    }
}

impl<B> Clone for Reactor<B>
where
    B: Clone,
{
    fn clone(&self) -> Self {
        Self {
            backend: self.backend.clone(),
            seq: Arc::clone(&self.seq),
        }
    }
}

impl<B> Reactor<B>
where
    B: Backend,
{
    pub fn handle(&mut self, message: ProtocolMessage) -> io::Result<Option<ProtocolMessage>> {
        tracing::debug!("inbound {:?}", message);

        match message {
            ProtocolMessage::Request(request) => {
                let request_seq = request.seq;
                let reply = Request::try_from(&request).map(|r| self.backend.request(r))?;
                let seq = self.get_seq() as u64;
                let reply = reply.into_protocol(seq, request_seq);
                let reply = ProtocolMessage::Response(reply);

                tracing::debug!("outbound {:?}", reply);

                Ok(Some(reply))
            }

            ProtocolMessage::Event(event) => {
                Event::try_from(&event).map(|e| self.backend.event(e))?;

                Ok(None)
            }

            ProtocolMessage::Response(_) => {
                Err(io::Error::new(io::ErrorKind::Other, "not implemented"))
            }
        }
    }
}

impl<B> Reactor<B>
where
    B: AsyncBackend,
{
    pub async fn handle_async(
        &mut self,
        message: ProtocolMessage,
    ) -> io::Result<Option<ProtocolMessage>> {
        tracing::debug!("inbound {:?}", message);

        match message {
            ProtocolMessage::Request(request) => {
                let request_seq = request.seq;
                let request = Request::try_from(&request)?;
                let reply = self.backend.request(request).await;
                let seq = self.get_seq() as u64;
                let reply = reply.into_protocol(seq, request_seq);
                let reply = ProtocolMessage::Response(reply);

                tracing::debug!("outbound {:?}", reply);

                Ok(Some(reply))
            }

            ProtocolMessage::Event(event) => {
                Event::try_from(&event)
                    .map(|e| self.backend.event(e))?
                    .await;

                Ok(None)
            }

            ProtocolMessage::Response(_) => {
                Err(io::Error::new(io::ErrorKind::Other, "not implemented"))
            }
        }
    }

    async fn handle_socket(reactor: Arc<sync::RwLock<Self>>, stream: net::TcpStream) {
        tracing::trace!("socket received");

        let mut buffer = tokio::io::BufReader::new(stream);

        loop {
            let len;
            let mut consumed = 0;

            // attempt to fetch content-length
            {
                let mut lines = (&mut buffer).lines();

                loop {
                    let line = match lines.next_line().await {
                        Ok(Some(l)) => l.to_ascii_lowercase(),
                        Ok(None) => return,
                        Err(_e) => return,
                    };

                    consumed += line.len() + 1;

                    let value = match line.trim_end_matches('\r').split_once(": ") {
                        Some((key, value)) if key == "content-length" => value,
                        _ => continue,
                    };

                    len = match usize::from_str_radix(value, 10) {
                        Ok(n) => n,
                        Err(e) => {
                            tracing::warn!("invalid content-lenght: {}", e);
                            continue;
                        }
                    };

                    break;
                }

                // skip while line not empty
                loop {
                    let line = match lines.next_line().await {
                        Ok(Some(l)) => l,
                        _ => return,
                    };

                    consumed += line.len() + 1;

                    if line.trim_end_matches('\r').is_empty() {
                        break;
                    }
                }
            }

            let mut content = vec![0u8; len];

            if let Err(e) = buffer.read_exact(&mut content).await {
                tracing::warn!("couldn't read message len: {}", e);
                continue;
            }

            buffer.consume(len + consumed);

            let message = match ProtocolMessage::try_from_json_bytes(content) {
                Ok(m) => m,
                Err(e) => {
                    tracing::warn!("invalid message: {}", e);
                    continue;
                }
            };

            let reply = match reactor.write().await.handle_async(message).await {
                Ok(Some(r)) => r,
                Ok(None) => continue,
                Err(e) => {
                    tracing::error!("error handling message: {}", e);
                    continue;
                }
            };

            if let Err(e) = buffer
                .get_mut()
                .write_all(reply.into_adapter_message().as_bytes())
                .await
            {
                tracing::error!("error replying message: {}", e);
            }
        }
    }

    pub async fn bind_async<S>(self, socket: S) -> io::Result<()>
    where
        S: net::ToSocketAddrs,
    {
        let reactor = sync::RwLock::new(self);
        let reactor = Arc::new(reactor);

        let listener = net::TcpListener::bind(socket).await?;

        tracing::info!(
            "listening on {}",
            listener
                .local_addr()
                .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?
        );

        loop {
            match listener.accept().await {
                Ok((stream, _)) => Self::handle_socket(Arc::clone(&reactor), stream).await,
                Err(e) => tracing::error!("error accepting socket: {}", e),
            }
        }
    }
}
