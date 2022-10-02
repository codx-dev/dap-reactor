#[cfg(test)]
mod tests;

use crate::error::{Cause, Error};
use crate::models::*;
use crate::protocol::ProtocolResponse;
use crate::protocol::ProtocolResponseError;
use crate::utils;

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
    BreakpointLocations {
        body: Option<BreakpointLocationsResponse>,
    },
    ConfigurationDone,
    Continue {
        body: ContinueResponse,
    },
    Evaluate {
        body: EvaluateResponse,
    },
    ExceptionInfo {
        body: ExceptionInfoResponse,
    },
    Goto,
    Initialize {
        body: InitializeResponse,
    },
    Launch,
    LoadedSources {
        body: LoadedSourcesResponse,
    },
    Next,
    ReverseContinue,
    SetBreakpoints {
        body: SetBreakpointsResponse,
    },
    StepBack,
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

impl Response {
    pub fn into_protocol(self, seq: u64, request_seq: u64) -> ProtocolResponse {
        match self {
            Response::Error { command, error } => ProtocolResponse {
                seq,
                request_seq,
                command,
                result: Err(error),
            },
            Response::Attach => ProtocolResponse {
                seq,
                request_seq,
                command: "attach".to_string(),
                result: Ok(None),
            },
            Response::Restart => ProtocolResponse {
                seq,
                request_seq,
                command: "restart".to_string(),
                result: Ok(None),
            },
            Response::Disconnect => ProtocolResponse {
                seq,
                request_seq,
                command: "disconnect".to_string(),
                result: Ok(None),
            },
            Response::Terminate => ProtocolResponse {
                seq,
                request_seq,
                command: "terminate".to_string(),
                result: Ok(None),
            },
            Response::BreakpointLocations { body } => ProtocolResponse {
                seq,
                request_seq,
                command: "breakpointLocations".to_string(),
                result: Ok(Some(body.into())),
            },
            Response::ConfigurationDone => ProtocolResponse {
                seq,
                request_seq,
                command: "configurationDone".to_string(),
                result: Ok(None),
            },
            Response::Continue { body } => ProtocolResponse {
                seq,
                request_seq,
                command: "continue".to_string(),
                result: Ok(Some(body.into())),
            },
            Response::Evaluate { body } => ProtocolResponse {
                seq,
                request_seq,
                command: "evaluate".to_string(),
                result: Ok(Some(body.into())),
            },
            Response::ExceptionInfo { body } => ProtocolResponse {
                seq,
                request_seq,
                command: "exceptionInfo".to_string(),
                result: Ok(Some(body.into())),
            },
            Response::Goto => ProtocolResponse {
                seq,
                request_seq,
                command: "goto".to_string(),
                result: Ok(None),
            },
            Response::Initialize { body } => ProtocolResponse {
                seq,
                request_seq,
                command: "initialize".to_string(),
                result: Ok(Some(body.into())),
            },
            Response::Launch => ProtocolResponse {
                seq,
                request_seq,
                command: "launch".to_string(),
                result: Ok(None),
            },
            Response::LoadedSources { body } => ProtocolResponse {
                seq,
                request_seq,
                command: "loadedSources".to_string(),
                result: Ok(Some(body.into())),
            },
            Response::Next => ProtocolResponse {
                seq,
                request_seq,
                command: "next".to_string(),
                result: Ok(None),
            },
            Response::ReverseContinue => ProtocolResponse {
                seq,
                request_seq,
                command: "reverseContinue".to_string(),
                result: Ok(None),
            },
            Response::SetBreakpoints { body } => ProtocolResponse {
                seq,
                request_seq,
                command: "setBreakpoints".to_string(),
                result: Ok(Some(body.into())),
            },
            Response::StepBack => ProtocolResponse {
                seq,
                request_seq,
                command: "stepBack".to_string(),
                result: Ok(None),
            },
        }
    }
}

impl TryFrom<&ProtocolResponse> for Response {
    type Error = Error;

    fn try_from(re: &ProtocolResponse) -> Result<Self, Error> {
        let result = match &re.result {
            Ok(r) => r,
            Err(e) => {
                return Ok(Self::Error {
                    command: re.command.clone(),
                    error: e.clone(),
                });
            }
        };

        let result = result
            .as_ref()
            .ok_or(Error::new("result", Cause::IsMandatory))?
            .as_object()
            .ok_or(Error::new("result", Cause::MustBeObject))?;

        match re.command.as_str() {
            "attach" => Ok(Self::Attach),
            "restart" => Ok(Self::Restart),
            "disconnect" => Ok(Self::Disconnect),
            "terminate" => Ok(Self::Terminate),
            "breakpointLocations" => Ok(Self::BreakpointLocations {
                body: Some(BreakpointLocationsResponse::try_from(result)?),
            }),
            "configurationDone" => Ok(Self::ConfigurationDone),
            "continue" => Ok(Self::Continue {
                body: ContinueResponse::try_from(result)?,
            }),
            "evaluate" => Ok(Self::Evaluate {
                body: EvaluateResponse::try_from(result)?,
            }),
            "exceptionInfo" => Ok(Self::ExceptionInfo {
                body: ExceptionInfoResponse::try_from(result)?,
            }),
            "goto" => Ok(Self::Goto),
            "initialize" => Ok(Self::Initialize {
                body: InitializeResponse::try_from(result)?,
            }),
            "launch" => Ok(Self::Launch),
            "loadedSources" => Ok(Self::LoadedSources {
                body: LoadedSourcesResponse::try_from(result)?,
            }),
            "next" => Ok(Self::Next),
            "reverseContinue" => Ok(Self::ReverseContinue),
            "setBreakpoints" => Ok(Self::SetBreakpoints {
                body: SetBreakpointsResponse::try_from(result)?,
            }),
            "stepBack" => Ok(Self::StepBack),
            _ => Err(Error::new("response", Cause::ExpectsEnum)),
        }
    }
}

impl ReverseResponse {
    pub fn into_protocol(self, seq: u64, request_seq: u64) -> ProtocolResponse {
        match self {
            ReverseResponse::Error { command, error } => ProtocolResponse {
                seq,
                request_seq,
                command,
                result: Err(error),
            },
            ReverseResponse::RunInTerminal {
                process_id,
                shell_process_id,
            } => {
                let process_id = utils::attribute_u32_optional("processId", process_id);
                let shell_process_id =
                    utils::attribute_u32_optional("shellProcessId", shell_process_id);

                let result = utils::finalize_object(process_id.chain(shell_process_id));

                ProtocolResponse {
                    seq,
                    request_seq,
                    command: "runInTerminal".to_string(),
                    result: Ok(Some(result)),
                }
            }
        }
    }
}

impl TryFrom<&ProtocolResponse> for ReverseResponse {
    type Error = Error;

    fn try_from(re: &ProtocolResponse) -> Result<Self, Error> {
        let result = match &re.result {
            Ok(r) => r,
            Err(e) => {
                return Ok(Self::Error {
                    command: re.command.clone(),
                    error: e.clone(),
                });
            }
        };

        match re.command.as_str() {
            "runInTerminal" => {
                let map = result
                    .as_ref()
                    .ok_or(Error::new("result", Cause::IsMandatory))?
                    .as_object()
                    .ok_or(Error::new("result", Cause::MustBeObject))?;

                let process_id = utils::get_u32_optional(map, "processId")?;
                let shell_process_id = utils::get_u32_optional(map, "shellProcessId")?;

                Ok(Self::RunInTerminal {
                    process_id,
                    shell_process_id,
                })
            }

            _ => Err(Error::new("reverseResponse", Cause::ExpectsEnum)),
        }
    }
}
