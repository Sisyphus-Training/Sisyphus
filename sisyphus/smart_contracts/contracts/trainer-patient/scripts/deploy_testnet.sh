#!/bin/bash
set -e

# Check if Soroban CLI is installed
if ! command -v soroban &> /dev/null; then
    echo "Error: Soroban CLI is not installed. Please install it first."
    echo "See https://soroban.stellar.org/docs/getting-started/setup"
    exit 1
fi

# Build the contract
echo "Building contract..."
cargo build --target wasm32-unknown-unknown --release

# Set WASM file path
WASM_FILE="/Users/godswill/web3projects/sisyphustraining/Sisyphus/sisyphus/smart_contracts/target/wasm32-unknown-unknown/release/trainer_patient.wasm"

if [ ! -f "$WASM_FILE" ]; then
    echo "Error: WASM file not found at $WASM_FILE"
    exit 1
fi

echo "Found WASM file: $WASM_FILE"
soroban contract optimize --wasm "$WASM_FILE"

# Check if STELLAR_SECRET_KEY is set
if [ -z "$STELLAR_SECRET_KEY" ]; then
    echo "Please set your STELLAR_SECRET_KEY environment variable"
    echo "Example: export STELLAR_SECRET_KEY=S..."
    exit 1
fi

# Setup account for testnet
echo "Setting up Stellar testnet account..."
NETWORK="testnet"

# Make sure the network is added
stellar network add $NETWORK --network-passphrase "Test SDF Network ; September 2015" --rpc-url https://soroban-testnet.stellar.org:443 2>/dev/null || true

# Create identity from secret key
stellar account import --secret-key "$STELLAR_SECRET_KEY" deployer

# Deploy contract
echo "Deploying contract to testnet..."
CONTRACT_ID=$(soroban contract deploy \
    --wasm "$WASM_FILE" \
    --source deployer \
    --network $NETWORK)

echo "Contract deployed successfully to testnet!"
echo "Contract ID: $CONTRACT_ID"

# Save contract ID to file
echo "$CONTRACT_ID" > contract_id.txt
echo "Contract ID saved to contract_id.txt"

# Initialize the contract
echo "Initializing contract..."
ADMIN_ADDRESS=$(stellar address deployer)
soroban contract invoke \
    --id "$CONTRACT_ID" \
    --source deployer \
    --network $NETWORK \
    -- \
    initialize \
    --admin "$ADMIN_ADDRESS"

echo "Contract initialized successfully with admin: $ADMIN_ADDRESS"
echo
echo "To use this contract:"
echo "1. Export your contract ID: export CONTRACT_ID=$CONTRACT_ID"
echo "2. Register a trainer: soroban contract invoke --id \$CONTRACT_ID --source deployer --network testnet -- register_trainer --trainer_address <TRAINER_ADDRESS>"
echo
echo "See README.md for more usage examples" 