# Development

## Requirements (Phase 1)

- Rust toolchain (stable)

## Commands

```bash
cargo test
cargo run -p terminator-core
```

## Principles

- Prefer message passing over shared state.
- Define ports at the core; implement adapters at the edges.
- Treat payloads as raw bytes; use schemas only at boundaries.
