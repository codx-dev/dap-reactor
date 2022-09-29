use super::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GotoArguments {
    pub thread_id: u64,
    pub target_id: u64,
}

impl From<GotoArguments> for Value {
    fn from(args: GotoArguments) -> Self {
        let GotoArguments {
            thread_id,
            target_id,
        } = args;

        let thread_id = utils::attribute_u64("threadId", thread_id);
        let target_id = utils::attribute_u64("targetId", target_id);

        utils::finalize_object(thread_id.chain(target_id))
    }
}

impl TryFrom<&Map<String, Value>> for GotoArguments {
    type Error = Error;

    fn try_from(map: &Map<String, Value>) -> Result<Self, Self::Error> {
        let thread_id = utils::get_u64(map, "threadId")?;
        let target_id = utils::get_u64(map, "targetId")?;

        Ok(Self {
            thread_id,
            target_id,
        })
    }
}
