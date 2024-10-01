#![no_main]

use libfuzzer_sys::fuzz_target;
use soroban_sdk::*;
use soroban_sdk::testutils::*;

#[derive(Arbitrary)]
pub struct Input {
    pub a: i128,
    pub b: i128,
    pub amount: i128,
}

fuzz_target!(|input: Input| {
    let env = Env::default();
    let admin = Address::generate(&env);
    let id = env.register(Token, (&admin,));
    let token = TokenClient::new(&env, &id);

    let a = Address::generate(&env);
    token.mock_all_auths().mint(&a, &input.a);
    let b = Address::generate(&env);
    token.mock_all_auths().mint(&b, &input.b);

    let result = token.mock_all_auths().try_transfer(&a, &b, &input.amount);
    match result {
        Ok(Ok(())) => {}
        Ok(Err(_)) => panic!("unexpected type"),
        Err(Ok(_)) => {}
        Err(Err(_)) => panic!("unexpected error"),
    }
});
