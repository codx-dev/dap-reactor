use super::*;
use crate::models::*;
use crate::prelude::*;

use serde_json::json;

#[test]
fn encode_requests() {
    let cases = vec![
        RequestTestCase {
            seq: 12,
            encoded: json!({
                "command": "attach",
                "arguments": {
                    "__restart": "some-test",
                }
            }),
            decoded: Request::Attach {
                arguments: AttachArguments {
                    restart: Some(Value::String("some-test".into())),
                },
            },
        },
        RequestTestCase {
            seq: 13,
            encoded: json!({
                "command": "restart",
                "arguments": {
                    "noDebug": true,
                    "__restart": "some-test",
                }
            }),
            decoded: Request::Restart {
                arguments: Some(RestartArguments::Launch(LaunchArguments {
                    no_debug: true,
                    restart: Some(Value::String("some-test".into())),
                })),
            },
        },
        RequestTestCase {
            seq: 14,
            encoded: json!({
                "command": "disconnect",
                "arguments": {
                    "restart": true,
                    "terminateDebuggee": true,
                    "suspendDebuggee": true,
                }
            }),
            decoded: Request::Disconnect {
                arguments: Some(DisconnectArguments {
                    restart: true,
                    terminate_debuggee: true,
                    suspend_debuggee: true,
                }),
            },
        },
        RequestTestCase {
            seq: 15,
            encoded: json!({
                "command": "terminate",
                "arguments": {
                    "restart": true,
                }
            }),
            decoded: Request::Terminate {
                arguments: Some(TerminateArguments { restart: true }),
            },
        },
        RequestTestCase {
            seq: 16,
            encoded: json!({
                "command": "breakpointLocations",
                "arguments": {
                    "source": {
                        "name": "hello",
                        "origin": "home",
                    },
                    "line": 50,
                    "endColumn": 55,
                }
            }),
            decoded: Request::BreakpointLocations {
                arguments: Some(BreakpointLocationsArguments {
                    source: Source {
                        name: Some(String::from("hello")),
                        source_reference: None,
                        presentation_hint: None,
                        origin: Some(String::from("home")),
                        sources: Vec::new(),
                        adapter_data: None,
                        checksums: Vec::new(),
                    },
                    line: 50,
                    column: None,
                    end_line: None,
                    end_column: Some(55),
                }),
            },
        },
        RequestTestCase {
            seq: 17,
            encoded: json!({
                "command": "configurationDone",
                "arguments": {},
            }),
            decoded: Request::ConfigurationDone {
                arguments: Some(ConfigurationDoneArguments {}),
            },
        },
        RequestTestCase {
            seq: 18,
            encoded: json!({
                "command": "continue",
                "arguments": {
                    "threadId": 40,
                    "singleThread": true
                }
            }),
            decoded: Request::Continue {
                arguments: ContinueArguments {
                    thread_id: 40,
                    single_thread: true,
                },
            },
        },
        RequestTestCase {
            seq: 19,
            encoded: json!({
                "command": "evaluate",
                "arguments": {
                    "expression": "expression",
                    "frameId": 60,
                    "context": "context",
                    "format": {
                        "hex": true
                    }
                }
            }),
            decoded: Request::Evaluate {
                arguments: EvaluateArguments {
                    expression: String::from("expression"),
                    frame_id: Some(60),
                    context: Some(Context::Custom(String::from("context"))),
                    format: Some(ValueFormat { hex: true }),
                },
            },
        },
        RequestTestCase {
            seq: 20,
            encoded: json!({
                "command": "exceptionInfo",
                "arguments": {
                    "threadId": 20
                }
            }),
            decoded: Request::ExceptionInfo {
                arguments: ExceptionInfoArguments { thread_id: 20 },
            },
        },
        RequestTestCase {
            seq: 21,
            encoded: json!({
                "command": "goto",
                "arguments": {
                    "threadId": 10,
                    "targetId": 11,
                }
            }),
            decoded: Request::Goto {
                arguments: GotoArguments {
                    thread_id: 10,
                    target_id: 11,
                },
            },
        },
        RequestTestCase {
            seq: 22,
            encoded: json!({
                "command": "initialize",
                "arguments": {
                    "clientId": "clientId",
                    "clientName": "clientName",
                    "adapterId": "adapterId",
                    "locale": "locale",
                    "linesStartAt1": true,
                    "columnStartAt1": true,
                    "pathFormat": "uri",
                    "supportsVariableType": true,
                    "supportsVariablePaging": true,
                    "supportsRunInTerminalRequest": true,
                    "supportsMemoryReferences": true,
                    "supportsProgressReporting": true,
                    "supportsInvalidatedEvent": true,
                    "supportsMemoryEvent": true,
                    "supportsArgsCanBeInterpretedByShell": true,
                }
            }),
            decoded: Request::Initialize {
                arguments: InitializeArguments {
                    client_id: Some(String::from("clientId")),
                    client_name: Some(String::from("clientName")),
                    adapter_id: String::from("adapterId"),
                    locale: Some(String::from("locale")),
                    lines_start_at_1: true,
                    column_start_at_1: true,
                    path_format: Some(PathFormat::Uri),
                    supports_variable_type: true,
                    supports_variable_paging: true,
                    supports_run_in_terminal_request: true,
                    supports_memory_references: true,
                    supports_progress_reporting: true,
                    supports_invalidated_event: true,
                    supports_memory_event: true,
                    supports_args_can_be_interpreted_by_shell: true,
                },
            },
        },
        RequestTestCase {
            seq: 23,
            encoded: json!({
                "command": "launch",
                "arguments": {
                    "noDebug": true,
                    "__restart": 0
                }
            }),
            decoded: Request::Launch {
                arguments: LaunchArguments {
                    no_debug: true,
                    restart: Some(Value::from(0)),
                },
            },
        },
        RequestTestCase {
            seq: 24,
            encoded: json!({
                "command": "loadedSources",
                "arguments": {}
            }),
            decoded: Request::LoadedSources {
                arguments: Some(LoadedSourcesArguments {}),
            },
        },
        RequestTestCase {
            seq: 25,
            encoded: json!({
                "command": "next",
                "arguments": {
                    "threadId": 50,
                    "singleThread": true,
                    "granularity": "instruction"
                }
            }),
            decoded: Request::Next {
                arguments: Some(NextArguments {
                    thread_id: 50,
                    single_thread: true,
                    granularity: Some(SteppingGranularity::Instruction),
                }),
            },
        },
        RequestTestCase {
            seq: 26,
            encoded: json!({
                "command": "reverseContinue",
                "arguments": {
                    "threadId": 40,
                    "singleThread": true
                }
            }),
            decoded: Request::ReverseContinue {
                arguments: ReverseContinueArguments {
                    thread_id: 40,
                    single_thread: true,
                },
            },
        },
        RequestTestCase {
            seq: 27,
            encoded: json!({
                "command": "setBreakpoints",
                "arguments": {
                    "source": {
                        "name": "hello",
                        "origin": "home",
                    },
                    "breakpoints": [{
                        "line": 100,
                        "column": 50,
                        "condition": "condition",
                        "hitCondition": "hitCondition",
                        "logMessage": "logMessage",
                    }],
                    "lines": [40, 50, 60],
                    "sourceModified": true
                }
            }),
            decoded: Request::SetBreakpoints {
                arguments: SetBreakpointsArguments {
                    source: Source {
                        name: Some(String::from("hello")),
                        source_reference: None,
                        presentation_hint: None,
                        origin: Some(String::from("home")),
                        sources: Vec::new(),
                        adapter_data: None,
                        checksums: Vec::new(),
                    },
                    breakpoints: vec![SourceBreakpoint {
                        line: 100,
                        column: Some(50),
                        condition: Some(String::from("condition")),
                        hit_condition: Some(String::from("hitCondition")),
                        log_message: Some(String::from("logMessage")),
                    }],
                    lines: vec![40, 50, 60],
                    source_modified: true,
                },
            },
        },
        RequestTestCase {
            seq: 28,
            encoded: json!({
                "command": "stepBack",
                "arguments": {
                    "threadId": 60,
                    "singleThread": true,
                    "granularity": "line"
                }
            }),
            decoded: Request::StepBack {
                arguments: StepBackArguments {
                    thread_id: 60,
                    single_thread: true,
                    granularity: Some(SteppingGranularity::Line),
                },
            },
        },
        RequestTestCase {
            seq: 28,
            encoded: json!({
                "command": "customAddBreakpoint",
                "arguments": {
                    "breakpoint": {
                    "id": 60,
                    "verified": true,
                    "message": "foo",
                    "source": {
                        "name": "hello",
                        "origin": "home",
                    },
                    "line": 50,
                    "column": 90,
                    "endLine": 110,
                    "endColumn": 120,
                    "instructionReference": "bar",
                    "offset": 130,
                    }
                }
            }),
            decoded: Request::CustomAddBreakpoint {
                arguments: CustomAddBreakpointArguments {
                    breakpoint: Breakpoint {
                        id: Some(60),
                        verified: true,
                        message: Some(String::from("foo")),
                        source: Some(Source {
                            name: Some(String::from("hello")),
                            source_reference: None,
                            presentation_hint: None,
                            origin: Some(String::from("home")),
                            sources: Vec::new(),
                            adapter_data: None,
                            checksums: Vec::new(),
                        }),
                        line: Some(50),
                        column: Some(90),
                        end_line: Some(110),
                        end_column: Some(120),
                        instruction_reference: Some(String::from("bar")),
                        offset: Some(130),
                    },
                },
            },
        },
        RequestTestCase {
            seq: 28,
            encoded: json!({
                "command": "customRemoveBreakpoint",
                "arguments": {
                    "id": 15,
                }
            }),
            decoded: Request::CustomRemoveBreakpoint {
                arguments: CustomRemoveBreakpointArguments { id: 15 },
            },
        },
    ];

    cases.into_iter().for_each(|case| case.run());
}

struct RequestTestCase {
    seq: u64,
    encoded: Value,
    decoded: Request,
}

impl RequestTestCase {
    fn run(self) {
        let Self {
            seq,
            mut encoded,
            decoded,
        } = self;

        encoded["seq"] = seq.into();
        encoded["type"] = "request".into();

        let encoded =
            ProtocolMessage::try_from(&encoded).expect("failed to parse encoded protocol message");

        let protocol = decoded.into_protocol(seq);
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
