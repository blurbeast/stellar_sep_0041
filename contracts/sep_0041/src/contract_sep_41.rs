
use soroban_sdk::{contract, contractimpl, Address, String, Env, log};
use crate::i_sep_41::ISep0041;
use crate::storage::{AllowanaceDetails, DataKey, SECONDS_IN_TIME};
use crate::errors::Sep0041Error;


#[contract]
pub struct Sep0041;

#[contractimpl]
impl Sep0041 {
    pub fn __constructor(env: &Env, admin: Address, name: String, symbol: String, ) {
        env.storage().instance().set(&DataKey::Admin, &admin);
        env.storage().instance().set(&DataKey::Name, &name);
        env.storage().instance().set(&DataKey::Symbol, &symbol);
        env.storage().instance().set(&DataKey::Decimal, &18_u32);
    }

    pub fn total_supply(env: &Env) -> i128 {
        Self::_total_supply(env)
    }
}



#[contractimpl]
impl ISep0041 for Sep0041 {
    // fn init() -> Result<>
    fn balance(env: &Env, id: Address) -> i128 {
        // get the balance from the storage
        Self::_balance(env, &id)
    }

    fn name(env: &Env) -> String {
        Self::_name(env)
    }
    fn decimals(env: &Env) -> u32 {
        Self::_decimal(env)
    }
    fn symbol(env: &Env) -> String {
        Self::_symbol(env)
    }

    fn mint(env: &Env, to: Address, amount: i128) -> Result<bool, Sep0041Error> {

        // only admin
        let admin: Address = Self::_admin(env);
        // throw error next
        admin.require_auth();

        Self::_check_for_zero_amount(amount);
        log!(env, "before balance");

        let to_balance = Self::_balance(&env, &to);

        log!(env, "after balance");

        let new_balance: i128 = to_balance + amount;
        // save the new balance
        Self::_update_balance(env, &to, new_balance);
        let total_supply: i128 = Self::_total_supply(env);
        Self::_update_total_supply(env, total_supply + amount);
        Ok(true)
    }

    fn allowance(env: &Env, from: Address, spender: Address) -> i128 {
        let (amount, _) = Self::_allowance(env, &from, &spender);
        amount
    }

    fn approve(env: &Env, from: Address, spender: Address, amount: i128, live_until_ledger: u32) {

        // authenticate the approver 'from'
        from.require_auth();

        Self::_check_for_zero_amount(amount);

        let deadline_time_stamp: u64 = live_until_ledger as u64 * SECONDS_IN_TIME;

        assert!(deadline_time_stamp > Self::_current_time_stame(env));

        //now create the details and save
        let tx_details: AllowanaceDetails = Self::_create_allowance_details(amount, env.ledger().timestamp() + (live_until_ledger as u64 * SECONDS_IN_TIME));

        Self::_update_allowance(env, from, spender, tx_details);
    }

    fn transfer(env: &Env, from: Address, to: Address, amount: i128) {
        from.require_auth();

        Self::_check_for_zero_amount(amount);
        Self::_transfer(env, &from, &to, amount);
    }

    fn burn(env: &Env, from: Address, amount: i128) {
        from.require_auth();

        //get from the from balance
        let from_balance: i128 = Self::_balance(env, &from);

        assert!(from_balance>=amount,);

        Self::_burn(env, &from, amount, from_balance);
    }

    fn transfer_from(env: &Env, spender: Address, from: Address, to: Address, amount: i128) {
        spender.require_auth();
        // check allowance
        // check allowance deadline
        let (allowance, deadline) = Self::_allowance(env, &from, &spender);

        let current_time_stamp: u64 = env.ledger().timestamp();

        assert!(allowance >= amount && deadline >= current_time_stamp, "insufficient allowance or exceed deadline");

        // transfer
        Self::_transfer(env, &from, &to, amount);
        // update allowance
        let tx_details: AllowanaceDetails = Self::_create_allowance_details(allowance - amount, deadline);
        Self::_update_allowance(env, from, spender, tx_details);
    }

    fn burn_from(env: &Env, spender: Address, from: Address, amount: i128) {
        spender.require_auth();
        let (allowance, deadline) = Self::_allowance(env, &from, &spender);
        let current_time_stamp: u64 = env.ledger().timestamp();

        assert!(allowance >= amount && deadline >= current_time_stamp, "insufficient allowance or exceed deadline");

        let from_balance: i128 = Self::_balance(env, &from);
        Self::_burn(env, &from, amount, from_balance);

        let tx_details: AllowanaceDetails = Self::_create_allowance_details(allowance - amount, deadline);
        Self::_update_allowance(env, from, spender, tx_details);
    }
}

impl Sep0041 {



    fn _check_for_zero_amount(amount: i128) {
        assert!(amount>0, "invalid amount");
    }
    fn _current_time_stame(env: &Env) -> u64 {
        env.ledger().timestamp()
    }

    fn _name(env: &Env) -> String {
        env.storage().instance().get(&DataKey::Name).unwrap()
    }
    fn _decimal(env: &Env) -> u32 {
        env.storage().instance().get(&DataKey::Decimal).unwrap()   
    }
    fn _symbol(env: &Env) -> String {
        env.storage().instance().get(&DataKey::Symbol).unwrap()
    }

    fn _admin(env: &Env) -> Address {
        env.storage().instance().get(&DataKey::Admin).unwrap()
    }

    fn _create_allowance_details(amount: i128, deadline: u64) -> AllowanaceDetails {
        AllowanaceDetails { amount, deadline }
    }

    fn _burn(env: &Env, from: &Address, amount: i128, from_balance: i128) {
        
        // we update the states, from balance and the total supply
        let from_new_balance: i128 = from_balance - amount;

        // call the update method
        Self::_update_balance(env, &from, from_new_balance);

        // get total_supply
        let total_supply: i128 = Self::_total_supply(env);
        // update total supply
        Self::_update_total_supply(env, total_supply-amount);
    }
    

    fn _transfer(env: &Env, from: &Address, to: &Address, amount: i128) {

         let from_balance: i128 = Self::_balance(env, from);
         assert!(from_balance >= amount);

        // to balance
        let to_balance: i128 = Self::_balance(env, to);

        let from_new_balance: i128 = from_balance - amount;
        let to_new_balance: i128 = to_balance + amount;

        Self::_update_balance(env, from, from_new_balance);
        Self::_update_balance(env, to, to_new_balance);
    }

    fn _balance(env: &Env, id: &Address) -> i128 {
        let balance_key: DataKey = DataKey::Balance(id.clone());
        log!(env, "balance key {}", balance_key);
        let result = env.storage().instance().get(&balance_key).unwrap_or(0_i128);
        log!(env, "we should see here  {}", balance_key);
        return result;
    }

    fn _update_balance(env: &Env, id: &Address, amount: i128) {
        env.storage().instance().set(&DataKey::Balance(id.clone()), &amount);
        log!(env, "updated balance")
    }

    fn _update_allowance(env: &Env, from: Address, spender: Address, tx_details: AllowanaceDetails) {
        env.storage().instance().set(&DataKey::Allowance(from.clone(), spender.clone()), &tx_details);
    }
    fn _allowance(env: &Env, from: &Address, spender: &Address) -> (i128, u64) {
        let tx_details: AllowanaceDetails = env.storage().instance().get(&DataKey::Allowance(from.clone(), spender.clone())).unwrap();
        (tx_details.amount, tx_details.deadline)
    }

    fn _total_supply(env: &Env) -> i128 {
        env.storage().instance().get(&DataKey::TotalSupply).unwrap_or(0)
    }

    fn _update_total_supply(env: &Env, total_supply: i128) {
        env.storage().instance().set(&DataKey::TotalSupply, &(total_supply));
    }
}