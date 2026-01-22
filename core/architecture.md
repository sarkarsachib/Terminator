# Terminator Meta-Architecture (Phase 1)

This document defines the initial **unified meta-architecture** that Terminator uses to evolve from a scaffold into a multi-language autonomous system.

## Architectural synthesis

Terminator combines several long-lived architectural patterns into a single runtime model:

- **Actor / Reactive**: every component is an independent concurrent task communicating via async messages.
- **Microservice mindset**: components are independently deployable, but Phase 1 runs them in-process.
- **Clean Architecture**: orchestration depends only on interfaces; external systems are adapters.
- **CQRS + Event Sourcing**: commands/events flow through a bus; state is derived from an event log.
- **Hexagonal (Ports/Adapters)**: the orchestrator defines ports; implementations live at the edges.
- **Graph cognition**: knowledge is represented as a mutable semantic graph (scaffolded).

## Runtime model

- **Orchestrator** (Rust): boots the runtime, loads config, starts components.
- **Message bus** (Rust): pub/sub event stream + bounded event log.
- **Components**: long-running tasks implementing a common interface.
- **Tools**: callable capabilities exposed to the reasoning layer.
- **Output gating**: policy engine to suppress or emit outputs.

## Non-tokenized interfaces

All cross-component payloads are treated as **raw bytes**. Higher-level structure is layered on top (e.g. Protobuf, JSON, domain-specific binary formats).

## Next steps

Phase 2 adds:

- gRPC adapters around the message bus and tool registry
- Python learning services connected via RPC
- durable persistence for event log and memories
