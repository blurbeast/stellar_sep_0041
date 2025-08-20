
use soroban_sdk::{Address, contracttype};

pub static SECONDS_IN_TIME: u64 = 60;

#[contracttype]
#[derive(Debug, )]
pub struct AllowanaceDetails {
    pub amount: i128,
    pub deadline: u64
}

#[contracttype]
#[derive(Debug)]
pub enum DataKey {
    Balance(Address),
    Name,
    Symbol,
    Decimal,
    Admin,
    TotalSupply,
    Allowance(Address, Address)
}
