use soroban_sdk::{Env, Map, String, Vec};
use crate::types::{Exercise, ExerciseRoutine, Meal, MealPlan, ProgressUpdate};

pub struct DataStore;

// Helper functions for creating and managing data objects
impl DataStore {
    // Create an exercise
    pub fn create_exercise(
        env: &Env,
        name: &str,
        sets: u32,
        reps: u32,
        description: &str,
        video_link: &str,
        notes: &str,
    ) -> Exercise {
        Exercise {
            name: String::from_str(env, name),
            sets,
            reps,
            description: String::from_str(env, description),
            video_link: String::from_str(env, video_link),
            notes: String::from_str(env, notes),
        }
    }

    // Create an exercise routine
    pub fn create_exercise_routine(
        env: &Env,
        name: &str,
        description: &str,
        exercises: Vec<Exercise>,
    ) -> ExerciseRoutine {
        ExerciseRoutine {
            name: String::from_str(env, name),
            description: String::from_str(env, description),
            exercises,
            last_updated: env.ledger().timestamp(),
        }
    }

    // Create a meal
    pub fn create_meal(
        env: &Env,
        name: &str,
        time: &str,
        foods: Vec<String>,
        calories: u32,
        notes: &str,
    ) -> Meal {
        Meal {
            name: String::from_str(env, name),
            time: String::from_str(env, time),
            foods,
            calories,
            notes: String::from_str(env, notes),
        }
    }

    // Create a meal plan
    pub fn create_meal_plan(
        env: &Env,
        name: &str,
        description: &str,
        meals: Vec<Meal>,
    ) -> MealPlan {
        MealPlan {
            name: String::from_str(env, name),
            description: String::from_str(env, description),
            meals,
            last_updated: env.ledger().timestamp(),
        }
    }

    // Create a progress update
    pub fn create_progress_update(
        env: &Env,
        date: u64,
        metrics: Map<String, String>,
        notes: &str,
    ) -> ProgressUpdate {
        ProgressUpdate {
            date,
            metrics,
            notes: String::from_str(env, notes),
            last_updated: env.ledger().timestamp(),
        }
    }

    // Add an exercise to a routine
    pub fn add_exercise_to_routine(
        env: &Env,
        mut routine: ExerciseRoutine,
        exercise: Exercise,
    ) -> ExerciseRoutine {
        routine.exercises.push_back(exercise);
        routine.last_updated = env.ledger().timestamp();
        routine
    }

    // Add a meal to a meal plan
    pub fn add_meal_to_plan(
        env: &Env,
        mut plan: MealPlan,
        meal: Meal,
    ) -> MealPlan {
        plan.meals.push_back(meal);
        plan.last_updated = env.ledger().timestamp();
        plan
    }

    // Add a metric to progress update
    pub fn add_metric_to_progress(
        env: &Env,
        mut progress: ProgressUpdate,
        key: &str,
        value: &str,
    ) -> ProgressUpdate {
        progress.metrics.set(
            String::from_str(env, key),
            String::from_str(env, value),
        );
        progress.last_updated = env.ledger().timestamp();
        progress
    }
} 