# Component Specification

## Component contract

A component is a long-running concurrent unit that:

- has a stable name
- is started by the orchestrator
- subscribes to the message bus and/or publishes messages

Rust interface: `core/rust/src/interfaces.rs`.

## Canonical topics

- `core.*`: orchestration + lifecycle events
- `input.*`: multimodal observation events
- `learning.*`: learning signals and training outcomes
- `knowledge.*`: graph/memory updates
- `reasoning.*`: context and decisions
- `tools.*`: tool calls/results
- `output.*`: emit/suppress decisions
- `personality.*`: trait and disorder state

## State and event sourcing

When a component maintains durable state, it should be derivable from an append-only event stream.
