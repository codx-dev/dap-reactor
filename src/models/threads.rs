use crate::error::Error;

use serde_json::{Map, Value};

use crate::utils;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ThreadsResponse {
    pub threads: Vec<Thread>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Thread {
    pub id: u64,
    pub name: String,
}

impl From<ThreadsResponse> for Value {
    fn from(args: ThreadsResponse) -> Self {
        let ThreadsResponse { threads } = args;

        let threads = utils::attribute_array("threads", threads);

        utils::finalize_object(threads)
    }
}

impl TryFrom<&Map<String, Value>> for ThreadsResponse {
    type Error = Error;

    fn try_from(map: &Map<String, Value>) -> Result<Self, Self::Error> {
        let threads = utils::get_array_of_object(map, "threads")?;

        Ok(Self { threads })
    }
}

impl From<Thread> for Value {
    fn from(thread: Thread) -> Self {
        let Thread { id, name } = thread;

        let id = utils::attribute_u64("id", id);
        let name = utils::attribute_string("name", name);

        utils::finalize_object(id.chain(name))
    }
}

impl TryFrom<&Map<String, Value>> for Thread {
    type Error = Error;

    fn try_from(map: &Map<String, Value>) -> Result<Self, Self::Error> {
        let id = utils::get_u64(map, "id")?;
        let name = utils::get_string(map, "name")?;

        Ok(Self { id, name })
    }
}
