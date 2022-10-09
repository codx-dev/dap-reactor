use serde_json::Value;

use crate::types::{Breakpoint, Capabilities, Source};

mod impls;

#[cfg(test)]
mod tests;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BreakpointReason {
    Changed,
    New,
    Removed,
    Custom(String),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StoppedReason {
    Step,
    Breakpoint,
    Exception,
    Pause,
    Entry,
    Goto,
    FunctionBreakpoint,
    DataBreakpoint,
    InstructionBreakpoint,
    Custom(String),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ThreadReason {
    Started,
    Exited,
    Custom(String),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum OutputCategory {
    Console,
    Important,
    Stdout,
    Stderr,
    Telemetry,
    Custom(String),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OutputGroup {
    Start,
    StartCollapsed,
    End,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LoadedSourceReason {
    New,
    Changed,
    Removed,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProcessStartMethod {
    Launch,
    Attach,
    AttachForSuspendedLaunch,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Event {
    Breakpoint {
        reason: BreakpointReason,
        breakpoint: Breakpoint,
    },
    Capabilities {
        capabilities: Capabilities,
    },
    Continued {
        thread_id: u64,
        all_threads_continued: bool,
    },
    Exited {
        exit_code: u64,
    },
    Initialized,
    LoadedSource {
        reason: LoadedSourceReason,
        source: Source,
    },
    Output {
        category: Option<OutputCategory>,
        output: String,
        group: Option<OutputGroup>,
        variables_reference: Option<u32>,
        source: Option<Source>,
        line: Option<u64>,
        column: Option<u64>,
        data: Option<Value>,
    },
    Process {
        name: String,
        system_process_id: Option<u64>,
        is_local_process: bool,
        start_method: Option<ProcessStartMethod>,
        pointer_size: Option<u64>,
    },
    Stopped {
        reason: StoppedReason,
        description: Option<String>,
        thread_id: Option<u64>,
        preserve_focus_hint: bool,
        text: Option<String>,
        all_threads_stopped: bool,
        hit_breakpoint_ids: Vec<usize>,
    },
    Terminated {
        restart: Option<Value>,
    },
    Thread {
        reason: ThreadReason,
        thread_id: u64,
    },
    Custom {
        body: Option<Value>,
    },
}
