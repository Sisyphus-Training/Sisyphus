# Trainer-Patient Smart Contract

A Stellar/Soroban smart contract that securely links patients to personal trainers and manages fitness-related data on the blockchain.

## Overview

This contract enables personal trainers to manage their relationships with patients and share data securely. Key features include:

- **Trainer-Patient Linking**: Securely connect patients to trainers using blockchain identities
- **Patient Data Management**: Store and update exercise routines, meal plans, and progress reports
- **Access Control**: Ensure only authorized parties can view or modify data
- **Stellar Optimized**: Built for Stellar's low fees and high throughput

## Contract Architecture

The contract follows a modular design pattern with separation of concerns:

```
trainer-patient/
├── src/
│   ├── lib.rs           # Main contract entry point
│   ├── types.rs         # Data structures
│   ├── storage.rs       # Persistent storage management
│   ├── trainer.rs       # Trainer-specific functionality
│   ├── patient.rs       # Patient-specific functionality
│   ├── access.rs        # Access control and permissions
│   ├── data.rs          # Data manipulation utilities
│   ├── error.rs         # Error handling
│   └── test.rs          # Unit tests
├── scripts/
│   ├── deploy_testnet.sh # Deployment script
│   └── test_contract.sh  # Contract testing script
```

## Prerequisites

- Rust toolchain (1.68.0+)
- Soroban CLI (0.8.0+)
- Stellar account with funds (for testnet deployment)

## Installation

1. **Install Rust**:
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```

2. **Install Soroban CLI**:
   ```bash
   cargo install --locked soroban-cli
   ```

3. **Clone the repository**:
   ```bash
   git clone <repository-url>
   cd sisyphus/smart_contracts
   ```

4. **Build the contract**:
   ```bash
   cd contracts/trainer-patient
   cargo build --target wasm32-unknown-unknown --release
   soroban contract optimize --wasm target/wasm32-unknown-unknown/release/trainer_patient.wasm
   ```

## Testing

### Local Unit Tests

Run the unit tests to verify contract functionality:

```bash
cargo test
```

### Testnet Deployment and Testing

1. **Make scripts executable**:
   ```bash
   chmod +x scripts/deploy_testnet.sh
   chmod +x scripts/test_contract.sh
   ```

2. **Set up your Stellar secret key**:
   ```bash
   export STELLAR_SECRET_KEY=S...  # Your Stellar secret key
   ```

3. **Deploy to testnet**:
   ```bash
   ./scripts/deploy_testnet.sh
   ```
   This will:
   - Build the contract
   - Deploy to Stellar testnet
   - Initialize the contract
   - Save the contract ID

4. **Test with sample data**:
   ```bash
   ./scripts/test_contract.sh
   ```
   This will run a complete test flow:
   - Register a trainer
   - Add a patient
   - Update exercise routine, meal plan, and progress
   - Retrieve data as a patient

## Contract Usage

### Main Contract Functions

#### Contract Initialization

```rust
initialize(env: Env, admin: Address)
```
- Initializes the contract with an admin address
- Can only be called once

#### Trainer Operations

**Register as a trainer**:
```rust
register_trainer(env: Env, trainer_address: Address)
```

**Add a patient**:
```rust
add_patient(
    env: Env, 
    trainer: Address, 
    patient_address: Address,
    patient_id: BytesN<32>
)
```

**Update exercise routine**:
```rust
update_exercise_routine(
    env: Env,
    trainer: Address,
    patient_id: BytesN<32>,
    routine: ExerciseRoutine
)
```

**Update meal plan**:
```rust
update_meal_plan(
    env: Env,
    trainer: Address,
    patient_id: BytesN<32>,
    plan: MealPlan
)
```

**Update progress**:
```rust
update_progress(
    env: Env,
    trainer: Address,
    patient_id: BytesN<32>,
    progress: ProgressUpdate
)
```

#### Patient Operations

**Get exercise routine**:
```rust
get_exercise_routine(
    env: Env,
    patient: Address,
)
```

**Get meal plan**:
```rust
get_meal_plan(
    env: Env,
    patient: Address,
)
```

**Get progress**:
```rust
get_progress(
    env: Env,
    patient: Address,
)
```

**Get all data**:
```rust
get_all_data(
    env: Env,
    patient: Address,
)
```

### Using Soroban CLI

After deploying, you can interact with the contract using the Soroban CLI:

```bash
# Set contract ID
export CONTRACT_ID=<your_contract_id>

# Register as a trainer
soroban contract invoke \
    --id $CONTRACT_ID \
    --source <your_identity> \
    --network testnet \
    -- \
    register_trainer \
    --trainer_address <trainer_address>
```

Check `scripts/test_contract.sh` for more examples of CLI interaction.

## Data Structures

### Exercise Routine

```rust
pub struct ExerciseRoutine {
    pub name: String,
    pub description: String,
    pub exercises: Vec<Exercise>,
    pub last_updated: u64,
}

pub struct Exercise {
    pub name: String,
    pub sets: u32,
    pub reps: u32,
    pub description: String,
    pub video_link: String,
    pub notes: String,
}
```

### Meal Plan

```rust
pub struct MealPlan {
    pub name: String,
    pub description: String,
    pub meals: Vec<Meal>,
    pub last_updated: u64,
}

pub struct Meal {
    pub name: String,
    pub time: String,
    pub foods: Vec<String>,
    pub calories: u32,
    pub notes: String,
}
```

### Progress Update

```rust
pub struct ProgressUpdate {
    pub date: u64,
    pub metrics: Map<String, String>,
    pub notes: String,
    pub last_updated: u64,
}
```

## Security Considerations

- Only trainers can add patients and update their data
- Only patients can access their own data
- All operations verify the caller's identity
- Data is stored with proper isolation between patients

## Performance Optimization

The contract is optimized for Stellar's environment:
- Uses short symbols for storage keys
- Minimizes storage operations
- Uses efficient data structures
- Clean interfaces to reduce transaction complexity

## Mainnet Deployment Checklist

Before deploying to mainnet:
1. Complete a thorough security audit
2. Test with a variety of real-world scenarios
3. Optimize gas usage further if needed
4. Develop a monitoring and maintenance plan
5. Consider backup and recovery strategies

## License

This project is licensed under the MIT License - see the LICENSE file for details.