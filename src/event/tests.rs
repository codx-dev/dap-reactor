use crate::prelude::*;
use serde_json::{json, Value};

#[test]
fn encode_events() {
    let cases = vec![
        EventTestCase {
            seq: 12,
            encoded: json!({
                "event": "initialized",
            }),
            decoded: Event::Initialized,
        },
        EventTestCase {
            seq: 15,
            encoded: json!({
                "event": "stopped",
                "body": {
                    "reason": "goto",
                    "description": "desc",
                    "threadId": 15,
                    "preserveFocusHint": true,
                    "text": "txt",
                    "allThreadsStopped": true,
                    "hitBreakpointIds": [15, 20],
                }
            }),
            decoded: Event::Stopped {
                reason: StoppedReason::Goto,
                description: Some("desc".into()),
                thread_id: Some(15),
                preserve_focus_hint: true,
                text: Some("txt".into()),
                all_threads_stopped: true,
                hit_breakpoint_ids: vec![15, 20],
            },
        },
        EventTestCase {
            seq: 16,
            encoded: json!({
                "event": "continued",
                "body": {
                    "threadId": 15,
                    "allThreadsContinued": true,
                }
            }),
            decoded: Event::Continued {
                thread_id: 15,
                all_threads_continued: true,
            },
        },
        EventTestCase {
            seq: 17,
            encoded: json!({
                "event": "breakpoint",
                "body": {
                    "reason": "removed",
                    "breakpoint": {
                        "id": 829,
                        "verified": true,
                        "message": "some msg",
                        "source": {
                            "name": "source name",
                            "sourceReference": 15,
                            "presentationHint": "emphasize",
                            "origin": "some origin",
                            "sources": [
                                {
                                    "name": "source name",
                                    "path": "/some/path",
                                    "presentationHint": "deemphasize",
                                    "origin": "some origin",
                                    "adapterData": 15,
                                }
                            ],
                            "adapterData": {
                                "foo": "bar"
                            },
                            "checksums": [
                                {
                                    "algorithm": "SHA1",
                                    "checksum": "beef",
                                }
                            ]
                        },
                        "line": 92,
                        "column": 28,
                        "endLine": 72,
                        "endColumn": 57,
                        "instructionReference": "some ref",
                        "offset": -15,
                    },
                }
            }),
            decoded: Event::Breakpoint {
                reason: BreakpointReason::Removed,
                breakpoint: Breakpoint {
                    id: Some(829),
                    verified: true,
                    message: Some("some msg".into()),
                    source: Some(Source {
                        name: Some("source name".into()),
                        source_reference: Some(SourceReference::Reference(15)),
                        presentation_hint: Some(SourcePresentationHint::Emphasize),
                        origin: Some("some origin".into()),
                        sources: vec![Source {
                            name: Some("source name".into()),
                            source_reference: Some(SourceReference::Path("/some/path".into())),
                            presentation_hint: Some(SourcePresentationHint::Deemphasize),
                            origin: Some("some origin".into()),
                            sources: vec![],
                            adapter_data: Some(15.into()),
                            checksums: vec![],
                        }],
                        adapter_data: Some(json!({
                            "foo": "bar"
                        })),
                        checksums: vec![Checksum {
                            algorithm: ChecksumAlgorithm::Sha1,
                            checksum: "beef".into(),
                        }],
                    }),
                    line: Some(92),
                    column: Some(28),
                    end_line: Some(72),
                    end_column: Some(57),
                    instruction_reference: Some("some ref".into()),
                    offset: Some(-15),
                },
            },
        },
        EventTestCase {
            seq: 17,
            encoded: json!({
                "event": "capabilities",
                "body": {
                    "capabilities": {
                        "supportsConfigurationDoneRequest": true,
                        "supportsFunctionBreakpoints": true,
                        "supportsConditionalBreakpoints": true,
                        "supportsHitConditionalBreakpoints": true,
                        "supportsEvaluateForHovers": true,
                        "exceptionBreakpointFilters": [
                        {
                          "filter": "some filter",
                          "label": "some label",
                          "description": "some descr",
                          "default": true,
                          "supportsCondition": true,
                          "conditionDescription": "some cond",
                        }
                        ],
                        "supportsStepBack": true,
                        "supportsSetVariable": true,
                        "supportsRestartFrame": true,
                        "supportsGotoTargetsRequest": true,
                        "supportsStepInTargetsRequest": true,
                        "supportsCompletionsRequest": true,
                        "completionTriggerCharacters": [
                            "foo", "bar"
                        ],
                        "supportsModulesRequest": true,
                        "additionalModuleColumns": [
                        {
                          "attributeName": "some name",
                          "label": "some label",
                          "format": "some format",
                          "type": "unixTimestampUTC",
                          "width": 82
                        }
                        ],
                        "supportedChecksumAlgorithms": [
                            "SHA1", "timestamp"
                        ],
                    }
                }
            }),
            decoded: Event::Capabilities {
                capabilities: Capabilities {
                    supports_configuration_done_request: true,
                    supports_function_breakpoints: true,
                    supports_conditional_breakpoints: true,
                    supports_hit_conditional_breakpoints: true,
                    supports_evaluate_for_hovers: true,
                    exception_breakpoint_filters: vec![ExceptionBreakpointsFilter {
                        filter: "some filter".into(),
                        label: "some label".into(),
                        description: Some("some descr".into()),
                        default: true,
                        supports_condition: true,
                        condition_description: Some("some cond".into()),
                    }],
                    supports_step_back: true,
                    supports_set_variable: true,
                    supports_restart_frame: true,
                    supports_goto_targets_request: true,
                    supports_step_in_targets_request: true,
                    supports_completions_request: true,
                    completion_trigger_characters: vec!["foo".into(), "bar".into()],
                    supports_modules_request: true,
                    additional_module_columns: vec![ColumnDescriptor {
                        attribute_name: "some name".into(),
                        label: "some label".into(),
                        format: Some("some format".into()),
                        ty: Some(ColumnDescriptorType::UnixTimestampUTC),
                        width: Some(82),
                    }],
                    supported_checksum_algorithms: vec![
                        ChecksumAlgorithm::Sha1,
                        ChecksumAlgorithm::Timestamp,
                    ],
                    supports_restart_request: false,
                    supports_exception_options: false,
                    supports_value_formatting_options: false,
                    supports_exception_info_request: false,
                    support_terminate_debuggee: false,
                    support_suspend_debuggee: false,
                    supports_delayed_stack_trace_loading: false,
                    supports_loaded_sources_request: false,
                    supports_log_points: false,
                    supports_terminate_threads_request: false,
                    supports_set_expression: false,
                    supports_terminate_request: false,
                    supports_data_breakpoints: false,
                    supports_read_memory_request: false,
                    supports_write_memory_request: false,
                    supports_disassemble_request: false,
                    supports_cancel_request: false,
                    supports_breakpoint_locations_request: false,
                    supports_clipboard_context: false,
                    supports_stepping_granularity: false,
                    supports_instruction_breakpoints: false,
                    supports_exception_filter_options: false,
                    supports_single_thread_execution_requests: false,
                },
            },
        },
        EventTestCase {
            seq: 17,
            encoded: json!({
                "event": "capabilities",
                "body": {
                    "capabilities": {
                        "supportsRestartRequest": true,
                        "supportsExceptionOptions": true,
                        "supportsValueFormattingOptions": true,
                        "supportsExceptionInfoRequest": true,
                        "supportTerminateDebuggee": true,
                        "supportSuspendDebuggee": true,
                        "supportsDelayedStackTraceLoading": true,
                        "supportsLoadedSourcesRequest": true,
                        "supportsLogPoints": true,
                        "supportsTerminateThreadsRequest": true,
                        "supportsSetExpression": true,
                        "supportsTerminateRequest": true,
                        "supportsDataBreakpoints": true,
                        "supportsReadMemoryRequest": true,
                        "supportsWriteMemoryRequest": true,
                        "supportsDisassembleRequest": true,
                        "supportsCancelRequest": true,
                        "supportsBreakpointLocationsRequest": true,
                        "supportsClipboardContext": true,
                        "supportsSteppingGranularity": true,
                        "supportsInstructionBreakpoints": true,
                        "supportsExceptionFilterOptions": true,
                        "supportsSingleThreadExecutionRequests": true,
                    }
                }
            }),
            decoded: Event::Capabilities {
                capabilities: Capabilities {
                    supports_configuration_done_request: false,
                    supports_function_breakpoints: false,
                    supports_conditional_breakpoints: false,
                    supports_hit_conditional_breakpoints: false,
                    supports_evaluate_for_hovers: false,
                    exception_breakpoint_filters: vec![],
                    supports_step_back: false,
                    supports_set_variable: false,
                    supports_restart_frame: false,
                    supports_goto_targets_request: false,
                    supports_step_in_targets_request: false,
                    supports_completions_request: false,
                    completion_trigger_characters: vec![],
                    supports_modules_request: false,
                    additional_module_columns: vec![],
                    supported_checksum_algorithms: vec![],
                    supports_restart_request: true,
                    supports_exception_options: true,
                    supports_value_formatting_options: true,
                    supports_exception_info_request: true,
                    support_terminate_debuggee: true,
                    support_suspend_debuggee: true,
                    supports_delayed_stack_trace_loading: true,
                    supports_loaded_sources_request: true,
                    supports_log_points: true,
                    supports_terminate_threads_request: true,
                    supports_set_expression: true,
                    supports_terminate_request: true,
                    supports_data_breakpoints: true,
                    supports_read_memory_request: true,
                    supports_write_memory_request: true,
                    supports_disassemble_request: true,
                    supports_cancel_request: true,
                    supports_breakpoint_locations_request: true,
                    supports_clipboard_context: true,
                    supports_stepping_granularity: true,
                    supports_instruction_breakpoints: true,
                    supports_exception_filter_options: true,
                    supports_single_thread_execution_requests: true,
                },
            },
        },
        EventTestCase {
            seq: 18,
            encoded: json!({
                "event": "exited",
                "body": {
                    "exitCode": 892
                }
            }),
            decoded: Event::Exited { exit_code: 892 },
        },
        EventTestCase {
            seq: 18,
            encoded: json!({
                "event": "terminated",
                "body": {
                    "restart": {
                        "foo": "bar"
                    }
                }
            }),
            decoded: Event::Terminated { restart: Some(json!({"foo": "bar"})) },
        },
    ];

    cases.into_iter().for_each(|case| case.run());
}

struct EventTestCase {
    seq: u64,
    encoded: Value,
    decoded: Event,
}

impl EventTestCase {
    fn run(self) {
        let Self {
            seq,
            mut encoded,
            decoded,
        } = self;

        encoded["seq"] = seq.into();
        encoded["type"] = "event".into();

        let encoded =
            ProtocolMessage::try_from(&encoded).expect("failed to parse encoded protocol message");

        let protocol = decoded.clone().into_protocol(seq);
        let protocol = ProtocolMessage::from(protocol);

        assert_eq!(encoded, protocol);

        let protocol_bytes = protocol.clone().into_adapter_message();
        let len = protocol_bytes.len();

        let (consumed, protocol_decoded) =
            ProtocolMessage::try_from_bytes(protocol_bytes).expect("failed to decode message");

        assert_eq!(len, consumed);
        assert_eq!(protocol, protocol_decoded);
    }
}
