use super::*;

use crate::types::Scope;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ScopesArguments {
    pub frame_id: u64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ScopesResponse {
    pub scopes: Vec<Scope>,
}

impl From<ScopesArguments> for Value {
    fn from(args: ScopesArguments) -> Self {
        let ScopesArguments { frame_id } = args;

        let frame_id = utils::attribute_u64("frameId", frame_id);

        utils::finalize_object(frame_id)
    }
}

impl TryFrom<&Map<String, Value>> for ScopesArguments {
    type Error = Error;

    fn try_from(map: &Map<String, Value>) -> Result<Self, Self::Error> {
        let frame_id = utils::get_u64(map, "frameId")?;

        Ok(Self { frame_id })
    }
}

impl From<ScopesResponse> for Value {
    fn from(response: ScopesResponse) -> Self {
        let ScopesResponse { scopes } = response;

        let scopes = utils::attribute_array("scopes", scopes);

        utils::finalize_object(scopes)
    }
}

impl TryFrom<&Map<String, Value>> for ScopesResponse {
    type Error = Error;

    fn try_from(map: &Map<String, Value>) -> Result<Self, Self::Error> {
        let scopes = utils::get_array_of_object(map, "scopes")?;

        Ok(Self { scopes })
    }
}
