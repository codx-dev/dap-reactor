mod impls;

use crate::models::ChecksumAlgorithm;

use std::collections::HashMap;

use serde_json::Value;

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
pub struct Thread {
    pub id: u64,
    pub name: String,
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ValueFormat {
    pub hex: bool,
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
