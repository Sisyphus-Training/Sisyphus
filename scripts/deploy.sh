#!/bin/bash

set -e

echo "Cleaning previous builds..."
cd contracts/trainer_patient_link
cargo clean

echo "Updating dependencies..."
cargo update

echo "Building contract..."
cargo build --target wasm32-unknown-unknown --release

echo "Optimizing WASM..."
soroban contract optimize --wasm target/wasm32-unknown-unknown/release/trainer_patient_link.wasm

echo "Deploying to testnet..."
CONTRACT_ID=$(soroban contract deploy \
    --wasm target/wasm32-unknown-unknown/release/trainer_patient_link.wasm \
    --source alice \
    --network testnet)

echo "Contract deployed with ID: $CONTRACT_ID"

echo "Initializing contract..."
soroban contract invoke \
    --id $CONTRACT_ID \
    --source alice \
    --network testnet \
    -- \
    initialize

echo "Contract initialized successfully!"
echo "Contract ID: $CONTRACT_ID" > ../../contract_id.txt
