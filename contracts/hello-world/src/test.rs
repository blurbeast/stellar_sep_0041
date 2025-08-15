// #![cfg(test)]

// use super::*;
// use soroban_sdk::{vec, Env, String};

// #[test]
// fn test() {
//     let env = Env::default();
//     let contract_id = env.register(Contract, ());
//     let client = ContractClient::new(&env, &contract_id);

//     let words = client.hello(&String::from_str(&env, "Dev"));
//     assert_eq!(
//         words,
//         vec![
//             &env,
//             String::from_str(&env, "Hello"),
//             String::from_str(&env, "Dev"),
//         ]
//     );
// }


#[cfg(test)]
mod test {
    use core::ops::Add;

    use soroban_sdk::{testutils::Address as _, Address, log, Env, String};

    fn generate_addresses(env: &Env) -> (Address, Address, Address) {
        let first_address: Address = Address::generate(env);
        let second_addess: Address = Address::generate(env);
        let third_address: Address = Address::generate(env);

        (first_address, second_addess, third_address)
    }

    use crate::{Sep0041, Sep0041Client};
    fn setup() -> (Env, Sep0041Client<'static>, Address) {
        let env: Env = Env::default();

        let (admin, _, _) = generate_addresses(&env);
        let contract_id = env.register(Sep0041, (admin, String::from_str(&env, "loaded"), String::from_str(&env, "lsd",)));
        let contract_client = Sep0041Client::new(&env, &contract_id);
        env.mock_all_auths();
        (env, contract_client, contract_id)
    }
    #[test]
    fn test_name_and_symbol() {
        let (env, contract_instance, _) = setup();
        let contract_name = contract_instance.name();
        let constract_symbol: String = contract_instance.symbol();

        assert_eq!(contract_name, String::from_str(&env, "loaded"));
        assert_eq!(constract_symbol, String::from_str(&env, "lsd"))

        
    }
}