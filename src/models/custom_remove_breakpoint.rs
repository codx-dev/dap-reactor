use super::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CustomRemoveBreakpointArguments {
    pub id: u64,
}

impl From<CustomRemoveBreakpointArguments> for Value {
    fn from(args: CustomRemoveBreakpointArguments) -> Self {
        let CustomRemoveBreakpointArguments { id } = args;

        let id = utils::attribute_u64("id", id);

        utils::finalize_object(id)
    }
}

impl TryFrom<&Map<String, Value>> for CustomRemoveBreakpointArguments {
    type Error = Error;

    fn try_from(map: &Map<String, Value>) -> Result<Self, Self::Error> {
        let id = utils::get_u64(map, "id")?;

        Ok(Self { id })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CustomRemoveBreakpointResponse {
    pub id: u64,
    pub removed: bool,
}

impl From<CustomRemoveBreakpointResponse> for Value {
    fn from(args: CustomRemoveBreakpointResponse) -> Self {
        let CustomRemoveBreakpointResponse { id, removed } = args;

        let id = utils::attribute_u64("id", id);
        let removed = utils::attribute_bool("removed", removed);

        utils::finalize_object(id.chain(removed))
    }
}

impl TryFrom<&Map<String, Value>> for CustomRemoveBreakpointResponse {
    type Error = Error;

    fn try_from(map: &Map<String, Value>) -> Result<Self, Self::Error> {
        let id = utils::get_u64(map, "id")?;
        let removed = utils::get_bool(map, "removed")?;

        Ok(Self { id, removed })
    }
}
