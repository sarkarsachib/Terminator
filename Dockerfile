# Minimal development container for the Rust orchestrator.
FROM rust:1.76-slim AS build
WORKDIR /app
COPY . .
RUN cargo build -p terminator-core --release

FROM debian:bookworm-slim
WORKDIR /app
COPY --from=build /app/target/release/terminator-core /usr/local/bin/terminatord
CMD ["/usr/local/bin/terminatord"]
