mod impls;

#[cfg(test)]
mod tests;

use std::collections::HashMap;

use serde_json::Value;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LaunchArguments {
    pub no_debug: bool,
    pub restart: Option<Value>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AttachArguments {
    pub restart: Option<Value>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RestartArguments {
    Launch(LaunchArguments),
    Attach(AttachArguments),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DisconnectArguments {
    pub restart: bool,
    pub terminate_debuggee: bool,
    pub suspend_debuggee: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TerminateArguments {
    pub restart: bool,
}

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
