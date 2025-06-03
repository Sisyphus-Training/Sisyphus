#![cfg(test)]

use super::*;
use soroban_sdk::{testutils::Address as _, vec, Address, Env, String};

extern crate std;

#[test]
fn test_trainer_registration() {
    let env = Env::default();
    let contract_id = env.register_contract(None, TrainerPatientContract);
    let client = TrainerPatientContractClient::new(&env, &contract_id);

    let trainer = Address::generate(&env);
    env.mock_all_auths();

    client.initialize();
    client.register_trainer(&trainer);

    let trainer_info = client.get_trainer_info(&trainer);
    assert_eq!(trainer_info.trainer_id, trainer);
    assert_eq!(trainer_info.active, true);
    assert_eq!(trainer_info.patient_count, 0);
}

#[test]
fn test_link_patient() {
    let env = Env::default();
    let contract_id = env.register_contract(None, TrainerPatientContract);
    let client = TrainerPatientContractClient::new(&env, &contract_id);

    let trainer = Address::generate(&env);
    let patient = Address::generate(&env);
    env.mock_all_auths();

    client.initialize();
    client.register_trainer(&trainer);
    client.link_patient(&trainer, &patient);

    let linked_trainer = client.get_patient_trainer(&patient);
    assert_eq!(linked_trainer, trainer);

    let trainer_info = client.get_trainer_info(&trainer);
    assert_eq!(trainer_info.patient_count, 1);
}

#[test]
fn test_update_patient_data() {
    let env = Env::default();
    let contract_id = env.register_contract(None, TrainerPatientContract);
    let client = TrainerPatientContractClient::new(&env, &contract_id);

    let trainer = Address::generate(&env);
    let patient = Address::generate(&env);
    env.mock_all_auths();

    client.initialize();
    client.register_trainer(&trainer);
    client.link_patient(&trainer, &patient);

    let routines = vec![
        &env,
        String::from_str(&env, "Push-ups: 3x15"),
        String::from_str(&env, "Squats: 3x20"),
    ];

    let meals = vec![
        &env,
        String::from_str(&env, "Breakfast: Oatmeal"),
        String::from_str(&env, "Lunch: Salad"),
    ];

    let progress = vec![
        &env,
        String::from_str(&env, "Week 1: 10% up"),
        String::from_str(&env, "Week 2: 15% up"),
    ];

    client.update_exercise_routines(&trainer, &patient, &routines);
    client.update_meal_plans(&trainer, &patient, &meals);
    client.update_progress(&trainer, &patient, &progress);

    let patient_data = client.get_patient_data(&patient);
    assert_eq!(patient_data.exercise_routines, routines);
    assert_eq!(patient_data.meal_plans, meals);
    assert_eq!(patient_data.progress_updates, progress);
}

#[test]
fn test_unauthorized_update() {
    let env = Env::default();
    let contract_id = env.register_contract(None, TrainerPatientContract);
    let client = TrainerPatientContractClient::new(&env, &contract_id);

    let trainer1 = Address::generate(&env);
    let trainer2 = Address::generate(&env);
    let patient = Address::generate(&env);
    env.mock_all_auths();

    client.initialize();
    client.register_trainer(&trainer1);
    client.register_trainer(&trainer2);
    client.link_patient(&trainer1, &patient);

    let routines = vec![&env, String::from_str(&env, "Unauthorized")];
    let result = client.try_update_exercise_routines(&trainer2, &patient, &routines);
    assert!(result.is_err());

    if let Err(Ok(error)) = result {
        assert_eq!(error, ContractError::WrongTrainer);
    }
}

#[test]
fn test_unlink_patient() {
    let env = Env::default();
    let contract_id = env.register_contract(None, TrainerPatientContract);
    let client = TrainerPatientContractClient::new(&env, &contract_id);

    let trainer = Address::generate(&env);
    let patient = Address::generate(&env);
    env.mock_all_auths();

    client.initialize();
    client.register_trainer(&trainer);
    client.link_patient(&trainer, &patient);

    let trainer_info = client.get_trainer_info(&trainer);
    assert_eq!(trainer_info.patient_count, 1);

    client.unlink_patient(&trainer, &patient);

    let trainer_info = client.get_trainer_info(&trainer);
    assert_eq!(trainer_info.patient_count, 0);
}

#[test]
fn test_duplicate_trainer_registration() {
    let env = Env::default();
    let contract_id = env.register_contract(None, TrainerPatientContract);
    let client = TrainerPatientContractClient::new(&env, &contract_id);

    let trainer = Address::generate(&env);
    env.mock_all_auths();

    client.initialize();
    client.register_trainer(&trainer);

    let result = client.try_register_trainer(&trainer);
    assert!(result.is_err());

    if let Err(Ok(error)) = result {
        assert_eq!(error, ContractError::TrainerExists);
    }
}

#[test]
fn test_patient_already_linked() {
    let env = Env::default();
    let contract_id = env.register_contract(None, TrainerPatientContract);
    let client = TrainerPatientContractClient::new(&env, &contract_id);

    let trainer1 = Address::generate(&env);
    let trainer2 = Address::generate(&env);
    let patient = Address::generate(&env);
    env.mock_all_auths();

    client.initialize();
    client.register_trainer(&trainer1);
    client.register_trainer(&trainer2);
    client.link_patient(&trainer1, &patient);

    let result = client.try_link_patient(&trainer2, &patient);
    assert!(result.is_err());

    if let Err(Ok(error)) = result {
        assert_eq!(error, ContractError::Linked);
    }
}

#[test]
fn test_get_nonexistent_data() {
    let env = Env::default();
    let contract_id = env.register_contract(None, TrainerPatientContract);
    let client = TrainerPatientContractClient::new(&env, &contract_id);

    let patient = Address::generate(&env);
    env.mock_all_auths();

    client.initialize();

    let result = client.try_get_patient_data(&patient);
    assert!(result.is_err());

    if let Err(Ok(error)) = result {
        assert_eq!(error, ContractError::NoData);
    }
}

#[test]
fn test_get_nonexistent_trainer() {
    let env = Env::default();
    let contract_id = env.register_contract(None, TrainerPatientContract);
    let client = TrainerPatientContractClient::new(&env, &contract_id);

    let trainer = Address::generate(&env);
    env.mock_all_auths();

    client.initialize();

    let result = client.try_get_trainer_info(&trainer);
    assert!(result.is_err());

    if let Err(Ok(error)) = result {
        assert_eq!(error, ContractError::NoTrainer);
    }
}

#[test]
fn test_patient_access_own_data() {
    let env = Env::default();
    let contract_id = env.register_contract(None, TrainerPatientContract);
    let client = TrainerPatientContractClient::new(&env, &contract_id);

    let trainer = Address::generate(&env);
    let patient = Address::generate(&env);
    env.mock_all_auths();

    client.initialize();
    client.register_trainer(&trainer);
    client.link_patient(&trainer, &patient);

    let routines = vec![
        &env,
        String::from_str(&env, "Daily walk: 30min"),
        String::from_str(&env, "Stretching: 15min"),
    ];

    client.update_exercise_routines(&trainer, &patient, &routines);

    let patient_data = client.get_patient_data(&patient);
    assert_eq!(patient_data.patient_id, patient);
    assert_eq!(patient_data.trainer_id, trainer);
    assert_eq!(patient_data.exercise_routines, routines);
}

#[test]
fn test_multiple_patients_per_trainer() {
    let env = Env::default();
    let contract_id = env.register_contract(None, TrainerPatientContract);
    let client = TrainerPatientContractClient::new(&env, &contract_id);

    let trainer = Address::generate(&env);
    let patient1 = Address::generate(&env);
    let patient2 = Address::generate(&env);
    env.mock_all_auths();

    client.initialize();
    client.register_trainer(&trainer);
    client.link_patient(&trainer, &patient1);
    client.link_patient(&trainer, &patient2);

    let trainer_info = client.get_trainer_info(&trainer);
    assert_eq!(trainer_info.patient_count, 2);

    let linked_trainer1 = client.get_patient_trainer(&patient1);
    let linked_trainer2 = client.get_patient_trainer(&patient2);
    assert_eq!(linked_trainer1, trainer);
    assert_eq!(linked_trainer2, trainer);
}
