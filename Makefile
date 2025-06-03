.PHONY: build test deploy clean

build:
	cd contracts/trainer_patient_link && \
	cargo build --target wasm32-unknown-unknown --release && \
	soroban contract optimize --wasm target/wasm32-unknown-unknown/release/trainer_patient_link.wasm

test:
	cd contracts/trainer_patient_link && \
	cargo test

deploy:
	./scripts/deploy.sh

test-integration:
	./scripts/test_interactions.sh

clean:
	cd contracts/trainer_patient_link && \
	cargo clean

fmt:
	cd contracts/trainer_patient_link && \
	cargo fmt

clippy:
	cd contracts/trainer_patient_link && \
	cargo clippy -- -D warnings

all: fmt clippy build test
