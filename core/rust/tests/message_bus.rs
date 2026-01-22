use terminator_core::{
    config::{OutputConfig, OutputMode},
    message_bus::{Message, MessageBus},
    output::{GatingEngine, OutputDecision, OutputSignal},
};

#[tokio::test]
async fn message_bus_delivers_messages_to_subscribers() {
    let bus = MessageBus::new(100);
    let mut rx = bus.subscribe();

    bus.publish(Message::new("test.topic", b"hello".to_vec()));

    let msg = rx.recv().await.expect("recv");
    assert_eq!(msg.topic, "test.topic");
    assert_eq!(&msg.payload, b"hello");
}

#[test]
fn gating_engine_respects_silent_mode() {
    let engine = GatingEngine::new(OutputConfig {
        mode: OutputMode::Silent,
        confidence_threshold: 0.9,
    });

    let decision = engine.decide(&OutputSignal {
        user_requested: false,
        critical: true,
        confidence: 1.0,
    });
    assert!(matches!(decision, OutputDecision::Suppress));

    let decision = engine.decide(&OutputSignal {
        user_requested: true,
        critical: false,
        confidence: 0.0,
    });
    assert!(matches!(decision, OutputDecision::Emit));
}
