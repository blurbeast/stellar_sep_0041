use crate::errors::Sep0041Error;
use soroban_sdk::{contractclient, Address, Env, String};

#[contractclient(name = "Sep0041Client")]
pub trait ISep0041 {
    fn balance(env: &Env, id: Address) -> i128;
    fn name(env: &Env) -> String;
    fn decimals(env: &Env) -> u32;
    fn symbol(env: &Env) -> String;
    fn mint(env: &Env, to: Address, amount: i128) -> Result<bool, Sep0041Error>;
    fn allowance(env: &Env, from: Address, spender: Address) -> i128;
    fn approve(env: &Env, from: Address, spender: Address, amount: i128, live_until_ledger: u32);
    fn transfer(env: &Env, from: Address, to: Address, amount: i128);
    fn burn(env: &Env, from: Address, amount: i128);
    fn transfer_from(env: &Env, spender: Address, from: Address, to: Address, amount: i128);
    fn burn_from(env: &Env, spender: Address, from: Address, amount: i128);
}
