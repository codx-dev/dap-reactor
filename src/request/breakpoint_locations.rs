use super::*;

use crate::prelude::Source;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BreakpointLocationsArguments {
    pub source: Source,
    pub line: u64,
    pub column: Option<u64>,
    pub end_line: Option<u64>,
    pub end_column: Option<u64>,
}

impl From<BreakpointLocationsArguments> for Value {
    fn from(args: BreakpointLocationsArguments) -> Self {
        let BreakpointLocationsArguments {
            source,
            line,
            column,
            end_line,
            end_column,
        } = args;

        let source = utils::attribute("source", source);
        let line = utils::attribute_u64("line", line);
        let column = utils::attribute_u64_optional("column", column);
        let end_line = utils::attribute_u64_optional("endLine", end_line);
        let end_column = utils::attribute_u64_optional("endColumn", end_column);

        utils::finalize_object(
            source
                .chain(line)
                .chain(column)
                .chain(end_line)
                .chain(end_column),
        )
    }
}

impl TryFrom<&Map<String, Value>> for BreakpointLocationsArguments {
    type Error = Error;

    fn try_from(map: &Map<String, Value>) -> Result<Self, Self::Error> {
        let source = utils::get_object(map, "source")?;
        let line = utils::get_u64(map, "line")?;
        let column = utils::get_u64_optional(map, "column")?;
        let end_line = utils::get_u64_optional(map, "endLine")?;
        let end_column = utils::get_u64_optional(map, "endColumn")?;

        Ok(Self {
            source,
            line,
            column,
            end_line,
            end_column,
        })
    }
}
