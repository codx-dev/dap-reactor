use super::*;

use crate::prelude::Source;
use std::iter::empty;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LoadedSourcesArguments {}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LoadedSourcesResponse {
    pub sources: Vec<Source>,
}

impl From<LoadedSourcesArguments> for Value {
    fn from(_: LoadedSourcesArguments) -> Self {
        utils::finalize_object(empty())
    }
}

impl From<&Map<String, Value>> for LoadedSourcesArguments {
    fn from(_: &Map<String, Value>) -> Self {
        Self {}
    }
}

impl From<LoadedSourcesResponse> for Value {
    fn from(args: LoadedSourcesResponse) -> Self {
        let LoadedSourcesResponse { sources } = args;

        let sources = utils::attribute_optional("sources", Some(sources));

        utils::finalize_object(sources)
    }
}

impl TryFrom<&Map<String, Value>> for LoadedSourcesResponse {
    type Error = Error;

    fn try_from(map: &Map<String, Value>) -> Result<Self, Self::Error> {
        let sources = utils::get_array_optional(map, "sources")?;

        Ok(Self { sources })
    }
}
