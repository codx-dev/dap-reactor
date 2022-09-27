use super::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InitializeArguments {
    pub client_id: Option<String>,
    pub client_name: Option<String>,
    pub adapter_id: String,
    pub locale: Option<String>,
    pub lines_start_at_1: bool,
    pub column_start_at_1: bool,
    pub path_format: Option<PathFormat>,
    pub supports_variable_type: bool,
    pub supports_variable_paging: bool,
    pub supports_run_in_terminal_request: bool,
    pub supports_memory_references: bool,
    pub supports_progress_reporting: bool,
    pub supports_invalidated_event: bool,
    pub supports_memory_event: bool,
    pub supports_args_can_be_interpreted_by_shell: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PathFormat {
    Path,
    Uri,
    Custom(String),
}

impl From<InitializeArguments> for Value {
    fn from(args: InitializeArguments) -> Self {
        let InitializeArguments {
            client_id,
            client_name,
            adapter_id,
            locale,
            lines_start_at_1,
            column_start_at_1,
            path_format,
            supports_variable_type,
            supports_variable_paging,
            supports_run_in_terminal_request,
            supports_memory_references,
            supports_progress_reporting,
            supports_invalidated_event,
            supports_memory_event,
            supports_args_can_be_interpreted_by_shell,
        } = args;

        let client_id = utils::attribute_string_optional("clientId", client_id);
        let client_name = utils::attribute_string_optional("clientName", client_name);
        let adapter_id = utils::attribute_string("adapterId", adapter_id);
        let locale = utils::attribute_string_optional("locale", locale);
        let lines_start_at_1 = utils::attribute_bool_optional("linesStartAt1", lines_start_at_1);
        let column_start_at_1 = utils::attribute_bool_optional("columnStartAt1", column_start_at_1);
        let path_format = utils::attribute_string_optional("pathFormat", path_format);
        let supports_variable_type =
            utils::attribute_bool_optional("supportsVariableType", supports_variable_type);
        let supports_variable_paging =
            utils::attribute_bool_optional("supportsVariablePaging", supports_variable_paging);
        let supports_run_in_terminal_request = utils::attribute_bool_optional(
            "supportsRunInTerminalRequest",
            supports_run_in_terminal_request,
        );
        let supports_memory_references =
            utils::attribute_bool_optional("supportsMemoryReferences", supports_memory_references);
        let support_progress_reporting = utils::attribute_bool_optional(
            "supportsProgressReporting",
            supports_progress_reporting,
        );
        let supports_invalidated_event =
            utils::attribute_bool_optional("supportsInvalidatedEvent", supports_invalidated_event);
        let supports_memory_event =
            utils::attribute_bool_optional("supportsMemoryEvent", supports_memory_event);
        let supports_args_can_be_interpreted_by_shell = utils::attribute_bool_optional(
            "supportsArgsCanBeInterpretedByShell",
            supports_args_can_be_interpreted_by_shell,
        );

        utils::finalize_object(client_id.chain(client_name).chain(adapter_id))
    }
}

impl TryFrom<&Map<String, Value>> for InitializeArguments {
    type Error = Error;

    fn try_from(map: &Map<String, Value>) -> Result<Self, Self::Error> {
        let client_id = utils::get_string_optional(map, "clientId")?;
        let client_name = utils::get_string_optional(map, "clientName")?;
        let adapter_id = utils::get_string(map, "adapterId")?;
        let locale = utils::get_string_optional(map, "locale")?;
        let lines_start_at_1 = utils::get_bool_optional(map, "linesStartAt1")?;
        let column_start_at_1 = utils::get_bool_optional(map, "columnStartAt1")?;
        let path_format = utils::get_string_optional(map, "pathFormat")?.map(PathFormat::from);
        let supports_variable_type = utils::get_bool_optional(map, "supportsVariableType")?;
        let supports_variable_paging = utils::get_bool_optional(map, "supportsVariablePaging")?;
        let supports_run_in_terminal_request =
            utils::get_bool_optional(map, "supportsRunInTerminalRequest")?;
        let supports_memory_references = utils::get_bool_optional(map, "supportsMemoryReferences")?;
        let supports_progress_reporting =
            utils::get_bool_optional(map, "supportsProgressReporting")?;
        let supports_invalidated_event = utils::get_bool_optional(map, "supportsInvalidatedEvent")?;
        let supports_memory_event = utils::get_bool_optional(map, "supportsMemoryEvent")?;
        let supports_args_can_be_interpreted_by_shell =
            utils::get_bool_optional(map, "supportsArgsCanBeInterpretedByShell")?;

        Ok(Self {
            client_id,
            client_name,
            adapter_id,
            locale,
            lines_start_at_1,
            column_start_at_1,
            path_format,
            supports_variable_type,
            supports_variable_paging,
            supports_run_in_terminal_request,
            supports_memory_references,
            supports_progress_reporting,
            supports_invalidated_event,
            supports_memory_event,
            supports_args_can_be_interpreted_by_shell,
        })
    }
}

impl From<PathFormat> for String {
    fn from(r: PathFormat) -> Self {
        use self::PathFormat::*;

        match r {
            Path => "path".into(),
            Uri => "uri".into(),
            Custom(s) => s,
        }
    }
}

impl From<String> for PathFormat {
    fn from(s: String) -> Self {
        use self::PathFormat::*;

        match s.as_str() {
            "path" => Path,
            "uri" => Uri,
            _ => Custom(s),
        }
    }
}
