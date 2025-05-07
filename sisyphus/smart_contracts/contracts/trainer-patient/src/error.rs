use soroban_sdk::contracterror;

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum ContractError {
    // Authentication/Authorization errors
    Unauthorized = 1,
    NotAdmin = 2,
    NotTrainer = 3,
    NotPatient = 4,
    
    // Registration errors
    AlreadyInitialized = 10,
    TrainerAlreadyRegistered = 11,
    PatientAlreadyLinked = 12,
    
    // Runtime errors
    TrainerNotFound = 20,
    PatientNotFound = 21,
    PatientNotLinkedWithTrainer = 22,
    InvalidDataFormat = 23,
    
    // General errors
    InternalError = 100,
} 