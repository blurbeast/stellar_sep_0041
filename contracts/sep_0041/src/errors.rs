use soroban_sdk::contracterror;

#[contracterror]
#[derive(Debug, PartialEq)]
pub enum Sep0041Error {
    InsufficientBalance = 1,
}
