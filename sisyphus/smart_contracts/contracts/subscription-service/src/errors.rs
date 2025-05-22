use soroban_sdk::{contracterror, String};

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
// #[repr(u32)]
pub enum Error {
    InvalidInput = 1,
    ActiveSubscriptionExists = 2,
    InvalidTransactionId = 3,
    SubscriptionNotFound = 4,
    SubscriptionStillActive = 5,
}

impl Error {
    pub fn to_string(&self, env: &soroban_sdk::Env) -> String {
        match self {
            Error::InvalidInput => String::from_str(env, "Invalid duration or payment amount"),
            Error::ActiveSubscriptionExists => String::from_str(env, "Active subscription already exists"),
            Error::InvalidTransactionId => String::from_str(env, "Invalid transaction ID"),
            Error::SubscriptionNotFound => String::from_str(env, "Subscription not found"),
            Error::SubscriptionStillActive => String::from_str(env, "Subscription still active"),
        }
    }
}
