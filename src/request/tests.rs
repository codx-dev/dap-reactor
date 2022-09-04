use crate::prelude::*;
use serde_json::{json, Value};

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
