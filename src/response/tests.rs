use crate::models::*;
use crate::prelude::*;

use serde_json::{json, Value};

#[test]
fn encode_responses() {
    let cases = vec![
        ResponseTestCase {
            seq: 1500,
            request_seq: 13,
            encoded: json!({
                "command": "attach",
                "success": false,
                "message": "error msg",
                "body": {
                    "id": 83,
                    "format": "some format",
                    "variables": {
                        "var a": "a",
                        "var b": "b",
                    },
                    "sendTelemetry": true,
                    "showUser": true,
                    "url": "https://www.fsf.org/",
                    "urlLabel": "fsf",
                },
            }),
            decoded: Response::Error {
                command: "attach".into(),
                error: ProtocolResponseError {
                    message: Some("error msg".into()),
                    body: Some(Message {
                        id: 83,
                        format: "some format".into(),
                        variables: Some(
                            vec![("var a".into(), "a".into()), ("var b".into(), "b".into())]
                                .into_iter()
                                .collect(),
                        ),
                        send_telemetry: true,
                        show_user: true,
                        url: Some("https://www.fsf.org/".into()),
                        url_label: Some("fsf".into()),
                    }),
                },
            },
        },
        ResponseTestCase {
            seq: 1500,
            request_seq: 13,
            encoded: json!({
                "command": "attach",
                "success": true,
            }),
            decoded: Response::Attach,
        },
        ResponseTestCase {
            seq: 1501,
            request_seq: 14,
            encoded: json!({
                "command": "restart",
                "success": true,
            }),
            decoded: Response::Restart,
        },
        ResponseTestCase {
            seq: 1502,
            request_seq: 15,
            encoded: json!({
                "command": "disconnect",
                "success": true,
            }),
            decoded: Response::Disconnect,
        },
        ResponseTestCase {
            seq: 1503,
            request_seq: 15,
            encoded: json!({
                "command": "terminate",
                "success": true,
            }),
            decoded: Response::Terminate,
        },
        ResponseTestCase {
            seq: 1504,
            request_seq: 16,
            encoded: json!({
                "command": "breakpointLocations",
                "success": true,
                "body": {
                    "breakpoints": [{
                        "line": 30,
                        "column": 40,
                        "endLine": 50,
                        "endColumn": 60,
                    }]
                }
            }),
            decoded: Response::BreakpointLocations {
                body: Some(BreakpointLocationsResponse {
                    breakpoints: vec![BreakpointLocation {
                        line: 30,
                        column: Some(40),
                        end_line: Some(50),
                        end_column: Some(60),
                    }],
                }),
            },
        },
        ResponseTestCase {
            seq: 1505,
            request_seq: 17,
            encoded: json!({
                "command": "configurationDone",
                "success": true,
            }),
            decoded: Response::ConfigurationDone,
        },
        ResponseTestCase {
            seq: 1506,
            request_seq: 18,
            encoded: json!({
                "command": "continue",
                "success": true,
                "body": {
                    "allThreadsContinued": true
                }
            }),
            decoded: Response::Continue {
                body: ContinueResponse {
                    all_threads_continued: true,
                },
            },
        },
        ResponseTestCase {
            seq: 1507,
            request_seq: 19,
            encoded: json!({
                "command": "evaluate",
                "success": true,
                "body": {
                    "result": "result",
                    "type": "type",
                    "presentationHint": {
                        "kind": "property",
                        "attributes": ["static", "constant"],
                        "visibility": "public",
                        "lazy": true,
                    },
                    "variablesReference": 2,
                    "namedVariables": 4,
                    "indexedVariables": 3,
                    "memoryReference": "memoryReference"
                }
            }),
            decoded: Response::Evaluate {
                body: EvaluateResponse {
                    result: String::from("result"),
                    r#type: Some(String::from("type")),
                    presentation_hint: Some(VariablePresentationHint {
                        kind: Some(VariablePresentationHintKind::Property),
                        attributes: vec![
                            VariablePresentationHintAttribute::Static,
                            VariablePresentationHintAttribute::Constant,
                        ],
                        visibility: Some(VariablePresentationHintVisibility::Public),
                        lazy: true,
                    }),
                    variables_reference: 2,
                    named_variables: Some(4),
                    indexed_variables: Some(3),
                    memory_reference: Some(String::from("memoryReference")),
                },
            },
        },
        ResponseTestCase {
            seq: 1508,
            request_seq: 20,
            encoded: json!({
                "command": "exceptionInfo",
                "success": true,
                "body": {
                    "exceptionId": "exceptionId",
                    "description": "description",
                    "breakMode": "always",
                    "details": {
                        "message":  "message",
                        "typeName": "typeName",
                        "fullTypeName": "fullTypeName",
                        "evaluateName": "evaluateName",
                        "stackTrace": "stackTrace",
                        "innerException": [{
                            "message":  "message2",
                            "typeName": "typeName2",
                            "fullTypeName": "fullTypeName2",
                            "evaluateName": "evaluateName2",
                            "stackTrace": "stackTrace2",
                            "innerException": [],
                        }],
                    }
                }
            }),
            decoded: Response::ExceptionInfo {
                body: ExceptionInfoResponse {
                    exception_id: String::from("exceptionId"),
                    description: Some(String::from("description")),
                    break_mode: ExceptionBreakMode::Always,
                    details: Some(ExceptionDetails {
                        message: Some(String::from("message")),
                        type_name: Some(String::from("typeName")),
                        full_type_name: Some(String::from("fullTypeName")),
                        evaluate_name: Some(String::from("evaluateName")),
                        stack_trace: Some(String::from("stackTrace")),
                        inner_exception: vec![ExceptionDetails {
                            message: Some(String::from("message2")),
                            type_name: Some(String::from("typeName2")),
                            full_type_name: Some(String::from("fullTypeName2")),
                            evaluate_name: Some(String::from("evaluateName2")),
                            stack_trace: Some(String::from("stackTrace2")),
                            inner_exception: vec![],
                        }],
                    }),
                },
            },
        },
        ResponseTestCase {
            seq: 1509,
            request_seq: 21,
            encoded: json!({
                "command": "goto",
                "success": true,
            }),
            decoded: Response::Goto,
        },
        ResponseTestCase {
            seq: 1510,
            request_seq: 22,
            encoded: {
                let body = json!({
                    "supportsConfigurationDoneRequest": true,
                    "supportsFunctionBreakpoints": true,
                    "supportsConditionalBreakpoints": true,
                    "supportsHitConditionalBreakpoints": true,
                    "supportsEvaluateForHovers": true,
                    "exceptionBreakpointFilters": [{
                        "filter": "filter",
                        "label": "label",
                        "description": "description",
                        "default": true,
                        "supportsCondition": true,
                        "conditionDescription": "conditionDescription",
                    }],
                    "supportsStepBack": true,
                    "supportsSetVariable": true,
                    "supportsRestartFrame": true,
                    "supportsGotoTargetsRequest": true,
                    "supportsStepInTargetsRequest": true,
                    "supportsCompletionsRequest": true,
                    "completionTriggerCharacters": ["a"],
                    "supportsModulesRequest": true,
                    "additionalModuleColumns": [{
                        "attributeName": "attribute",
                        "label": "label",
                        "format": "format",
                        "type": "boolean",
                        "width": 50
                    }],
                    "supportedChecksumAlgorithms": ["MD5"],
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
                    "supportsExceptionFilterOptions": true,
                    "supportsInstructionBreakpoints": true,
                    "supportsSingleThreadExecutionRequests": true,
                    "supportsSteppingGranularity": true,
                });

                json!({
                    "command": "initialize",
                    "success": true,
                    "body": body
                })
            },
            decoded: Response::Initialize {
                body: InitializeResponse {
                    supports_configuration_done_request: true,
                    supports_function_breakpoints: true,
                    supports_conditional_breakpoints: true,
                    supports_hit_conditional_breakpoints: true,
                    supports_evaluate_for_hovers: true,
                    exception_breakpoint_filters: vec![ExceptionBreakpointsFilter {
                        filter: String::from("filter"),
                        label: String::from("label"),
                        description: Some(String::from("description")),
                        default: true,
                        supports_condition: true,
                        condition_description: Some(String::from("conditionDescription")),
                    }],
                    supports_step_back: true,
                    supports_set_variable: true,
                    supports_restart_frame: true,
                    supports_goto_targets_request: true,
                    supports_step_in_targets_request: true,
                    supports_completions_request: true,
                    completion_trigger_characters: vec!["a".to_string()],
                    supports_modules_request: true,
                    additional_module_columns: vec![ColumnDescriptor {
                        attribute_name: String::from("attribute"),
                        label: String::from("label"),
                        format: Some(String::from("format")),
                        ty: Some(ColumnDescriptorType::Boolean),
                        width: Some(50),
                    }],
                    supported_checksum_algorithms: vec![ChecksumAlgorithm::Md5],
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
        ResponseTestCase {
            seq: 1511,
            request_seq: 23,
            encoded: json!({
                "command": "launch",
                "success": true,
            }),
            decoded: Response::Launch,
        },
        ResponseTestCase {
            seq: 1512,
            request_seq: 24,
            encoded: json!({
                "command": "loadedSources",
                "success": true,
                "body": {
                    "sources": [{
                        "name": "name",
                        "sourceReference": 10,
                        "presentationHint": "normal",
                        "origin": "origin",
                        "adapterData": 0,
                        "checksums": [{
                            "algorithm": "MD5",
                            "checksum": "checksum",
                        }]
                    }]
                }
            }),
            decoded: Response::LoadedSources {
                body: LoadedSourcesResponse {
                    sources: vec![Source {
                        name: Some(String::from("name")),
                        source_reference: Some(SourceReference::Reference(10)),
                        presentation_hint: Some(SourcePresentationHint::Normal),
                        origin: Some(String::from("origin")),
                        sources: Vec::new(),
                        adapter_data: Some(Value::from(0)),
                        checksums: vec![Checksum {
                            algorithm: ChecksumAlgorithm::Md5,
                            checksum: String::from("checksum"),
                        }],
                    }],
                },
            },
        },
        ResponseTestCase {
            seq: 1513,
            request_seq: 25,
            encoded: json!({
                "command": "next",
                "success": true
            }),
            decoded: Response::Next,
        },
        ResponseTestCase {
            seq: 1513,
            request_seq: 25,
            encoded: json!({
                "command": "reverseContinue",
                "success": true
            }),
            decoded: Response::ReverseContinue,
        },
        ResponseTestCase {
            seq: 1514,
            request_seq: 26,
            encoded: json!({
                "command": "setBreakpoints",
                "success": true,
                "body": {
                    "breakpoints": [{
                        "id": 10,
                        "verified": true,
                        "message": "message",
                        "line": 20,
                        "column": 30,
                        "endLine": 40,
                        "endColumn": 50,
                        "offset": 0,
                    }]
                }
            }),
            decoded: Response::SetBreakpoints {
                body: SetBreakpointsResponse {
                    breakpoints: vec![Breakpoint {
                        id: Some(10),
                        verified: true,
                        message: Some(String::from("message")),
                        source: None,
                        line: Some(20),
                        column: Some(30),
                        end_line: Some(40),
                        end_column: Some(50),
                        instruction_reference: None,
                        offset: Some(0),
                    }],
                },
            },
        },
        ResponseTestCase {
            seq: 1515,
            request_seq: 27,
            encoded: json!({
                "command": "stepBack",
                "success": true
            }),
            decoded: Response::StepBack,
        },
        ResponseTestCase {
            seq: 1512,
            request_seq: 24,
            encoded: json!({
                "command": "custom",
                "success": true,
                "body": "foo"
            }),
            decoded: Response::Custom {
                body: Some("foo".into()),
            },
        },
        ResponseTestCase {
            seq: 1514,
            request_seq: 26,
            encoded: json!({
                "command": "threads",
                "success": true,
                "body": {
                    "threads": [{
                        "id": 10,
                        "name": "foo"
                    }]
                }
            }),
            decoded: Response::Threads {
                body: ThreadsResponse {
                    threads: vec![Thread {
                        id: 10,
                        name: String::from("foo"),
                    }],
                },
            },
        },
        ResponseTestCase {
            seq: 1514,
            request_seq: 26,
            encoded: json!({
                "command": "stackTrace",
                "success": true,
                "body": {
                    "stackFrames": [{
                        "id": 18,
                        "name": "foo",
                        "source": {
                            "name": "name",
                            "sourceReference": 10,
                            "presentationHint": "normal",
                            "origin": "origin",
                            "adapterData": 0,
                            "checksums": [{
                                "algorithm": "MD5",
                                "checksum": "checksum",
                            }]
                        },
                        "line": 31,
                        "column": 82,
                        "endLine": 28,
                        "endColumn": 84,
                        "canRestart": true,
                        "instructionPointerReference": "bar",
                        "moduleId": "baz",
                        "presentationHint": "label",
                    }],
                    "totalFrames": 15,
                }
            }),
            decoded: Response::StackTrace {
                body: StackTraceResponse {
                    stack_frames: vec![StackFrame {
                        id: 18,
                        name: "foo".into(),
                        source: Some(Source {
                            name: Some(String::from("name")),
                            source_reference: Some(SourceReference::Reference(10)),
                            presentation_hint: Some(SourcePresentationHint::Normal),
                            origin: Some(String::from("origin")),
                            sources: Vec::new(),
                            adapter_data: Some(Value::from(0)),
                            checksums: vec![Checksum {
                                algorithm: ChecksumAlgorithm::Md5,
                                checksum: String::from("checksum"),
                            }],
                        }),
                        line: 31,
                        column: 82,
                        end_line: Some(28),
                        end_column: Some(84),
                        can_restart: true,
                        instruction_pointer_reference: Some("bar".into()),
                        module_id: Some(StackFrameModuleId::String("baz".into())),
                        presentation_hint: Some(StackFramePresentationHint::Label),
                    }],
                    total_frames: Some(15),
                },
            },
        },
        ResponseTestCase {
            seq: 1514,
            request_seq: 26,
            encoded: json!({
                "command": "scopes",
                "success": true,
                "body": {
                    "scopes": [{
                        "name": "foo",
                        "presentationHint": "locals",
                        "variablesReference": 28,
                        "namedVariables": 92,
                        "indexedVariables": 91,
                        "expensive": true,
                        "source": {
                            "name": "name",
                            "sourceReference": 10,
                            "presentationHint": "normal",
                            "origin": "origin",
                            "adapterData": 0,
                            "checksums": [{
                                "algorithm": "MD5",
                                "checksum": "checksum",
                            }]
                        },
                        "line": 11,
                        "column": 12,
                        "endLine": 13,
                        "endColumn": 14,
                    }]
                }
            }),
            decoded: Response::Scopes {
                body: ScopesResponse {
                    scopes: vec![Scope {
                        name: "foo".into(),
                        presentation_hint: Some(ScopePresentationHint::Locals),
                        variables_reference: 28,
                        named_variables: Some(92),
                        indexed_variables: Some(91),
                        expensive: true,
                        source: Some(Source {
                            name: Some(String::from("name")),
                            source_reference: Some(SourceReference::Reference(10)),
                            presentation_hint: Some(SourcePresentationHint::Normal),
                            origin: Some(String::from("origin")),
                            sources: Vec::new(),
                            adapter_data: Some(Value::from(0)),
                            checksums: vec![Checksum {
                                algorithm: ChecksumAlgorithm::Md5,
                                checksum: String::from("checksum"),
                            }],
                        }),
                        line: Some(11),
                        column: Some(12),
                        end_line: Some(13),
                        end_column: Some(14),
                    }],
                },
            },
        },
        ResponseTestCase {
            seq: 1514,
            request_seq: 26,
            encoded: json!({
                "command": "variables",
                "success": true,
                "body": {
                    "variables": [{
                        "name": "foo",
                        "value": "bar",
                        "type": "baz",
                        "presentationHint": {
                            "kind": "dataBreakpoint",
                            "attributes": [
                                "static",
                                "constant"
                            ],
                            "visibility": "private",
                            "lazy": true
                        },
                        "evaluateName": "eval",
                        "variablesReference": 55,
                        "namedVariables": 56,
                        "indexedVariables": 57,
                        "memoryReference": "ref",
                    }]
                }
            }),
            decoded: Response::Variables {
                body: VariablesResponse {
                    variables: vec![Variable {
                        name: "foo".into(),
                        value: "bar".into(),
                        r#type: Some("baz".into()),
                        presentation_hint: Some(VariablePresentationHint {
                            kind: Some(VariablePresentationHintKind::DataBreakpoint),
                            attributes: vec![
                                VariablePresentationHintAttribute::Static,
                                VariablePresentationHintAttribute::Constant,
                            ],
                            visibility: Some(VariablePresentationHintVisibility::Private),
                            lazy: true,
                        }),
                        evaluate_name: Some("eval".into()),
                        variables_reference: 55,
                        named_variables: Some(56),
                        indexed_variables: Some(57),
                        memory_reference: Some("ref".into()),
                    }],
                },
            },
        },
    ];

    cases.into_iter().for_each(|case| case.run());
}

struct ResponseTestCase {
    seq: u64,
    request_seq: u64,
    encoded: Value,
    decoded: Response,
}

impl ResponseTestCase {
    fn run(self) {
        let Self {
            seq,
            request_seq,
            mut encoded,
            decoded,
        } = self;

        encoded["seq"] = seq.into();
        encoded["request_seq"] = request_seq.into();
        encoded["type"] = "response".into();

        let encoded =
            ProtocolMessage::try_from(&encoded).expect("failed to parse encoded protocol message");

        let protocol = decoded.into_protocol(seq, request_seq);
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
