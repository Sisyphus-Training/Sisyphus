#![cfg(test)]

use super::*;
use soroban_sdk::{
    testutils::{Address as AddressTestUtils},
    Address, Env, Map,
};

#[test]
fn test_create_user() {
    let env = Env::default();
    let contract_id = env.register(UserRegistry, ());
    let client = UserRegistryClient::new(&env, &contract_id);

    // Create a test user ID
    let user = Address::generate(&env);

    // Create empty metadata map
    let mut metadata = Map::new(&env);
    metadata.set(String::from_str(&env, "account_type"), String::from_str(&env, "standard"));

    // Create mock user with authorization
    env.mock_all_auths();

    // Create a new user
    let result = client.create_user(
        &user,
        &String::from_str(&env, "John Doe"),
        &String::from_str(&env, "john.doe@example.com"),
        &metadata,
    );

    // Assert user creation was successful
    assert!(result);

    // Attempt to create the same user again
    let result = client.create_user(
        &user,
        &String::from_str(&env, "John Doe"),
        &String::from_str(&env, "john.doe@example.com"),
        &metadata,
    );

    // Assert duplicate user creation fails
    assert!(!result);
}

#[test]
fn test_get_user() {
    let env = Env::default();
    let contract_id = env.register(UserRegistry, ());
    let client = UserRegistryClient::new(&env, &contract_id);

    // Create a test user ID
    let user = Address::generate(&env);

    // Create metadata map
    let mut metadata = Map::new(&env);
    metadata.set(String::from_str(&env, "account_type"), String::from_str(&env, "premium"));

    // Mock authorization
    env.mock_all_auths();

    // Create a new user
    client.create_user(
        &user,
        &String::from_str(&env, "Jane Smith"),
        &String::from_str(&env, "jane.smith@example.com"),
        &metadata,
    );

    // Retrieve the user
    let user_data = client.get_user(&user);
    
    // Assert user data matches what was stored
    assert!(user_data.is_some());
    let data = user_data.unwrap();
    assert_eq!(data.name, String::from_str(&env, "Jane Smith"));
    assert_eq!(data.email, String::from_str(&env, "jane.smith@example.com"));
    
    // Verify metadata
    let account_type = data.metadata.get(String::from_str(&env, "account_type")).unwrap();
    assert_eq!(account_type, String::from_str(&env, "premium"));
}

#[test]
fn test_update_user() {
    let env = Env::default();
    let contract_id = env.register(UserRegistry, ());
    let client = UserRegistryClient::new(&env, &contract_id);

    // Create a test user ID
    let user = Address::generate(&env);

    // Create initial metadata
    let mut metadata = Map::new(&env);
    metadata.set(String::from_str(&env, "account_type"), String::from_str(&env, "standard"));

    // Mock authorization
    env.mock_all_auths();

    // Create a new user
    client.create_user(
        &user,
        &String::from_str(&env, "Alex Johnson"),
        &String::from_str(&env, "alex.johnson@example.com"),
        &metadata,
    );

    // Update metadata
    let mut updated_metadata = Map::new(&env);
    updated_metadata.set(String::from_str(&env, "account_type"), String::from_str(&env, "premium"));
    updated_metadata.set(String::from_str(&env, "subscription"), String::from_str(&env, "annual"));

    // Update the user
    let result = client.update_user(
        &user,
        &String::from_str(&env, "Alexander Johnson"),
        &String::from_str(&env, "alexander.johnson@example.com"),
        &updated_metadata,
    );

    // Assert update was successful
    assert!(result);

    // Retrieve the updated user
    let user_data = client.get_user(&user);
    
    // Assert user data was properly updated
    let data = user_data.unwrap();
    assert_eq!(data.name, String::from_str(&env, "Alexander Johnson"));
    assert_eq!(data.email, String::from_str(&env, "alexander.johnson@example.com"));
    
    // Verify updated metadata
    let account_type = data.metadata.get(String::from_str(&env, "account_type")).unwrap();
    assert_eq!(account_type, String::from_str(&env, "premium"));
    
    let subscription = data.metadata.get(String::from_str(&env, "subscription")).unwrap();
    assert_eq!(subscription, String::from_str(&env, "annual"));
}

#[test]
fn test_delete_user() {
    let env = Env::default();
    let contract_id = env.register(UserRegistry, ());
    let client = UserRegistryClient::new(&env, &contract_id);

    // Create test user IDs
    let user1 = Address::generate(&env);
    let user2 = Address::generate(&env);

    // Create metadata
    let mut metadata = Map::new(&env);
    metadata.set(String::from_str(&env, "account_type"), String::from_str(&env, "standard"));

    // Mock authorization
    env.mock_all_auths();

    // Create users
    client.create_user(
        &user1,
        &String::from_str(&env, "User One"),
        &String::from_str(&env, "user.one@example.com"),
        &metadata,
    );
    
    client.create_user(
        &user2,
        &String::from_str(&env, "User Two"),
        &String::from_str(&env, "user.two@example.com"),
        &metadata,
    );

    // Verify both users exist
    assert!(client.get_user(&user1).is_some());
    assert!(client.get_user(&user2).is_some());
    
    // Delete user1
    let result = client.delete_user(&user1);
    assert!(result);
    
    // Verify user1 no longer exists
    assert!(client.get_user(&user1).is_none());
    
    // Verify user2 still exists
    assert!(client.get_user(&user2).is_some());
    
    // Try to delete non-existent user
    let non_existent_user = Address::generate(&env);
    let result = client.delete_user(&non_existent_user);
    assert!(!result);
} 