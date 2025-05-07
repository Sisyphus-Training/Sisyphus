use soroban_sdk::{Address, BytesN, Env, Symbol, symbol_short};
use soroban_sdk::xdr::ToXdr;
use crate::types::{TrainerInfo, PatientInfo, ExerciseRoutine, MealPlan, ProgressUpdate};

// Storage identifiers
const ADMIN: Symbol = symbol_short!("ADMIN");
const TRAINER: Symbol = symbol_short!("TRAINER");
const PATIENT: Symbol = symbol_short!("PATIENT");
const EX_ROUTINE: Symbol = symbol_short!("EX_RTN");
const MEAL_PLAN: Symbol = symbol_short!("MEAL_PLN");
const PROGRESS: Symbol = symbol_short!("PROGRESS");

// Admin operations
pub fn get_admin(env: &Env) -> Option<Address> {
    env.storage().instance().get(&ADMIN)
}

pub fn set_admin(env: &Env, admin: &Address) {
    env.storage().instance().set(&ADMIN, admin);
}

// Trainer operations
pub fn has_trainer(env: &Env, trainer_id: &BytesN<32>) -> bool {
    env.storage().persistent().has(&(TRAINER, trainer_id))
}

pub fn get_trainer(env: &Env, trainer_id: &BytesN<32>) -> Option<TrainerInfo> {
    env.storage().persistent().get(&(TRAINER, trainer_id))
}

pub fn put_trainer(env: &Env, trainer_id: &BytesN<32>, trainer_info: &TrainerInfo) {
    env.storage().persistent().set(&(TRAINER, trainer_id), trainer_info);
}

// Patient operations
pub fn has_patient(env: &Env, patient_id: &BytesN<32>) -> bool {
    env.storage().persistent().has(&(PATIENT, patient_id))
}

pub fn get_patient(env: &Env, patient_id: &BytesN<32>) -> Option<PatientInfo> {
    env.storage().persistent().get(&(PATIENT, patient_id))
}

pub fn put_patient(env: &Env, patient_id: &BytesN<32>, patient_info: &PatientInfo) {
    env.storage().persistent().set(&(PATIENT, patient_id), patient_info);
}

// Patient data operations
pub fn get_exercise_routine(env: &Env, patient_id: &BytesN<32>) -> Option<ExerciseRoutine> {
    env.storage().persistent().get(&(EX_ROUTINE, patient_id))
}

pub fn put_exercise_routine(env: &Env, patient_id: &BytesN<32>, routine: &ExerciseRoutine) {
    env.storage().persistent().set(&(EX_ROUTINE, patient_id), routine);
}

pub fn get_meal_plan(env: &Env, patient_id: &BytesN<32>) -> Option<MealPlan> {
    env.storage().persistent().get(&(MEAL_PLAN, patient_id))
}

pub fn put_meal_plan(env: &Env, patient_id: &BytesN<32>, plan: &MealPlan) {
    env.storage().persistent().set(&(MEAL_PLAN, patient_id), plan);
}

pub fn get_progress(env: &Env, patient_id: &BytesN<32>) -> Option<ProgressUpdate> {
    env.storage().persistent().get(&(PROGRESS, patient_id))
}

pub fn put_progress(env: &Env, patient_id: &BytesN<32>, progress: &ProgressUpdate) {
    env.storage().persistent().set(&(PROGRESS, patient_id), progress);
}

// Utility function to get patient ID from address
pub fn get_patient_id_from_address(env: &Env, address: &Address) -> Option<BytesN<32>> {
    // We can use a cryptographic hash of the address as the patient ID
    Some(env.crypto().sha256(&address.to_xdr(env)).into())
}

// Utility function to get trainer ID from address
pub fn get_trainer_id_from_address(env: &Env, address: &Address) -> Option<BytesN<32>> {
    // We can use a cryptographic hash of the address as the trainer ID
    Some(env.crypto().sha256(&address.to_xdr(env)).into())
} 