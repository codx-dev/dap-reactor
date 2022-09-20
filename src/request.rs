/// Every request has its module so the impls are grouped
mod attach;
mod breakpoint_locations;
mod configuration_done;
mod r#continue;
mod disconnect;
mod evaluate;
mod launch;
mod restart;
mod terminate;

#[cfg(test)]
mod tests;

use self::attach::AttachArguments;
use self::breakpoint_locations::BreakpointLocationsArguments;
use self::configuration_done::ConfigurationDoneArguments;
use self::disconnect::DisconnectArguments;
use self::launch::LaunchArguments;
use self::r#continue::ContinueArguments;
use self::restart::RestartArguments;
use self::terminate::TerminateArguments;

use crate::error::{Cause, Error};
use crate::protocol::ProtocolRequest;
use crate::utils;

use crate::request::evaluate::EvaluateArguments;
use serde_json::{Map, Value};
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Request {
    Attach {
        arguments: AttachArguments,
    },
    Restart {
        arguments: Option<RestartArguments>,
    },
    Disconnect {
        arguments: Option<DisconnectArguments>,
    },
    Terminate {
        arguments: Option<TerminateArguments>,
    },
    BreakpointLocations {
        arguments: Option<BreakpointLocationsArguments>,
    },
    ConfigurationDone {
        arguments: Option<ConfigurationDoneArguments>,
    },
    Continue {
        arguments: ContinueArguments,
    },
    Evaluate {
        arguments: EvaluateArguments,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RunInTerminalKind {
    Integrated,
    External,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ReverseRequest {
    RunInTerminal {
        kind: Option<RunInTerminalKind>,
        title: Option<String>,
        cwd: String,
        args: Vec<String>,
        env: Option<HashMap<String, Option<String>>>,
        args_can_be_interpreted_by_shell: bool,
    },
}

impl Request {
    pub fn into_protocol(self, seq: u64) -> ProtocolRequest {
        let (command, arguments) = match self {
            Request::Attach { arguments } => {
                let command = "attach";
                let arguments = Some(Value::from(arguments));

                (command, arguments)
            }

            Request::Restart { arguments } => {
                let command = "restart";
                let arguments = arguments.map(Value::from);

                (command, arguments)
            }

            Request::Disconnect { arguments } => {
                let command = "disconnect";
                let arguments = arguments.map(Value::from);

                (command, arguments)
            }

            Request::Terminate { arguments } => {
                let command = "terminate";
                let arguments = arguments.map(Value::from);

                (command, arguments)
            }

            Request::BreakpointLocations { arguments } => {
                let command = "breakpointLocations";
                let arguments = arguments.map(Value::from);

                (command, arguments)
            }

            Request::ConfigurationDone { arguments } => {
                let command = "configurationDone";
                let arguments = arguments.map(Value::from);

                (command, arguments)
            }

            Request::Continue { arguments } => {
                let command = "continue";
                let arguments = arguments.into();

                (command, Some(arguments))
            }

            Request::Evaluate { arguments } => {
                let command = "evaluate";
                let arguments = arguments.into();

                (command, Some(arguments))
            }
        };

        ProtocolRequest {
            seq,
            command: command.to_string(),
            arguments,
        }
    }
}

impl TryFrom<&ProtocolRequest> for Request {
    type Error = Error;

    fn try_from(rq: &ProtocolRequest) -> Result<Self, Self::Error> {
        let arguments = rq.arguments.as_ref().and_then(|b| b.as_object());

        match rq.command.as_str() {
            "attach" => {
                let arguments =
                    arguments.ok_or_else(|| Error::new("arguments", Cause::IsMandatory))?;

                let arguments = AttachArguments::from(arguments);

                Ok(Self::Attach { arguments })
            }

            "restart" => {
                let arguments = arguments.map(RestartArguments::try_from).transpose()?;

                Ok(Self::Restart { arguments })
            }

            "disconnect" => {
                let arguments = arguments.map(DisconnectArguments::try_from).transpose()?;

                Ok(Self::Disconnect { arguments })
            }

            "terminate" => {
                let arguments = arguments.map(TerminateArguments::try_from).transpose()?;

                Ok(Self::Terminate { arguments })
            }

            "breakpointLocations" => {
                let arguments = arguments
                    .map(BreakpointLocationsArguments::try_from)
                    .transpose()?;

                Ok(Self::BreakpointLocations { arguments })
            }

            _ => Err(Error::new("request", Cause::ExpectsEnum)),
        }
    }
}

impl From<RunInTerminalKind> for String {
    fn from(k: RunInTerminalKind) -> Self {
        match k {
            RunInTerminalKind::Integrated => "integrated".to_string(),
            RunInTerminalKind::External => "external".to_string(),
        }
    }
}

impl TryFrom<&str> for RunInTerminalKind {
    type Error = Error;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        match s {
            "integrated" => Ok(Self::Integrated),
            "external" => Ok(Self::External),
            _ => Err(Error::new("kind", Cause::ExpectsEnum)),
        }
    }
}

impl ReverseRequest {
    pub fn into_protocol(self, seq: u64) -> ProtocolRequest {
        let (command, arguments) = match self {
            ReverseRequest::RunInTerminal {
                kind,
                title,
                cwd,
                args,
                env,
                args_can_be_interpreted_by_shell,
            } => {
                let command = "runInTerminal";

                let kind = utils::attribute_string_optional("kind", kind);
                let title = utils::attribute_string_optional("title", title);
                let cwd = utils::attribute_string("cwd", cwd);
                let args = utils::attribute_array("args", args);
                let env = utils::attribute_map_optional("env", env);
                let args_can_be_interpreted_by_shell = utils::attribute_bool_optional(
                    "argsCanBeInterpretedByShell",
                    args_can_be_interpreted_by_shell,
                );

                let arguments = utils::finalize_object(
                    kind.chain(title)
                        .chain(cwd)
                        .chain(args)
                        .chain(env)
                        .chain(args_can_be_interpreted_by_shell),
                );

                (command, Some(arguments))
            }
        };

        ProtocolRequest {
            seq,
            command: command.to_string(),
            arguments,
        }
    }
}

impl TryFrom<&ProtocolRequest> for ReverseRequest {
    type Error = Error;

    fn try_from(rq: &ProtocolRequest) -> Result<Self, Self::Error> {
        let arguments = rq.arguments.as_ref().and_then(|b| b.as_object());

        match rq.command.as_str() {
            "runInTerminal" => {
                let map = arguments.ok_or(Error::new("arguments", Cause::IsMandatory))?;

                let kind = utils::get_str_optional(map, "kind")?
                    .map(RunInTerminalKind::try_from)
                    .transpose()?;

                let title = utils::get_string_optional(map, "title")?;
                let cwd = utils::get_string(map, "cwd")?;
                let args = utils::get_array_of_string(map, "args")?;
                let env = utils::get_map_to_string_or_null_optional(map, "env")?;
                let args_can_be_interpreted_by_shell =
                    utils::get_bool_optional(map, "argsCanBeInterpretedByShell")?;

                Ok(Self::RunInTerminal {
                    kind,
                    title,
                    cwd,
                    args,
                    env,
                    args_can_be_interpreted_by_shell,
                })
            }

            _ => Err(Error::new("reverseRequest", Cause::ExpectsEnum)),
        }
    }
}
