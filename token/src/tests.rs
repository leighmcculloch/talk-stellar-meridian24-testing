#![cfg(test)]

use soroban_sdk::{testutils::Address as _, testutils::Events as _, vec, Address, Env};

use super::*;

#[test]
fn test() {
    let env = Env::default();
    let a = Address::generate(&env);
    let id = env.register(Token, (vec![&env, (a.clone(), 10i128)],));
    let token = TokenClient::new(&env, &id);
    assert_eq!(token.balance(&a), 10);
}

#[test]
fn test_transfer() {
    let env = Env::default();
    let a = Address::generate(&env);
    let b = Address::generate(&env);
    let id = env.register(
        Token,
        (vec![&env, (a.clone(), 10i128), (b.clone(), 11i128)],),
    );
    let token = TokenClient::new(&env, &id);
    assert_eq!(token.balance(&a), 10);
    assert_eq!(token.balance(&b), 11);
    token.mock_all_auths().transfer(&a, &b, &2);
    assert_eq!(
        env.events().all(),
        vec![
            &env,
            (
                id,
                (symbol_short!("transfer"), &a, &b).into_val(&env),
                2i128.into_val(&env)
            )
        ]
    );
    assert_eq!(token.balance(&a), 8);
    assert_eq!(token.balance(&b), 13);
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
    let pause_id = env.register(Pause, ());
    // Use pause mock in Token setup ...
}
