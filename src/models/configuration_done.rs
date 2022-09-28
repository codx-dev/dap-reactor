use super::*;

use std::iter::empty;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConfigurationDoneArguments {}

impl From<ConfigurationDoneArguments> for Value {
    fn from(_: ConfigurationDoneArguments) -> Self {
        utils::finalize_object(empty())
    }
}

impl From<&Map<String, Value>> for ConfigurationDoneArguments {
    fn from(_: &Map<String, Value>) -> Self {
        Self {}
    }
}
