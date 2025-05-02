#![no_std]
use soroban_sdk::{contract, contractimpl, Address, BytesN, Env};

mod types;
mod error;
mod storage;
mod trainer;
mod patient;
mod data;
mod access;

use types::{PatientData, ExerciseRoutine, MealPlan, ProgressUpdate};
use error::ContractError;
use trainer::TrainerInterface;
use patient::PatientInterface;

#[contract]
pub struct TrainerPatientContract;

#[contractimpl]
impl TrainerPatientContract {
    // Initialize contract
    pub fn initialize(env: Env, admin: Address) -> Result<(), ContractError> {
        // Verify the contract is not already initialized
        if storage::get_admin(&env).is_some() {
            return Err(ContractError::AlreadyInitialized);
        }

        // Set the contract administrator
        storage::set_admin(&env, &admin);
        Ok(())
    }

    // Trainer operations
    pub fn register_trainer(env: Env, trainer_address: Address) -> Result<(), ContractError> {
        TrainerInterface::register(&env, &trainer_address)
    }

    pub fn add_patient(
        env: Env, 
        trainer: Address, 
        patient_address: Address,
        patient_id: BytesN<32>
    ) -> Result<(), ContractError> {
        TrainerInterface::add_patient(&env, &trainer, &patient_address, &patient_id)
    }

    pub fn update_exercise_routine(
        env: Env,
        trainer: Address,
        patient_id: BytesN<32>,
        routine: ExerciseRoutine
    ) -> Result<(), ContractError> {
        TrainerInterface::update_exercise_routine(&env, &trainer, &patient_id, &routine)
    }

    pub fn update_meal_plan(
        env: Env,
        trainer: Address,
        patient_id: BytesN<32>,
        plan: MealPlan
    ) -> Result<(), ContractError> {
        TrainerInterface::update_meal_plan(&env, &trainer, &patient_id, &plan)
    }

    pub fn update_progress(
        env: Env,
        trainer: Address,
        patient_id: BytesN<32>,
        progress: ProgressUpdate
    ) -> Result<(), ContractError> {
        TrainerInterface::update_progress(&env, &trainer, &patient_id, &progress)
    }

    // Patient operations
    pub fn get_exercise_routine(
        env: Env,
        patient: Address,
    ) -> Result<ExerciseRoutine, ContractError> {
        PatientInterface::get_exercise_routine(&env, &patient)
    }

    pub fn get_meal_plan(
        env: Env,
        patient: Address,
    ) -> Result<MealPlan, ContractError> {
        PatientInterface::get_meal_plan(&env, &patient)
    }

    pub fn get_progress(
        env: Env,
        patient: Address,
    ) -> Result<ProgressUpdate, ContractError> {
        PatientInterface::get_progress(&env, &patient)
    }

    pub fn get_all_data(
        env: Env,
        patient: Address,
    ) -> Result<PatientData, ContractError> {
        PatientInterface::get_all_data(&env, &patient)
    }
    
    // Debug helper to get patient ID from address
    pub fn get_patient_id(env: Env, address: Address) -> BytesN<32> {
        storage::get_patient_id_from_address(&env, &address).unwrap()
    }
}

#[cfg(test)]
mod test; 