#![no_main]

use libfuzzer_sys::fuzz_target;
use soroban_sdk::testutils::arbitrary::*;
use soroban_sdk::testutils::Address as _;
use soroban_sdk::Address;
use soroban_sdk::Env;
use token::*;

#[derive(Arbitrary, Debug)]
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
    _ = token.mock_all_auths().try_mint(&a, &input.a);
    let b = Address::generate(&env);
    _ = token.mock_all_auths().try_mint(&b, &input.b);

    let result = token.mock_all_auths().try_transfer(&a, &b, &input.amount);
    match result {
        Ok(Ok(())) => {}
        Ok(Err(_)) => panic!("unexpected type"),
        Err(Ok(_)) => {}
        Err(Err(_)) => panic!("unexpected error"),
    }
});
