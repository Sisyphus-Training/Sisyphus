use soroban_sdk::{Address, Env};
use crate::error::ContractError;
use crate::types::{PatientData, ExerciseRoutine, MealPlan, ProgressUpdate};
use crate::storage;
use crate::access::AccessControl;

pub struct PatientInterface;

impl PatientInterface {
    // Get the patient's exercise routine
    pub fn get_exercise_routine(
        env: &Env,
        patient_address: &Address,
    ) -> Result<ExerciseRoutine, ContractError> {
        // Verify the patient can access their data
        let patient_id = AccessControl::patient_can_access_data(env, patient_address)?;
        
        // Get the exercise routine
        let routine = storage::get_exercise_routine(env, &patient_id)
            .unwrap_or_else(|| ExerciseRoutine::new(env));
        
        Ok(routine)
    }

    // Get the patient's meal plan
    pub fn get_meal_plan(
        env: &Env,
        patient_address: &Address,
    ) -> Result<MealPlan, ContractError> {
        // Verify the patient can access their data
        let patient_id = AccessControl::patient_can_access_data(env, patient_address)?;
        
        // Get the meal plan
        let meal_plan = storage::get_meal_plan(env, &patient_id)
            .unwrap_or_else(|| MealPlan::new(env));
        
        Ok(meal_plan)
    }

    // Get the patient's progress
    pub fn get_progress(
        env: &Env,
        patient_address: &Address,
    ) -> Result<ProgressUpdate, ContractError> {
        // Verify the patient can access their data
        let patient_id = AccessControl::patient_can_access_data(env, patient_address)?;
        
        // Get the progress
        let progress = storage::get_progress(env, &patient_id)
            .unwrap_or_else(|| ProgressUpdate::new(env));
        
        Ok(progress)
    }

    // Get all of the patient's data at once
    pub fn get_all_data(
        env: &Env,
        patient_address: &Address,
    ) -> Result<PatientData, ContractError> {
        // Verify the patient can access their data
        let patient_id = AccessControl::patient_can_access_data(env, patient_address)?;
        
        // Get all data
        let exercise_routine = storage::get_exercise_routine(env, &patient_id)
            .unwrap_or_else(|| ExerciseRoutine::new(env));
            
        let meal_plan = storage::get_meal_plan(env, &patient_id)
            .unwrap_or_else(|| MealPlan::new(env));
            
        let progress = storage::get_progress(env, &patient_id)
            .unwrap_or_else(|| ProgressUpdate::new(env));
        
        // Combine data
        let patient_data = PatientData {
            exercise_routine,
            meal_plan,
            progress,
        };
        
        Ok(patient_data)
    }
}