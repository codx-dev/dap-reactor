use super::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LaunchArguments {
    pub no_debug: bool,
    pub restart: Option<Value>,
}

impl From<LaunchArguments> for Value {
    fn from(args: LaunchArguments) -> Self {
        let LaunchArguments { no_debug, restart } = args;

        let no_debug = utils::attribute_bool_optional("noDebug", no_debug);
        let restart = utils::attribute_optional("__restart", restart);

        utils::finalize_object(no_debug.chain(restart))
    }
}

impl TryFrom<&Map<String, Value>> for LaunchArguments {
    type Error = Error;

    fn try_from(map: &Map<String, Value>) -> Result<Self, Self::Error> {
        let no_debug = utils::get_bool_optional(map, "noDebug")?;
        let restart = utils::get_optional(map, "__restart");

        Ok(Self { no_debug, restart })
    }
}
