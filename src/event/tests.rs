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
