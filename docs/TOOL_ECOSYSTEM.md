# Tool Ecosystem

Tools are first-class capabilities callable by the reasoning layer.

## Phase 1

- Rust scaffolding for tools lives in `core/rust/src/tools.rs`.
- Tools are expected to produce and consume raw payloads.

## Phase 2+

- Add a dynamic tool registry (TypeScript) and remote tool invocation.
- Extend with adapters for filesystem, processes, HTTP, databases, etc.
