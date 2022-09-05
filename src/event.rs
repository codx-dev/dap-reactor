use crate::types::{Breakpoint, Capabilities};

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
    Stopped {
        reason: StoppedReason,
        description: Option<String>,
        thread_id: Option<u64>,
        preserve_focus_hint: bool,
        text: Option<String>,
        all_threads_stopped: bool,
        hit_breakpoint_ids: Vec<usize>,
    },
}
