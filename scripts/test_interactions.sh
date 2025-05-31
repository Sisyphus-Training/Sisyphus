#!/bin/bash

set -e

CONTRACT_ID=$(cat contract_id.txt)
TRAINER_KEY="GDQERTYUIOPASDFGHJKLZXCVBNM1234567890QWERTYUIOPASDFGH"
PATIENT_KEY="GBASDFGHJKLZXCVBNM1234567890QWERTYUIOPASDFGHJKLZXCVBNM"

echo "Registering trainer..."
soroban contract invoke \
    --id $CONTRACT_ID \
    --source trainer \
    --network testnet \
    -- \
    register_trainer \
    --trainer_id $TRAINER_KEY

echo "Linking patient to trainer..."
soroban contract invoke \
    --id $CONTRACT_ID \
    --source trainer \
    --network testnet \
    -- \
    link_patient \
    --trainer_id $TRAINER_KEY \
    --patient_id $PATIENT_KEY

echo "Updating exercise routines..."
soroban contract invoke \
    --id $CONTRACT_ID \
    --source trainer \
    --network testnet \
    -- \
    update_exercise_routines \
    --trainer_id $TRAINER_KEY \
    --patient_id $PATIENT_KEY \
    --routines '["Push-ups: 3x15", "Squats: 3x20", "Plank: 3x1min"]'

echo "Updating meal plans..."
soroban contract invoke \
    --id $CONTRACT_ID \
    --source trainer \
    --network testnet \
    -- \
    update_meal_plans \
    --trainer_id $TRAINER_KEY \
    --patient_id $PATIENT_KEY \
    --meal_plans '["Breakfast: Oatmeal", "Lunch: Salad", "Dinner: Grilled fish"]'

echo "Getting patient data..."
soroban contract invoke \
    --id $CONTRACT_ID \
    --source patient \
    --network testnet \
    -- \
    get_patient_data \
    --patient_id $PATIENT_KEY

echo "Test completed successfully!"
