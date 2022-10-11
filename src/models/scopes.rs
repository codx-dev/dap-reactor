use super::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ScopesArguments {
    pub frame_id: u64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ScopesResponse {
    pub scopes: Vec<Scope>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ScopePresentationHint {
    Arguments,
    Locals,
    Registers,
    Custom(String),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Scope {
    pub name: String,
    pub presentation_hint: Option<ScopePresentationHint>,
    pub variables_reference: u64,
    pub named_variables: Option<u64>,
    pub indexed_variables: Option<u64>,
    pub expensive: bool,
    pub source: Option<Source>,
    pub line: Option<u64>,
    pub column: Option<u64>,
    pub end_line: Option<u64>,
    pub end_column: Option<u64>,
}

impl From<ScopesArguments> for Value {
    fn from(args: ScopesArguments) -> Self {
        let ScopesArguments { frame_id } = args;

        let frame_id = utils::attribute_u64("frameId", frame_id);

        utils::finalize_object(frame_id)
    }
}

impl TryFrom<&Map<String, Value>> for ScopesArguments {
    type Error = Error;

    fn try_from(map: &Map<String, Value>) -> Result<Self, Self::Error> {
        let frame_id = utils::get_u64(map, "frameId")?;

        Ok(Self { frame_id })
    }
}

impl From<ScopesResponse> for Value {
    fn from(response: ScopesResponse) -> Self {
        let ScopesResponse { scopes } = response;

        let scopes = utils::attribute_array("scopes", scopes);

        utils::finalize_object(scopes)
    }
}

impl TryFrom<&Map<String, Value>> for ScopesResponse {
    type Error = Error;

    fn try_from(map: &Map<String, Value>) -> Result<Self, Self::Error> {
        let scopes = utils::get_array_of_object(map, "scopes")?;

        Ok(Self { scopes })
    }
}

impl From<ScopePresentationHint> for String {
    fn from(hint: ScopePresentationHint) -> Self {
        match hint {
            ScopePresentationHint::Arguments => "arguments".into(),
            ScopePresentationHint::Locals => "locals".into(),
            ScopePresentationHint::Registers => "registers".into(),
            ScopePresentationHint::Custom(s) => s,
        }
    }
}

impl From<&str> for ScopePresentationHint {
    fn from(s: &str) -> Self {
        match s {
            "arguments" => ScopePresentationHint::Arguments,
            "locals" => ScopePresentationHint::Locals,
            "registers" => ScopePresentationHint::Registers,
            _ => ScopePresentationHint::Custom(s.into()),
        }
    }
}

impl From<Scope> for Value {
    fn from(scope: Scope) -> Self {
        let Scope {
            name,
            presentation_hint,
            variables_reference,
            named_variables,
            indexed_variables,
            expensive,
            source,
            line,
            column,
            end_line,
            end_column,
        } = scope;

        let name = utils::attribute_string("name", name);
        let presentation_hint =
            utils::attribute_string_optional("presentationHint", presentation_hint);
        let variables_reference = utils::attribute_u64("variablesReference", variables_reference);
        let named_variables = utils::attribute_u64_optional("namedVariables", named_variables);
        let indexed_variables =
            utils::attribute_u64_optional("indexedVariables", indexed_variables);
        let expensive = utils::attribute_bool("expensive", expensive);
        let source = utils::attribute_optional("source", source);
        let line = utils::attribute_u64_optional("line", line);
        let column = utils::attribute_u64_optional("column", column);
        let end_line = utils::attribute_u64_optional("endLine", end_line);
        let end_column = utils::attribute_u64_optional("endColumn", end_column);

        utils::finalize_object(
            name.chain(presentation_hint)
                .chain(variables_reference)
                .chain(named_variables)
                .chain(indexed_variables)
                .chain(expensive)
                .chain(source)
                .chain(line)
                .chain(column)
                .chain(end_line)
                .chain(end_column),
        )
    }
}

impl TryFrom<&Map<String, Value>> for Scope {
    type Error = Error;

    fn try_from(map: &Map<String, Value>) -> Result<Self, Self::Error> {
        let name = utils::get_string(map, "name")?;
        let presentation_hint =
            utils::get_str_optional(map, "presentationHint")?.map(ScopePresentationHint::from);
        let variables_reference = utils::get_u64(map, "variablesReference")?;
        let named_variables = utils::get_u64_optional(map, "namedVariables")?;
        let indexed_variables = utils::get_u64_optional(map, "indexedVariables")?;
        let expensive = utils::get_bool(map, "expensive")?;
        let source = utils::get_object_optional(map, "source")?;
        let line = utils::get_u64_optional(map, "line")?;
        let column = utils::get_u64_optional(map, "column")?;
        let end_line = utils::get_u64_optional(map, "endLine")?;
        let end_column = utils::get_u64_optional(map, "endColumn")?;

        Ok(Self {
            name,
            presentation_hint,
            variables_reference,
            named_variables,
            indexed_variables,
            expensive,
            source,
            line,
            column,
            end_line,
            end_column,
        })
    }
}
