use crate::models::Breakpoint;

use super::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CustomAddBreakpointArguments {
    pub breakpoint: Breakpoint,
}

impl From<CustomAddBreakpointArguments> for Value {
    fn from(args: CustomAddBreakpointArguments) -> Self {
        let CustomAddBreakpointArguments { breakpoint } = args;

        let breakpoint = utils::attribute("breakpoint", breakpoint);

        utils::finalize_object(breakpoint)
    }
}

impl TryFrom<&Map<String, Value>> for CustomAddBreakpointArguments {
    type Error = Error;

    fn try_from(map: &Map<String, Value>) -> Result<Self, Self::Error> {
        let breakpoint = utils::get_object(map, "breakpoint")?;

        Ok(Self { breakpoint })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CustomAddBreakpointResponse {
    pub id: u64,
}

impl From<CustomAddBreakpointResponse> for Value {
    fn from(args: CustomAddBreakpointResponse) -> Self {
        let CustomAddBreakpointResponse { id } = args;

        let id = utils::attribute_u64("id", id);

        utils::finalize_object(id)
    }
}

impl TryFrom<&Map<String, Value>> for CustomAddBreakpointResponse {
    type Error = Error;

    fn try_from(map: &Map<String, Value>) -> Result<Self, Self::Error> {
        let id = utils::get_u64(map, "id")?;

        Ok(Self { id })
    }
}
