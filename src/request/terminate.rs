use super::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TerminateArguments {
    pub restart: bool,
}

impl From<TerminateArguments> for Value {
    fn from(args: TerminateArguments) -> Self {
        let TerminateArguments { restart } = args;

        let restart = utils::attribute_bool_optional("restart", restart);

        utils::finalize_object(restart)
    }
}

impl TryFrom<&Map<String, Value>> for TerminateArguments {
    type Error = Error;

    fn try_from(map: &Map<String, Value>) -> Result<Self, Self::Error> {
        let restart = utils::get_bool_optional(map, "restart")?;

        Ok(Self { restart })
    }
}
