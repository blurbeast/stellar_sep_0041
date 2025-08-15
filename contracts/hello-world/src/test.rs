
#[cfg(test)]
mod test {

    use soroban_sdk::{testutils::{Address as _, Ledger}, Address, Env, String};

    fn generate_addresses(env: &Env) -> (Address, Address, Address) {
        let first_address: Address = Address::generate(env);
        let second_addess: Address = Address::generate(env);
        let third_address: Address = Address::generate(env);

        (first_address, second_addess, third_address)
    }

    use crate::contract_sep_41::{Sep0041, Sep0041Client};
    fn setup() -> (Env, Sep0041Client<'static>, Address) {
        let env: Env = Env::default();

        let (admin, _, _) = generate_addresses(&env);
        let contract_id = env.register(Sep0041, (admin, String::from_str(&env, "loaded"), String::from_str(&env, "lsd",)));
        let contract_client = Sep0041Client::new(&env, &contract_id);
        env.mock_all_auths();
        (env, contract_client, contract_id)
    }
    
     #[test]
    fn test_constructor_values() {
        let (env, contract, _) = setup();
        assert_eq!(contract.name(), String::from_str(&env, "loaded"));
        assert_eq!(contract.symbol(), String::from_str(&env, "lsd"));
        assert_eq!(contract.decimals(), 18_u32);
    }

     #[test]
    fn test_mint_and_balance() {
        let (env, contract_instance, _) = setup();
        let (_, user1, _) = generate_addresses(&env);
        assert_eq!(contract_instance.balance(&user1), 0);
        let res = contract_instance.mint(&user1, &500);
        
        assert_eq!(res, true);
        assert_eq!(contract_instance.balance(&user1), 500);
        assert_eq!(contract_instance.total_supply(), 500);
    }

    #[test]
    fn test_transfer() {
        let (env, contract_instance, _) = setup();
        let (_, user1, user2) = generate_addresses(&env);

        contract_instance.mint(&user1, &300);
        contract_instance.transfer(&user1, &user2, &100);

        assert_eq!(contract_instance.balance(&user1), 200);
        assert_eq!(contract_instance.balance(&user2), 100);
    }

     #[test]
    fn test_approve_and_allowance() {
        let (env, contract_instance, _) = setup();
        let (_, owner, spender) = generate_addresses(&env);

        contract_instance.mint(&owner, &400);
        contract_instance.approve(&owner, &spender, &150, &5);

        let allowance = contract_instance.allowance(&owner, &spender);
        assert_eq!(allowance, 150);
    }

    #[test]
    fn test_transfer_from_updates_allowance() {
        let (env, contract_instance, _) = setup();
        let (_, owner, spender) = generate_addresses(&env);
        let (_, _, recipient) = generate_addresses(&env);

        contract_instance.mint(&owner, &500);
        contract_instance.approve(&owner, &spender, &200, &5);
        contract_instance.transfer_from(&spender, &owner, &recipient, &150);

        assert_eq!(contract_instance.balance(&owner), 350);
        assert_eq!(contract_instance.balance(&recipient), 150);
        assert_eq!(contract_instance.allowance(&owner, &spender), 50);
    }

    #[test]
    fn test_burn_and_burn_from() {
        let (env, contract_instance, _) = setup();
        let (_, owner, spender) = generate_addresses(&env);

        // Burn
        contract_instance.mint(&owner, &300);
        contract_instance.burn(&owner, &100);
        assert_eq!(contract_instance.balance(&owner), 200);
        assert_eq!(contract_instance.total_supply(), 200);

        // Burn From
        contract_instance.approve(&owner, &spender, &150, &5);
        contract_instance.burn_from(&spender, &owner, &100);
        assert_eq!(contract_instance.balance(&owner), 100);
        assert_eq!(contract_instance.allowance(&owner, &spender), 50);
    }

    #[test]
    #[should_panic]
    fn test_transfer_insufficient_balance_fails() {
        let (env, contract_instance, _) = setup();
        let (_, user1, user2) = generate_addresses(&env);
        contract_instance.transfer(&user1, &user2, &50);
    }

    #[test]
    #[should_panic(expected = "invalid amount")]
    fn test_transfer_zero_amount_fails() {
        let (env, contract_instance, _) = setup();
        let (_, user1, user2) = generate_addresses(&env);
        contract_instance.transfer(&user1, &user2, &0);
    }

    #[test]
    fn test_multiple_transfers() {
        let (env, contract_instance, _) = setup();
        let (_, user1, user2) = generate_addresses(&env);

        contract_instance.mint(&user1, &500);
        contract_instance.transfer(&user1, &user2, &100);
        contract_instance.transfer(&user1, &user2, &50);

        assert_eq!(contract_instance.balance(&user1), 350);
        assert_eq!(contract_instance.balance(&user2), 150);
    }

     #[test]
    #[should_panic]
    fn test_transfer_from_exceeds_allowance_fails() {
        let (env, contract_instance, _) = setup();
        let (_, owner, spender) = generate_addresses(&env);
        let (_, _, recipient) = generate_addresses(&env);

        contract_instance.mint(&owner, &500);
        contract_instance.approve(&owner, &spender, &100, &5);
        contract_instance.transfer_from(&spender, &owner, &recipient, &150);
    }

    #[test]
    #[should_panic]
    fn test_allowance_expiry_fails() {
        let (env, contract_instance, _) = setup();
        let (_, owner, spender) = generate_addresses(&env);
        let (_, _, recipient) = generate_addresses(&env);

        contract_instance.mint(&owner, &300);
        contract_instance.approve(&owner, &spender, &100, &1);

        // Simulate ledger time passing beyond deadline
        env.ledger().set_timestamp(env.ledger().timestamp() + (60 *3));

            contract_instance.transfer_from(&spender, &owner, &recipient, &50);
        
    }
}