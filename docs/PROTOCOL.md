# Protocols

## Message format

All cross-component communication is represented as a `RawMessage`:

- `topic`: routing key
- `payload`: raw bytes (no tokenization requirement)
- `id`, `timestamp`: tracing and ordering

See `core/protocol.proto`.

## RPC

Phase 1 uses in-process pub/sub. Phase 2 will expose the bus as gRPC:

- `Publish`: submit a message to the bus
- `Subscribe`: receive a filtered stream
