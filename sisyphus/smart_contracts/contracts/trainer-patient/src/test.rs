#![cfg(test)]

use soroban_sdk::{Env, Address, BytesN, vec, Map, String};
use soroban_sdk::testutils::Address as _;
use soroban_sdk::xdr::ToXdr;
use crate::{TrainerPatientContract, types::{Exercise, ExerciseRoutine, Meal, MealPlan, ProgressUpdate}};

// Helper function to setup a test environment with initialized contract
fn setup() -> (Env, Address, Address) {
    let env = Env::default();
    
    // Register the contract
    let contract_id = env.register(TrainerPatientContract, ());
    
    // Generate a random admin address
    let admin = Address::generate(&env);
    
    // Initialize contract
    env.as_contract(&contract_id, || {
        TrainerPatientContract::initialize(env.clone(), admin.clone())
    }).unwrap();
    
    (env, contract_id, admin)
}

#[test]
fn test_initialize() {
    let env = Env::default();
    let contract_id = env.register(TrainerPatientContract, ());
    let admin = Address::generate(&env);
    
    // Test successful initialization
    let result = env.as_contract(&contract_id, || {
        TrainerPatientContract::initialize(env.clone(), admin.clone())
    });
    assert!(result.is_ok());
    
    // Test initializing an already initialized contract (should fail)
    let result = env.as_contract(&contract_id, || {
        TrainerPatientContract::initialize(env.clone(), admin.clone())
    });
    assert!(result.is_err());
}

#[test]
fn test_register_trainer() {
    let (env, contract_id, _admin) = setup();
    
    // Register a trainer
    let trainer = Address::generate(&env);
    let result = env.as_contract(&contract_id, || {
        TrainerPatientContract::register_trainer(env.clone(), trainer.clone())
    });
    
    assert!(result.is_ok());
}

#[test]
fn test_add_patient() {
    let (env, contract_id, _admin) = setup();
    
    // Register a trainer
    let trainer = Address::generate(&env);
    env.as_contract(&contract_id, || {
        TrainerPatientContract::register_trainer(env.clone(), trainer.clone())
    }).unwrap();
    
    // Add a patient
    let patient = Address::generate(&env);
    // Create a patient ID by hashing the patient address
    let patient_xdr = patient.clone().to_xdr(&env);
    let patient_id = env.crypto().sha256(&patient_xdr);
    
    let result = env.as_contract(&contract_id, || {
        TrainerPatientContract::add_patient(
            env.clone(), 
            trainer.clone(), 
            patient.clone(),
            BytesN::from_array(&env, &patient_id.to_array())
        )
    });
    
    assert!(result.is_ok());
}

#[test]
fn test_update_and_get_exercise_routine() {
    let (env, contract_id, _admin) = setup();
    
    // Register a trainer
    let trainer = Address::generate(&env);
    env.as_contract(&contract_id, || {
        TrainerPatientContract::register_trainer(env.clone(), trainer.clone())
    }).unwrap();
    
    // Add a patient
    let patient = Address::generate(&env);
    // Create a patient ID by hashing the patient address
    let patient_xdr = patient.clone().to_xdr(&env);
    let patient_id = env.crypto().sha256(&patient_xdr);
    let patient_bytes = BytesN::from_array(&env, &patient_id.to_array());
    
    env.as_contract(&contract_id, || {
        TrainerPatientContract::add_patient(
            env.clone(), 
            trainer.clone(), 
            patient.clone(),
            patient_bytes.clone()
        )
    }).unwrap();
    
    // Create an exercise routine
    let exercise = Exercise {
        name: String::from_str(&env, "Push-ups"),
        sets: 3,
        reps: 10,
        description: String::from_str(&env, "Standard push-ups"),
        video_link: String::from_str(&env, "https://example.com/pushups"),
        notes: String::from_str(&env, "Keep back straight"),
    };
    
    let mut exercises = vec![&env];
    exercises.push_back(exercise);
    
    let routine = ExerciseRoutine {
        name: String::from_str(&env, "Upper Body"),
        description: String::from_str(&env, "Upper body strength workout"),
        exercises,
        last_updated: env.ledger().timestamp(),
    };
    
    // Update exercise routine
    let result = env.as_contract(&contract_id, || {
        TrainerPatientContract::update_exercise_routine(
            env.clone(), 
            trainer.clone(), 
            patient_bytes.clone(),
            routine.clone()
        )
    });
    assert!(result.is_ok());
    
    // Get exercise routine as patient
    let retrieved_routine = env.as_contract(&contract_id, || {
        TrainerPatientContract::get_exercise_routine(
            env.clone(),
            patient.clone()
        )
    }).unwrap();
    
    assert_eq!(retrieved_routine.name, String::from_str(&env, "Upper Body"));
    assert_eq!(retrieved_routine.exercises.len(), 1);
}

#[test]
fn test_update_and_get_meal_plan() {
    let (env, contract_id, _admin) = setup();
    
    // Register a trainer
    let trainer = Address::generate(&env);
    env.as_contract(&contract_id, || {
        TrainerPatientContract::register_trainer(env.clone(), trainer.clone())
    }).unwrap();
    
    // Add a patient
    let patient = Address::generate(&env);
    // Create a patient ID by hashing the patient address
    let patient_xdr = patient.clone().to_xdr(&env);
    let patient_id = env.crypto().sha256(&patient_xdr);
    let patient_bytes = BytesN::from_array(&env, &patient_id.to_array());
    
    env.as_contract(&contract_id, || {
        TrainerPatientContract::add_patient(
            env.clone(), 
            trainer.clone(), 
            patient.clone(),
            patient_bytes.clone()
        )
    }).unwrap();
    
    // Create a meal plan
    let mut foods = vec![&env];
    foods.push_back(String::from_str(&env, "Chicken"));
    foods.push_back(String::from_str(&env, "Rice"));
    foods.push_back(String::from_str(&env, "Broccoli"));
    
    let meal = Meal {
        name: String::from_str(&env, "Lunch"),
        time: String::from_str(&env, "12:00 PM"),
        foods,
        calories: 500,
        notes: String::from_str(&env, "High protein meal"),
    };
    
    let mut meals = vec![&env];
    meals.push_back(meal);
    
    let plan = MealPlan {
        name: String::from_str(&env, "Protein Diet"),
        description: String::from_str(&env, "High protein diet plan"),
        meals,
        last_updated: env.ledger().timestamp(),
    };
    
    // Update meal plan
    let result = env.as_contract(&contract_id, || {
        TrainerPatientContract::update_meal_plan(
            env.clone(), 
            trainer.clone(), 
            patient_bytes.clone(),
            plan.clone()
        )
    });
    assert!(result.is_ok());
    
    // Get meal plan as patient
    let retrieved_plan = env.as_contract(&contract_id, || {
        TrainerPatientContract::get_meal_plan(
            env.clone(),
            patient.clone()
        )
    }).unwrap();
    
    assert_eq!(retrieved_plan.name, String::from_str(&env, "Protein Diet"));
    assert_eq!(retrieved_plan.meals.len(), 1);
}

#[test]
fn test_update_and_get_progress() {
    let (env, contract_id, _admin) = setup();
    
    // Register a trainer
    let trainer = Address::generate(&env);
    env.as_contract(&contract_id, || {
        TrainerPatientContract::register_trainer(env.clone(), trainer.clone())
    }).unwrap();
    
    // Add a patient
    let patient = Address::generate(&env);
    // Create a patient ID by hashing the patient address
    let patient_xdr = patient.clone().to_xdr(&env);
    let patient_id = env.crypto().sha256(&patient_xdr);
    let patient_bytes = BytesN::from_array(&env, &patient_id.to_array());
    
    env.as_contract(&contract_id, || {
        TrainerPatientContract::add_patient(
            env.clone(), 
            trainer.clone(), 
            patient.clone(),
            patient_bytes.clone()
        )
    }).unwrap();
    
    // Create progress update
    let mut metrics = Map::new(&env);
    metrics.set(
        String::from_str(&env, "weight"), 
        String::from_str(&env, "180lbs")
    );
    metrics.set(
        String::from_str(&env, "body_fat"), 
        String::from_str(&env, "15%")
    );
    
    let progress = ProgressUpdate {
        date: env.ledger().timestamp(),
        metrics,
        notes: String::from_str(&env, "Making good progress"),
        last_updated: env.ledger().timestamp(),
    };
    
    // Update progress
    let result = env.as_contract(&contract_id, || {
        TrainerPatientContract::update_progress(
            env.clone(), 
            trainer.clone(), 
            patient_bytes.clone(),
            progress.clone()
        )
    });
    assert!(result.is_ok());
    
    // Get progress as patient
    let retrieved_progress = env.as_contract(&contract_id, || {
        TrainerPatientContract::get_progress(
            env.clone(),
            patient.clone()
        )
    }).unwrap();
    
    assert_eq!(retrieved_progress.notes, String::from_str(&env, "Making good progress"));
    assert_eq!(retrieved_progress.metrics.len(), 2);
}

#[test]
fn test_get_all_data() {
    let (env, contract_id, _admin) = setup();
    
    // Register a trainer
    let trainer = Address::generate(&env);
    env.as_contract(&contract_id, || {
        TrainerPatientContract::register_trainer(env.clone(), trainer.clone())
    }).unwrap();
    
    // Add a patient
    let patient = Address::generate(&env);
    // Create a patient ID by hashing the patient address
    let patient_xdr = patient.clone().to_xdr(&env);
    let patient_id = env.crypto().sha256(&patient_xdr);
    let patient_bytes = BytesN::from_array(&env, &patient_id.to_array());
    
    env.as_contract(&contract_id, || {
        TrainerPatientContract::add_patient(
            env.clone(), 
            trainer.clone(), 
            patient.clone(),
            patient_bytes.clone()
        )
    }).unwrap();
    
    // Create and update exercise routine
    let exercise = Exercise {
        name: String::from_str(&env, "Push-ups"),
        sets: 3,
        reps: 10,
        description: String::from_str(&env, "Standard push-ups"),
        video_link: String::from_str(&env, "https://example.com/pushups"),
        notes: String::from_str(&env, "Keep back straight"),
    };
    
    let mut exercises = vec![&env];
    exercises.push_back(exercise);
    
    let routine = ExerciseRoutine {
        name: String::from_str(&env, "Upper Body"),
        description: String::from_str(&env, "Upper body strength workout"),
        exercises,
        last_updated: env.ledger().timestamp(),
    };
    
    env.as_contract(&contract_id, || {
        TrainerPatientContract::update_exercise_routine(
            env.clone(), 
            trainer.clone(), 
            patient_bytes.clone(),
            routine.clone()
        )
    }).unwrap();
    
    // Create and update meal plan
    let mut foods = vec![&env];
    foods.push_back(String::from_str(&env, "Chicken"));
    
    let meal = Meal {
        name: String::from_str(&env, "Lunch"),
        time: String::from_str(&env, "12:00 PM"),
        foods,
        calories: 500,
        notes: String::from_str(&env, "High protein meal"),
    };
    
    let mut meals = vec![&env];
    meals.push_back(meal);
    
    let plan = MealPlan {
        name: String::from_str(&env, "Protein Diet"),
        description: String::from_str(&env, "High protein diet plan"),
        meals,
        last_updated: env.ledger().timestamp(),
    };
    
    env.as_contract(&contract_id, || {
        TrainerPatientContract::update_meal_plan(
            env.clone(), 
            trainer.clone(), 
            patient_bytes.clone(),
            plan.clone()
        )
    }).unwrap();
    
    // Create and update progress
    let mut metrics = Map::new(&env);
    metrics.set(
        String::from_str(&env, "weight"), 
        String::from_str(&env, "180lbs")
    );
    
    let progress = ProgressUpdate {
        date: env.ledger().timestamp(),
        metrics,
        notes: String::from_str(&env, "Making good progress"),
        last_updated: env.ledger().timestamp(),
    };
    
    env.as_contract(&contract_id, || {
        TrainerPatientContract::update_progress(
            env.clone(), 
            trainer.clone(), 
            patient_bytes.clone(),
            progress.clone()
        )
    }).unwrap();
    
    // Get all data as patient
    let all_data = env.as_contract(&contract_id, || {
        TrainerPatientContract::get_all_data(
            env.clone(),
            patient.clone()
        )
    }).unwrap();
    
    assert_eq!(all_data.exercise_routine.name, String::from_str(&env, "Upper Body"));
    assert_eq!(all_data.meal_plan.name, String::from_str(&env, "Protein Diet"));
    assert_eq!(all_data.progress.notes, String::from_str(&env, "Making good progress"));
} 