use super::*;

use crate::prelude::Breakpoint;
use crate::prelude::Source;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SetBreakpointsArguments {
    pub source: Source,
    pub breakpoints: Vec<SourceBreakpoint>,
    pub lines: Vec<u64>,
    pub source_modified: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SourceBreakpoint {
    pub line: u64,
    pub column: Option<u64>,
    pub condition: Option<String>,
    pub hit_condition: Option<String>,
    pub log_message: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SetBreakpointsResponse {
    pub breakpoints: Vec<Breakpoint>,
}

impl From<SetBreakpointsArguments> for Value {
    fn from(args: SetBreakpointsArguments) -> Self {
        let SetBreakpointsArguments {
            source,
            breakpoints,
            lines,
            source_modified,
        } = args;

        let source = utils::attribute("source", source);
        let breakpoints = utils::attribute_optional("breakpoints", Some(breakpoints));
        let lines = utils::attribute_optional("lines", Some(lines));
        let source_modified = utils::attribute_bool_optional("sourceModified", source_modified);

        utils::finalize_object(
            source
                .chain(breakpoints)
                .chain(lines)
                .chain(source_modified),
        )
    }
}

impl TryFrom<&Map<String, Value>> for SetBreakpointsArguments {
    type Error = Error;

    fn try_from(map: &Map<String, Value>) -> Result<Self, Self::Error> {
        let source = utils::get_object(map, "source")?;
        let breakpoints = utils::get_array_optional(map, "breakpoints")?;
        let lines = utils::get_array_of_u64_optional(map, "lines")?;
        let source_modified = utils::get_bool_optional(map, "sourceModified")?;

        Ok(Self {
            source,
            breakpoints,
            lines,
            source_modified,
        })
    }
}

impl From<SourceBreakpoint> for Value {
    fn from(args: SourceBreakpoint) -> Self {
        let SourceBreakpoint {
            line,
            column,
            condition,
            hit_condition,
            log_message,
        } = args;

        let line = utils::attribute_u64("line", line);
        let column = utils::attribute_u64_optional("column", column);
        let condition = utils::attribute_string_optional("condition", condition);
        let hit_condition = utils::attribute_string_optional("hitCondition", hit_condition);
        let log_message = utils::attribute_string_optional("logMessage", log_message);

        utils::finalize_object(
            line.chain(column)
                .chain(condition)
                .chain(hit_condition)
                .chain(log_message),
        )
    }
}

impl TryFrom<&Map<String, Value>> for SourceBreakpoint {
    type Error = Error;

    fn try_from(map: &Map<String, Value>) -> Result<Self, Self::Error> {
        let line = utils::get_u64(map, "line")?;
        let column = utils::get_u64_optional(map, "column")?;
        let condition = utils::get_string_optional(map, "condition")?;
        let hit_condition = utils::get_string_optional(map, "hitCondition")?;
        let log_message = utils::get_string_optional(map, "logMessage")?;

        Ok(Self {
            line,
            column,
            condition,
            hit_condition,
            log_message,
        })
    }
}

impl From<SetBreakpointsResponse> for Value {
    fn from(args: SetBreakpointsResponse) -> Self {
        let SetBreakpointsResponse { breakpoints } = args;

        let breakpoints = utils::attribute_array("breakpoints", breakpoints);

        utils::finalize_object(breakpoints)
    }
}

impl TryFrom<&Map<String, Value>> for SetBreakpointsResponse {
    type Error = Error;

    fn try_from(map: &Map<String, Value>) -> Result<Self, Self::Error> {
        let breakpoints = utils::get_array_optional(map, "breakpoints")?;

        Ok(Self { breakpoints })
    }
}
