use super::*;

use std::iter::empty;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LoadedSourcesArguments {}

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
