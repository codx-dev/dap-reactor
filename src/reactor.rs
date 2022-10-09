mod client;

#[cfg(test)]
mod tests;

use std::io;
use std::marker::PhantomData;
use std::ops::Deref;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;

use tokio::io::{AsyncBufReadExt, AsyncReadExt as _, AsyncWriteExt as _};
use tokio::net;
use tokio::sync::{self, mpsc};

use crate::event::Event;
use crate::protocol::ProtocolMessage;
use crate::request::{Request, ReverseRequest};
use crate::response::Response;

pub use async_trait::async_trait;
pub use client::*;
pub use serde_json::Value;
pub use tokio::sync::mpsc::Sender;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReactorReverseRequest {
    /// Id to be received in `response`.
    ///
    /// If `None`, the reactor will pick the next available `seq`.
    pub id: Option<u64>,

    /// Reverse request to be sent to the client.
    pub request: ReverseRequest,
}

impl From<ReverseRequest> for ReactorReverseRequest {
    fn from(request: ReverseRequest) -> Self {
        Self { id: None, request }
    }
}

#[async_trait]
pub trait Backend {
    /// Initialize a new instance of a backend.
    ///
    /// The provided mpsc senders will be listened by the reactor and forwarded to the client
    ///
    /// If an id is provided via `Option<u64>`, it will be the id of the response. Otherwise, the
    /// reactor will pick the next available id.
    async fn init(events: Sender<Event>, requests: Sender<ReactorReverseRequest>) -> Self;

    /// A request was sent by the client. It should be replied as response.
    ///
    /// This is infallible because any error that might have happened should be described as a
    /// valid response with success set to `false`.
    ///
    /// Ideally, a backend will always produce a response out of a request. However, this is an
    /// implementation decision so we require `Option` instead. If `None` is passed, the reactor
    /// will not submit a response to the incoming request - this need to be used carefully because
    /// the client might end up in a dangling state for the protocol asks to always provide a
    /// response.
    async fn request(&mut self, request: Request) -> Option<Response>;

    /// The client replied to a reverse request
    async fn response(&mut self, id: u64, response: Response);
}

pub struct Reactor<B> {
    capacity: usize,
    provider: PhantomData<B>,
}

impl<B> Reactor<B>
where
    B: Backend + Send,
{
    pub const fn new() -> Self {
        Self {
            capacity: 100,
            provider: PhantomData,
        }
    }

    pub fn with_capacity(&mut self, capacity: usize) -> &mut Self {
        self.capacity = capacity;
        self
    }

    pub async fn bind<S>(&mut self, socket: S) -> io::Result<ReactorListener<B>>
    where
        S: net::ToSocketAddrs,
    {
        let listener = net::TcpListener::bind(socket).await?;

        Ok(ReactorListener {
            capacity: self.capacity,
            listener,
            provider: PhantomData,
        })
    }
}

pub struct ReactorListener<B> {
    capacity: usize,
    listener: net::TcpListener,
    provider: PhantomData<B>,
}

impl<B> Deref for ReactorListener<B> {
    type Target = net::TcpListener;

    fn deref(&self) -> &Self::Target {
        &self.listener
    }
}

impl<B> ReactorListener<B>
where
    B: Backend + Send,
{
    pub async fn listen(self) -> io::Result<()> {
        let socket = self
            .local_addr()
            .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;

        tracing::info!("listening on {}", socket);

        let Self {
            capacity, listener, ..
        } = self;

        loop {
            match listener.accept().await {
                Ok((stream, addr)) => {
                    tracing::trace!("incoming connection from {}", addr);

                    // this service is not particularly expected to be target of adversarial
                    // clients. this way, we can simplify the implementation with a naive approach
                    // to spawn threads for every socket.
                    //
                    // this is easily attacked by malicious clients because they can send huge
                    // amounts of connections and it would quickly exhaust the reactor. if this
                    // becomes a concern, we can alternatively use some server implementation that
                    // treats such cases for us - as example, actix-server. it will distribute the
                    // incoming requests around a given number of workers

                    let (events_tx, events_rx) = mpsc::channel::<Event>(capacity);
                    let (requests_tx, requests_rx) =
                        mpsc::channel::<ReactorReverseRequest>(capacity);
                    let (mut events, mut requests) = (events_rx, requests_rx);

                    // overflowing a seq in a DAP usage is not really feasible since the limit of
                    // u64 is far beyond any normal usage. so we don't really need to put some
                    // special guard here to check for overflows and we can just benefit from
                    // atomic performance and security
                    let seq_event = Arc::new(AtomicU64::new(1));
                    let seq_request = Arc::clone(&seq_event);
                    let seq_reverse = Arc::clone(&seq_event);

                    let (inbound, outbound) = stream.into_split();

                    let outbound = sync::RwLock::new(outbound);
                    let outbound_event = Arc::new(outbound);
                    let outbound_request = Arc::clone(&outbound_event);
                    let outbound_reverse = Arc::clone(&outbound_event);

                    // thread to handle outbound events generated by the backend
                    tokio::spawn(async move {
                        let outbound = outbound_event;
                        let seq = seq_event;

                        while let Some(ev) = events.recv().await {
                            let seq = seq.fetch_add(1, Ordering::SeqCst);

                            let ev = ev.into_protocol(seq);
                            let ev = ProtocolMessage::from(ev);
                            let ev = ev.into_adapter_message();

                            tracing::debug!("outbound {:?}", ev);

                            if let Err(e) = outbound.write().await.write_all(ev.as_bytes()).await {
                                tracing::error!("error sending event: {}", e);
                            }
                        }
                    });

                    // thread to handle reverse requests from the backend to the client
                    tokio::spawn(async move {
                        let seq = seq_reverse;
                        let outbound = outbound_reverse;

                        while let Some(re) = requests.recv().await {
                            let seq = re.id.unwrap_or_else(|| seq.fetch_add(1, Ordering::SeqCst));
                            let request = re.request.into_protocol(seq);
                            let request = ProtocolMessage::from(request);
                            let request = request.into_adapter_message();

                            if let Err(e) =
                                outbound.write().await.write_all(request.as_bytes()).await
                            {
                                tracing::error!("error sending reverse request: {}", e);
                            }
                        }
                    });

                    // thread to handle inbound requests to be processed by the backend
                    tokio::spawn(async move {
                        let mut backend = B::init(events_tx, requests_tx).await;
                        let seq = seq_request;

                        let mut buffer = tokio::io::BufReader::new(inbound);
                        let outbound = outbound_request;

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

                                    len = match value.parse::<usize>() {
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

                            tracing::debug!("received message {:?}", message);

                            let request = match message {
                                ProtocolMessage::Request(re) => re,

                                ProtocolMessage::Response(re) => {
                                    let id = re.request_seq;

                                    let response = match Response::try_from(&re) {
                                        Ok(re) => re,
                                        Err(e) => {
                                            tracing::debug!(
                                                "error parsing a response from the client: {}",
                                                e
                                            );
                                            continue;
                                        }
                                    };

                                    backend.response(id, response).await;
                                    continue;
                                }

                                ProtocolMessage::Event(ev) => {
                                    tracing::debug!(
                                        "received unexpected event from client: {:?}",
                                        ev
                                    );
                                    continue;
                                }
                            };

                            let request_seq = request.seq;
                            let request = match Request::try_from(&request) {
                                Ok(re) => re,

                                Err(e) => {
                                    tracing::warn!("received invalid request from client: {}", e);
                                    continue;
                                }
                            };

                            let response = match backend.request(request).await {
                                Some(re) => re,

                                None => {
                                    tracing::debug!("request didn't produce a response");
                                    continue;
                                }
                            };

                            let seq = seq.fetch_add(1, Ordering::SeqCst);
                            let response = response.into_protocol(seq, request_seq);
                            let response =
                                ProtocolMessage::Response(response).into_adapter_message();

                            tracing::debug!("outbound {:?}", response);

                            if let Err(e) =
                                outbound.write().await.write_all(response.as_bytes()).await
                            {
                                tracing::error!("error sending response: {}", e);
                            }
                        }
                    });
                }

                Err(e) => tracing::error!("error accepting socket: {}", e),
            }
        }
    }
}
