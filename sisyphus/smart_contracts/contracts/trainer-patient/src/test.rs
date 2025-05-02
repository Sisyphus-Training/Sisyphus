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

#[test]
fn test_access_control() {
    let (env, contract_id, admin) = setup();
    
    // Test is_admin and require_admin
    let random_address = Address::generate(&env);
    
    // Test with actual admin
    env.as_contract(&contract_id, || {
        let is_admin_result = crate::access::AccessControl::is_admin(&env, &admin);
        assert!(is_admin_result);
        
        let require_admin_result = crate::access::AccessControl::require_admin(&env, &admin);
        assert!(require_admin_result.is_ok());
    });
    
    // Test with non-admin
    env.as_contract(&contract_id, || {
        let is_admin_result = crate::access::AccessControl::is_admin(&env, &random_address);
        assert!(!is_admin_result);
        
        let require_admin_result = crate::access::AccessControl::require_admin(&env, &random_address);
        assert!(require_admin_result.is_err());
    });
    
    // Register a trainer to test trainer functions
    let trainer = Address::generate(&env);
    env.as_contract(&contract_id, || {
        TrainerPatientContract::register_trainer(env.clone(), trainer.clone())
    }).unwrap();
    
    // Test is_trainer
    env.as_contract(&contract_id, || {
        let is_trainer_result = crate::access::AccessControl::is_trainer(&env, &trainer);
        assert!(is_trainer_result);
        
        let is_trainer_random = crate::access::AccessControl::is_trainer(&env, &random_address);
        assert!(!is_trainer_random);
    });
    
    // Add a patient to test patient functions
    let patient = Address::generate(&env);
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
    
    // Test is_patient
    env.as_contract(&contract_id, || {
        let is_patient_result = crate::access::AccessControl::is_patient(&env, &patient);
        assert!(is_patient_result);
        
        let is_patient_random = crate::access::AccessControl::is_patient(&env, &random_address);
        assert!(!is_patient_random);
    });
}

#[test]
fn test_storage_functions() {
    let (env, contract_id, _admin) = setup();
    
    // Register a trainer
    let trainer = Address::generate(&env);
    env.as_contract(&contract_id, || {
        TrainerPatientContract::register_trainer(env.clone(), trainer.clone())
    }).unwrap();
    
    // Add a patient
    let patient = Address::generate(&env);
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
    
    // Test the get_patient function
    env.as_contract(&contract_id, || {
        let patient_info = crate::storage::get_patient(&env, &patient_bytes);
        assert!(patient_info.is_some());
        
        let info = patient_info.unwrap();
        // Check that the trainer ID matches our trainer (after hashing)
        let trainer_xdr = trainer.clone().to_xdr(&env);
        let trainer_id = env.crypto().sha256(&trainer_xdr);
        let trainer_bytes = BytesN::from_array(&env, &trainer_id.to_array());
        assert_eq!(info.trainer, trainer_bytes);
    });
}

#[test]
fn test_data_store_functions() {
    let env = Env::default();
    
    // Test create_exercise
    let exercise = crate::data::DataStore::create_exercise(
        &env,
        "Squats",
        3,
        15,
        "Full body squats",
        "https://example.com/squats",
        "Keep back straight and go deep"
    );
    
    assert_eq!(exercise.name, String::from_str(&env, "Squats"));
    assert_eq!(exercise.sets, 3);
    assert_eq!(exercise.reps, 15);
    
    // Create a vector for exercises
    let mut exercises = vec![&env];
    exercises.push_back(exercise);
    
    // Test create_exercise_routine
    let routine = crate::data::DataStore::create_exercise_routine(
        &env,
        "Leg Day",
        "Comprehensive leg workout",
        exercises.clone()
    );
    
    assert_eq!(routine.name, String::from_str(&env, "Leg Day"));
    assert_eq!(routine.exercises.len(), 1);
    
    // Test add_exercise_to_routine
    let new_exercise = crate::data::DataStore::create_exercise(
        &env,
        "Lunges",
        2,
        10,
        "Forward lunges",
        "https://example.com/lunges",
        "Step forward with control"
    );
    
    let updated_routine = crate::data::DataStore::add_exercise_to_routine(
        &env,
        routine,
        new_exercise
    );
    
    assert_eq!(updated_routine.exercises.len(), 2);
    
    // Test create_meal
    let mut foods = vec![&env];
    foods.push_back(String::from_str(&env, "Greek Yogurt"));
    foods.push_back(String::from_str(&env, "Granola"));
    foods.push_back(String::from_str(&env, "Berries"));
    
    let meal = crate::data::DataStore::create_meal(
        &env,
        "Breakfast",
        "8:00 AM",
        foods,
        350,
        "High protein breakfast"
    );
    
    assert_eq!(meal.name, String::from_str(&env, "Breakfast"));
    assert_eq!(meal.foods.len(), 3);
    
    // Test create_meal_plan
    let mut meals = vec![&env];
    meals.push_back(meal);
    
    let plan = crate::data::DataStore::create_meal_plan(
        &env,
        "Recovery Diet",
        "Diet plan for recovery days",
        meals
    );
    
    assert_eq!(plan.name, String::from_str(&env, "Recovery Diet"));
    assert_eq!(plan.meals.len(), 1);
    
    // Test create_progress_update
    let mut metrics = Map::new(&env);
    metrics.set(
        String::from_str(&env, "weight"),
        String::from_str(&env, "175 lbs")
    );
    
    let progress = crate::data::DataStore::create_progress_update(
        &env,
        env.ledger().timestamp(),
        metrics,
        "Making good progress on strength goals"
    );
    
    assert_eq!(progress.metrics.len(), 1);
    
    // Test add_metric_to_progress
    let updated_progress = crate::data::DataStore::add_metric_to_progress(
        &env,
        progress,
        "body_fat",
        "18%"
    );
    
    assert_eq!(updated_progress.metrics.len(), 2);
    
    // Test add_meal_to_plan
    let lunch_foods = vec![&env, String::from_str(&env, "Chicken"), String::from_str(&env, "Rice")];
    let lunch = crate::data::DataStore::create_meal(
        &env,
        "Lunch",
        "12:30 PM",
        lunch_foods,
        450,
        "Post-workout meal"
    );
    
    let updated_plan = crate::data::DataStore::add_meal_to_plan(
        &env,
        plan,
        lunch
    );
    
    assert_eq!(updated_plan.meals.len(), 2);
} 