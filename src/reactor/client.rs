use std::io;

use bytes::BytesMut;
use tokio::net;
use tokio::sync::mpsc;
use tokio::sync::mpsc::error::SendError;
use tokio::task;

use crate::event::Event;
use crate::protocol::ProtocolMessage;
use crate::request::Request;
use crate::response::Response;

pub struct ClientBuilder {
    pub capacity: usize,
    pub buffer: usize,
}

impl Default for ClientBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl ClientBuilder {
    pub fn new() -> Self {
        Self {
            capacity: 50,
            buffer: 4096,
        }
    }

    pub fn with_capacity(mut self, capacity: usize) -> Self {
        self.capacity = capacity;
        self
    }

    pub fn with_buffer(mut self, buffer: usize) -> Self {
        self.buffer = buffer;
        self
    }

    pub async fn connect<S>(self, socket: S) -> io::Result<Client>
    where
        S: net::ToSocketAddrs,
    {
        let Self { capacity, buffer } = self;

        Client::connect(capacity, buffer, socket).await
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ClientRequest {
    pub seq: Option<u64>,
    pub request: Request,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ClientResponse {
    pub seq: u64,
    pub response: Response,
}

pub struct Client {
    pub responses: mpsc::Receiver<ClientResponse>,
    pub events: mpsc::Receiver<Event>,
    pub requests: mpsc::Sender<ClientRequest>,
    pub inbound: task::JoinHandle<()>,
    pub outbound: task::JoinHandle<()>,
}

impl Client {
    pub async fn connect<S>(capacity: usize, buffer: usize, socket: S) -> io::Result<Self>
    where
        S: net::ToSocketAddrs,
    {
        let (read, write) = net::TcpStream::connect(socket).await?.into_split();

        let (responses_tx, responses) = mpsc::channel(capacity);
        let (events_tx, events) = mpsc::channel(capacity);
        let (requests, mut requests_rx) = mpsc::channel(capacity);

        let inbound = tokio::spawn(async move {
            let mut buf = BytesMut::with_capacity(buffer);

            while read.readable().await.is_ok() {
                let n = match read.try_read_buf(&mut buf) {
                    Ok(n) => n,
                    Err(_) => continue,
                };

                let mut bytes = &buf.as_ref()[..n];

                while !bytes.is_empty() {
                    let message = match ProtocolMessage::try_from_bytes(bytes) {
                        Ok((n, message)) => {
                            bytes = &bytes[n..];
                            message
                        }
                        Err(e) => {
                            tracing::warn!("invalid message received: {}", e);
                            continue;
                        }
                    };

                    match message {
                        ProtocolMessage::Request(_) => {
                            tracing::warn!("unexpected request from backend");
                        }

                        ProtocolMessage::Response(re) => {
                            let seq = re.request_seq;

                            match Response::try_from(&re) {
                                Ok(response) => {
                                    if let Err(e) =
                                        responses_tx.send(ClientResponse { seq, response }).await
                                    {
                                        tracing::error!("error submitting response: {}", e);
                                    }
                                }

                                Err(e) => {
                                    tracing::warn!("invalid response received: {}", e);
                                }
                            };
                        }

                        ProtocolMessage::Event(ev) => match Event::try_from(&ev) {
                            Ok(ev) => {
                                if let Err(e) = events_tx.send(ev).await {
                                    tracing::error!("error submitting event: {}", e);
                                }
                            }

                            Err(e) => {
                                tracing::warn!("invalid event received: {}", e);
                            }
                        },
                    }
                }

                buf.clear();
            }
        });

        let outbound = tokio::spawn(async move {
            let mut id = 0u64;

            while let Some(ClientRequest { seq, request }) = requests_rx.recv().await {
                if write.writable().await.is_err() {
                    break;
                }

                id = seq.unwrap_or_else(|| id.wrapping_add(1).min(1));

                let request = request.into_protocol(id);
                let message = ProtocolMessage::from(request).into_adapter_message();

                if let Err(e) = write.try_write(message.as_bytes()) {
                    tracing::error!("error sending request: {}", e);
                }
            }
        });

        Ok(Self {
            responses,
            events,
            requests,
            inbound,
            outbound,
        })
    }

    pub async fn request(&self, request: Request) -> Result<(), SendError<ClientRequest>> {
        self.requests
            .send(ClientRequest { seq: None, request })
            .await
    }

    pub async fn request_with_seq(
        &self,
        seq: u64,
        request: Request,
    ) -> Result<(), SendError<ClientRequest>> {
        self.requests
            .send(ClientRequest {
                seq: Some(seq),
                request,
            })
            .await
    }
}
