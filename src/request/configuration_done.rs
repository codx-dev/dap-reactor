use super::*;

use std::iter::empty;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConfigurationDoneArguments {}

impl From<ConfigurationDoneArguments> for Value {
    fn from(_: ConfigurationDoneArguments) -> Self {
        utils::finalize_object(empty())
    }
}

impl TryFrom<&Map<String, Value>> for ConfigurationDoneArguments {
    type Error = Error;

    fn try_from(_: &Map<String, Value>) -> Result<Self, Self::Error> {
        Ok(Self {})
    }
}
