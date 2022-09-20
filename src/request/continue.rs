use super::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ContinueArguments {
    pub thread_id: u64,
    pub single_thread: bool,
}

impl From<ContinueArguments> for Value {
    fn from(args: ContinueArguments) -> Self {
        let ContinueArguments {
            thread_id,
            single_thread,
        } = args;

        let thread_id = utils::attribute_u64("threadId", thread_id);
        let single_thread = utils::attribute_bool_optional("singleThread", single_thread);

        utils::finalize_object(thread_id.chain(single_thread))
    }
}
impl TryFrom<&Map<String, Value>> for ContinueArguments {
    type Error = Error;

    fn try_from(map: &Map<String, Value>) -> Result<Self, Self::Error> {
        let thread_id = utils::get_u64(map, "threadId")?;
        let single_thread = utils::get_bool_optional(map, "singleThread")?;

        Ok(Self {
            thread_id,
            single_thread,
        })
    }
}
