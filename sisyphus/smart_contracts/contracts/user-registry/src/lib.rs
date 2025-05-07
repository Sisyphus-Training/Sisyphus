#![no_std]
use soroban_sdk::{
    contract, contractimpl, contracttype, 
    Address, Env, Map, String, Vec,
};

/// User data structure to store on the blockchain
#[contracttype]
#[derive(Clone)]
pub struct UserData {
    pub name: String,
    pub email: String,
    pub metadata: Map<String, String>,
}

/// Main contract for user registry management
#[contract]
pub struct UserRegistry;

/// DataKey enum to define storage keys
#[contracttype]
pub enum DataKey {
    /// Key for storing all registered user IDs
    UserIds,
    /// Key for storing user data, combined with the user's unique identifier
    UserData(Address),
}

#[contractimpl]
impl UserRegistry {
    /// Create a new user with the provided data
    /// 
    /// # Arguments
    ///
    /// * `env` - The contract environment
    /// * `user_id` - The unique identifier for the user (Stellar address)
    /// * `name` - The user's name
    /// * `email` - The user's email address
    /// * `metadata` - Additional user metadata as key-value pairs
    ///
    /// # Returns
    ///
    /// Boolean indicating success or failure
    pub fn create_user(
        env: Env,
        user_id: Address,
        name: String,
        email: String,
        metadata: Map<String, String>,
    ) -> bool {
        // Verify the user is not already registered
        if Self::user_exists(&env, &user_id) {
            return false;
        }

        // Ensure the invoker is the same as the user_id or has proper authorization
        user_id.require_auth();

        // Create user data object
        let user_data = UserData {
            name,
            email,
            metadata,
        };

        // Store the user data
        env.storage().persistent().set(&DataKey::UserData(user_id.clone()), &user_data);

        // Add user ID to the list of registered users
        let mut user_ids = Self::get_all_user_ids(&env);
        user_ids.push_back(user_id);
        env.storage().persistent().set(&DataKey::UserIds, &user_ids);

        true
    }

    /// Update existing user data
    ///
    /// # Arguments
    ///
    /// * `env` - The contract environment
    /// * `user_id` - The unique identifier for the user
    /// * `name` - The user's name
    /// * `email` - The user's email address
    /// * `metadata` - Additional user metadata as key-value pairs
    ///
    /// # Returns
    ///
    /// Boolean indicating success or failure
    pub fn update_user(
        env: Env,
        user_id: Address,
        name: String,
        email: String,
        metadata: Map<String, String>,
    ) -> bool {
        // Verify the user exists
        if !Self::user_exists(&env, &user_id) {
            return false;
        }

        // Ensure the invoker is the same as the user_id or has proper authorization
        user_id.require_auth();

        // Create updated user data object
        let user_data = UserData {
            name,
            email,
            metadata,
        };

        // Update the user data
        env.storage().persistent().set(&DataKey::UserData(user_id), &user_data);

        true
    }

    /// Get user data by ID
    ///
    /// # Arguments
    ///
    /// * `env` - The contract environment
    /// * `user_id` - The unique identifier for the user
    ///
    /// # Returns
    ///
    /// The user data if found, or None if not found
    pub fn get_user(env: Env, user_id: Address) -> Option<UserData> {
        if Self::user_exists(&env, &user_id) {
            let user_data: UserData = env.storage().persistent().get(&DataKey::UserData(user_id)).unwrap();
            Some(user_data)
        } else {
            None
        }
    }

    /// Get a list of all registered user IDs
    ///
    /// # Arguments
    ///
    /// * `env` - The contract environment
    ///
    /// # Returns
    ///
    /// Vector containing all registered user IDs
    pub fn get_all_user_ids(env: &Env) -> Vec<Address> {
        match env.storage().persistent().get::<DataKey, Vec<Address>>(&DataKey::UserIds) {
            Some(ids) => ids,
            None => Vec::new(env),
        }
    }

    /// Check if a user exists in the registry
    ///
    /// # Arguments
    ///
    /// * `env` - The contract environment
    /// * `user_id` - The unique identifier for the user
    ///
    /// # Returns
    ///
    /// Boolean indicating if the user exists
    fn user_exists(env: &Env, user_id: &Address) -> bool {
        env.storage().persistent().has(&DataKey::UserData(user_id.clone()))
    }

    /// Delete a user from the registry
    ///
    /// # Arguments
    ///
    /// * `env` - The contract environment
    /// * `user_id` - The unique identifier for the user
    ///
    /// # Returns
    ///
    /// Boolean indicating success or failure
    pub fn delete_user(env: Env, user_id: Address) -> bool {
        // Verify the user exists
        if !Self::user_exists(&env, &user_id) {
            return false;
        }

        // Ensure the invoker is the same as the user_id or has proper authorization
        user_id.require_auth();

        // Remove user data
        env.storage().persistent().remove(&DataKey::UserData(user_id.clone()));

        // Remove user ID from the list of registered users
        let user_ids = Self::get_all_user_ids(&env);
        let mut new_ids = Vec::new(&env);
        
        for id in user_ids.iter() {
            if id != user_id {
                new_ids.push_back(id);
            }
        }
        
        env.storage().persistent().set(&DataKey::UserIds, &new_ids);

        true
    }
}

mod test; 