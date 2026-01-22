# Deployment

Phase 1 runs as a single Rust process.

## Docker

```bash
docker compose up --build
```

## Future

- Split components into separate services.
- Replace the in-process bus with gRPC and a durable event store.
