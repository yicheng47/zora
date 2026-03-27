.PHONY: build run test lint clean check fmt

build:
	cargo build --release

run:
	cargo run -- serve

test:
	cargo test

check:
	cargo check

lint:
	cargo clippy -- -D warnings

fmt:
	cargo fmt

clean:
	cargo clean
