#![cfg(test)]

use soroban_sdk::{testutils::Address as _, Address, Env, String};

use super::*;

#[test]
fn test() {
    let env = Env::default();
    let a = Address::generate(&env);
    let id = env.register_contract_with_constructor(None, Token, (&a, 10i128));
    let token = TokenClient::new(&env, &id);
    assert_eq!(token.balance(&a), 10);
}

#[contract]
struct Pause;

#[contractimpl]
impl Pause {
    pub fn paused() -> bool {
        false
    }
}

#[test]
fn test_mock() {
    let env = Env::default();
    let pause_id = env.register_contract(None, Pause);
    // Use pause mock in Token setup ...
}
