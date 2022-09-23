use super::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DisconnectArguments {
    pub restart: bool,
    pub terminate_debuggee: bool,
    pub suspend_debuggee: bool,
}

impl From<DisconnectArguments> for Value {
    fn from(args: DisconnectArguments) -> Self {
        let DisconnectArguments {
            restart,
            terminate_debuggee,
            suspend_debuggee,
        } = args;

        let restart = utils::attribute_bool_optional("restart", restart);
        let terminate_debuggee =
            utils::attribute_bool_optional("terminateDebuggee", terminate_debuggee);
        let suspend_debuggee = utils::attribute_bool_optional("suspendDebuggee", suspend_debuggee);

        utils::finalize_object(restart.chain(terminate_debuggee).chain(suspend_debuggee))
    }
}

impl TryFrom<&Map<String, Value>> for DisconnectArguments {
    type Error = Error;

    fn try_from(map: &Map<String, Value>) -> Result<Self, Self::Error> {
        let restart = utils::get_bool_optional(map, "restart")?;
        let terminate_debuggee = utils::get_bool_optional(map, "terminateDebuggee")?;
        let suspend_debuggee = utils::get_bool_optional(map, "suspendDebuggee")?;

        Ok(Self {
            restart,
            terminate_debuggee,
            suspend_debuggee,
        })
    }
}
