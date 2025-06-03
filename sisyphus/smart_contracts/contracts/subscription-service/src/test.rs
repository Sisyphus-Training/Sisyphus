use super::*;
use crate::types::{Subscription, DataKey};
use crate::errors::Error;
use soroban_sdk::{
    testutils::{Address as _, Ledger},
    Address, Env, String, Vec,
};

#[cfg(test)]
mod test_setup {
    use super::*;

    /// Create a test environment with the subscription contract and client
    pub fn create_test_contract(env: &Env) -> (Address, SubscriptionContractClient) {
        let contract_id = env.register(SubscriptionContract, ());
        let client = SubscriptionContractClient::new(env, &contract_id);
        (contract_id, client)
    }

    /// Setup arguments for creating a subscription
    pub fn setup_subscription_args(env: &Env) -> (Address, u32, i128, String) {
        let user = Address::generate(env);
        let duration = 30; // 30 days
        let payment_amount = 10_000_000; // 10 XLM in stroops
        let transaction_id = String::from_str(env, "tx123");
        (user, duration, payment_amount, transaction_id)
    }
}

mod test_initialization {
    use super::*;

    #[test]
    fn test_init() {
        let env = Env::default();
        env.mock_all_auths();
        let (contract_id, client) = test_setup::create_test_contract(&env);

        client.init();

        env.as_contract(&contract_id, || {
            let all_subs: Vec<Subscription> = env
                .storage()
                .persistent()
                .get(&DataKey::AllSubscriptions)
                .unwrap();
            assert_eq!(all_subs.len(), 0, "AllSubscriptions should be empty");
        });
    }

    #[test]
    fn test_init_already_initialized() {
        let env = Env::default();
        env.mock_all_auths();
        let (contract_id, client) = test_setup::create_test_contract(&env);

        client.init();
        client.init(); // Second init should not overwrite

        env.as_contract(&contract_id, || {
            let all_subs: Vec<Subscription> = env
                .storage()
                .persistent()
                .get(&DataKey::AllSubscriptions)
                .unwrap();
            assert_eq!(all_subs.len(), 0, "AllSubscriptions should remain empty");
        });
    }
}

mod test_add_subscription {
    use super::*;

    #[test]
    fn test_add_subscription_success() {
        let env = Env::default();
        env.mock_all_auths();
        let (contract_id, client) = test_setup::create_test_contract(&env);
        let (user, duration, payment_amount, transaction_id) = test_setup::setup_subscription_args(&env);

        client.init();
        let result = client.try_add_subscription(&user, &duration, &payment_amount, &transaction_id);
        assert!(result.is_ok(), "Add subscription failed: {:?}", result);

        env.as_contract(&contract_id, || {
            let sub: Subscription = env
                .storage()
                .persistent()
                .get(&DataKey::Subscriptions(user.clone()))
                .unwrap();
            assert_eq!(sub.user, user, "User address mismatch");
            assert_eq!(sub.duration, duration, "Duration mismatch");
            assert_eq!(sub.payment_amount, payment_amount, "Payment amount mismatch");
            assert_eq!(sub.transaction_id, transaction_id, "Transaction ID mismatch");
            assert_eq!(sub.status, String::from_str(&env, "Active"), "Status should be Active");
            assert_eq!(sub.start_date, env.ledger().timestamp(), "Start date mismatch");
            assert_eq!(sub.payment_date, env.ledger().timestamp(), "Payment date mismatch");

            let all_subs: Vec<Subscription> = env
                .storage()
                .persistent()
                .get(&DataKey::AllSubscriptions)
                .unwrap();
            assert_eq!(all_subs.len(), 1, "AllSubscriptions should have one entry");
        });
    }

    #[test]
    fn test_add_subscription_invalid_duration() {
        let env = Env::default();
        env.mock_all_auths();
        let (_, client) = test_setup::create_test_contract(&env);
        let (user, _, payment_amount, transaction_id) = test_setup::setup_subscription_args(&env);

        client.init();
        let result = client.try_add_subscription(&user, &0, &payment_amount, &transaction_id);
        assert_eq!(
            result,
            Err(Ok(Error::InvalidInput)),
            "Expected InvalidInput error"
        );
    }

    #[test]
    fn test_add_subscription_invalid_payment_amount() {
        let env = Env::default();
        env.mock_all_auths();
        let (_, client) = test_setup::create_test_contract(&env);
        let (user, duration, _, transaction_id) = test_setup::setup_subscription_args(&env);

        client.init();
        let result = client.try_add_subscription(&user, &duration, &0, &transaction_id);
        assert_eq!(
            result,
            Err(Ok(Error::InvalidInput)),
            "Expected InvalidInput error"
        );
    }

    #[test]
    fn test_add_subscription_invalid_transaction_id() {
        let env = Env::default();
        env.mock_all_auths();
        let (_, client) = test_setup::create_test_contract(&env);
        let (user, duration, payment_amount, _) = test_setup::setup_subscription_args(&env);
        let invalid_transaction_id = String::from_str(&env, "");

        client.init();
        let result = client.try_add_subscription(&user, &duration, &payment_amount, &invalid_transaction_id);
        assert_eq!(
            result,
            Err(Ok(Error::InvalidTransactionId)),
            "Expected InvalidTransactionId error"
        );
    }

    #[test]
    fn test_add_subscription_active_exists() {
        let env = Env::default();
        env.mock_all_auths();
        let (_, client) = test_setup::create_test_contract(&env);
        let (user, duration, payment_amount, transaction_id) = test_setup::setup_subscription_args(&env);

        client.init();
        client.add_subscription(&user, &duration, &payment_amount, &transaction_id);
        let result = client.try_add_subscription(&user, &duration, &payment_amount, &transaction_id);
        assert_eq!(
            result,
            Err(Ok(Error::ActiveSubscriptionExists)),
            "Expected ActiveSubscriptionExists error"
        );
    }
}

mod test_renew_subscription {
    use super::*;

    #[test]
    fn test_renew_subscription_success() {
        let env = Env::default();
        env.mock_all_auths();
        let (contract_id, client) = test_setup::create_test_contract(&env);
        let (user, duration, payment_amount, transaction_id) = test_setup::setup_subscription_args(&env);

        client.init();
        client.add_subscription(&user, &duration, &payment_amount, &transaction_id);

        // Simulate expiration
        env.ledger().with_mut(|li| {
            li.timestamp += duration as u64 * 86_400 + 1; // Past expiration
        });

        let new_duration = 60;
        let new_payment_amount = 20_000_000; // 2 XLM
        let new_transaction_id = String::from_str(&env, "tx456");
        let result = client.try_renew_subscription(&user, &new_duration, &new_payment_amount, &new_transaction_id);
        assert!(result.is_ok(), "Renew subscription failed: {:?}", result);

        env.as_contract(&contract_id, || {
            let sub: Subscription = env
                .storage()
                .persistent()
                .get(&DataKey::Subscriptions(user.clone()))
                .unwrap();
            assert_eq!(sub.duration, new_duration, "New duration mismatch");
            assert_eq!(sub.payment_amount, new_payment_amount, "New payment amount mismatch");
            assert_eq!(sub.transaction_id, new_transaction_id, "New transaction ID mismatch");
            assert_eq!(sub.status, String::from_str(&env, "Active"), "Status should be Active");
            assert_eq!(sub.start_date, env.ledger().timestamp(), "New start date mismatch");

            let all_subs: Vec<Subscription> = env
                .storage()
                .persistent()
                .get(&DataKey::AllSubscriptions)
                .unwrap();
            assert_eq!(all_subs.len(), 1, "AllSubscriptions should have one entry");
            assert_eq!(all_subs.get(0).unwrap().duration, new_duration, "AllSubscriptions duration mismatch");
        });
    }

    #[test]
    fn test_renew_subscription_no_subscription() {
        let env = Env::default();
        env.mock_all_auths();
        let (_, client) = test_setup::create_test_contract(&env);
        let (user, duration, payment_amount, transaction_id) = test_setup::setup_subscription_args(&env);

        client.init();
        let result = client.try_renew_subscription(&user, &duration, &payment_amount, &transaction_id);
        assert_eq!(
            result,
            Err(Ok(Error::SubscriptionNotFound)),
            "Expected SubscriptionNotFound error"
        );
    }

    #[test]
    fn test_renew_subscription_still_active() {
        let env = Env::default();
        env.mock_all_auths();
        let (_, client) = test_setup::create_test_contract(&env);
        let (user, duration, payment_amount, transaction_id) = test_setup::setup_subscription_args(&env);

        client.init();
        client.add_subscription(&user, &duration, &payment_amount, &transaction_id);
        let result = client.try_renew_subscription(&user, &duration, &payment_amount, &transaction_id);
        assert_eq!(
            result,
            Err(Ok(Error::SubscriptionStillActive)),
            "Expected SubscriptionStillActive error"
        );
    }

    #[test]
    fn test_renew_subscription_invalid_duration() {
        let env = Env::default();
        env.mock_all_auths();
        let (_, client) = test_setup::create_test_contract(&env);
        let (user, duration, payment_amount, transaction_id) = test_setup::setup_subscription_args(&env);

        client.init();
        client.add_subscription(&user, &duration, &payment_amount, &transaction_id);
        env.ledger().with_mut(|li| {
            li.timestamp += duration as u64 * 86_400 + 1;
        });

        let result = client.try_renew_subscription(&user, &0, &payment_amount, &transaction_id);
        assert_eq!(
            result,
            Err(Ok(Error::InvalidInput)),
            "Expected InvalidInput error"
        );
    }

    #[test]
    fn test_renew_subscription_invalid_payment_amount() {
        let env = Env::default();
        env.mock_all_auths();
        let (_, client) = test_setup::create_test_contract(&env);
        let (user, duration, payment_amount, transaction_id) = test_setup::setup_subscription_args(&env);

        client.init();
        client.add_subscription(&user, &duration, &payment_amount, &transaction_id);
        env.ledger().with_mut(|li| {
            li.timestamp += duration as u64 * 86_400 + 1;
        });

        let result = client.try_renew_subscription(&user, &duration, &0, &transaction_id);
        assert_eq!(
            result,
            Err(Ok(Error::InvalidInput)),
            "Expected InvalidInput error"
        );
    }

    #[test]
    fn test_renew_subscription_invalid_transaction_id() {
        let env = Env::default();
        env.mock_all_auths();
        let (_, client) = test_setup::create_test_contract(&env);
        let (user, duration, payment_amount, transaction_id) = test_setup::setup_subscription_args(&env);
        let invalid_transaction_id = String::from_str(&env, "");

        client.init();
        client.add_subscription(&user, &duration, &payment_amount, &transaction_id);
        env.ledger().with_mut(|li| {
            li.timestamp += duration as u64 * 86_400 + 1;
        });

        let result = client.try_renew_subscription(&user, &duration, &payment_amount, &invalid_transaction_id);
        assert_eq!(
            result,
            Err(Ok(Error::InvalidTransactionId)),
            "Expected InvalidTransactionId error"
        );
    }
}

mod test_get_subscription {
    use super::*;

    #[test]
    fn test_get_subscription_success() {
        let env = Env::default();
        env.mock_all_auths();
        let (_, client) = test_setup::create_test_contract(&env);
        let (user, duration, payment_amount, transaction_id) = test_setup::setup_subscription_args(&env);

        client.init();
        client.add_subscription(&user, &duration, &payment_amount, &transaction_id);

        let sub = client.get_subscription(&user);
        assert_eq!(sub.user, user, "User address mismatch");
        assert_eq!(sub.duration, duration, "Duration mismatch");
        assert_eq!(sub.payment_amount, payment_amount, "Payment amount mismatch");
        assert_eq!(sub.transaction_id, transaction_id, "Transaction ID mismatch");
        assert_eq!(sub.status, String::from_str(&env, "Active"), "Status should be Active");
    }

    #[test]
    fn test_get_subscription_not_found() {
        let env = Env::default();
        env.mock_all_auths();
        let (_, client) = test_setup::create_test_contract(&env);
        let user = Address::generate(&env);

        client.init();
        let result = client.try_get_subscription(&user);
        assert_eq!(
            result,
            Err(Ok(Error::SubscriptionNotFound)),
            "Expected SubscriptionNotFound error"
        );
    }
}

mod test_get_active_subscriptions {
    use super::*;

    #[test]
    fn test_get_active_subscriptions_success() {
        let env = Env::default();
        env.mock_all_auths();
        let (contract_id, client) = test_setup::create_test_contract(&env);
        let (user1, duration, payment_amount, transaction_id) = test_setup::setup_subscription_args(&env);
        let user2 = Address::generate(&env);

        client.init();
        client.add_subscription(&user1, &duration, &payment_amount, &transaction_id);
        client.add_subscription(&user2, &duration, &payment_amount, &transaction_id);

        let active_subs = client.get_active_subscriptions();
        assert_eq!(active_subs.len(), 2, "Should have two active subscription");

        // Expire user2's subscription 
        env.ledger().with_mut(|li| {
            li.timestamp += duration as u64 * 86_400 + 1;
        });
        client.update_status(&user2);

        // Check active subscriptions again (user1 is expired with Active status and user2 is expired with Expired status)
        let active_subs = client.get_active_subscriptions();
        assert_eq!(active_subs.len(), 0, "Should have no active subscription");

        env.as_contract(&contract_id, || {
            let all_subs: Vec<Subscription> = env
                .storage()
                .persistent()
                .get(&DataKey::AllSubscriptions)
                .unwrap();
            assert_eq!(all_subs.len(), 2, "AllSubscriptions should have two entries");
        });
    }

    #[test]
    fn test_get_active_subscriptions_none() {
        let env = Env::default();
        env.mock_all_auths();
        let (_, client) = test_setup::create_test_contract(&env);

        client.init();
        let active_subs = client.get_active_subscriptions();
        assert_eq!(active_subs.len(), 0, "Should have no active subscriptions");
    }
}

mod test_update_status {
    use super::*;

    #[test]
    fn test_update_status_success() {
        let env = Env::default();
        env.mock_all_auths();
        let (contract_id, client) = test_setup::create_test_contract(&env);
        let (user, duration, payment_amount, transaction_id) = test_setup::setup_subscription_args(&env);

        client.init();
        client.add_subscription(&user, &duration, &payment_amount, &transaction_id);

        env.ledger().with_mut(|li| {
            li.timestamp += duration as u64 * 86_400 + 1;
        });

        let result = client.try_update_status(&user);
        assert!(result.is_ok(), "Update status failed: {:?}", result);

        env.as_contract(&contract_id, || {
            let sub: Subscription = env
                .storage()
                .persistent()
                .get(&DataKey::Subscriptions(user.clone()))
                .unwrap();
            assert_eq!(sub.status, String::from_str(&env, "Expired"), "Status should be Expired");

            let all_subs: Vec<Subscription> = env
                .storage()
                .persistent()
                .get(&DataKey::AllSubscriptions)
                .unwrap();
            assert_eq!(all_subs.get(0).unwrap().status, String::from_str(&env, "Expired"), "AllSubscriptions status mismatch");
        });
    }

    #[test]
    fn test_update_status_not_expired() {
        let env = Env::default();
        env.mock_all_auths();
        let (contract_id, client) = test_setup::create_test_contract(&env);
        let (user, duration, payment_amount, transaction_id) = test_setup::setup_subscription_args(&env);

        client.init();
        client.add_subscription(&user, &duration, &payment_amount, &transaction_id);

        let result = client.try_update_status(&user);
        assert!(result.is_ok(), "Update status failed: {:?}", result);

        env.as_contract(&contract_id, || {
            let sub: Subscription = env
                .storage()
                .persistent()
                .get(&DataKey::Subscriptions(user.clone()))
                .unwrap();
            assert_eq!(sub.status, String::from_str(&env, "Active"), "Status should remain Active");
        });
    }

    #[test]
    fn test_update_status_not_found() {
        let env = Env::default();
        env.mock_all_auths();
        let (_, client) = test_setup::create_test_contract(&env);
        let user = Address::generate(&env);

        client.init();
        let result = client.try_update_status(&user);
        assert_eq!(
            result,
            Err(Ok(Error::SubscriptionNotFound)),
            "Expected SubscriptionNotFound error"
        );
    }
}
