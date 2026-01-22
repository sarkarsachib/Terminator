.PHONY: test run fmt

test:
	cargo test

run:
	cargo run -p terminator-core

fmt:
	cargo fmt
