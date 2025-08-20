#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, Address, Env, String,};
use sep_0041::Sep0041Client;


#[contracttype]
#[derive(Debug)]
pub struct Employee{
    pub name: String,
    pub address: Address,
    pub pay: u128
}


#[contracttype]
#[derive(Debug)]
pub enum DataKey{
    Owner,
    Contract,
    EmployeeCount,
    Employee(Address),
    Exist(Address), // to ensure no duplicate
}


#[contract]
pub struct EmployeeContract;

#[contractimpl]
impl EmployeeContract {

    pub fn __constructor(env: &Env, admin: Address, token_contract_address: Address) {
        if env.storage().instance().has(&DataKey::Owner) {
            // throw error
        }
        // save the owner here
        env.storage().instance().set(&DataKey::Owner, &admin);
        env.storage().instance().set(&DataKey::Contract, &token_contract_address);
    }

    pub fn get_owner(env: &Env) -> Address {
        env.storage().instance().get(&DataKey::Owner).expect("not owner yet")
    }

    pub fn get_employee_count(env: &Env) -> u128 {
        env.storage().instance().get(&DataKey::EmployeeCount).unwrap_or(0_u128)
    }
    // add an employee
    pub fn add_employee(env: &Env, admin: Address, new_employee_name: String, new_employee_address: Address, employee_pay: u128) {
        admin.require_auth();
        // check the employee address has not been taken
        if env.storage().instance().has(&DataKey::Exist(new_employee_address.clone())) {
            // throw duplicate error
        }

        // create the instance and save the employee
        let new_employee = Employee{
            name: new_employee_name,
            address: new_employee_address.clone(),
            pay: employee_pay
        };

        env.storage().instance().set(&DataKey::Employee(new_employee_address.clone()), &new_employee);
        env.storage().instance().set(&DataKey::Exist(new_employee_address.clone()), &true);
        let present_count: u128 = env.storage().instance().get(&DataKey::EmployeeCount).unwrap_or(0_u128);
        env.storage().instance().set(&DataKey::EmployeeCount, &(present_count + 1));
    }


    pub fn pay_employee(env: &Env, admin: Address, employee_address: Address,) {

        admin.require_auth();

        //get the pay of the employee
        let employee_details: Employee = env.storage().instance().get(&DataKey::Employee(employee_address.clone())).unwrap();
        let employee_pay: u128 = employee_details.pay;

        let token_address: Address = env.storage().instance().get(&DataKey::Contract).expect("");

        let sep_0041_instance = Sep0041Client::new(env, &token_address);

        let contract_address: Address = env.current_contract_address();

        sep_0041_instance.transfer_from(&contract_address, &admin, &employee_details.address, &(employee_pay as i128) );

    }
}


mod test;
