use super::*;

use crate::error::Cause;
use crate::types::{ValueFormat, Variable};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VariablesArgumentsFilter {
    Indexed,
    Named,
}

impl From<VariablesArgumentsFilter> for &'static str {
    fn from(filter: VariablesArgumentsFilter) -> Self {
        match filter {
            VariablesArgumentsFilter::Indexed => "indexed",
            VariablesArgumentsFilter::Named => "named",
        }
    }
}

impl From<VariablesArgumentsFilter> for String {
    fn from(filter: VariablesArgumentsFilter) -> Self {
        <&'static str>::from(filter).into()
    }
}

impl TryFrom<&str> for VariablesArgumentsFilter {
    type Error = Error;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        match s {
            "indexed" => Ok(VariablesArgumentsFilter::Indexed),
            "named" => Ok(VariablesArgumentsFilter::Named),
            _ => Err(Error::new("filter", Cause::ExpectsEnum)),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VariablesArguments {
    pub variables_reference: u64,
    pub filter: Option<VariablesArgumentsFilter>,
    pub start: Option<u64>,
    pub count: Option<u64>,
    pub format: Option<ValueFormat>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VariablesResponse {
    pub variables: Vec<Variable>,
}

impl From<VariablesArguments> for Value {
    fn from(args: VariablesArguments) -> Self {
        let VariablesArguments {
            variables_reference,
            filter,
            start,
            count,
            format,
        } = args;

        let variables_reference = utils::attribute_u64("variablesReference", variables_reference);
        let filter = utils::attribute_string_optional("filter", filter);
        let start = utils::attribute_u64_optional("start", start);
        let count = utils::attribute_u64_optional("count", count);
        let format = utils::attribute_optional("format", format);

        utils::finalize_object(
            variables_reference
                .chain(filter)
                .chain(start)
                .chain(count)
                .chain(format),
        )
    }
}

impl TryFrom<&Map<String, Value>> for VariablesArguments {
    type Error = Error;

    fn try_from(map: &Map<String, Value>) -> Result<Self, Self::Error> {
        let variables_reference = utils::get_u64(map, "variablesReference")?;
        let filter = utils::get_str_optional(map, "filter")?
            .map(VariablesArgumentsFilter::try_from)
            .transpose()?;
        let start = utils::get_u64_optional(map, "start")?;
        let count = utils::get_u64_optional(map, "count")?;
        let format = utils::get_object_optional(map, "format")?;

        Ok(Self {
            variables_reference,
            filter,
            start,
            count,
            format,
        })
    }
}

impl From<VariablesResponse> for Value {
    fn from(response: VariablesResponse) -> Self {
        let VariablesResponse { variables } = response;

        let variables = utils::attribute_array("variables", variables);

        utils::finalize_object(variables)
    }
}

impl TryFrom<&Map<String, Value>> for VariablesResponse {
    type Error = Error;

    fn try_from(map: &Map<String, Value>) -> Result<Self, Self::Error> {
        let variables = utils::get_array_of_object(map, "variables")?;

        Ok(Self { variables })
    }
}
