mod impls;

#[cfg(test)]
mod tests;

use crate::protocol::ProtocolResponseError;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Response {
    Error {
        command: String,
        error: ProtocolResponseError,
    },
    Attach,
    Restart,
    Disconnect,
    Terminate,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ReverseResponse {
    Error {
        command: String,
        error: ProtocolResponseError,
    },
    RunInTerminal {
        process_id: Option<u32>,
        shell_process_id: Option<u32>,
    },
}
