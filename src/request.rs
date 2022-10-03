#[cfg(test)]
mod tests;

use crate::error::{Cause, Error};
use crate::models::*;
use crate::protocol::ProtocolRequest;
use crate::utils;

use serde_json::Value;
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
    ExceptionInfo {
        arguments: ExceptionInfoArguments,
    },
    Goto {
        arguments: GotoArguments,
    },
    Initialize {
        arguments: InitializeArguments,
    },
    Launch {
        arguments: LaunchArguments,
    },
    LoadedSources {
        arguments: Option<LoadedSourcesArguments>,
    },
    Next {
        arguments: Option<NextArguments>,
    },
    ReverseContinue {
        arguments: ReverseContinueArguments,
    },
    SetBreakpoints {
        arguments: SetBreakpointsArguments,
    },
    StepBack {
        arguments: StepBackArguments,
    },

    CustomAddBreakpoint {
        arguments: CustomAddBreakpointArguments,
    },
    CustomRemoveBreakpoint {
        arguments: CustomRemoveBreakpointArguments,
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

            Request::ExceptionInfo { arguments } => {
                let command = "exceptionInfo";
                let arguments = arguments.into();

                (command, Some(arguments))
            }

            Request::Goto { arguments } => {
                let command = "goto";
                let arguments = arguments.into();

                (command, Some(arguments))
            }

            Request::Initialize { arguments } => {
                let command = "initialize";
                let arguments = arguments.into();

                (command, Some(arguments))
            }

            Request::Launch { arguments } => {
                let command = "launch";
                let arguments = arguments.into();

                (command, Some(arguments))
            }

            Request::LoadedSources { arguments } => {
                let command = "loadedSources";
                let arguments = arguments.into();

                (command, Some(arguments))
            }

            Request::Next { arguments } => {
                let command = "next";
                let arguments = arguments.into();

                (command, Some(arguments))
            }

            Request::ReverseContinue { arguments } => {
                let command = "reverseContinue";
                let arguments = arguments.into();

                (command, Some(arguments))
            }

            Request::SetBreakpoints { arguments } => {
                let command = "setBreakpoints";
                let arguments = arguments.into();

                (command, Some(arguments))
            }

            Request::StepBack { arguments } => {
                let command = "stepBack";
                let arguments = arguments.into();

                (command, Some(arguments))
            }

            Request::CustomAddBreakpoint { arguments } => {
                let command = "customAddBreakpoint";
                let arguments = arguments.into();

                (command, Some(arguments))
            }
            Request::CustomRemoveBreakpoint { arguments } => {
                let command = "customRemoveBreakpoint";
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

            "configurationDone" => {
                let arguments = arguments.map(ConfigurationDoneArguments::from);

                Ok(Self::ConfigurationDone { arguments })
            }

            "continue" => {
                let arguments =
                    arguments.ok_or_else(|| Error::new("arguments", Cause::IsMandatory))?;

                let arguments = ContinueArguments::try_from(arguments)?;

                Ok(Self::Continue { arguments })
            }

            "evaluate" => {
                let arguments =
                    arguments.ok_or_else(|| Error::new("arguments", Cause::IsMandatory))?;

                let arguments = EvaluateArguments::try_from(arguments)?;

                Ok(Self::Evaluate { arguments })
            }

            "exceptionInfo" => {
                let arguments =
                    arguments.ok_or_else(|| Error::new("arguments", Cause::IsMandatory))?;

                let arguments = ExceptionInfoArguments::try_from(arguments)?;

                Ok(Self::ExceptionInfo { arguments })
            }

            "goto" => {
                let arguments =
                    arguments.ok_or_else(|| Error::new("arguments", Cause::IsMandatory))?;

                let arguments = GotoArguments::try_from(arguments)?;

                Ok(Self::Goto { arguments })
            }

            "initialize" => {
                let arguments =
                    arguments.ok_or_else(|| Error::new("arguments", Cause::IsMandatory))?;

                let arguments = InitializeArguments::try_from(arguments)?;

                Ok(Self::Initialize { arguments })
            }

            "launch" => {
                let arguments =
                    arguments.ok_or_else(|| Error::new("arguments", Cause::IsMandatory))?;

                let arguments = LaunchArguments::try_from(arguments)?;

                Ok(Self::Launch { arguments })
            }

            "loadedSources" => {
                let arguments = arguments.map(LoadedSourcesArguments::from);

                Ok(Self::LoadedSources { arguments })
            }

            "next" => {
                let arguments = arguments.map(NextArguments::try_from).transpose()?;

                Ok(Self::Next { arguments })
            }

            "reverseContinue" => {
                let arguments =
                    arguments.ok_or_else(|| Error::new("arguments", Cause::IsMandatory))?;

                let arguments = ReverseContinueArguments::try_from(arguments)?;

                Ok(Self::ReverseContinue { arguments })
            }

            "setBreakpoints" => {
                let arguments =
                    arguments.ok_or_else(|| Error::new("arguments", Cause::IsMandatory))?;

                let arguments = SetBreakpointsArguments::try_from(arguments)?;

                Ok(Self::SetBreakpoints { arguments })
            }

            "stepBack" => {
                let arguments =
                    arguments.ok_or_else(|| Error::new("arguments", Cause::IsMandatory))?;

                let arguments = StepBackArguments::try_from(arguments)?;

                Ok(Self::StepBack { arguments })
            }

            "customAddBreakpoint" => {
                let arguments =
                    arguments.ok_or_else(|| Error::new("arguments", Cause::IsMandatory))?;

                let arguments = CustomAddBreakpointArguments::try_from(arguments)?;

                Ok(Self::CustomAddBreakpoint { arguments })
            }

            "customRemoveBreakpoint" => {
                let arguments =
                    arguments.ok_or_else(|| Error::new("arguments", Cause::IsMandatory))?;

                let arguments = CustomRemoveBreakpointArguments::try_from(arguments)?;

                Ok(Self::CustomRemoveBreakpoint { arguments })
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
