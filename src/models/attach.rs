use super::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AttachArguments {
    pub restart: Option<Value>,
}

impl From<AttachArguments> for Value {
    fn from(args: AttachArguments) -> Self {
        let AttachArguments { restart } = args;

        let restart = utils::attribute_optional("__restart", restart);

        utils::finalize_object(restart)
    }
}

impl From<&Map<String, Value>> for AttachArguments {
    fn from(map: &Map<String, Value>) -> Self {
        let restart = utils::get_optional(map, "__restart");

        Self { restart }
    }
}
