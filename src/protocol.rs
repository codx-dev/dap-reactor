mod impls;

use serde_json::Value;

use crate::types::Message;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProtocolRequest {
    pub seq: u64,
    pub command: String,
    pub arguments: Option<Value>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProtocolEvent {
    pub seq: u64,
    pub event: String,
    pub body: Option<Value>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProtocolResponseError {
    pub message: Option<String>,
    pub body: Option<Message>,
}

pub type ProtocolResponseResult = Result<Option<Value>, ProtocolResponseError>;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProtocolResponse {
    pub seq: u64,
    pub request_seq: u64,
    pub command: String,
    pub result: ProtocolResponseResult,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ProtocolMessage {
    Request(ProtocolRequest),
    Response(ProtocolResponse),
    Event(ProtocolEvent),
}

impl From<ProtocolRequest> for ProtocolMessage {
    fn from(r: ProtocolRequest) -> Self {
        Self::Request(r)
    }
}

impl From<ProtocolResponse> for ProtocolMessage {
    fn from(r: ProtocolResponse) -> Self {
        Self::Response(r)
    }
}

impl From<ProtocolEvent> for ProtocolMessage {
    fn from(ev: ProtocolEvent) -> Self {
        Self::Event(ev)
    }
}
