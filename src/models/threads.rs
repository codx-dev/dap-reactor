use crate::error::Error;

use serde_json::{Map, Value};

use crate::types::Thread;
use crate::utils;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ThreadsResponse {
    pub threads: Vec<Thread>,
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
