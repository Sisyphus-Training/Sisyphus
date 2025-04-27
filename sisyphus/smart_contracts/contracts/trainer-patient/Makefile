all: build

build:
	cargo build --target wasm32-unknown-unknown --release
	soroban contract optimize --wasm target/wasm32-unknown-unknown/release/trainer_patient.wasm

test:
	cargo test

clean:
	cargo clean
	rm -rf .soroban

.PHONY: all build test clean 