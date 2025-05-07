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

# Check if STELLAR_SECRET_KEY is set
if [ -z "$STELLAR_SECRET_KEY" ]; then
    echo "Please set your STELLAR_SECRET_KEY environment variable"
    echo "Example: export STELLAR_SECRET_KEY=S..."
    exit 1
fi

NETWORK="testnet"
ADMIN="deployer"

# Setup additional identities for testing
echo "Setting up test identities..."
# Generate random secret keys for trainer and patient
TRAINER_SECRET=$(soroban keys generate)
PATIENT_SECRET=$(soroban keys generate)

# Extract public and secret keys
TRAINER_PUBLIC=$(echo "$TRAINER_SECRET" | grep "Public" | awk '{print $3}')
TRAINER_SECRET=$(echo "$TRAINER_SECRET" | grep "Secret" | awk '{print $3}')
PATIENT_PUBLIC=$(echo "$PATIENT_SECRET" | grep "Public" | awk '{print $3}')
PATIENT_SECRET=$(echo "$PATIENT_SECRET" | grep "Secret" | awk '{print $3}')

# Add identities to Soroban config
soroban config identity add trainer --global --secret-key "$TRAINER_SECRET"
soroban config identity add patient --global --secret-key "$PATIENT_SECRET"

# Fund the accounts on testnet
echo "Funding test accounts on testnet..."
soroban config identity fund trainer --network $NETWORK
soroban config identity fund patient --network $NETWORK

echo "Trainer address: $TRAINER_PUBLIC"
echo "Patient address: $PATIENT_PUBLIC"

# Register trainer
echo "1. Registering trainer..."
soroban contract invoke \
    --id "$CONTRACT_ID" \
    --source trainer \
    --network $NETWORK \
    -- \
    register_trainer \
    --trainer_address "$TRAINER_PUBLIC"

echo "Trainer registered successfully!"

# Generate patient ID (hash of patient address)
PATIENT_ID=$(soroban contract invoke \
    --id "$CONTRACT_ID" \
    --source trainer \
    --network $NETWORK \
    --fn xdr_to_bytes \
    --arg "$PATIENT_PUBLIC" \
    | xxd -p -r | sha256sum | awk '{print $1}')

echo "Patient ID (for reference): $PATIENT_ID"

# Add patient to trainer
echo "2. Adding patient to trainer..."
soroban contract invoke \
    --id "$CONTRACT_ID" \
    --source trainer \
    --network $NETWORK \
    -- \
    add_patient \
    --trainer "$TRAINER_PUBLIC" \
    --patient_address "$PATIENT_PUBLIC" \
    --patient_id "$PATIENT_ID"

echo "Patient added successfully!"

# Create an exercise routine
echo "3. Adding exercise routine for patient..."
soroban contract invoke \
    --id "$CONTRACT_ID" \
    --source trainer \
    --network $NETWORK \
    -- \
    update_exercise_routine \
    --trainer "$TRAINER_PUBLIC" \
    --patient_id "$PATIENT_ID" \
    --routine "{\"name\":\"Beginner Workout\",\"description\":\"Full body workout for beginners\",\"exercises\":[{\"name\":\"Push-ups\",\"sets\":3,\"reps\":10,\"description\":\"Standard push-ups\",\"video_link\":\"https://example.com/pushups\",\"notes\":\"Keep back straight\"}],\"last_updated\":0}"

echo "Exercise routine added successfully!"

# Add meal plan
echo "4. Adding meal plan for patient..."
soroban contract invoke \
    --id "$CONTRACT_ID" \
    --source trainer \
    --network $NETWORK \
    -- \
    update_meal_plan \
    --trainer "$TRAINER_PUBLIC" \
    --patient_id "$PATIENT_ID" \
    --plan "{\"name\":\"High Protein Diet\",\"description\":\"Meal plan focused on protein\",\"meals\":[{\"name\":\"Breakfast\",\"time\":\"8:00 AM\",\"foods\":[\"Eggs\",\"Oatmeal\",\"Protein Shake\"],\"calories\":500,\"notes\":\"Eat within 1 hour of waking up\"}],\"last_updated\":0}"

echo "Meal plan added successfully!"

# Add progress update
echo "5. Adding progress update for patient..."
soroban contract invoke \
    --id "$CONTRACT_ID" \
    --source trainer \
    --network $NETWORK \
    -- \
    update_progress \
    --trainer "$TRAINER_PUBLIC" \
    --patient_id "$PATIENT_ID" \
    --progress "{\"date\":$(date +%s),\"metrics\":{\"weight\":\"180\",\"body_fat\":\"15%\"},\"notes\":\"Good progress this week\",\"last_updated\":0}"

echo "Progress update added successfully!"

# Patient retrieves their data
echo "6. Patient retrieving their exercise routine..."
soroban contract invoke \
    --id "$CONTRACT_ID" \
    --source patient \
    --network $NETWORK \
    -- \
    get_exercise_routine \
    --patient "$PATIENT_PUBLIC"

echo "7. Patient retrieving their meal plan..."
soroban contract invoke \
    --id "$CONTRACT_ID" \
    --source patient \
    --network $NETWORK \
    -- \
    get_meal_plan \
    --patient "$PATIENT_PUBLIC"

echo "8. Patient retrieving their progress..."
soroban contract invoke \
    --id "$CONTRACT_ID" \
    --source patient \
    --network $NETWORK \
    -- \
    get_progress \
    --patient "$PATIENT_PUBLIC"

echo "9. Patient retrieving all their data..."
soroban contract invoke \
    --id "$CONTRACT_ID" \
    --source patient \
    --network $NETWORK \
    -- \
    get_all_data \
    --patient "$PATIENT_PUBLIC"

echo "Contract testing completed successfully!" 