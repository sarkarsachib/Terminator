# API Reference

Phase 1 public API is the Rust crate surface.

- `terminator_core::run()` starts the runtime.
- `terminator_core::message_bus::MessageBus` is the event bus.
- `terminator_core::interfaces::Component` defines component lifecycle.
- `terminator_core::output::GatingEngine` implements selective speaking.

Future external API will be defined via gRPC in `core/protocol.proto`.
