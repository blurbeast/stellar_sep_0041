#![no_std]
use soroban_sdk::{contract, contracterror, contractimpl, contracttype, vec, Address, Env, String, Vec};

mod test;


static SECONDS_IN_TIME: u64 = 60;

#[contracterror]
#[derive(Debug, PartialEq,)]
pub enum Sep0041Error {
    InsufficientBalance = 1,

}


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


#[contracttype]
pub struct Approve {
    from: Address,
    spender: Address
}


#[contract]
pub struct Sep0041;

trait ISep0041 {
    fn balance(env: &Env, id: Address) -> i128;
    fn name(env: &Env) -> String;
    fn decimals(env: &Env) -> u32;
    fn symbol(env: Env) -> String;
    fn mint(env: &Env, to: Address, amount: i128)  -> Result<bool, Sep0041Error> ;
    fn allowance(env: &Env, from: Address, spender: Address) -> i128;
    fn approve(env: &Env, from: Address, spender: Address, amount: i128, live_until_ledger: u32);
    fn transfer(env: &Env, from: Address, to: Address, amount: i128);
}

#[contractimpl]
impl Sep0041 {
    pub fn __constructor(env: &Env, admin: Address, name: String, symbol: String, ) {
        env.storage().instance().set(&DataKey::Admin, &admin);
        env.storage().instance().set(&DataKey::Name, &name);
        env.storage().instance().set(&DataKey::Symbol, &symbol);
        env.storage().instance().set(&DataKey::Decimal, &18_u32);
    }
}



#[contractimpl]
impl ISep0041 for Sep0041 {
    // fn init() -> Result<>
    fn balance(env: &Env, id: Address) -> i128 {
        // get the balace from the storage
        let balance_key: DataKey = DataKey::Balance(id);
        env.storage().instance().get(&balance_key).unwrap_or(0_i128)
    }

    fn name(env: &Env) -> String {
        env.storage().instance().get(&DataKey::Name).unwrap()
    }
    fn decimals(env: &Env) -> u32 {
        env.storage().instance().get(&DataKey::Decimal).unwrap()
    }
    fn symbol(env: Env) -> String {
        env.storage().instance().get(&DataKey::Symbol).unwrap()
    }

    fn mint(env: &Env, to: Address, amount: i128) -> Result<bool, Sep0041Error>{

        // only admin
        let admin: Address = env.storage().instance().get(&DataKey::Admin).unwrap();
        // throw error next 
        admin.require_auth();
        // get the balance to

        let to_balance = Self::balance(&env, to.clone());

        // throw error if zero or less
        if amount <= 0 {
            return Err(Sep0041Error::InsufficientBalance);
        }
        let new_balance:i128 = to_balance + amount;   
        // save the new balance
        env.storage().instance().set(&DataKey::Balance(to.clone()),&(new_balance));
        let total_supply: i128 = env.storage().instance().get(&DataKey::TotalSupply).unwrap();
        env.storage().instance().set(&DataKey::TotalSupply, &(total_supply + amount));
        Ok(true)
    }

    fn allowance(env: &Env, from: Address, spender: Address) -> i128 {
        let tx_details: AllowanaceDetails = env.storage().instance().get(&DataKey::Allowance(from, spender)).unwrap();
        tx_details.amount
    }

    fn approve(env: &Env, from: Address, spender: Address, amount: i128, live_until_ledger: u32) {

        // authenticate the approver 'from'
        from.require_auth();

        // check the amount 
        if amount <= 0 {
            todo!()
        }

        // check the time 
        if env.ledger().timestamp() >= (live_until_ledger as u64 * SECONDS_IN_TIME) {
            todo!()
        }

        // check if there was a previous one before , if yes , remove
        if env.storage().instance().has(&DataKey::Allowance(from.clone(), spender.clone())) {
            env.storage().instance().remove(&DataKey::Allowance(from.clone(), spender.clone()));   
        }

        //now create the details and save 
        let tx_details: AllowanaceDetails = AllowanaceDetails {
            amount: amount,
            deadline: (env.ledger().timestamp() + (live_until_ledger as u64 * SECONDS_IN_TIME))
        };

        env.storage().instance().set(&DataKey::Allowance(from.clone(), spender.clone()), &tx_details);

    }

    fn transfer(env: &Env, from: Address, to: Address, amount: i128) {

        from.require_auth();

        if amount <= 0 {
            todo!()
        }

        Self::_transfer(env, from, to, amount);
        // // check from balance 
        // let from_balance: i128 = env.storage().instance().get(&DataKey::Balance(from.clone())).unwrap();

        // assert!(from_balance >= amount, "insufficient");

        // // 
        // let to_balance: i128 = env.storage().instance().get(&DataKey::Balance(to.clone())).unwrap();

        // let from_new_balance: i128 = from_balance - amount;
        // let to_new_balance: i128 = to_balance + amount;

        // // update the state
        // env.storage().instance().set(&DataKey::Balance(from.clone()), &from_new_balance);
        // env.storage().instance().set(&DataKey::Balance(to.clone()), &to_new_balance);
    }
}

impl Sep0041 {
    

    fn _transfer(env: &Env, from: Address, to: Address, amount: i128) {

         let from_balance: i128 = env.storage().instance().get(&DataKey::Balance(from.clone())).unwrap();

        // to balance
        let to_balance: i128 = env.storage().instance().get(&DataKey::Balance(to.clone())).unwrap();

        let from_new_balance: i128 = from_balance - amount;
        let to_new_balance: i128 = to_balance + amount;

        env.storage().instance().set(&DataKey::Balance(from.clone()), &from_new_balance);
        env.storage().instance().set(&DataKey::Balance(to.clone()), &to_new_balance);
    }
}