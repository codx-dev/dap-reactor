use super::*;

use crate::error::Cause;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExceptionInfoArguments {
    pub thread_id: u64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExceptionInfoResponse {
    pub exception_id: String,
    pub description: Option<String>,
    pub break_mode: ExceptionBreakMode,
    pub details: Option<ExceptionDetails>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ExceptionBreakMode {
    Never,
    Always,
    Unhandled,
    UserUnhandled,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExceptionDetails {
    pub message: Option<String>,
    pub type_name: Option<String>,
    pub full_type_name: Option<String>,
    pub evaluate_name: Option<String>,
    pub stack_trace: Option<String>,
    pub inner_exception: Vec<ExceptionDetails>,
}

impl From<ExceptionInfoArguments> for Value {
    fn from(args: ExceptionInfoArguments) -> Self {
        let ExceptionInfoArguments { thread_id } = args;

        let thread_id = utils::attribute_u64("threadId", thread_id);

        utils::finalize_object(thread_id)
    }
}

impl TryFrom<&Map<String, Value>> for ExceptionInfoArguments {
    type Error = Error;

    fn try_from(map: &Map<String, Value>) -> Result<Self, Self::Error> {
        let thread_id = utils::get_u64(map, "threadId")?;

        Ok(Self { thread_id })
    }
}

impl TryFrom<&Map<String, Value>> for ExceptionInfoResponse {
    type Error = Error;

    fn try_from(map: &Map<String, Value>) -> Result<Self, Self::Error> {
        let exception_id = utils::get_string(map, "exceptionId")?;
        let description = utils::get_string_optional(map, "description")?;
        let break_mode = ExceptionBreakMode::try_from(utils::get_string(map, "breakMode")?)?;
        let details = utils::get_object_optional(map, "details")?;

        Ok(Self {
            exception_id,
            description,
            break_mode,
            details,
        })
    }
}

impl From<ExceptionInfoResponse> for Value {
    fn from(args: ExceptionInfoResponse) -> Self {
        let ExceptionInfoResponse {
            exception_id,
            description,
            break_mode,
            details,
        } = args;

        let exception_id = utils::attribute_string("exceptionId", exception_id);
        let description = utils::attribute_string_optional("description", description);
        let break_mode = utils::attribute_string("breakMode", break_mode);
        let details = utils::attribute_optional("details", details);

        utils::finalize_object(
            exception_id
                .chain(description)
                .chain(break_mode)
                .chain(details),
        )
    }
}

impl TryFrom<&Map<String, Value>> for ExceptionDetails {
    type Error = Error;

    fn try_from(map: &Map<String, Value>) -> Result<Self, Self::Error> {
        let message = utils::get_string_optional(map, "message")?;
        let type_name = utils::get_string_optional(map, "typeName")?;
        let full_type_name = utils::get_string_optional(map, "fullTypeName")?;
        let evaluate_name = utils::get_string_optional(map, "evaluateName")?;
        let stack_trace = utils::get_string_optional(map, "stackTrace")?;
        let inner_exception = utils::get_array_optional(map, "innerException")?;

        Ok(Self {
            message,
            type_name,
            full_type_name,
            evaluate_name,
            stack_trace,
            inner_exception,
        })
    }
}

impl From<ExceptionDetails> for Value {
    fn from(args: ExceptionDetails) -> Self {
        let ExceptionDetails {
            message,
            type_name,
            full_type_name,
            evaluate_name,
            stack_trace,
            inner_exception,
        } = args;

        let message = utils::attribute_string_optional("message", message);
        let type_name = utils::attribute_string_optional("typeName", type_name);
        let full_type_name = utils::attribute_string_optional("fullTypeName", full_type_name);
        let evaluate_name = utils::attribute_string_optional("evaluateName", evaluate_name);
        let stack_trace = utils::attribute_string_optional("stackTrace", stack_trace);
        let inner_exception = utils::attribute_optional("innerException", Some(inner_exception));

        utils::finalize_object(
            message
                .chain(type_name)
                .chain(full_type_name)
                .chain(evaluate_name)
                .chain(stack_trace)
                .chain(inner_exception),
        )
    }
}

impl From<ExceptionBreakMode> for String {
    fn from(r: ExceptionBreakMode) -> Self {
        use self::ExceptionBreakMode::*;

        match r {
            Never => "never".into(),
            Always => "always".into(),
            Unhandled => "unhandled".into(),
            UserUnhandled => "userUnhandled".into(),
        }
    }
}

impl TryFrom<String> for ExceptionBreakMode {
    type Error = Error;

    fn try_from(s: String) -> Result<Self, Self::Error> {
        use self::ExceptionBreakMode::*;

        Ok(match s.as_str() {
            "never" => Never,
            "always" => Always,
            "unhandled" => Unhandled,
            "userUnhandled" => UserUnhandled,
            _ => return Err(Error::new("ExceptionBreakMode", Cause::IsInvalid)),
        })
    }
}
