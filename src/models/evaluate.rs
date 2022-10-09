use super::*;

use crate::types::{ValueFormat, VariablePresentationHint};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EvaluateArguments {
    pub expression: String,
    pub frame_id: Option<u64>,
    pub context: Option<EvaluateContext>,
    pub format: Option<ValueFormat>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EvaluateContext {
    Variables,
    Watch,
    Repl,
    Hover,
    Clipboard,
    Custom(String),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EvaluateResponse {
    pub result: String,
    pub r#type: Option<String>,
    pub presentation_hint: Option<VariablePresentationHint>,
    pub variables_reference: u64,
    pub named_variables: Option<u64>,
    pub indexed_variables: Option<u64>,
    pub memory_reference: Option<String>,
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
        let context = utils::get_string_optional(map, "context")?.map(EvaluateContext::from);
        let format = utils::get_object_optional(map, "format")?;

        Ok(Self {
            expression,
            frame_id,
            context,
            format,
        })
    }
}

impl TryFrom<&Map<String, Value>> for EvaluateResponse {
    type Error = Error;

    fn try_from(map: &Map<String, Value>) -> Result<Self, Self::Error> {
        let result = utils::get_string(map, "result")?;
        let r#type = utils::get_string_optional(map, "type")?;
        let presentation_hint = utils::get_object_optional(map, "presentationHint")?;
        let variables_reference = utils::get_u64(map, "variablesReference")?;
        let named_variables = utils::get_u64_optional(map, "namedVariables")?;
        let indexed_variables = utils::get_u64_optional(map, "indexedVariables")?;
        let memory_reference = utils::get_string_optional(map, "memoryReference")?;

        Ok(Self {
            result,
            r#type,
            presentation_hint,
            variables_reference,
            named_variables,
            indexed_variables,
            memory_reference,
        })
    }
}

impl From<EvaluateResponse> for Value {
    fn from(response: EvaluateResponse) -> Self {
        let EvaluateResponse {
            result,
            r#type,
            presentation_hint,
            variables_reference,
            named_variables,
            indexed_variables,
            memory_reference,
        } = response;

        let result = utils::attribute_string("result", result);
        let r#type = utils::attribute_string_optional("type", r#type);
        let presentation_hint = utils::attribute("presentationHint", presentation_hint);
        let variables_reference = utils::attribute_u64("variablesReference", variables_reference);
        let named_variables = utils::attribute_u64_optional("namedVariables", named_variables);
        let indexed_variables =
            utils::attribute_u64_optional("indexedVariables", indexed_variables);
        let memory_reference =
            utils::attribute_string_optional("memoryReference", memory_reference);

        utils::finalize_object(
            result
                .chain(r#type)
                .chain(presentation_hint)
                .chain(variables_reference)
                .chain(named_variables)
                .chain(indexed_variables)
                .chain(memory_reference),
        )
    }
}

impl From<EvaluateContext> for String {
    fn from(r: EvaluateContext) -> Self {
        use self::EvaluateContext::*;

        match r {
            Variables => "variables".into(),
            Watch => "watch".into(),
            Repl => "repl".into(),
            Hover => "hover".into(),
            Clipboard => "clipboard".into(),
            Custom(s) => s,
        }
    }
}

impl From<String> for EvaluateContext {
    fn from(s: String) -> Self {
        use self::EvaluateContext::*;

        match s.as_str() {
            "variables" => Variables,
            "watch" => Watch,
            "repl" => Repl,
            "hover" => Hover,
            "clipboard" => Clipboard,
            _ => Custom(s),
        }
    }
}
