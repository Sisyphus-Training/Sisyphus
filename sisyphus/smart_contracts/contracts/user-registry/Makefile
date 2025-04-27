all: build

build:
	cargo build --target wasm32-unknown-unknown --release
	@ls -l ../../target/wasm32-unknown-unknown/release/user_registry.wasm

test:
	cargo test

clean:
	cargo clean

.PHONY: all build test clean 