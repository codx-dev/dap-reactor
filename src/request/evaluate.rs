use super::*;

use crate::prelude::ValueFormat;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EvaluateArguments {
    pub expression: String,
    pub frame_id: Option<u64>,
    pub context: Option<String>,
    pub format: Option<ValueFormat>,
}

impl From<EvaluateArguments> for Value {
    fn from(args: EvaluateArguments) -> Self {
        let EvaluateArguments {
            expression,
            frame_id,
            context,
            format,
        } = args;

        let expression = utils::attribute_string("expression", expression);
        let frame_id = utils::attribute_u64_optional("frameId", frame_id);
        let context = utils::attribute_string_optional("context", context);
        let format = utils::attribute_optional("format", format);

        utils::finalize_object(expression.chain(frame_id).chain(context).chain(format))
    }
}

impl TryFrom<&Map<String, Value>> for EvaluateArguments {
    type Error = Error;

    fn try_from(map: &Map<String, Value>) -> Result<Self, Self::Error> {
        let expression = utils::get_string(map, "expression")?;
        let frame_id = utils::get_u64_optional(map, "frameId")?;
        let context = utils::get_string_optional(map, "context")?;
        let format = utils::get_object_optional(map, "format")?;

        Ok(Self {
            expression,
            frame_id,
            context,
            format,
        })
    }
}
