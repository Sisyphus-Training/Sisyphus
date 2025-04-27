# User Registry Contract Deployment Guide

This document provides step-by-step instructions for deploying the User Registry smart contract to the Stellar testnet and eventually to the Stellar mainnet.

## Prerequisites

1. Install Rust and Cargo:
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```

2. Add the WebAssembly target:
   ```bash
   rustup target add wasm32-unknown-unknown
   ```

3. Install Soroban CLI:
   ```bash
   cargo install stellar-cli
   ```

## Building the Contract

1. Navigate to the smart_contracts directory:
   ```bash
   cd /path/to/sisyphus/smart_contracts
   ```

2. Build the contract:
   ```bash
   cd contracts/user-registry
   make build
   ```

   This will compile the contract to WebAssembly at `../../target/wasm32-unknown-unknown/release/user_registry.wasm`.

## Testing the Contract Locally

1. Run tests to ensure everything is working as expected:
   ```bash
   make test
   ```

   All tests should pass before proceeding to deployment.

## Deploying to Stellar Testnet

1. Set up your Stellar CLI configuration:
   ```bash
   stellar network add testnet --rpc-url https://soroban-testnet.stellar.org:443 --network-passphrase "Test SDF Network ; September 2015"
   ```

2. Generate a test account:
   ```bash
   stellar keys generate account-name
   ```
   This command will create and fund a new test account on the testnet.

3. Deploy the contract:
   ```bash
   stellar contract deploy --wasm ../../target/wasm32-unknown-unknown/release/user_registry.wasm --source account-name --network testnet
   ```

   This command will output the contract ID, which you'll need for interacting with the contract.

4. Save the contract ID:
   ```bash
   export CONTRACT_ID=<DEPLOYED_CONTRACT_ID>
   ```

## Interacting with the Deployed Contract

### Creating a User

First, create a user account:
```bash
stellar keys generate user1
```

Then create a user profile in the registry:
```bash
stellar contract invoke --id $CONTRACT_ID --source user1 --network testnet -- create_user --user_id <USER_PUBLIC_KEY> --name "John Doe" --email "john.doe@example.com" --metadata '{"account_type":"standard"}'
```

### Retrieving a User

```bash
stellar contract invoke --id $CONTRACT_ID --source account-name --network testnet -- get_user --user_id <USER_PUBLIC_KEY>
```

### Updating a User

```bash
stellar contract invoke --id $CONTRACT_ID --source user1 --network testnet -- update_user --user_id <USER_PUBLIC_KEY> --name "John Smith" --email "john.smith@example.com" --metadata '{"account_type":"premium","subscription":"annual"}'
```

### Deleting a User

```bash
stellar contract invoke --id $CONTRACT_ID --source user1 --network testnet -- delete_user --user_id <USER_PUBLIC_KEY>
```

## Deploying to Stellar Mainnet

⚠️ **Important**: Before deploying to mainnet, ensure thorough testing on testnet and consider a security audit.

1. Set up your Stellar CLI configuration for mainnet:
   ```bash
   stellar network add mainnet --rpc-url https://soroban-rpc.mainnet.stellar.org:443 --network-passphrase "Public Global Stellar Network ; September 2015"
   ```

2. Ensure your mainnet account has sufficient funds (XLM).

3. Deploy the contract to mainnet:
   ```bash
   stellar contract deploy --wasm ../../target/wasm32-unknown-unknown/release/user_registry.wasm --source <YOUR_MAINNET_ACCOUNT> --network mainnet
   ```

4. Save the mainnet contract ID:
   ```bash
   export MAINNET_CONTRACT_ID=<DEPLOYED_MAINNET_CONTRACT_ID>
   ```

## Integrating with Frontend Applications

Once deployed, you can integrate the contract with frontend applications using the Stellar JavaScript SDK or other language-specific SDKs.

### JavaScript/TypeScript Example:

```javascript
import { Contract } from 'stellar-sdk';
import { SorobanRpc } from '@stellar/ts-soroban-sdk';

// Setup RPC server connection
const server = new SorobanRpc.Server("https://soroban-testnet.stellar.org:443");

// Load the contract
const contractId = "<YOUR_CONTRACT_ID>";
const contract = new Contract(contractId);

// Create a user
async function createUser(userAccount, userName, userEmail, metadata) {
  const result = await contract.call(
    "create_user",
    userAccount,
    userName,
    userEmail,
    metadata
  );
  return result;
}

// Get user data
async function getUser(userAccount) {
  const result = await contract.call(
    "get_user",
    userAccount
  );
  return result;
}
```

## Security Considerations

When working with the contract:

1. Always verify the user has proper authorization before modifying their data
2. Keep sensitive information in the metadata to a minimum
3. Use proper error handling in your client applications
4. Regularly monitor contract usage and performance

## Troubleshooting

If you encounter issues:

1. Verify your account has sufficient XLM for transaction fees
2. Check that you're using the correct contract ID in your invocations
3. Ensure the user has proper authorization to perform the requested action
4. For read-only operations, add `--send=yes` if you need to submit the transaction

For detailed logs, use the `--verbose` flag with your commands. 