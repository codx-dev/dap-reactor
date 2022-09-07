use serde_json::json;

use super::*;
use crate::error::{Cause, Error};
use crate::protocol::ProtocolEvent;
use crate::utils;

impl From<StoppedReason> for String {
    fn from(r: StoppedReason) -> Self {
        match r {
            StoppedReason::Step => "step".into(),
            StoppedReason::Breakpoint => "breakpoint".into(),
            StoppedReason::Exception => "exception".into(),
            StoppedReason::Pause => "pause".into(),
            StoppedReason::Entry => "entry".into(),
            StoppedReason::Goto => "goto".into(),
            StoppedReason::FunctionBreakpoint => "function breakpoint".into(),
            StoppedReason::DataBreakpoint => "data breakpoint".into(),
            StoppedReason::InstructionBreakpoint => "instruction breakpoint".into(),
            StoppedReason::Custom(s) => s,
        }
    }
}

impl From<String> for StoppedReason {
    fn from(s: String) -> Self {
        match s.as_str() {
            "step" => StoppedReason::Step,
            "breakpoint" => StoppedReason::Breakpoint,
            "exception" => StoppedReason::Exception,
            "pause" => StoppedReason::Pause,
            "entry" => StoppedReason::Entry,
            "goto" => StoppedReason::Goto,
            "function breakpoint" => StoppedReason::FunctionBreakpoint,
            "data breakpoint" => StoppedReason::DataBreakpoint,
            "instruction breakpoint" => StoppedReason::InstructionBreakpoint,
            _ => StoppedReason::Custom(s),
        }
    }
}

impl From<BreakpointReason> for String {
    fn from(r: BreakpointReason) -> Self {
        match r {
            BreakpointReason::Changed => "changed".into(),
            BreakpointReason::New => "new".into(),
            BreakpointReason::Removed => "removed".into(),
            BreakpointReason::Custom(s) => s,
        }
    }
}

impl From<String> for BreakpointReason {
    fn from(s: String) -> Self {
        match s.as_str() {
            "changed" => BreakpointReason::Changed,
            "new" => BreakpointReason::New,
            "removed" => BreakpointReason::Removed,
            _ => Self::Custom(s),
        }
    }
}

impl From<ThreadReason> for String {
    fn from(r: ThreadReason) -> Self {
        match r {
            ThreadReason::Started => "started".into(),
            ThreadReason::Exited => "exited".into(),
            ThreadReason::Custom(s) => s,
        }
    }
}

impl From<String> for ThreadReason {
    fn from(s: String) -> Self {
        match s.as_str() {
            "started" => ThreadReason::Started,
            "exited" => ThreadReason::Exited,
            _ => ThreadReason::Custom(s),
        }
    }
}

impl From<OutputCategory> for String {
    fn from(c: OutputCategory) -> Self {
        match c {
            OutputCategory::Console => "console".into(),
            OutputCategory::Important => "important".into(),
            OutputCategory::Stdout => "stdout".into(),
            OutputCategory::Stderr => "stderr".into(),
            OutputCategory::Telemetry => "telemetry".into(),
            OutputCategory::Custom(s) => s,
        }
    }
}

impl From<String> for OutputCategory {
    fn from(s: String) -> Self {
        match s.as_str() {
            "console" => OutputCategory::Console,
            "important" => OutputCategory::Important,
            "stdout" => OutputCategory::Stdout,
            "stderr" => OutputCategory::Stderr,
            "telemetry" => OutputCategory::Telemetry,
            _ => OutputCategory::Custom(s),
        }
    }
}

impl From<OutputGroup> for &'static str {
    fn from(g: OutputGroup) -> Self {
        match g {
            OutputGroup::Start => "start",
            OutputGroup::StartCollapsed => "startCollapsed",
            OutputGroup::End => "end",
        }
    }
}

impl From<OutputGroup> for String {
    fn from(g: OutputGroup) -> Self {
        <&'static str>::from(g).into()
    }
}

impl TryFrom<&str> for OutputGroup {
    type Error = Error;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        match s {
            "start" => Ok(OutputGroup::Start),
            "startCollapsed" => Ok(OutputGroup::StartCollapsed),
            "end" => Ok(OutputGroup::End),
            _ => Err(Error::new("group", Cause::ExpectsEnum)),
        }
    }
}

impl TryFrom<String> for OutputGroup {
    type Error = Error;

    fn try_from(s: String) -> Result<Self, Self::Error> {
        s.as_str().try_into()
    }
}

impl Event {
    pub fn into_protocol(self, seq: u64) -> ProtocolEvent {
        let (event, body) = match self {
            Event::Breakpoint { reason, breakpoint } => {
                let event = "breakpoint";

                let reason = utils::attribute_string("reason", reason);
                let breakpoint = utils::attribute("breakpoint", breakpoint);

                let body = utils::finalize_object(reason.chain(breakpoint));

                (event, Some(body))
            }

            Event::Capabilities { capabilities } => {
                let event = "capabilities";

                let capabilities = utils::attribute("capabilities", capabilities);

                let body = utils::finalize_object(capabilities);

                (event, Some(body))
            }

            Event::Continued {
                thread_id,
                all_threads_continued,
            } => {
                let event = "continued";

                let thread_id = utils::attribute_u64("threadId", thread_id);
                let all_threads_continued =
                    utils::attribute_bool_optional("allThreadsContinued", all_threads_continued);

                let body = utils::finalize_object(thread_id.chain(all_threads_continued));

                (event, Some(body))
            }

            Event::Exited { exit_code } => {
                let event = "exited";

                let exit_code = utils::attribute_u64("exitCode", exit_code);

                let body = utils::finalize_object(exit_code);

                (event, Some(body))
            }

            Event::Initialized => ("initialized", None),

            Event::Output {
                category,
                output,
                group,
                variables_reference,
                source,
                line,
                column,
                data,
            } => {
                let event = "output";

                let category = utils::attribute_string_optional("category", category);
                let output = utils::attribute_string("output", output);
                let group = utils::attribute_string_optional("group", group);
                let variables_reference = utils::attribute_u32_optional(
                    "variablesReference",
                    (variables_reference > 0).then_some(variables_reference),
                );
                let source = utils::attribute_optional("source", source);
                let line = utils::attribute_u64_optional("line", line);
                let column = utils::attribute_u64_optional("column", column);
                let data = utils::attribute_optional("data", data);

                let body = utils::finalize_object(
                    category
                        .chain(output)
                        .chain(group)
                        .chain(variables_reference)
                        .chain(source)
                        .chain(line)
                        .chain(column)
                        .chain(data),
                );

                (event, Some(body))
            }

            Event::Stopped {
                reason,
                description,
                thread_id,
                preserve_focus_hint,
                text,
                all_threads_stopped,
                hit_breakpoint_ids,
            } => {
                let event = "stopped";

                let reason = utils::attribute_string("reason", reason);
                let description = utils::attribute_string_optional("description", description);
                let thread_id = utils::attribute_u64_optional("threadId", thread_id);
                let preserve_focus_hint =
                    utils::attribute_bool_optional("preserveFocusHint", preserve_focus_hint);
                let text = utils::attribute_string_optional("text", text);
                let all_threads_stopped =
                    utils::attribute_bool_optional("allThreadsStopped", all_threads_stopped);
                let hit_breakpoint_ids =
                    utils::attribute_array_optional("hitBreakpointIds", hit_breakpoint_ids);

                let body = utils::finalize_object(
                    reason
                        .chain(description)
                        .chain(thread_id)
                        .chain(preserve_focus_hint)
                        .chain(text)
                        .chain(all_threads_stopped)
                        .chain(hit_breakpoint_ids),
                );

                (event, Some(body))
            }

            Event::Terminated { restart } => {
                let event = "terminated";

                let body = restart.map(|v| json!({ "restart": v }));

                (event, body)
            }

            Event::Thread { reason, thread_id } => {
                let event = "thread";

                let reason = utils::attribute_string("reason", reason);
                let thread_id = utils::attribute_u64("threadId", thread_id);

                let body = utils::finalize_object(reason.chain(thread_id));

                (event, Some(body))
            }
        };

        ProtocolEvent {
            seq,
            event: event.to_string(),
            body,
        }
    }
}

impl TryFrom<&ProtocolEvent> for Event {
    type Error = Error;

    fn try_from(ev: &ProtocolEvent) -> Result<Self, Self::Error> {
        let body = ev.body.as_ref().and_then(|b| b.as_object());

        match ev.event.as_str() {
            "breakpoint" => {
                let map = &body.ok_or(Error::new("body", Cause::IsMandatory))?;

                let reason = utils::get_string(map, "reason").map(BreakpointReason::from)?;
                let breakpoint = utils::get_object(map, "breakpoint")?;

                Ok(Self::Breakpoint { reason, breakpoint })
            }

            "capabilities" => {
                let map = &body.ok_or(Error::new("body", Cause::IsMandatory))?;

                let capabilities = utils::get_object(map, "capabilities")?;

                Ok(Self::Capabilities { capabilities })
            }

            "continued" => {
                let map = &body.ok_or(Error::new("body", Cause::IsMandatory))?;

                let thread_id = utils::get_u64(map, "threadId")?;
                let all_threads_continued = utils::get_bool_optional(map, "allThreadsContinued")?;

                Ok(Self::Continued {
                    thread_id,
                    all_threads_continued,
                })
            }

            "exited" => {
                let map = &body.ok_or(Error::new("body", Cause::IsMandatory))?;

                let exit_code = utils::get_u64(map, "exitCode")?;

                Ok(Self::Exited { exit_code })
            }

            "initialized" => Ok(Self::Initialized),

            "output" => {
                let map = &body.ok_or(Error::new("body", Cause::IsMandatory))?;

                let category =
                    utils::get_string_optional(map, "category")?.map(OutputCategory::from);
                let output = utils::get_string(map, "output")?;
                let group = utils::get_string_optional(map, "group")?
                    .map(OutputGroup::try_from)
                    .transpose()?;
                let variables_reference =
                    utils::get_u32_optional(map, "variablesReference")?.unwrap_or(0);
                let source = utils::get_object_optional(map, "source")?;
                let line = utils::get_u64_optional(map, "line")?;
                let column = utils::get_u64_optional(map, "column")?;
                let data = utils::get_optional(map, "data");

                Ok(Self::Output {
                    category,
                    output,
                    group,
                    variables_reference,
                    source,
                    line,
                    column,
                    data,
                })
            }

            "stopped" => {
                let map = &body.ok_or(Error::new("body", Cause::IsMandatory))?;

                let reason = utils::get_string(map, "reason").map(StoppedReason::from)?;
                let description = utils::get_string_optional(map, "description")?;
                let thread_id = utils::get_u64_optional(map, "threadId")?;
                let preserve_focus_hint = utils::get_bool_optional(map, "preserveFocusHint")?;
                let text = utils::get_string_optional(map, "text")?;
                let all_threads_stopped = utils::get_bool_optional(map, "allThreadsStopped")?;
                let hit_breakpoint_ids = utils::get_array_usize_optional(map, "hitBreakpointIds")?;

                Ok(Self::Stopped {
                    reason,
                    description,
                    thread_id,
                    preserve_focus_hint,
                    text,
                    all_threads_stopped,
                    hit_breakpoint_ids,
                })
            }

            "terminated" => {
                let restart = body.and_then(|m| utils::get_optional(m, "restart"));

                Ok(Self::Terminated { restart })
            }

            "thread" => {
                let map = &body.ok_or(Error::new("body", Cause::IsMandatory))?;

                let reason = utils::get_string(map, "reason").map(ThreadReason::from)?;
                let thread_id = utils::get_u64(map, "threadId")?;

                Ok(Self::Thread { reason, thread_id })
            }

            _ => Err(Error::new("event", Cause::ExpectsEnum)),
        }
    }
}
