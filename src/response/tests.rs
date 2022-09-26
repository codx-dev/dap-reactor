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
                    presentation_hint: VariablePresentationHint {
                        kind: Kind::Property,
                        attributes: Some(vec![Attributes::Static, Attributes::Constant]),
                        visibility: Some(Visibility::Public),
                        lazy: true,
                    },
                    variables_reference: 2,
                    named_variables: Some(4),
                    indexed_variables: Some(3),
                    memory_reference: Some(String::from("memoryReference")),
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
