use soroban_sdk::{Address, BytesN, Env, Vec};
use crate::error::ContractError;
use crate::types::{TrainerInfo, PatientInfo, ExerciseRoutine, MealPlan, ProgressUpdate};
use crate::storage;
use crate::access::AccessControl;

pub struct TrainerInterface;

impl TrainerInterface {
    // Register a new trainer
    pub fn register(env: &Env, trainer_address: &Address) -> Result<(), ContractError> {
        // Get trainer ID
        let trainer_id = storage::get_trainer_id_from_address(env, trainer_address)
            .ok_or(ContractError::InternalError)?;
        
        // Check if the trainer is already registered
        if storage::has_trainer(env, &trainer_id) {
            return Err(ContractError::TrainerAlreadyRegistered);
        }
        
        // Create new trainer info
        let trainer_info = TrainerInfo {
            patients: Vec::new(env),
        };
        
        // Store trainer info
        storage::put_trainer(env, &trainer_id, &trainer_info);
        
        Ok(())
    }

    // Add a patient to a trainer's list
    pub fn add_patient(
        env: &Env,
        trainer_address: &Address,
        _patient_address: &Address,
        patient_id: &BytesN<32>,
    ) -> Result<(), ContractError> {
        // Verify the trainer is registered
        let trainer_id = AccessControl::require_trainer(env, trainer_address)?;
        
        // Get trainer's information
        let mut trainer_info = storage::get_trainer(env, &trainer_id)
            .ok_or(ContractError::TrainerNotFound)?;
        
        // Check if the patient is already linked
        if trainer_info.patients.contains(patient_id) {
            return Err(ContractError::PatientAlreadyLinked);
        }
        
        // Check if the patient is already linked to another trainer
        if storage::has_patient(env, patient_id) {
            return Err(ContractError::PatientAlreadyLinked);
        }
        
        // Add patient to trainer's list
        trainer_info.patients.push_back(patient_id.clone());
        storage::put_trainer(env, &trainer_id, &trainer_info);
        
        // Create patient info and link to trainer
        let patient_info = PatientInfo {
            trainer: trainer_id.clone(),
        };
        storage::put_patient(env, patient_id, &patient_info);
        
        // Initialize empty data structures for the patient
        let routine = ExerciseRoutine::new(env);
        let meal_plan = MealPlan::new(env);
        let progress = ProgressUpdate::new(env);
        
        storage::put_exercise_routine(env, patient_id, &routine);
        storage::put_meal_plan(env, patient_id, &meal_plan);
        storage::put_progress(env, patient_id, &progress);
        
        Ok(())
    }

    // Update a patient's exercise routine
    pub fn update_exercise_routine(
        env: &Env,
        trainer_address: &Address,
        patient_id: &BytesN<32>,
        routine: &ExerciseRoutine,
    ) -> Result<(), ContractError> {
        // Verify the trainer has access to this patient
        AccessControl::trainer_has_patient_access(env, trainer_address, patient_id)?;
        
        // Update the routine with current timestamp
        let mut updated_routine = routine.clone();
        updated_routine.last_updated = env.ledger().timestamp();
        
        // Store the updated routine
        storage::put_exercise_routine(env, patient_id, &updated_routine);
        
        Ok(())
    }

    // Update a patient's meal plan
    pub fn update_meal_plan(
        env: &Env,
        trainer_address: &Address,
        patient_id: &BytesN<32>,
        plan: &MealPlan,
    ) -> Result<(), ContractError> {
        // Verify the trainer has access to this patient
        AccessControl::trainer_has_patient_access(env, trainer_address, patient_id)?;
        
        // Update the meal plan with current timestamp
        let mut updated_plan = plan.clone();
        updated_plan.last_updated = env.ledger().timestamp();
        
        // Store the updated meal plan
        storage::put_meal_plan(env, patient_id, &updated_plan);
        
        Ok(())
    }

    // Update a patient's progress
    pub fn update_progress(
        env: &Env,
        trainer_address: &Address,
        patient_id: &BytesN<32>,
        progress: &ProgressUpdate,
    ) -> Result<(), ContractError> {
        // Verify the trainer has access to this patient
        AccessControl::trainer_has_patient_access(env, trainer_address, patient_id)?;
        
        // Update the progress with current timestamp
        let mut updated_progress = progress.clone();
        updated_progress.last_updated = env.ledger().timestamp();
        
        // Store the updated progress
        storage::put_progress(env, patient_id, &updated_progress);
        
        Ok(())
    }
} 