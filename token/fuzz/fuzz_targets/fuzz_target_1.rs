#![no_main]

use libfuzzer_sys::fuzz_target;
use soroban_sdk::*;

fuzz_target!(|transfer: i128| {
    let env = Env::default();
    let admin = Address::generate(&env);
    let id = env.register(Token, (&admin,));
    let token = TokenClient::new(&env, &id);

    let a = Address::generate(&env);
    token.mock_all_auths().mint(&a, &10);
    let b = Address::generate(&env);
    token.mock_all_auths().mint(&b, &0);

    let result = token.mock_all_auths().try_transfer(&a, &b, &transfer);
    match result {
        Ok(Ok(())) => {}
        Ok(Err(_)) => panic!("unexpected type"),
        Err(Ok(_)) => {}
        Err(Err(_)) => panic!("unexpected error"),
    }
});
