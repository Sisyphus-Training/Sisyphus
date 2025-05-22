use soroban_sdk::{contracttype, Address, String};

#[contracttype]
#[derive(Clone, Debug, PartialEq)]
pub struct Subscription {
    pub user: Address,          // User's public key
    pub start_date: u64,       // Unix timestamp
    pub duration: u32,         // Duration in days
    pub status: String,        // Active or Expired
    pub payment_amount: i128,  // Payment in XLM (stroops)
    pub payment_date: u64,     // Payment timestamp
    pub transaction_id: String, // Stellar transaction ID
}

#[contracttype]
pub enum DataKey {
    Subscriptions(Address),     // Maps user address to their subscription
    AllSubscriptions,          // List of all subscriptions
}
