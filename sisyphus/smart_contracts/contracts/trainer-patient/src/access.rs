use soroban_sdk::{Address, BytesN, Env};
use crate::error::ContractError;
use crate::storage;

pub struct AccessControl;

impl AccessControl {
    // Check if an address is the contract admin
    pub fn is_admin(env: &Env, address: &Address) -> bool {
        if let Some(admin) = storage::get_admin(env) {
            return &admin == address;
        }
        false
    }

    // Require an address to be the contract admin
    pub fn require_admin(env: &Env, address: &Address) -> Result<(), ContractError> {
        if Self::is_admin(env, address) {
            Ok(())
        } else {
            Err(ContractError::NotAdmin)
        }
    }

    // Check if an address is a registered trainer
    pub fn is_trainer(env: &Env, address: &Address) -> bool {
        if let Some(trainer_id) = storage::get_trainer_id_from_address(env, address) {
            return storage::has_trainer(env, &trainer_id);
        }
        false
    }

    // Require an address to be a registered trainer
    pub fn require_trainer(env: &Env, address: &Address) -> Result<BytesN<32>, ContractError> {
        let trainer_id = storage::get_trainer_id_from_address(env, address)
            .ok_or(ContractError::InternalError)?;
        
        if storage::has_trainer(env, &trainer_id) {
            Ok(trainer_id)
        } else {
            Err(ContractError::NotTrainer)
        }
    }

    // Check if a trainer has access to a patient
    pub fn trainer_has_patient_access(
        env: &Env,
        trainer_address: &Address,
        patient_id: &BytesN<32>
    ) -> Result<(), ContractError> {
        // Get trainer ID
        let trainer_id = Self::require_trainer(env, trainer_address)?;
        
        // Get trainer info
        let trainer_info = storage::get_trainer(env, &trainer_id)
            .ok_or(ContractError::TrainerNotFound)?;
            
        // Check if patient is linked to this trainer
        if !trainer_info.patients.contains(patient_id) {
            return Err(ContractError::PatientNotLinkedWithTrainer);
        }
        
        Ok(())
    }

    // Check if an address is a registered patient
    pub fn is_patient(env: &Env, address: &Address) -> bool {
        if let Some(patient_id) = storage::get_patient_id_from_address(env, address) {
            return storage::has_patient(env, &patient_id);
        }
        false
    }

    // Require an address to be a registered patient
    pub fn require_patient(env: &Env, address: &Address) -> Result<BytesN<32>, ContractError> {
        let patient_id = storage::get_patient_id_from_address(env, address)
            .ok_or(ContractError::InternalError)?;
        
        if storage::has_patient(env, &patient_id) {
            Ok(patient_id)
        } else {
            Err(ContractError::NotPatient)
        }
    }

    // Check if a patient can access their own data
    pub fn patient_can_access_data(
        env: &Env,
        patient_address: &Address
    ) -> Result<BytesN<32>, ContractError> {
        // Get patient ID and check they're registered
        let patient_id = Self::require_patient(env, patient_address)?;
        
        Ok(patient_id)
    }
} 