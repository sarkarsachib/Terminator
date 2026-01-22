# Terminator Architecture

This repository is a **polyglot scaffold** for an autonomous AI system. The implemented foundation focuses on an event-driven, componentized runtime that can later be distributed across processes and languages.

## Meta-architecture layer

The meta-architecture is defined in `core/architecture.md`.

## Layers

1. **Input layer**: multimodal stream adapters (vision/audio/motion/internet/text).
2. **Learning engine**: Observer, Trainer, Hybrid pipelines (Python services).
3. **Knowledge representation**: semantic graph + memory systems.
4. **Reasoning engine**: context aggregation, tool selection, decisions.
5. **Tool ecosystem**: first-class executable capabilities.
6. **Output interface**: selective speaking via output gating.
7. **Personality & autonomy**: configurable traits + controlled disorder.

## Runtime model (Phase 1)

- `terminator-core` (Rust) boots Tokio, provides an in-process message bus and starts components.
- Components are independent tasks communicating via events.

## Distribution model (Phase 2+)

- Move components into separate processes by replacing the in-process bus with the gRPC `MessageBus` service defined in `core/protocol.proto`.
