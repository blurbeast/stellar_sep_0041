pub use self::employee_contract_events::{
    emit_employee_paid, emit_employee_suspended, emit_remove_employee,
};

mod employee_contract_events {
    use soroban_sdk::{contracttype, Address, Env};

    #[contracttype]
    #[derive(Debug)]
    pub struct EmployeeRemoved {
        pub employee: Address,
    }

    #[contracttype]
    #[derive(Debug)]
    pub struct EmployeeSuspended {
        pub employee: Address,
    }

    #[contracttype]
    #[derive(Debug)]
    pub struct EmployeePaid {
        pub employee: Address,
        pub amount: u128,
    }

    pub fn emit_employee_paid(env: &Env, address: Address, amount: u128) {
        let employee_paid: EmployeePaid = EmployeePaid {
            employee: address,
            amount: amount,
        };
        env.events().publish(("employee_paid",), employee_paid);
    }

    pub fn emit_remove_employee(env: &Env, address: Address) {
        let employee_removed: EmployeeRemoved = EmployeeRemoved { employee: address };

        env.events()
            .publish(("employee_removed",), employee_removed);
    }

    pub fn emit_employee_suspended(env: &Env, address: &Address) {
        let employee_suspended: EmployeeSuspended = EmployeeSuspended {
            employee: address.clone(),
        };
        env.events()
            .publish(("employee_suspended",), employee_suspended);
    }
}
