# User Registry Smart Contract

A Soroban smart contract for the Stellar blockchain that creates, stores, and retrieves user information for the Sisyphus Exercise Routine Builder application.

## Overview

This smart contract establishes a secure and efficient foundation for storing user data on the Stellar blockchain. It enables seamless onboarding of new users into the Sisyphus Exercise Routine Builder ecosystem and provides a scalable mechanism for retrieving user information when needed.

## Current Status

✅ **Deployed and Tested**: The contract has been successfully deployed and tested on the Stellar testnet. The deployed contract ID is `CAMLNBEOERJ67J2G6PJQTSZGC767RIRIBCPZ25OG7NS23PSQPPTF74S2`. You can view it on the [Stellar Explorer](https://stellar.expert/explorer/testnet/contract/CAMLNBEOERJ67J2G6PJQTSZGC767RIRIBCPZ25OG7NS23PSQPPTF74S2).

All core functionality has been verified, including:
- Creating new users
- Retrieving user data
- Updating user information
- Deleting users
- Authentication protections

## Features

- **Create User**: Register a new user with name, email, and metadata
- **Update User**: Modify existing user information
- **Get User**: Retrieve a user's information by their unique identifier (Stellar address)
- **Delete User**: Remove a user from the registry
- **Security**: Authentication to ensure only the user or authorized parties can modify their data

## Directory Structure

```
user-registry/
├── src/               # Contract source code
│   ├── lib.rs         # Main contract implementation
│   └── test.rs        # Contract tests
├── client-examples/   # Client libraries for interacting with the contract
│   ├── javascript-client.js  # JavaScript client example
│   ├── typescript-client.ts  # TypeScript client example
│   └── package.json   # Dependencies for client examples
├── Cargo.toml         # Contract dependencies
├── Makefile           # Build and test commands
├── README.md          # This file
└── DEPLOYMENT.md      # Deployment instructions
```

## Contract Functions

### `create_user`

Creates a new user with the provided data.

**Arguments**:
- `user_id`: The Stellar address that uniquely identifies the user
- `name`: The user's name
- `email`: The user's email address
- `metadata`: Additional user metadata as key-value pairs

**Returns**: Boolean indicating success or failure

### `update_user`

Updates an existing user's information.

**Arguments**:
- `user_id`: The user's unique Stellar address
- `name`: The user's updated name
- `email`: The user's updated email address
- `metadata`: Updated metadata as key-value pairs

**Returns**: Boolean indicating success or failure

### `get_user`

Retrieves a user's information.

**Arguments**:
- `user_id`: The user's unique Stellar address

**Returns**: `UserData` object if found, or `None` if not found

### `delete_user`

Removes a user from the registry.

**Arguments**:
- `user_id`: The user's unique Stellar address

**Returns**: Boolean indicating success or failure

## Getting Started

### Prerequisites

- Rust toolchain
- WebAssembly target
- Stellar CLI (`stellar` command)

### Building

```bash
# Build the contract
make build
```

### Testing

```bash
# Run the tests
make test
```

## Deployment

For detailed deployment instructions, see the [DEPLOYMENT.md](DEPLOYMENT.md) file.

## Client Integration

The `client-examples` directory contains example code for integrating the contract with front-end applications. Currently, there are examples for:

- JavaScript
- TypeScript

To use these examples:

1. Navigate to the client-examples directory:
   ```bash
   cd client-examples
   ```

2. Install dependencies:
   ```bash
   npm install
   ```

3. Modify the example code with your contract ID and account information.

4. Run the example:
   ```bash
   npm start
   ```

## Security Considerations

- The contract uses Stellar's authentication system to ensure that only the user (or an authorized entity) can modify their data
- User IDs are tied to Stellar addresses, providing a secure way to identify users
- Always audit smart contracts before deploying to production
- Keep private keys secure and never hardcode them in your applications

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

This project is licensed under the MIT License - see the LICENSE file for details. 