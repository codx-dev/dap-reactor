mod attach;
mod breakpoint_locations;
mod configuration_done;
mod r#continue;
mod disconnect;
mod evaluate;
mod exception_info;
mod goto;
mod initialize;
mod launch;
mod loaded_sources;
mod next;
mod restart;
mod reverse_continue;
mod scopes;
mod set_breakpoints;
mod stack_trace;
mod step_back;
mod terminate;
mod threads;
mod variables;

pub use attach::*;
pub use breakpoint_locations::*;
pub use configuration_done::*;
pub use disconnect::*;
pub use evaluate::*;
pub use exception_info::*;
pub use goto::*;
pub use initialize::*;
pub use launch::*;
pub use loaded_sources::*;
pub use next::*;
pub use r#continue::*;
pub use restart::*;
pub use reverse_continue::*;
pub use scopes::*;
pub use set_breakpoints::*;
pub use stack_trace::*;
pub use step_back::*;
pub use terminate::*;
pub use threads::*;
pub use variables::*;

use crate::error::{Cause, Error};
use crate::utils;

use serde_json::{json, Map, Value};

use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Message {
    pub id: u64,
    pub format: String,
    pub variables: Option<HashMap<String, String>>,
    pub send_telemetry: bool,
    pub show_user: bool,
    pub url: Option<String>,
    pub url_label: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Checksum {
    pub algorithm: ChecksumAlgorithm,
    pub checksum: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum SourceReference {
    Path(String),
    Reference(u32),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SourcePresentationHint {
    Normal,
    Emphasize,
    Deemphasize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Source {
    pub name: Option<String>,
    pub source_reference: Option<SourceReference>,
    pub presentation_hint: Option<SourcePresentationHint>,
    pub origin: Option<String>,
    pub sources: Vec<Source>,
    pub adapter_data: Option<Value>,
    pub checksums: Vec<Checksum>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Breakpoint {
    pub id: Option<u64>,
    pub verified: bool,
    pub message: Option<String>,
    pub source: Option<Source>,
    pub line: Option<u64>,
    pub column: Option<u64>,
    pub end_line: Option<u64>,
    pub end_column: Option<u64>,
    pub instruction_reference: Option<String>,
    pub offset: Option<i64>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Capabilities {
    pub supports_configuration_done_request: bool,
    pub supports_function_breakpoints: bool,
    pub supports_conditional_breakpoints: bool,
    pub supports_hit_conditional_breakpoints: bool,
    pub supports_evaluate_for_hovers: bool,
    pub exception_breakpoint_filters: Vec<ExceptionBreakpointsFilter>,
    pub supports_step_back: bool,
    pub supports_set_variable: bool,
    pub supports_restart_frame: bool,
    pub supports_goto_targets_request: bool,
    pub supports_step_in_targets_request: bool,
    pub supports_completions_request: bool,
    pub completion_trigger_characters: Vec<String>,
    pub supports_modules_request: bool,
    pub additional_module_columns: Vec<ColumnDescriptor>,
    pub supported_checksum_algorithms: Vec<ChecksumAlgorithm>,
    pub supports_restart_request: bool,
    pub supports_exception_options: bool,
    pub supports_value_formatting_options: bool,
    pub supports_exception_info_request: bool,
    pub support_terminate_debuggee: bool,
    pub support_suspend_debuggee: bool,
    pub supports_delayed_stack_trace_loading: bool,
    pub supports_loaded_sources_request: bool,
    pub supports_log_points: bool,
    pub supports_terminate_threads_request: bool,
    pub supports_set_expression: bool,
    pub supports_terminate_request: bool,
    pub supports_data_breakpoints: bool,
    pub supports_read_memory_request: bool,
    pub supports_write_memory_request: bool,
    pub supports_disassemble_request: bool,
    pub supports_cancel_request: bool,
    pub supports_breakpoint_locations_request: bool,
    pub supports_clipboard_context: bool,
    pub supports_stepping_granularity: bool,
    pub supports_instruction_breakpoints: bool,
    pub supports_exception_filter_options: bool,
    pub supports_single_thread_execution_requests: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ColumnDescriptorType {
    String,
    Number,
    Boolean,
    UnixTimestampUTC,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ColumnDescriptor {
    pub attribute_name: String,
    pub label: String,
    pub format: Option<String>,
    pub ty: Option<ColumnDescriptorType>,
    pub width: Option<u64>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExceptionBreakpointsFilter {
    pub filter: String,
    pub label: String,
    pub description: Option<String>,
    pub default: bool,
    pub supports_condition: bool,
    pub condition_description: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StackFrameModuleId {
    Number(u64),
    String(String),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StackFramePresentationHint {
    Normal,
    Label,
    Subtle,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StackFrame {
    pub id: u64,
    pub name: String,
    pub source: Option<Source>,
    pub line: u64,
    pub column: u64,
    pub end_line: Option<u64>,
    pub end_column: Option<u64>,
    pub can_restart: bool,
    pub instruction_pointer_reference: Option<String>,
    pub module_id: Option<StackFrameModuleId>,
    pub presentation_hint: Option<StackFramePresentationHint>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct StackFrameFormat {
    pub parameters: bool,
    pub parameter_types: bool,
    pub parameter_names: bool,
    pub parameter_values: bool,
    pub line: bool,
    pub module: bool,
    pub include_all: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ValueFormat {
    pub hex: bool,
}

impl From<Message> for Value {
    fn from(message: Message) -> Self {
        let Message {
            id,
            format,
            variables,
            send_telemetry,
            show_user,
            url,
            url_label,
        } = message;

        let id = utils::attribute_u64("id", id);
        let format = utils::attribute_string("format", format);
        let variables = utils::attribute_map_optional("variables", variables);
        let send_telemetry = utils::attribute_bool_optional("sendTelemetry", send_telemetry);
        let show_user = utils::attribute_bool_optional("showUser", show_user);
        let url = utils::attribute_string_optional("url", url);
        let url_label = utils::attribute_string_optional("urlLabel", url_label);

        utils::finalize_object(
            id.chain(format)
                .chain(variables)
                .chain(send_telemetry)
                .chain(show_user)
                .chain(url)
                .chain(url_label),
        )
    }
}

impl TryFrom<&Map<String, Value>> for Message {
    type Error = Error;

    fn try_from(map: &Map<String, Value>) -> Result<Self, Self::Error> {
        let id = utils::get_u64(map, "id")?;
        let format = utils::get_string(map, "format")?;
        let variables = utils::get_map_to_string_optional(map, "variables")?;
        let send_telemetry = utils::get_bool_optional(map, "sendTelemetry")?;
        let show_user = utils::get_bool_optional(map, "showUser")?;
        let url = utils::get_string_optional(map, "url")?;
        let url_label = utils::get_string_optional(map, "urlLabel")?;

        Ok(Self {
            id,
            format,
            variables,
            send_telemetry,
            show_user,
            url,
            url_label,
        })
    }
}

impl From<Checksum> for Value {
    fn from(c: Checksum) -> Self {
        json!({
            "algorithm": Value::String(<&'static str>::from(c.algorithm).into()),
            "checksum": Value::String(c.checksum),
        })
    }
}

impl TryFrom<&Map<String, Value>> for Checksum {
    type Error = Error;

    fn try_from(map: &Map<String, Value>) -> Result<Self, Self::Error> {
        let algorithm = utils::get_str(map, "algorithm").and_then(ChecksumAlgorithm::try_from)?;
        let checksum = utils::get_string(map, "checksum")?;

        Ok(Self {
            algorithm,
            checksum,
        })
    }
}

impl From<SourcePresentationHint> for &'static str {
    fn from(p: SourcePresentationHint) -> Self {
        match p {
            SourcePresentationHint::Normal => "normal",
            SourcePresentationHint::Emphasize => "emphasize",
            SourcePresentationHint::Deemphasize => "deemphasize",
        }
    }
}

impl From<SourcePresentationHint> for String {
    fn from(s: SourcePresentationHint) -> Self {
        <&'static str>::from(s).into()
    }
}

impl TryFrom<&str> for SourcePresentationHint {
    type Error = Error;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        match s {
            "normal" => Ok(SourcePresentationHint::Normal),
            "emphasize" => Ok(SourcePresentationHint::Emphasize),
            "deemphasize" => Ok(SourcePresentationHint::Deemphasize),
            _ => Err(Error::new("sourcePresentationHint", Cause::ExpectsEnum)),
        }
    }
}

impl From<Source> for Value {
    fn from(source: Source) -> Self {
        let Source {
            name,
            source_reference,
            presentation_hint,
            origin,
            sources,
            adapter_data,
            checksums,
        } = source;

        let (path, source_reference) = source_reference
            .map(|r| match r {
                SourceReference::Path(path) => (Some(path), None),
                SourceReference::Reference(n) if n == 0 => (None, None),
                SourceReference::Reference(n) => (None, Some(n)),
            })
            .unwrap_or((None, None));

        let name = utils::attribute_string_optional("name", name);
        let path = utils::attribute_string_optional("path", path);
        let source_reference = utils::attribute_u32_optional("sourceReference", source_reference);
        let presentation_hint =
            utils::attribute_string_optional("presentationHint", presentation_hint);
        let origin = utils::attribute_string_optional("origin", origin);
        let sources = utils::attribute_array_optional("sources", sources);
        let adapter_data = utils::attribute_optional("adapterData", adapter_data);
        let checksums = utils::attribute_array_optional("checksums", checksums);

        utils::finalize_object(
            name.chain(path)
                .chain(source_reference)
                .chain(presentation_hint)
                .chain(origin)
                .chain(sources)
                .chain(adapter_data)
                .chain(checksums),
        )
    }
}

impl TryFrom<&Map<String, Value>> for Source {
    type Error = Error;

    fn try_from(map: &Map<String, Value>) -> Result<Self, Self::Error> {
        let name = utils::get_string_optional(map, "name")?;

        let path = utils::get_string_optional(map, "path")?.map(SourceReference::Path);
        let source_reference = utils::get_u32_optional(map, "sourceReference")?
            .filter(|n| n > &0)
            .map(SourceReference::Reference);

        let source_reference = source_reference.or(path);

        let presentation_hint = utils::get_str_optional(map, "presentationHint")?
            .map(SourcePresentationHint::try_from)
            .transpose()?;

        let origin = utils::get_string_optional(map, "origin")?;
        let sources = utils::get_array_optional(map, "sources")?;
        let adapter_data = map.get("adapterData").cloned();
        let checksums = utils::get_array_optional(map, "checksums")?;

        Ok(Self {
            name,
            source_reference,
            presentation_hint,
            origin,
            sources,
            adapter_data,
            checksums,
        })
    }
}

impl From<Breakpoint> for Value {
    fn from(breakpoint: Breakpoint) -> Self {
        let Breakpoint {
            id,
            verified,
            message,
            source,
            line,
            column,
            end_line,
            end_column,
            instruction_reference,
            offset,
        } = breakpoint;

        let id = utils::attribute_u64_optional("id", id);
        let verified = utils::attribute_bool("verified", verified);
        let message = utils::attribute_string_optional("message", message);
        let source = utils::attribute_optional("source", source);
        let line = utils::attribute_u64_optional("line", line);
        let column = utils::attribute_u64_optional("column", column);
        let end_line = utils::attribute_u64_optional("endLine", end_line);
        let end_column = utils::attribute_u64_optional("endColumn", end_column);
        let instruction_reference =
            utils::attribute_string_optional("instructionReference", instruction_reference);
        let offset = utils::attribute_i64_optional("offset", offset);

        utils::finalize_object(
            id.chain(verified)
                .chain(message)
                .chain(source)
                .chain(line)
                .chain(column)
                .chain(end_line)
                .chain(end_column)
                .chain(instruction_reference)
                .chain(offset),
        )
    }
}

impl TryFrom<&Map<String, Value>> for Breakpoint {
    type Error = Error;

    fn try_from(map: &Map<String, Value>) -> Result<Self, Self::Error> {
        let id = utils::get_u64_optional(map, "id")?;
        let verified = utils::get_bool(map, "verified")?;
        let message = utils::get_string_optional(map, "message")?;
        let source = utils::get_object_optional(map, "source")?;
        let line = utils::get_u64_optional(map, "line")?;
        let column = utils::get_u64_optional(map, "column")?;
        let end_line = utils::get_u64_optional(map, "endLine")?;
        let end_column = utils::get_u64_optional(map, "endColumn")?;
        let instruction_reference = utils::get_string_optional(map, "instructionReference")?;
        let offset = utils::get_i64_optional(map, "endColumn")?;

        Ok(Self {
            id,
            verified,
            message,
            source,
            line,
            column,
            end_line,
            end_column,
            instruction_reference,
            offset,
        })
    }
}

impl From<ExceptionBreakpointsFilter> for Value {
    fn from(filter: ExceptionBreakpointsFilter) -> Self {
        let ExceptionBreakpointsFilter {
            filter,
            label,
            description,
            default,
            supports_condition,
            condition_description,
        } = filter;

        let filter = utils::attribute_string("filter", filter);
        let label = utils::attribute_string("label", label);
        let description = utils::attribute_string_optional("description", description);
        let default = utils::attribute_bool_optional("default", default);
        let supports_condition =
            utils::attribute_bool_optional("supportsCondition", supports_condition);
        let condition_description =
            utils::attribute_string_optional("conditionDescription", condition_description);

        utils::finalize_object(
            filter
                .chain(label)
                .chain(description)
                .chain(default)
                .chain(supports_condition)
                .chain(condition_description),
        )
    }
}

impl TryFrom<&Map<String, Value>> for ExceptionBreakpointsFilter {
    type Error = Error;

    fn try_from(map: &Map<String, Value>) -> Result<Self, Self::Error> {
        let filter = utils::get_string(map, "filter")?;
        let label = utils::get_string(map, "label")?;
        let description = utils::get_string_optional(map, "description")?;
        let default = utils::get_bool_optional(map, "default")?;
        let supports_condition = utils::get_bool_optional(map, "supportsCondition")?;
        let condition_description = utils::get_string_optional(map, "conditionDescription")?;

        Ok(Self {
            filter,
            label,
            description,
            default,
            supports_condition,
            condition_description,
        })
    }
}

impl From<ColumnDescriptorType> for &'static str {
    fn from(t: ColumnDescriptorType) -> Self {
        match t {
            ColumnDescriptorType::String => "string",
            ColumnDescriptorType::Number => "number",
            ColumnDescriptorType::Boolean => "boolean",
            ColumnDescriptorType::UnixTimestampUTC => "unixTimestampUTC",
        }
    }
}

impl From<ColumnDescriptorType> for String {
    fn from(t: ColumnDescriptorType) -> Self {
        <&'static str>::from(t).into()
    }
}

impl TryFrom<&str> for ColumnDescriptorType {
    type Error = Error;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        match s {
            "string" => Ok(Self::String),
            "number" => Ok(Self::Number),
            "boolean" => Ok(Self::Boolean),
            "unixTimestampUTC" => Ok(Self::UnixTimestampUTC),
            _ => Err(Error::new("columnDescriptorType", Cause::ExpectsEnum)),
        }
    }
}

impl From<ColumnDescriptor> for Value {
    fn from(d: ColumnDescriptor) -> Self {
        let ColumnDescriptor {
            attribute_name,
            label,
            format,
            ty,
            width,
        } = d;

        let attribute_name = utils::attribute_string("attributeName", attribute_name);
        let label = utils::attribute_string("label", label);
        let format = utils::attribute_string_optional("format", format);
        let ty = utils::attribute_string_optional("type", ty);
        let width = utils::attribute_u64_optional("width", width);

        utils::finalize_object(
            attribute_name
                .chain(label)
                .chain(format)
                .chain(ty)
                .chain(width),
        )
    }
}

impl TryFrom<&Map<String, Value>> for ColumnDescriptor {
    type Error = Error;

    fn try_from(map: &Map<String, Value>) -> Result<Self, Self::Error> {
        let attribute_name = utils::get_string(map, "attributeName")?;
        let label = utils::get_string(map, "label")?;
        let format = utils::get_string_optional(map, "format")?;
        let ty = utils::get_str_optional(map, "type")?
            .map(ColumnDescriptorType::try_from)
            .transpose()?;
        let width = utils::get_u64_optional(map, "width")?;

        Ok(Self {
            attribute_name,
            label,
            format,
            ty,
            width,
        })
    }
}

impl From<Capabilities> for Value {
    fn from(c: Capabilities) -> Self {
        let Capabilities {
            supports_configuration_done_request,
            supports_function_breakpoints,
            supports_conditional_breakpoints,
            supports_hit_conditional_breakpoints,
            supports_evaluate_for_hovers,
            exception_breakpoint_filters,
            supports_step_back,
            supports_set_variable,
            supports_restart_frame,
            supports_goto_targets_request,
            supports_step_in_targets_request,
            supports_completions_request,
            completion_trigger_characters,
            supports_modules_request,
            additional_module_columns,
            supported_checksum_algorithms,
            supports_restart_request,
            supports_exception_options,
            supports_value_formatting_options,
            supports_exception_info_request,
            support_terminate_debuggee,
            support_suspend_debuggee,
            supports_delayed_stack_trace_loading,
            supports_loaded_sources_request,
            supports_log_points,
            supports_terminate_threads_request,
            supports_set_expression,
            supports_terminate_request,
            supports_data_breakpoints,
            supports_read_memory_request,
            supports_write_memory_request,
            supports_disassemble_request,
            supports_cancel_request,
            supports_breakpoint_locations_request,
            supports_clipboard_context,
            supports_stepping_granularity,
            supports_instruction_breakpoints,
            supports_exception_filter_options,
            supports_single_thread_execution_requests,
        } = c;

        let supports_configuration_done_request = utils::attribute_bool_optional(
            "supportsConfigurationDoneRequest",
            supports_configuration_done_request,
        );
        let supports_function_breakpoints = utils::attribute_bool_optional(
            "supportsFunctionBreakpoints",
            supports_function_breakpoints,
        );
        let supports_conditional_breakpoints = utils::attribute_bool_optional(
            "supportsConditionalBreakpoints",
            supports_conditional_breakpoints,
        );
        let supports_hit_conditional_breakpoints = utils::attribute_bool_optional(
            "supportsHitConditionalBreakpoints",
            supports_hit_conditional_breakpoints,
        );
        let supports_evaluate_for_hovers = utils::attribute_bool_optional(
            "supportsEvaluateForHovers",
            supports_evaluate_for_hovers,
        );
        let exception_breakpoint_filters = utils::attribute_array_optional(
            "exceptionBreakpointFilters",
            exception_breakpoint_filters,
        );
        let supports_step_back =
            utils::attribute_bool_optional("supportsStepBack", supports_step_back);
        let supports_set_variable =
            utils::attribute_bool_optional("supportsSetVariable", supports_set_variable);
        let supports_restart_frame =
            utils::attribute_bool_optional("supportsRestartFrame", supports_restart_frame);
        let supports_goto_targets_request = utils::attribute_bool_optional(
            "supportsGotoTargetsRequest",
            supports_goto_targets_request,
        );
        let supports_step_in_targets_request = utils::attribute_bool_optional(
            "supportsStepInTargetsRequest",
            supports_step_in_targets_request,
        );
        let supports_completions_request = utils::attribute_bool_optional(
            "supportsCompletionsRequest",
            supports_completions_request,
        );
        let completion_trigger_characters = utils::attribute_array_optional(
            "completionTriggerCharacters",
            completion_trigger_characters,
        );
        let supports_modules_request =
            utils::attribute_bool_optional("supportsModulesRequest", supports_modules_request);
        let additional_module_columns =
            utils::attribute_array_optional("additionalModuleColumns", additional_module_columns);
        let supported_checksum_algorithms = utils::attribute_array_optional(
            "supportedChecksumAlgorithms",
            supported_checksum_algorithms,
        );
        let supports_restart_request =
            utils::attribute_bool_optional("supportsRestartRequest", supports_restart_request);
        let supports_exception_options =
            utils::attribute_bool_optional("supportsExceptionOptions", supports_exception_options);
        let supports_value_formatting_options = utils::attribute_bool_optional(
            "supportsValueFormattingOptions",
            supports_value_formatting_options,
        );
        let supports_exception_info_request = utils::attribute_bool_optional(
            "supportsExceptionInfoRequest",
            supports_exception_info_request,
        );
        let support_terminate_debuggee =
            utils::attribute_bool_optional("supportTerminateDebuggee", support_terminate_debuggee);
        let support_suspend_debuggee =
            utils::attribute_bool_optional("supportSuspendDebuggee", support_suspend_debuggee);
        let supports_delayed_stack_trace_loading = utils::attribute_bool_optional(
            "supportsDelayedStackTraceLoading",
            supports_delayed_stack_trace_loading,
        );
        let supports_loaded_sources_request = utils::attribute_bool_optional(
            "supportsLoadedSourcesRequest",
            supports_loaded_sources_request,
        );
        let supports_log_points =
            utils::attribute_bool_optional("supportsLogPoints", supports_log_points);
        let supports_terminate_threads_request = utils::attribute_bool_optional(
            "supportsTerminateThreadsRequest",
            supports_terminate_threads_request,
        );
        let supports_set_expression =
            utils::attribute_bool_optional("supportsSetExpression", supports_set_expression);
        let supports_terminate_request =
            utils::attribute_bool_optional("supportsTerminateRequest", supports_terminate_request);
        let supports_data_breakpoints =
            utils::attribute_bool_optional("supportsDataBreakpoints", supports_data_breakpoints);
        let supports_read_memory_request = utils::attribute_bool_optional(
            "supportsReadMemoryRequest",
            supports_read_memory_request,
        );
        let supports_write_memory_request = utils::attribute_bool_optional(
            "supportsWriteMemoryRequest",
            supports_write_memory_request,
        );
        let supports_disassemble_request = utils::attribute_bool_optional(
            "supportsDisassembleRequest",
            supports_disassemble_request,
        );
        let supports_cancel_request =
            utils::attribute_bool_optional("supportsCancelRequest", supports_cancel_request);
        let supports_breakpoint_locations_request = utils::attribute_bool_optional(
            "supportsBreakpointLocationsRequest",
            supports_breakpoint_locations_request,
        );
        let supports_clipboard_context =
            utils::attribute_bool_optional("supportsClipboardContext", supports_clipboard_context);
        let supports_stepping_granularity = utils::attribute_bool_optional(
            "supportsSteppingGranularity",
            supports_stepping_granularity,
        );
        let supports_instruction_breakpoints = utils::attribute_bool_optional(
            "supportsInstructionBreakpoints",
            supports_instruction_breakpoints,
        );
        let supports_exception_filter_options = utils::attribute_bool_optional(
            "supportsExceptionFilterOptions",
            supports_exception_filter_options,
        );
        let supports_single_thread_execution_requests = utils::attribute_bool_optional(
            "supportsSingleThreadExecutionRequests",
            supports_single_thread_execution_requests,
        );

        utils::finalize_object(
            supports_configuration_done_request
                .chain(supports_function_breakpoints)
                .chain(supports_conditional_breakpoints)
                .chain(supports_hit_conditional_breakpoints)
                .chain(supports_evaluate_for_hovers)
                .chain(exception_breakpoint_filters)
                .chain(supports_step_back)
                .chain(supports_set_variable)
                .chain(supports_restart_frame)
                .chain(supports_goto_targets_request)
                .chain(supports_step_in_targets_request)
                .chain(supports_completions_request)
                .chain(completion_trigger_characters)
                .chain(supports_modules_request)
                .chain(additional_module_columns)
                .chain(supported_checksum_algorithms)
                .chain(supports_restart_request)
                .chain(supports_exception_options)
                .chain(supports_value_formatting_options)
                .chain(supports_exception_info_request)
                .chain(support_terminate_debuggee)
                .chain(support_suspend_debuggee)
                .chain(supports_delayed_stack_trace_loading)
                .chain(supports_loaded_sources_request)
                .chain(supports_log_points)
                .chain(supports_terminate_threads_request)
                .chain(supports_set_expression)
                .chain(supports_terminate_request)
                .chain(supports_data_breakpoints)
                .chain(supports_read_memory_request)
                .chain(supports_write_memory_request)
                .chain(supports_disassemble_request)
                .chain(supports_cancel_request)
                .chain(supports_breakpoint_locations_request)
                .chain(supports_clipboard_context)
                .chain(supports_stepping_granularity)
                .chain(supports_instruction_breakpoints)
                .chain(supports_exception_filter_options)
                .chain(supports_single_thread_execution_requests),
        )
    }
}

impl TryFrom<&Map<String, Value>> for Capabilities {
    type Error = Error;

    fn try_from(map: &Map<String, Value>) -> Result<Self, Self::Error> {
        let supports_configuration_done_request =
            utils::get_bool_optional(map, "supportsConfigurationDoneRequest")?;
        let supports_function_breakpoints =
            utils::get_bool_optional(map, "supportsFunctionBreakpoints")?;
        let supports_conditional_breakpoints =
            utils::get_bool_optional(map, "supportsConditionalBreakpoints")?;
        let supports_hit_conditional_breakpoints =
            utils::get_bool_optional(map, "supportsHitConditionalBreakpoints")?;
        let supports_evaluate_for_hovers =
            utils::get_bool_optional(map, "supportsEvaluateForHovers")?;
        let exception_breakpoint_filters =
            utils::get_array_optional(map, "exceptionBreakpointFilters")?;
        let supports_step_back = utils::get_bool_optional(map, "supportsStepBack")?;
        let supports_set_variable = utils::get_bool_optional(map, "supportsSetVariable")?;
        let supports_restart_frame = utils::get_bool_optional(map, "supportsRestartFrame")?;
        let supports_goto_targets_request =
            utils::get_bool_optional(map, "supportsGotoTargetsRequest")?;
        let supports_step_in_targets_request =
            utils::get_bool_optional(map, "supportsStepInTargetsRequest")?;
        let supports_completions_request =
            utils::get_bool_optional(map, "supportsCompletionsRequest")?;
        let completion_trigger_characters =
            utils::get_array_of_string_optional(map, "completionTriggerCharacters")?;
        let supports_modules_request = utils::get_bool_optional(map, "supportsModulesRequest")?;
        let additional_module_columns = utils::get_array_optional(map, "additionalModuleColumns")?;
        let supported_checksum_algorithms =
            utils::get_array_of_string_enum_optional(map, "supportedChecksumAlgorithms")?;
        let supports_restart_request = utils::get_bool_optional(map, "supportsRestartRequest")?;
        let supports_exception_options = utils::get_bool_optional(map, "supportsExceptionOptions")?;
        let supports_value_formatting_options =
            utils::get_bool_optional(map, "supportsValueFormattingOptions")?;
        let supports_exception_info_request =
            utils::get_bool_optional(map, "supportsExceptionInfoRequest")?;
        let support_terminate_debuggee = utils::get_bool_optional(map, "supportTerminateDebuggee")?;
        let support_suspend_debuggee = utils::get_bool_optional(map, "supportSuspendDebuggee")?;
        let supports_delayed_stack_trace_loading =
            utils::get_bool_optional(map, "supportsDelayedStackTraceLoading")?;
        let supports_loaded_sources_request =
            utils::get_bool_optional(map, "supportsLoadedSourcesRequest")?;
        let supports_log_points = utils::get_bool_optional(map, "supportsLogPoints")?;
        let supports_terminate_threads_request =
            utils::get_bool_optional(map, "supportsTerminateThreadsRequest")?;
        let supports_set_expression = utils::get_bool_optional(map, "supportsSetExpression")?;
        let supports_terminate_request = utils::get_bool_optional(map, "supportsTerminateRequest")?;
        let supports_data_breakpoints = utils::get_bool_optional(map, "supportsDataBreakpoints")?;
        let supports_read_memory_request =
            utils::get_bool_optional(map, "supportsReadMemoryRequest")?;
        let supports_write_memory_request =
            utils::get_bool_optional(map, "supportsWriteMemoryRequest")?;
        let supports_disassemble_request =
            utils::get_bool_optional(map, "supportsDisassembleRequest")?;
        let supports_cancel_request = utils::get_bool_optional(map, "supportsCancelRequest")?;
        let supports_breakpoint_locations_request =
            utils::get_bool_optional(map, "supportsBreakpointLocationsRequest")?;
        let supports_clipboard_context = utils::get_bool_optional(map, "supportsClipboardContext")?;
        let supports_stepping_granularity =
            utils::get_bool_optional(map, "supportsSteppingGranularity")?;
        let supports_instruction_breakpoints =
            utils::get_bool_optional(map, "supportsInstructionBreakpoints")?;
        let supports_exception_filter_options =
            utils::get_bool_optional(map, "supportsExceptionFilterOptions")?;
        let supports_single_thread_execution_requests =
            utils::get_bool_optional(map, "supportsSingleThreadExecutionRequests")?;

        Ok(Self {
            supports_configuration_done_request,
            supports_function_breakpoints,
            supports_conditional_breakpoints,
            supports_hit_conditional_breakpoints,
            supports_evaluate_for_hovers,
            exception_breakpoint_filters,
            supports_step_back,
            supports_set_variable,
            supports_restart_frame,
            supports_goto_targets_request,
            supports_step_in_targets_request,
            supports_completions_request,
            completion_trigger_characters,
            supports_modules_request,
            additional_module_columns,
            supported_checksum_algorithms,
            supports_restart_request,
            supports_exception_options,
            supports_value_formatting_options,
            supports_exception_info_request,
            support_terminate_debuggee,
            support_suspend_debuggee,
            supports_delayed_stack_trace_loading,
            supports_loaded_sources_request,
            supports_log_points,
            supports_terminate_threads_request,
            supports_set_expression,
            supports_terminate_request,
            supports_data_breakpoints,
            supports_read_memory_request,
            supports_write_memory_request,
            supports_disassemble_request,
            supports_cancel_request,
            supports_breakpoint_locations_request,
            supports_clipboard_context,
            supports_stepping_granularity,
            supports_instruction_breakpoints,
            supports_exception_filter_options,
            supports_single_thread_execution_requests,
        })
    }
}
impl From<StackFrameModuleId> for Value {
    fn from(id: StackFrameModuleId) -> Self {
        match id {
            StackFrameModuleId::Number(n) => n.into(),
            StackFrameModuleId::String(s) => s.into(),
        }
    }
}

impl StackFrameModuleId {
    pub fn try_from_map_optional(map: &Map<String, Value>) -> Result<Option<Self>, Error> {
        map.get("moduleId")
            .map(|id| match id {
                Value::Number(n) => n
                    .as_u64()
                    .ok_or_else(|| Error::new("moduleId", Cause::MustBeUnsignedInteger))
                    .map(Self::Number),
                Value::String(s) => Ok(Self::String(s.clone())),
                _ => Err(Error::new("moduleId", Cause::ExpectsEnum)),
            })
            .transpose()
    }
}

impl From<StackFramePresentationHint> for &'static str {
    fn from(hint: StackFramePresentationHint) -> Self {
        match hint {
            StackFramePresentationHint::Normal => "normal",
            StackFramePresentationHint::Label => "label",
            StackFramePresentationHint::Subtle => "subtle",
        }
    }
}

impl From<StackFramePresentationHint> for String {
    fn from(hint: StackFramePresentationHint) -> Self {
        <&'static str>::from(hint).into()
    }
}

impl TryFrom<&str> for StackFramePresentationHint {
    type Error = Error;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        match s {
            "normal" => Ok(StackFramePresentationHint::Normal),
            "label" => Ok(StackFramePresentationHint::Label),
            "subtle" => Ok(StackFramePresentationHint::Subtle),
            _ => Err(Error::new("presentationHint", Cause::ExpectsEnum)),
        }
    }
}

impl From<StackFrame> for Value {
    fn from(frame: StackFrame) -> Self {
        let StackFrame {
            id,
            name,
            source,
            line,
            column,
            end_line,
            end_column,
            can_restart,
            instruction_pointer_reference,
            module_id,
            presentation_hint,
        } = frame;

        let id = utils::attribute_u64("id", id);
        let name = utils::attribute_string("name", name);
        let source = utils::attribute_optional("source", source);
        let line = utils::attribute_u64("line", line);
        let column = utils::attribute_u64("column", column);
        let end_line = utils::attribute_u64_optional("endLine", end_line);
        let end_column = utils::attribute_u64_optional("endColumn", end_column);
        let can_restart = utils::attribute_bool_optional("canRestart", can_restart);
        let instruction_pointer_reference = utils::attribute_string_optional(
            "instructionPointerReference",
            instruction_pointer_reference,
        );
        let module_id = utils::attribute_optional("moduleId", module_id);
        let presentation_hint =
            utils::attribute_string_optional("presentationHint", presentation_hint);

        utils::finalize_object(
            id.chain(name)
                .chain(source)
                .chain(line)
                .chain(column)
                .chain(end_line)
                .chain(end_column)
                .chain(can_restart)
                .chain(instruction_pointer_reference)
                .chain(module_id)
                .chain(presentation_hint),
        )
    }
}

impl TryFrom<&Map<String, Value>> for StackFrame {
    type Error = Error;

    fn try_from(map: &Map<String, Value>) -> Result<Self, Self::Error> {
        let id = utils::get_u64(map, "id")?;
        let name = utils::get_string(map, "name")?;
        let source = utils::get_object_optional(map, "source")?;
        let line = utils::get_u64(map, "line")?;
        let column = utils::get_u64(map, "column")?;
        let end_line = utils::get_u64_optional(map, "endLine")?;
        let end_column = utils::get_u64_optional(map, "endColumn")?;
        let can_restart = utils::get_bool_optional(map, "canRestart")?;
        let instruction_pointer_reference =
            utils::get_string_optional(map, "instructionPointerReference")?;
        let module_id = StackFrameModuleId::try_from_map_optional(map)?;
        let presentation_hint = utils::get_str_optional(map, "presentationHint")?
            .map(StackFramePresentationHint::try_from)
            .transpose()?;

        Ok(Self {
            id,
            name,
            source,
            line,
            column,
            end_line,
            end_column,
            can_restart,
            instruction_pointer_reference,
            module_id,
            presentation_hint,
        })
    }
}

impl From<StackFrameFormat> for Value {
    fn from(format: StackFrameFormat) -> Self {
        let StackFrameFormat {
            parameters,
            parameter_types,
            parameter_names,
            parameter_values,
            line,
            module,
            include_all,
        } = format;

        let parameters = utils::attribute_bool_optional("parameters", parameters);
        let parameter_types = utils::attribute_bool_optional("parameterTypes", parameter_types);
        let parameter_names = utils::attribute_bool_optional("parameterNames", parameter_names);
        let parameter_values = utils::attribute_bool_optional("parameterValues", parameter_values);
        let line = utils::attribute_bool_optional("line", line);
        let module = utils::attribute_bool_optional("module", module);
        let include_all = utils::attribute_bool_optional("includeAll", include_all);

        utils::finalize_object(
            parameters
                .chain(parameter_types)
                .chain(parameter_names)
                .chain(parameter_values)
                .chain(line)
                .chain(module)
                .chain(include_all),
        )
    }
}

impl TryFrom<&Map<String, Value>> for StackFrameFormat {
    type Error = Error;

    fn try_from(map: &Map<String, Value>) -> Result<Self, Self::Error> {
        let parameters = utils::get_bool_optional(map, "parameters")?;
        let parameter_types = utils::get_bool_optional(map, "parameterTypes")?;
        let parameter_names = utils::get_bool_optional(map, "parameterNames")?;
        let parameter_values = utils::get_bool_optional(map, "parameterValues")?;
        let line = utils::get_bool_optional(map, "line")?;
        let module = utils::get_bool_optional(map, "module")?;
        let include_all = utils::get_bool_optional(map, "includeAll")?;

        Ok(Self {
            parameters,
            parameter_types,
            parameter_names,
            parameter_values,
            line,
            module,
            include_all,
        })
    }
}

impl From<ValueFormat> for Value {
    fn from(format: ValueFormat) -> Self {
        let ValueFormat { hex } = format;

        let hex = utils::attribute_bool_optional("hex", hex);

        utils::finalize_object(hex)
    }
}

impl TryFrom<&Map<String, Value>> for ValueFormat {
    type Error = Error;

    fn try_from(map: &Map<String, Value>) -> Result<Self, Self::Error> {
        let hex = utils::get_bool_optional(map, "hex")?;

        Ok(Self { hex })
    }
}
