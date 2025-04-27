#!/bin/bash
set -e

# Check if CONTRACT_ID is set
if [ -z "$CONTRACT_ID" ]; then
    if [ -f "contract_id.txt" ]; then
        export CONTRACT_ID=$(cat contract_id.txt)
        echo "Using contract ID from file: $CONTRACT_ID"
    else
        echo "Error: CONTRACT_ID environment variable not set and contract_id.txt not found"
        echo "Please run deploy_testnet.sh first or set CONTRACT_ID manually"
        exit 1
    fi
fi

NETWORK="testnet"

echo "Checking contract status for $CONTRACT_ID on Stellar testnet..."

# Get WASM from testnet
echo "Retrieving WASM from testnet..."
soroban contract inspect $CONTRACT_ID --network $NETWORK

# Check contract ledger entries
echo "Checking contract ledger entries..."
soroban contract events --id $CONTRACT_ID --network $NETWORK --limit 10

echo "Contract check completed" 