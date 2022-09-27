use super::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StepBackArguments {
    pub thread_id: u64,
    pub single_thread: bool,
    pub granularity: Option<SteppingGranularity>,
}

impl From<StepBackArguments> for Value {
    fn from(args: StepBackArguments) -> Self {
        let StepBackArguments {
            thread_id,
            single_thread,
            granularity,
        } = args;

        let thread_id = utils::attribute_u64("threadId", thread_id);
        let single_thread = utils::attribute_bool_optional("singleThread", single_thread);
        let granularity = utils::attribute_string_optional("granularity", granularity);

        utils::finalize_object(thread_id.chain(single_thread).chain(granularity))
    }
}

impl TryFrom<&Map<String, Value>> for StepBackArguments {
    type Error = Error;

    fn try_from(map: &Map<String, Value>) -> Result<Self, Self::Error> {
        let thread_id = utils::get_u64(map, "threadId")?;
        let single_thread = utils::get_bool_optional(map, "singleThread")?;
        let granularity = utils::get_string_optional(map, "granularity")?
            .map(SteppingGranularity::try_from)
            .transpose()?;

        Ok(Self {
            thread_id,
            single_thread,
            granularity,
        })
    }
}
