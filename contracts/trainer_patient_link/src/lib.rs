#![no_std]
use soroban_sdk::{
    contract, contractimpl, contracttype, symbol_short, vec, Address, Env, String, Vec,
    contracterror,
};

#[derive(Clone, Debug, PartialEq)]
#[contracttype]
pub struct PatientData {
    pub patient_id: Address,
    pub trainer_id: Address,
    pub exercise_routines: Vec<String>,
    pub meal_plans: Vec<String>,
    pub progress_updates: Vec<String>,
    pub last_updated: u64,
}

#[derive(Clone, Debug, PartialEq)]
#[contracttype]
pub struct TrainerInfo {
    pub trainer_id: Address,
    pub active: bool,
    pub patient_count: u32,
}

#[derive(Clone)]
#[contracttype]
pub enum DataKey {
    TrainerInfo(Address),
    PatientData(Address),
    TrainerPatientLink(Address, Address),
    PatientTrainerLink(Address),
}

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum ContractError {
    TrainerExists = 1,
    NoTrainer = 2,
    Inactive = 3,
    Linked = 4,
    NoData = 5,
    NoLink = 6,
    WrongTrainer = 7,
    NotLinked = 8,
}

#[contract]
pub struct TrainerPatientContract;

#[contractimpl]
impl TrainerPatientContract {
    pub fn initialize(env: Env) {
        env.storage().instance().set(&symbol_short!("init"), &true);
    }

    pub fn register_trainer(env: Env, trainer_id: Address) -> Result<(), ContractError> {
        trainer_id.require_auth();

        if env.storage().persistent().has(&DataKey::TrainerInfo(trainer_id.clone())) {
            return Err(ContractError::TrainerExists);
        }

        let trainer_info = TrainerInfo {
            trainer_id: trainer_id.clone(),
            active: true,
            patient_count: 0,
        };

        env.storage().persistent().set(&DataKey::TrainerInfo(trainer_id.clone()), &trainer_info);
        env.storage().persistent().extend_ttl(&DataKey::TrainerInfo(trainer_id.clone()), 518400, 518400);

        env.events().publish((symbol_short!("trainer"), symbol_short!("register")), trainer_id);
        Ok(())
    }

    pub fn link_patient(env: Env, trainer_id: Address, patient_id: Address) -> Result<(), ContractError> {
        trainer_id.require_auth();

        let mut trainer_info: TrainerInfo = env
            .storage()
            .persistent()
            .get(&DataKey::TrainerInfo(trainer_id.clone()))
            .ok_or(ContractError::NoTrainer)?;

        if !trainer_info.active {
            return Err(ContractError::Inactive);
        }

        if env.storage().persistent().has(&DataKey::PatientTrainerLink(patient_id.clone())) {
            return Err(ContractError::Linked);
        }

        env.storage().persistent().set(
            &DataKey::PatientTrainerLink(patient_id.clone()),
            &trainer_id,
        );
        env.storage().persistent().extend_ttl(&DataKey::PatientTrainerLink(patient_id.clone()), 518400, 518400);

        env.storage().persistent().set(
            &DataKey::TrainerPatientLink(trainer_id.clone(), patient_id.clone()),
            &true,
        );
        env.storage().persistent().extend_ttl(&DataKey::TrainerPatientLink(trainer_id.clone(), patient_id.clone()), 518400, 518400);

        let patient_data = PatientData {
            patient_id: patient_id.clone(),
            trainer_id: trainer_id.clone(),
            exercise_routines: vec![&env],
            meal_plans: vec![&env],
            progress_updates: vec![&env],
            last_updated: env.ledger().timestamp(),
        };

        env.storage().persistent().set(&DataKey::PatientData(patient_id.clone()), &patient_data);
        env.storage().persistent().extend_ttl(&DataKey::PatientData(patient_id.clone()), 518400, 518400);

        trainer_info.patient_count += 1;
        env.storage().persistent().set(&DataKey::TrainerInfo(trainer_id.clone()), &trainer_info);

        env.events().publish(
            (symbol_short!("link"), symbol_short!("created")),
            (trainer_id, patient_id),
        );
        Ok(())
    }

    pub fn update_exercise_routines(
        env: Env,
        trainer_id: Address,
        patient_id: Address,
        routines: Vec<String>,
    ) -> Result<(), ContractError> {
        trainer_id.require_auth();
        Self::verify_trainer_patient_link(&env, &trainer_id, &patient_id)?;

        let mut patient_data: PatientData = env
            .storage()
            .persistent()
            .get(&DataKey::PatientData(patient_id.clone()))
            .ok_or(ContractError::NoData)?;

        patient_data.exercise_routines = routines;
        patient_data.last_updated = env.ledger().timestamp();

        env.storage().persistent().set(&DataKey::PatientData(patient_id.clone()), &patient_data);
        env.storage().persistent().extend_ttl(&DataKey::PatientData(patient_id.clone()), 518400, 518400);

        env.events().publish(
            (symbol_short!("update"), symbol_short!("exercise")),
            patient_id,
        );
        Ok(())
    }

    pub fn update_meal_plans(
        env: Env,
        trainer_id: Address,
        patient_id: Address,
        meal_plans: Vec<String>,
    ) -> Result<(), ContractError> {
        trainer_id.require_auth();
        Self::verify_trainer_patient_link(&env, &trainer_id, &patient_id)?;

        let mut patient_data: PatientData = env
            .storage()
            .persistent()
            .get(&DataKey::PatientData(patient_id.clone()))
            .ok_or(ContractError::NoData)?;

        patient_data.meal_plans = meal_plans;
        patient_data.last_updated = env.ledger().timestamp();

        env.storage().persistent().set(&DataKey::PatientData(patient_id.clone()), &patient_data);
        env.storage().persistent().extend_ttl(&DataKey::PatientData(patient_id.clone()), 518400, 518400);

        env.events().publish(
            (symbol_short!("update"), symbol_short!("meal")),
            patient_id,
        );
        Ok(())
    }

    pub fn update_progress(
        env: Env,
        trainer_id: Address,
        patient_id: Address,
        progress_updates: Vec<String>,
    ) -> Result<(), ContractError> {
        trainer_id.require_auth();
        Self::verify_trainer_patient_link(&env, &trainer_id, &patient_id)?;

        let mut patient_data: PatientData = env
            .storage()
            .persistent()
            .get(&DataKey::PatientData(patient_id.clone()))
            .ok_or(ContractError::NoData)?;

        patient_data.progress_updates = progress_updates;
        patient_data.last_updated = env.ledger().timestamp();

        env.storage().persistent().set(&DataKey::PatientData(patient_id.clone()), &patient_data);
        env.storage().persistent().extend_ttl(&DataKey::PatientData(patient_id.clone()), 518400, 518400);

        env.events().publish(
            (symbol_short!("update"), symbol_short!("progress")),
            patient_id,
        );
        Ok(())
    }

    pub fn get_patient_data(env: Env, patient_id: Address) -> Result<PatientData, ContractError> {
        patient_id.require_auth();

        env.storage()
            .persistent()
            .get(&DataKey::PatientData(patient_id))
            .ok_or(ContractError::NoData)
    }

    pub fn get_trainer_info(env: Env, trainer_id: Address) -> Result<TrainerInfo, ContractError> {
        env.storage()
            .persistent()
            .get(&DataKey::TrainerInfo(trainer_id))
            .ok_or(ContractError::NoTrainer)
    }

    pub fn get_patient_trainer(env: Env, patient_id: Address) -> Result<Address, ContractError> {
        patient_id.require_auth();

        env.storage()
            .persistent()
            .get(&DataKey::PatientTrainerLink(patient_id))
            .ok_or(ContractError::NoLink)
    }

    pub fn unlink_patient(env: Env, trainer_id: Address, patient_id: Address) -> Result<(), ContractError> {
        trainer_id.require_auth();
        Self::verify_trainer_patient_link(&env, &trainer_id, &patient_id)?;

        env.storage().persistent().remove(&DataKey::PatientTrainerLink(patient_id.clone()));
        env.storage().persistent().remove(&DataKey::TrainerPatientLink(trainer_id.clone(), patient_id.clone()));

        let mut trainer_info: TrainerInfo = env
            .storage()
            .persistent()
            .get(&DataKey::TrainerInfo(trainer_id.clone()))
            .ok_or(ContractError::NoTrainer)?;

        if trainer_info.patient_count > 0 {
            trainer_info.patient_count -= 1;
        }
        env.storage().persistent().set(&DataKey::TrainerInfo(trainer_id.clone()), &trainer_info);

        env.events().publish(
            (symbol_short!("link"), symbol_short!("removed")),
            (trainer_id, patient_id),
        );
        Ok(())
    }

    fn verify_trainer_patient_link(
        env: &Env,
        trainer_id: &Address,
        patient_id: &Address,
    ) -> Result<(), ContractError> {
        let linked_trainer: Address = env
            .storage()
            .persistent()
            .get(&DataKey::PatientTrainerLink(patient_id.clone()))
            .ok_or(ContractError::NoLink)?;

        if linked_trainer != *trainer_id {
            return Err(ContractError::WrongTrainer);
        }

        let is_linked: bool = env
            .storage()
            .persistent()
            .get(&DataKey::TrainerPatientLink(trainer_id.clone(), patient_id.clone()))
            .unwrap_or(false);

        if !is_linked {
            return Err(ContractError::NotLinked);
        }

        Ok(())
    }
}

#[cfg(test)]
mod test;
