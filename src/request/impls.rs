use serde_json::Map;

use super::*;
use crate::error::{Cause, Error};
use crate::protocol::ProtocolRequest;
use crate::utils;

impl From<LaunchArguments> for Value {
    fn from(args: LaunchArguments) -> Self {
        let LaunchArguments { no_debug, restart } = args;

        let no_debug = utils::attribute_bool_optional("noDebug", no_debug);
        let restart = utils::attribute_optional("__restart", restart);

        utils::finalize_object(no_debug.chain(restart))
    }
}

impl TryFrom<&Map<String, Value>> for LaunchArguments {
    type Error = Error;

    fn try_from(map: &Map<String, Value>) -> Result<Self, Self::Error> {
        let no_debug = utils::get_bool_optional(map, "noDebug")?;
        let restart = utils::get_optional(map, "__restart");

        Ok(Self { no_debug, restart })
    }
}

impl From<AttachArguments> for Value {
    fn from(args: AttachArguments) -> Self {
        let AttachArguments { restart } = args;

        let restart = utils::attribute_optional("__restart", restart);

        utils::finalize_object(restart)
    }
}

impl From<&Map<String, Value>> for AttachArguments {
    fn from(map: &Map<String, Value>) -> Self {
        let restart = utils::get_optional(map, "__restart");

        Self { restart }
    }
}

impl From<RestartArguments> for Value {
    fn from(args: RestartArguments) -> Self {
        match args {
            RestartArguments::Launch(l) => l.into(),
            RestartArguments::Attach(a) => a.into(),
        }
    }
}

impl TryFrom<&Map<String, Value>> for RestartArguments {
    type Error = Error;

    fn try_from(map: &Map<String, Value>) -> Result<Self, Self::Error> {
        let launch = map.contains_key("noDebug");

        if launch {
            LaunchArguments::try_from(map).map(Self::Launch)
        } else {
            Ok(Self::Attach(AttachArguments::from(map)))
        }
    }
}

impl From<DisconnectArguments> for Value {
    fn from(args: DisconnectArguments) -> Self {
        let DisconnectArguments {
            restart,
            terminate_debuggee,
            suspend_debuggee,
        } = args;

        let restart = utils::attribute_bool_optional("restart", restart);
        let terminate_debuggee =
            utils::attribute_bool_optional("terminateDebuggee", terminate_debuggee);
        let suspend_debuggee = utils::attribute_bool_optional("suspendDebuggee", suspend_debuggee);

        utils::finalize_object(restart.chain(terminate_debuggee).chain(suspend_debuggee))
    }
}

impl TryFrom<&Map<String, Value>> for DisconnectArguments {
    type Error = Error;

    fn try_from(map: &Map<String, Value>) -> Result<Self, Self::Error> {
        let restart = utils::get_bool_optional(map, "restart")?;
        let terminate_debuggee = utils::get_bool_optional(map, "terminateDebuggee")?;
        let suspend_debuggee = utils::get_bool_optional(map, "suspendDebuggee")?;

        Ok(Self {
            restart,
            terminate_debuggee,
            suspend_debuggee,
        })
    }
}

impl From<TerminateArguments> for Value {
    fn from(args: TerminateArguments) -> Self {
        let TerminateArguments { restart } = args;

        let restart = utils::attribute_bool_optional("restart", restart);

        utils::finalize_object(restart)
    }
}

impl TryFrom<&Map<String, Value>> for TerminateArguments {
    type Error = Error;

    fn try_from(map: &Map<String, Value>) -> Result<Self, Self::Error> {
        let restart = utils::get_bool_optional(map, "restart")?;

        Ok(Self { restart })
    }
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
