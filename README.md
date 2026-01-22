# Terminator

Terminator is a polyglot research scaffold for an autonomous AI system built around **event-driven orchestration**, **ports/adapters**, and **tool-first execution**.

This repository currently implements **Phase 1 (Foundation)** of the architecture:

- Rust orchestrator runtime (Tokio)
- In-process message bus (actor-style pub/sub) with event log (event sourcing inspired)
- Core component interfaces (clean architecture boundaries)
- Output gating ("selective speaking")
- Tool registry scaffolding
- Protocol Buffer specifications for future gRPC interop
- Configuration + documentation structure

## Quick start (Rust)

```bash
cargo test
cargo run -p terminator-core
```

## Repository layout

See `docs/ARCHITECTURE.md` for the full meta-architecture and `docs/DEVELOPMENT.md` for contribution guidelines.
