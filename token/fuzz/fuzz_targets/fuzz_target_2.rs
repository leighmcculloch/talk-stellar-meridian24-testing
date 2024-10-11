#![no_main]

use libfuzzer_sys::fuzz_target;
use soroban_sdk::testutils::arbitrary::*;
use soroban_sdk::testutils::Address as _;
use soroban_sdk::Address;
use soroban_sdk::Env;
use soroban_sdk::IntoVal;
use token::*;

#[derive(Arbitrary, Debug)]
pub enum Action {
    Mint(<Address as SorobanArbitrary>::Prototype, i128),
    Balance(<Address as SorobanArbitrary>::Prototype),
    Transfer(
        <Address as SorobanArbitrary>::Prototype,
        <Address as SorobanArbitrary>::Prototype,
        i128,
    ),
}

fuzz_target!(|actions: std::vec::Vec<Action>| {
    let env = Env::default();
    env.mock_all_auths();
    let admin = Address::generate(&env);
    let id = env.register(Token, (&admin,));
    let token = TokenClient::new(&env, &id);

    for a in actions {
        match a {
            Action::Mint(addr, amount) => {
                _ = token.try_mint(&addr.into_val(&env), &amount);
                assert!(token.balance(&addr.into_val(&env)) >= 0);
            },
            Action::Balance(addr) => {
                _ = token.try_balance(&addr.into_val(&env));
                assert!(token.balance(&addr.into_val(&env)) >= 0);
            },
            Action::Transfer(from, to, amount) => {
                _ = token.try_transfer(&from.into_val(&env), &to.into_val(&env), &amount);
                assert!(token.balance(&from.into_val(&env)) >= 0);
                assert!(token.balance(&to.into_val(&env)) >= 0);
            }
        }
    }
});
