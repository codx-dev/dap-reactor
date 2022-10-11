use super::*;

use crate::error::Cause;
use crate::models::ValueFormat;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VariablesArgumentsFilter {
    Indexed,
    Named,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum VariablePresentationHintKind {
    Property,
    Method,
    Class,
    Data,
    Event,
    BaseClass,
    InnerClass,
    Interface,
    MostDerivedClass,
    Virtual,
    DataBreakpoint,
    Custom(String),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum VariablePresentationHintAttribute {
    Static,
    Constant,
    ReadOnly,
    RawString,
    HasObjectId,
    CanHaveObjectId,
    HasSideEffects,
    HasDataBreakpoint,
    Custom(String),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum VariablePresentationHintVisibility {
    Public,
    Private,
    Protected,
    Internal,
    Final,
    Custom(String),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VariablePresentationHint {
    pub kind: Option<VariablePresentationHintKind>,
    pub attributes: Vec<VariablePresentationHintAttribute>,
    pub visibility: Option<VariablePresentationHintVisibility>,
    pub lazy: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Variable {
    pub name: String,
    pub value: String,
    pub r#type: Option<String>,
    pub presentation_hint: Option<VariablePresentationHint>,
    pub evaluate_name: Option<String>,
    pub variables_reference: u64,
    pub named_variables: Option<u64>,
    pub indexed_variables: Option<u64>,
    pub memory_reference: Option<String>,
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

impl From<VariablePresentationHintKind> for String {
    fn from(kind: VariablePresentationHintKind) -> Self {
        match kind {
            VariablePresentationHintKind::Property => "property".into(),
            VariablePresentationHintKind::Method => "method".into(),
            VariablePresentationHintKind::Class => "class".into(),
            VariablePresentationHintKind::Data => "data".into(),
            VariablePresentationHintKind::Event => "event".into(),
            VariablePresentationHintKind::BaseClass => "baseClass".into(),
            VariablePresentationHintKind::InnerClass => "innerClass".into(),
            VariablePresentationHintKind::Interface => "interface".into(),
            VariablePresentationHintKind::MostDerivedClass => "mostDerivedClass".into(),
            VariablePresentationHintKind::Virtual => "virtual".into(),
            VariablePresentationHintKind::DataBreakpoint => "dataBreakpoint".into(),
            VariablePresentationHintKind::Custom(s) => s,
        }
    }
}

impl From<&str> for VariablePresentationHintKind {
    fn from(s: &str) -> Self {
        match s {
            "property" => VariablePresentationHintKind::Property,
            "method" => VariablePresentationHintKind::Method,
            "class" => VariablePresentationHintKind::Class,
            "data" => VariablePresentationHintKind::Data,
            "event" => VariablePresentationHintKind::Event,
            "baseClass" => VariablePresentationHintKind::BaseClass,
            "innerClass" => VariablePresentationHintKind::InnerClass,
            "interface" => VariablePresentationHintKind::Interface,
            "mostDerivedClass" => VariablePresentationHintKind::MostDerivedClass,
            "virtual" => VariablePresentationHintKind::Virtual,
            "dataBreakpoint" => VariablePresentationHintKind::DataBreakpoint,
            _ => VariablePresentationHintKind::Custom(s.into()),
        }
    }
}

impl From<VariablePresentationHintAttribute> for String {
    fn from(attribute: VariablePresentationHintAttribute) -> Self {
        match attribute {
            VariablePresentationHintAttribute::Static => "static".into(),
            VariablePresentationHintAttribute::Constant => "constant".into(),
            VariablePresentationHintAttribute::ReadOnly => "readOnly".into(),
            VariablePresentationHintAttribute::RawString => "rawString".into(),
            VariablePresentationHintAttribute::HasObjectId => "hasObjectId".into(),
            VariablePresentationHintAttribute::CanHaveObjectId => "canHaveObjectId".into(),
            VariablePresentationHintAttribute::HasSideEffects => "hasSideEffects".into(),
            VariablePresentationHintAttribute::HasDataBreakpoint => "hasDataBreakpoint".into(),
            VariablePresentationHintAttribute::Custom(s) => s,
        }
    }
}

impl From<&str> for VariablePresentationHintAttribute {
    fn from(s: &str) -> Self {
        match s {
            "static" => VariablePresentationHintAttribute::Static,
            "constant" => VariablePresentationHintAttribute::Constant,
            "readOnly" => VariablePresentationHintAttribute::ReadOnly,
            "rawString" => VariablePresentationHintAttribute::RawString,
            "hasObjectId" => VariablePresentationHintAttribute::HasObjectId,
            "canHaveObjectId" => VariablePresentationHintAttribute::CanHaveObjectId,
            "hasSideEffects" => VariablePresentationHintAttribute::HasSideEffects,
            "hasDataBreakpoint" => VariablePresentationHintAttribute::HasDataBreakpoint,
            _ => VariablePresentationHintAttribute::Custom(s.into()),
        }
    }
}

impl From<VariablePresentationHintVisibility> for String {
    fn from(visibility: VariablePresentationHintVisibility) -> Self {
        match visibility {
            VariablePresentationHintVisibility::Public => "public".into(),
            VariablePresentationHintVisibility::Private => "private".into(),
            VariablePresentationHintVisibility::Protected => "protected".into(),
            VariablePresentationHintVisibility::Internal => "internal".into(),
            VariablePresentationHintVisibility::Final => "final".into(),
            VariablePresentationHintVisibility::Custom(s) => s,
        }
    }
}

impl From<&str> for VariablePresentationHintVisibility {
    fn from(s: &str) -> Self {
        match s {
            "public" => VariablePresentationHintVisibility::Public,
            "private" => VariablePresentationHintVisibility::Private,
            "protected" => VariablePresentationHintVisibility::Protected,
            "internal" => VariablePresentationHintVisibility::Internal,
            "final" => VariablePresentationHintVisibility::Final,
            _ => VariablePresentationHintVisibility::Custom(s.into()),
        }
    }
}

impl From<VariablePresentationHint> for Value {
    fn from(hint: VariablePresentationHint) -> Self {
        let VariablePresentationHint {
            kind,
            attributes,
            visibility,
            lazy,
        } = hint;

        let kind = utils::attribute_string_optional("kind", kind);
        let attributes = utils::attribute_array_of_string_optional("attributes", Some(attributes));
        let visibility = utils::attribute_string_optional("visibility", visibility);
        let lazy = utils::attribute_bool_optional("lazy", lazy);

        utils::finalize_object(kind.chain(attributes).chain(visibility).chain(lazy))
    }
}

impl TryFrom<&Map<String, Value>> for VariablePresentationHint {
    type Error = Error;

    fn try_from(map: &Map<String, Value>) -> Result<Self, Self::Error> {
        let kind = utils::get_str_optional(map, "kind")?.map(VariablePresentationHintKind::from);
        let attributes = utils::get_array_of_string_optional(map, "attributes")?
            .iter()
            .map(|s| s.as_str())
            .map(VariablePresentationHintAttribute::from)
            .collect();
        let visibility = utils::get_str_optional(map, "visibility")?
            .map(VariablePresentationHintVisibility::from);
        let lazy = utils::get_bool_optional(map, "lazy")?;

        Ok(Self {
            kind,
            attributes,
            visibility,
            lazy,
        })
    }
}

impl From<Variable> for Value {
    fn from(variable: Variable) -> Self {
        let Variable {
            name,
            value,
            r#type,
            presentation_hint,
            evaluate_name,
            variables_reference,
            named_variables,
            indexed_variables,
            memory_reference,
        } = variable;

        let name = utils::attribute_string("name", name);
        let value = utils::attribute_string("value", value);
        let r#type = utils::attribute_string_optional("type", r#type);
        let presentation_hint = utils::attribute_optional("presentationHint", presentation_hint);
        let evaluate_name = utils::attribute_string_optional("evaluateName", evaluate_name);
        let variables_reference = utils::attribute_u64("variablesReference", variables_reference);
        let named_variables = utils::attribute_u64_optional("namedVariables", named_variables);
        let indexed_variables =
            utils::attribute_u64_optional("indexedVariables", indexed_variables);
        let memory_reference =
            utils::attribute_string_optional("memoryReference", memory_reference);

        utils::finalize_object(
            name.chain(value)
                .chain(r#type)
                .chain(presentation_hint)
                .chain(evaluate_name)
                .chain(variables_reference)
                .chain(named_variables)
                .chain(indexed_variables)
                .chain(memory_reference),
        )
    }
}

impl TryFrom<&Map<String, Value>> for Variable {
    type Error = Error;

    fn try_from(map: &Map<String, Value>) -> Result<Self, Self::Error> {
        let name = utils::get_string(map, "name")?;
        let value = utils::get_string(map, "value")?;
        let r#type = utils::get_string_optional(map, "type")?;
        let presentation_hint = utils::get_object_optional(map, "presentationHint")?;
        let evaluate_name = utils::get_string_optional(map, "evaluateName")?;
        let variables_reference = utils::get_u64(map, "variablesReference")?;
        let named_variables = utils::get_u64_optional(map, "namedVariables")?;
        let indexed_variables = utils::get_u64_optional(map, "indexedVariables")?;
        let memory_reference = utils::get_string_optional(map, "memoryReference")?;

        Ok(Self {
            name,
            value,
            r#type,
            presentation_hint,
            evaluate_name,
            variables_reference,
            named_variables,
            indexed_variables,
            memory_reference,
        })
    }
}
