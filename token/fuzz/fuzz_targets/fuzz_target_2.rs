#![no_main]

use ::token::*;
use libfuzzer_sys::fuzz_target;
use soroban_sdk::testutils::arbitrary::*;
use soroban_sdk::testutils::Address as _;
use soroban_sdk::*;

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
            Action::Mint(addr, amount) => _ = token.try_mint(&addr.into_val(&env), &amount),
            Action::Balance(addr) => _ = token.try_balance(&addr.into_val(&env)),
            Action::Transfer(from, to, amount) => {
                _ = token.try_transfer(&from.into_val(&env), &to.into_val(&env), &amount)
            }
        }
    }

    // assert on invariants
});
