use super::*;
use crate::prelude::{ValueFormat, *};

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
        // FIXME: The test cases with bool fields and false values don't pass the test
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
                    context: Some(String::from("context")),
                    format: Some(ValueFormat { hex: true }),
                },
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
