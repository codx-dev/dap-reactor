use super::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EvaluateArguments {
    pub expression: String,
    pub frame_id: Option<u64>,
    pub context: Option<Context>,
    pub format: Option<ValueFormat>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ValueFormat {
    pub hex: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Context {
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
    pub presentation_hint: VariablePresentationHint,
    pub variables_reference: u64,
    pub named_variables: Option<u64>,
    pub indexed_variables: Option<u64>,
    pub memory_reference: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VariablePresentationHint {
    pub kind: Kind,
    pub attributes: Vec<Attributes>,
    pub visibility: Option<Visibility>,
    pub lazy: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Kind {
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
pub enum Attributes {
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
pub enum Visibility {
    Public,
    Private,
    Protected,
    Internal,
    Final,
    Custom(String),
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
        let context = utils::get_string_optional(map, "context")?.map(Context::from);
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
        let presentation_hint = utils::get_object(map, "presentationHint")?;
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

impl TryFrom<&Map<String, Value>> for VariablePresentationHint {
    type Error = Error;

    fn try_from(map: &Map<String, Value>) -> Result<Self, Self::Error> {
        let kind = utils::get_string(map, "kind")?.into();
        let attributes = utils::get_array_of_string_optional(map, "attributes")?
            .iter()
            // FIXME: implement for &str too
            .map(|e| Attributes::from(e.to_owned()))
            .collect::<Vec<Attributes>>();

        let visibility = utils::get_string_optional(map, "visibility")?.map(Visibility::from);
        let lazy = utils::get_bool_optional(map, "lazy")?;

        Ok(Self {
            kind,
            attributes,
            visibility,
            lazy,
        })
    }
}

impl From<VariablePresentationHint> for Value {
    fn from(response: VariablePresentationHint) -> Self {
        let VariablePresentationHint {
            kind,
            attributes,
            visibility,
            lazy,
        } = response;

        let kind = utils::attribute_string("kind", kind);
        let attributes = utils::attribute_array_of_string_optional("attributes", Some(attributes));
        let visibility = utils::attribute_string_optional("visibility", visibility);
        let lazy = utils::attribute_bool_optional("lazy", lazy);

        utils::finalize_object(kind.chain(attributes).chain(visibility).chain(lazy))
    }
}

impl From<Context> for String {
    fn from(r: Context) -> Self {
        use self::Context::*;

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

impl From<String> for Context {
    fn from(s: String) -> Self {
        use self::Context::*;

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

impl From<Kind> for String {
    fn from(r: Kind) -> Self {
        use self::Kind::*;

        match r {
            Property => "property".into(),
            Method => "method".into(),
            Class => "class".into(),
            Data => "data".into(),
            Event => "event".into(),
            BaseClass => "baseClass".into(),
            InnerClass => "innerClass".into(),
            Interface => "interface".into(),
            MostDerivedClass => "mostDerivedClass".into(),
            Virtual => "virtual".into(),
            DataBreakpoint => "dataBreakpoint".into(),
            Custom(s) => s,
        }
    }
}

impl From<String> for Kind {
    fn from(s: String) -> Self {
        use self::Kind::*;

        match s.as_str() {
            "property" => Property,
            "method" => Method,
            "class" => Class,
            "data" => Data,
            "event" => Event,
            "baseClass" => BaseClass,
            "innerClass" => InnerClass,
            "interface" => Interface,
            "mostDerivedClass" => MostDerivedClass,
            "virtual" => Virtual,
            "dataBreakpoint" => DataBreakpoint,
            _ => Custom(s),
        }
    }
}

impl From<Attributes> for String {
    fn from(r: Attributes) -> Self {
        use self::Attributes::*;

        match r {
            Static => "static".into(),
            Constant => "constant".into(),
            ReadOnly => "readOnly".into(),
            RawString => "rawString".into(),
            HasObjectId => "hasObjectId".into(),
            CanHaveObjectId => "canHaveObjectId".into(),
            HasSideEffects => "hasSideEffects".into(),
            HasDataBreakpoint => "hasDataBreakpoint".into(),
            Custom(x) => x,
        }
    }
}

impl From<String> for Attributes {
    fn from(s: String) -> Self {
        use self::Attributes::*;

        match s.as_str() {
            "static" => Static,
            "constant" => Constant,
            "readOnly" => ReadOnly,
            "rawString" => RawString,
            "hasObjectId" => HasObjectId,
            "canHaveObjectId" => CanHaveObjectId,
            "hasSideEffects" => HasSideEffects,
            "hasDataBreakpoint" => HasDataBreakpoint,
            _ => Custom(s),
        }
    }
}

impl From<Visibility> for String {
    fn from(r: Visibility) -> Self {
        use self::Visibility::*;

        match r {
            Public => "public".into(),
            Private => "private".into(),
            Protected => "protected".into(),
            Internal => "internal".into(),
            Final => "final".into(),
            Custom(x) => x,
        }
    }
}

impl From<String> for Visibility {
    fn from(s: String) -> Self {
        use self::Visibility::*;

        match s.as_str() {
            "public" => Public,
            "private" => Private,
            "protected" => Protected,
            "internal" => Internal,
            "final" => Final,
            _ => Custom(s),
        }
    }
}

impl From<ValueFormat> for Value {
    fn from(format: ValueFormat) -> Self {
        let ValueFormat { hex } = format;

        let name = utils::attribute_bool_optional("hex", hex);

        utils::finalize_object(name)
    }
}

impl TryFrom<&Map<String, Value>> for ValueFormat {
    type Error = Error;

    fn try_from(map: &Map<String, Value>) -> Result<Self, Self::Error> {
        let hex = utils::get_bool_optional(map, "hex")?;

        Ok(Self { hex })
    }
}
