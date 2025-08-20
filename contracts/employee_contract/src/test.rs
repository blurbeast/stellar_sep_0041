#[cfg(test)]
mod test {
    use crate::{EmployeeContract, EmployeeContractClient};
    use sep_0041::contract_sep_41::Sep0041;
    use sep_0041::Sep0041Client;
    use soroban_sdk::{log, testutils::Address as _, Address, Env, String};

    fn generate_addresses(env: &Env) -> (Address, Address, Address) {
        let first_address: Address = Address::generate(env);
        let second_address: Address = Address::generate(env);
        let third_address: Address = Address::generate(env);

        (first_address, second_address, third_address)
    }

    fn setup() -> (
        Env,
        EmployeeContractClient<'static>,
        Address,
        Address,
        Sep0041Client<'static>,
        Address,
    ) {
        let env: Env = Env::default();

        let (admin, second_admin, _) = generate_addresses(&env);
        log!(&env, "before ::: the sep0041");
        let contract_sep0041_id = env.register(
            Sep0041,
            (
                admin.clone(),
                String::from_str(&env, "loaded"),
                String::from_str(&env, "lsd"),
            ),
        );

        let sep0041_instance = Sep0041Client::new(&env, &contract_sep0041_id.clone());

        log!(&env, "after the sep0041");

        let contract_employee_id = env.register(
            EmployeeContract,
            (second_admin.clone(), contract_sep0041_id),
        );

        log!(&env, "employee contract deployed successfully");

        env.mock_all_auths();

        let employee_client = EmployeeContractClient::new(&env, &contract_employee_id.clone());

        (
            env,
            employee_client,
            admin,
            second_admin,
            sep0041_instance,
            contract_employee_id,
        )
    }

    #[test]
    fn test_deploy_employee() {
        let (env, employee_client, _, sec_admin, _, _) = setup();
        assert_eq!(employee_client.get_owner(), sec_admin)
    }

    #[test]
    fn test_add_employee() {
        let (env, employee_client, _, sec_admin, _, _) = setup();
        let (a, b, _) = generate_addresses(&env);
        let employee_name = String::from_str(&env, "dele");

        // add an employee
        employee_client.add_employee(&sec_admin, &employee_name, &a, &2000);

        // count is :::
        assert_eq!(employee_client.get_employee_count(), 1);

        employee_client.add_employee(&sec_admin, &employee_name, &b, &2000);
        assert_eq!(employee_client.get_employee_count(), 2);
    }

    // #[test]
    // #[should_panic]
    // fn test_pay_employee_no_allowance() {
    //     let (env, employee_client, _, sec_admin, _) = setup();
    //     let (a, b, _) = generate_addresses(&env);
    //     let employee_name = String::from_str(&env, "dele");
    //
    //     // add an employee
    //     employee_client.add_employee(&sec_admin, &employee_name, &a, &2000);
    //
    //     employee_client.pay_employee(&sec_admin, &a);
    // }

    #[test]
    fn test_pay_employee() {
        let (env, employee_client, _, sec_admin, sep41_client, c) = setup();
        let (a, b, _) = generate_addresses(&env);
        let employee_name = String::from_str(&env, "dele");

        // add an employe
        employee_client.add_employee(&sec_admin, &employee_name, &a, &2000);
        //first mint
        sep41_client.mint(&sec_admin, &10_000);
        //check the balance of both the employer and employee
        let employer_balance = sep41_client.balance(&sec_admin);
        let employee_balance = sep41_client.balance(&b);
        //
        assert_eq!(employer_balance, 10_000);
        assert_eq!(employee_balance, 0);

        // log!(&env, "before the approve is called");
        // let contract_addr = env.current_contract_address();
        // approve the contract
        sep41_client.approve(&sec_admin, &c, &2000, &3);

        // employer pays employee
        employee_client.pay_employee(&sec_admin, &a);
        // confirm balance changes
        let employer_balance = sep41_client.balance(&sec_admin);
        let employee_balance = sep41_client.balance(&a);

        assert_eq!(employer_balance, 8_000);
        assert_eq!(employee_balance, 2_000);
    }
}
