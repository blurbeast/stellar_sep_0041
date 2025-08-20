use soroban_sdk::{contracttype, Address};

#[contracttype]
pub struct Approve {
    from: Address,
    spender: Address,
}
