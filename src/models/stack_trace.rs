use super::*;

use crate::models::{StackFrame, StackFrameFormat};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StackTraceArguments {
    pub thread_id: u64,
    pub start_frame: Option<u64>,
    pub levels: Option<u64>,
    pub format: Option<StackFrameFormat>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StackTraceResponse {
    pub stack_frames: Vec<StackFrame>,
    pub total_frames: Option<u64>,
}

impl From<StackTraceArguments> for Value {
    fn from(args: StackTraceArguments) -> Self {
        let StackTraceArguments {
            thread_id,
            start_frame,
            levels,
            format,
        } = args;

        let thread_id = utils::attribute_u64("threadId", thread_id);
        let start_frame = utils::attribute_u64_optional("startFrame", start_frame);
        let levels = utils::attribute_u64_optional("levels", levels);
        let format = utils::attribute_optional("format", format);

        utils::finalize_object(thread_id.chain(start_frame).chain(levels).chain(format))
    }
}

impl TryFrom<&Map<String, Value>> for StackTraceArguments {
    type Error = Error;

    fn try_from(map: &Map<String, Value>) -> Result<Self, Self::Error> {
        let thread_id = utils::get_u64(map, "threadId")?;
        let start_frame = utils::get_u64_optional(map, "startFrame")?;
        let levels = utils::get_u64_optional(map, "levels")?;
        let format = utils::get_object_optional(map, "format")?;

        Ok(Self {
            thread_id,
            start_frame,
            levels,
            format,
        })
    }
}

impl From<StackTraceResponse> for Value {
    fn from(response: StackTraceResponse) -> Self {
        let StackTraceResponse {
            stack_frames,
            total_frames,
        } = response;

        let stack_frames = utils::attribute_array("stackFrames", stack_frames);
        let total_frames = utils::attribute_u64_optional("totalFrames", total_frames);

        utils::finalize_object(stack_frames.chain(total_frames))
    }
}

impl TryFrom<&Map<String, Value>> for StackTraceResponse {
    type Error = Error;

    fn try_from(map: &Map<String, Value>) -> Result<Self, Self::Error> {
        let stack_frames = utils::get_array_of_object(map, "stackFrames")?;
        let total_frames = utils::get_u64_optional(map, "totalFrames")?;

        Ok(Self {
            stack_frames,
            total_frames,
        })
    }
}
