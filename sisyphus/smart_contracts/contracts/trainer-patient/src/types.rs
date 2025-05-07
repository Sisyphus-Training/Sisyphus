use soroban_sdk::{contracttype, Vec, String, Map, BytesN, Env};

// Main data structures for a patient's data
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ExerciseRoutine {
    pub name: String,
    pub description: String,
    pub exercises: Vec<Exercise>,
    pub last_updated: u64,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Exercise {
    pub name: String,
    pub sets: u32,
    pub reps: u32,
    pub description: String,
    pub video_link: String,
    pub notes: String,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MealPlan {
    pub name: String,
    pub description: String,
    pub meals: Vec<Meal>,
    pub last_updated: u64,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Meal {
    pub name: String,
    pub time: String,
    pub foods: Vec<String>,
    pub calories: u32,
    pub notes: String,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ProgressUpdate {
    pub date: u64,
    pub metrics: Map<String, String>,
    pub notes: String,
    pub last_updated: u64,
}

// Combined patient data structure
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PatientData {
    pub exercise_routine: ExerciseRoutine,
    pub meal_plan: MealPlan,
    pub progress: ProgressUpdate,
}

// Trainer data structure
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TrainerInfo {
    pub patients: Vec<BytesN<32>>,
}

// Patient data structure
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PatientInfo {
    pub trainer: BytesN<32>,
}

// Default implementations for new objects
impl ExerciseRoutine {
    pub fn new(env: &Env) -> Self {
        Self {
            name: String::from_str(env, ""),
            description: String::from_str(env, ""),
            exercises: Vec::new(env),
            last_updated: 0,
        }
    }
}

impl MealPlan {
    pub fn new(env: &Env) -> Self {
        Self {
            name: String::from_str(env, ""),
            description: String::from_str(env, ""),
            meals: Vec::new(env),
            last_updated: 0,
        }
    }
}

impl ProgressUpdate {
    pub fn new(env: &Env) -> Self {
        Self {
            date: 0,
            metrics: Map::new(env),
            notes: String::from_str(env, ""),
            last_updated: 0,
        }
    }
}

impl PatientData {
    pub fn new(env: &Env) -> Self {
        Self {
            exercise_routine: ExerciseRoutine::new(env),
            meal_plan: MealPlan::new(env),
            progress: ProgressUpdate::new(env),
        }
    }
} 