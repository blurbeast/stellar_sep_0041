#![no_std]
use sep_0041::Sep0041Client;
use soroban_sdk::{contract, contracterror, contractimpl, contracttype, Address, Env, String};

use crate::events::{emit_employee_paid, emit_employee_suspended, emit_remove_employee};

#[contracttype]
#[derive(Debug)]
pub struct Employee {
    pub name: String,
    pub address: Address,
    pub pay: u128,
    pub rank: Rank,
}

#[contracttype]
#[derive(Debug, PartialEq)]
pub enum Rank {
    Level_1,
    Level_2,
    Level_3,
}

#[contracttype]
#[derive(Debug)]
pub enum DataKey {
    Owner,
    Contract,
    EmployeeCount,
    Employee(Address),
    Exist(Address), // to ensure no duplicate
    Suspended(Address),
}

#[contracterror]
#[derive(Debug)]
pub enum EmployeeContractError {
    NotAnEmployee = 1,
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
        env.storage()
            .instance()
            .set(&DataKey::Contract, &token_contract_address);
    }

    pub fn get_owner(env: &Env) -> Address {
        env.storage()
            .instance()
            .get(&DataKey::Owner)
            .expect("no owner yet")
    }

    pub fn get_employee_count(env: &Env) -> u128 {
        env.storage()
            .instance()
            .get(&DataKey::EmployeeCount)
            .unwrap_or(0_u128)
    }

    pub fn get_employee(env: &Env, address: Address) -> Option<Employee> {
        Self::_get_employee(env, &address)
    }
    // add an employee
    pub fn add_employee(
        env: &Env,
        admin: Address,
        new_employee_name: String,
        new_employee_address: Address,
        employee_pay: u128,
    ) {
        admin.require_auth();
        // check the employee address has not been taken
        if env
            .storage()
            .instance()
            .has(&DataKey::Exist(new_employee_address.clone()))
        {
            // throw duplicate error
        }

        // create the instance and save the employee
        let new_employee = Employee {
            name: new_employee_name,
            address: new_employee_address.clone(),
            pay: employee_pay,
            rank: Rank::Level_1,
        };

        env.storage().instance().set(
            &DataKey::Employee(new_employee_address.clone()),
            &new_employee,
        );
        env.storage()
            .instance()
            .set(&DataKey::Exist(new_employee_address.clone()), &true);
        let present_count: u128 = env
            .storage()
            .instance()
            .get(&DataKey::EmployeeCount)
            .unwrap_or(0_u128);
        env.storage()
            .instance()
            .set(&DataKey::EmployeeCount, &(present_count + 1));
    }

    pub fn remove_employee(
        env: &Env,
        employee_address: Address,
    ) -> Result<(), EmployeeContractError> {
        // check if employee exist

        let is_employee: bool = env
            .storage()
            .instance()
            .get(&DataKey::Exist(employee_address.clone()))
            .unwrap_or_else(|| false);
        if !is_employee {
            return Err(EmployeeContractError::NotAnEmployee);
        }

        // get the admin
        let admin: Address = Self::_get_owner(env);
        admin.require_auth();

        // remove the employee
        env.storage()
            .instance()
            .remove(&DataKey::Employee(employee_address.clone()));
        env.storage()
            .instance()
            .set(&DataKey::Exist(employee_address.clone()), &false);
        // reduce the count
        let current_employee_count: u128 = env
            .storage()
            .instance()
            .get(&DataKey::EmployeeCount)
            .unwrap();
        env.storage()
            .instance()
            .set(&DataKey::EmployeeCount, &(current_employee_count - 1));

        emit_remove_employee(env, employee_address);
        Ok(())
    }

    pub fn pay_employee(env: &Env, admin: Address, employee_address: Address) {
        admin.require_auth();

        //get the pay of the employee
        let employee_details: Employee = env
            .storage()
            .instance()
            .get(&DataKey::Employee(employee_address.clone()))
            .unwrap();
        let employee_pay: u128 = employee_details.pay;

        let token_address: Address = env.storage().instance().get(&DataKey::Contract).expect("");

        let sep_0041_instance = Sep0041Client::new(env, &token_address);

        let contract_address: Address = env.current_contract_address();

        sep_0041_instance.transfer_from(
            &contract_address,
            &admin,
            &employee_details.address,
            &(employee_pay as i128),
        );

        emit_employee_paid(env, employee_address, employee_details.pay);
    }

    pub fn is_employee_suspended(env: &Env, address: Address) -> Option<bool> {
        Self::_get_employee_suspension_status(env, &address)
    }

    pub fn suspend_employee(
        env: &Env,
        employee_address: Address,
    ) -> Result<(), EmployeeContractError> {
        let _ = Self::_check_if_address_is_an_employee(env, &employee_address);

        Self::_suspend_employee(env, &employee_address);
        emit_employee_suspended(env, &employee_address);
        Ok(())
    }

    pub fn promote_employee(env: &Env, address: Address, level: u32) {
        let mut employee_details = Self::_check_if_address_is_an_employee(env, &address).unwrap();

        let rank = Rank::check_level(level as u8);

        employee_details.rank = rank;

        env.storage()
            .instance()
            .set(&DataKey::Employee(address), &employee_details);
    }
}

impl Rank {
    fn check_level(level: u8) -> Rank {
        match level {
            1 => Rank::Level_2,
            2 => Rank::Level_3,
            _ => Rank::Level_1,
        }
    }
}
impl EmployeeContract {
    fn _get_owner(env: &Env) -> Address {
        env.storage().instance().get(&DataKey::Owner).unwrap()
    }

    fn _check_if_address_is_an_employee(
        env: &Env,
        employee_address: &Address,
    ) -> Result<Employee, EmployeeContractError> {
        let is_employee: bool = env
            .storage()
            .instance()
            .has(&DataKey::Exist(employee_address.clone()));

        if !is_employee {
            return Err(EmployeeContractError::NotAnEmployee);
        }

        Ok(env
            .storage()
            .instance()
            .get(&DataKey::Employee(employee_address.clone()))
            .unwrap())
    }

    fn _suspend_employee(env: &Env, address: &Address) {
        env.storage()
            .instance()
            .set(&DataKey::Suspended(address.clone()), &true);
    }

    fn _get_employee_suspension_status(env: &Env, address: &Address) -> Option<bool> {
        env.storage()
            .instance()
            .get(&DataKey::Suspended(address.clone()))
    }

    fn _get_employee(env: &Env, address: &Address) -> Option<Employee> {
        env.storage()
            .instance()
            .get(&DataKey::Employee(address.clone()))
    }
}

mod events;
mod test;
