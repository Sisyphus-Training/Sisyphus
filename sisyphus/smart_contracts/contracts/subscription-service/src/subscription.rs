use soroban_sdk::{contract, contractimpl, Address, Env, String, Vec};
use crate::types::{Subscription, DataKey};
use crate::errors::Error;

#[contract]
pub struct SubscriptionContract;

#[contractimpl]
impl SubscriptionContract {
    // Initialize contract (optional, for future extensions)
    pub fn init(env: Env) {
        let storage = env.storage().persistent();
        if !storage.has(&DataKey::AllSubscriptions) {
            storage.set(&DataKey::AllSubscriptions, &Vec::<Subscription>::new(&env));
        }
    }

    // Add a new subscription with payment
    pub fn add_subscription(
        env: Env,
        user: Address,
        duration: u32,
        payment_amount: i128,
        transaction_id: String,
    ) -> Result<(), Error> {
        // Input validation
        if duration == 0 || payment_amount <= 0 {
            return Err(Error::InvalidInput);
        }

        let storage = env.storage().persistent();
        let key = DataKey::Subscriptions(user.clone());

        // Check for existing subscription
        if storage.has(&key) {
            let sub: Subscription = storage.get(&key).unwrap();
            if sub.status == String::from_str(&env, "Active") {
                return Err(Error::ActiveSubscriptionExists);
            }
        }

        // Verify payment (simplified for testnet)
        // In production, integrate with Stellar Horizon API to verify transaction
        if transaction_id.is_empty() {
            return Err(Error::InvalidTransactionId);
        }

        // Create new subscription
        let subscription = Subscription {
            user: user.clone(),
            start_date: env.ledger().timestamp(),
            duration,
            status: String::from_str(&env, "Active"),
            payment_amount,
            payment_date: env.ledger().timestamp(),
            transaction_id,
        };

        // Store subscription
        storage.set(&key, &subscription);

        // Update all subscriptions list
        let mut all_subs: Vec<Subscription> = storage.get(&DataKey::AllSubscriptions).unwrap();
        all_subs.push_back(subscription.clone());
        storage.set(&DataKey::AllSubscriptions, &all_subs);

        Ok(())
    }

    // Renew an existing subscription
    pub fn renew_subscription(
        env: Env,
        user: Address,
        duration: u32,
        payment_amount: i128,
        transaction_id: String,
    ) -> Result<(), Error> {
        let storage = env.storage().persistent();
        let key = DataKey::Subscriptions(user.clone());

        // Check if subscription exists
        if !storage.has(&key) {
            return Err(Error::SubscriptionNotFound);
        }

        // Validate inputs
        if duration == 0 || payment_amount <= 0 {
            return Err(Error::InvalidInput);
        }

        // Verify payment (simplified for testnet)
        if transaction_id.is_empty() {
            return Err(Error::InvalidTransactionId);
        }

        let mut sub: Subscription = storage.get(&key).unwrap();

        // Check if subscription is expired
        let current_time = env.ledger().timestamp();
        let expiry_time = sub.start_date + (sub.duration as u64 * 86_400); // Convert days to seconds
        if current_time < expiry_time && sub.status == String::from_str(&env, "Active") {
            return Err(Error::SubscriptionStillActive);
        }

        // Update subscription
        sub.start_date = current_time;
        sub.duration = duration;
        sub.status = String::from_str(&env, "Active");
        sub.payment_amount = payment_amount;
        sub.payment_date = current_time;
        sub.transaction_id = transaction_id;

        // Store updated subscription
        storage.set(&key, &sub);

        // Update all subscriptions list
        let mut all_subs: Vec<Subscription> = storage.get(&DataKey::AllSubscriptions).unwrap();
        for i in 0..all_subs.len() {
            if all_subs.get(i).unwrap().user == user {
                all_subs.set(i, sub.clone());
                break;
            }
        }
        storage.set(&DataKey::AllSubscriptions, &all_subs);

        Ok(())
    }

    // Query subscription by user
    pub fn get_subscription(env: Env, user: Address) -> Result<Subscription, Error> {
        let key = DataKey::Subscriptions(user.clone());
        let storage = env.storage().persistent();

        if !storage.has(&key) {
            return Err(Error::SubscriptionNotFound);
        }

        let sub: Subscription = storage.get(&key).unwrap();
        Ok(sub)
    }

    // Query all active subscriptions
    pub fn get_active_subscriptions(env: Env) -> Vec<Subscription> {
        let storage = env.storage().persistent();
        let all_subs: Vec<Subscription> = storage.get(&DataKey::AllSubscriptions).unwrap();
        let current_time = env.ledger().timestamp();

        let mut active_subs = Vec::new(&env);
        for sub in all_subs.iter() {
            let expiry_time = sub.start_date + (sub.duration as u64 * 86_400);
            if current_time < expiry_time && sub.status == String::from_str(&env, "Active") {
                active_subs.push_back(sub);
            }
        }
        active_subs
    }

    // Check and update subscription status
    pub fn update_status(env: Env, user: Address) -> Result<(), Error> {
        let storage = env.storage().persistent();
        let key = DataKey::Subscriptions(user.clone());

        if !storage.has(&key) {
            return Err(Error::SubscriptionNotFound);
        }

        let mut sub: Subscription = storage.get(&key).unwrap();
        let current_time = env.ledger().timestamp();
        let expiry_time = sub.start_date + (sub.duration as u64 * 86_400);

        if current_time >= expiry_time && sub.status != String::from_str(&env, "Expired") {
            sub.status = String::from_str(&env, "Expired");
            storage.set(&key, &sub);

            // Update all subscriptions list
            let mut all_subs: Vec<Subscription> = storage.get(&DataKey::AllSubscriptions).unwrap();
            for i in 0..all_subs.len() {
                if all_subs.get(i).unwrap().user == user {
                    all_subs.set(i, sub.clone());
                    break;
                }
            }
            storage.set(&DataKey::AllSubscriptions, &all_subs);
        }

        Ok(())
    }
}
