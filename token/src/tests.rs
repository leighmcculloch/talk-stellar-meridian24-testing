#![cfg(test)]

use soroban_sdk::{testutils::Address as _, vec, Address, Env};
use testutils::{AuthorizedFunction, AuthorizedInvocation, Events};

use super::*;

#[test]
fn test() {
    let env = Env::default();

    let admin = Address::generate(&env);
    let id = env.register(Token, (&admin,));
    let token = TokenClient::new(&env, &id);

    let a = Address::generate(&env);
    assert_eq!(token.balance(&a), 0);
}

#[test]
fn test_transfer() {
    let env = Env::default();
    let admin = Address::generate(&env);
    let id = env.register(Token, (&admin,));
    let token = TokenClient::new(&env, &id);

    let a = Address::generate(&env);
    token.mock_all_auths().mint(&a, &10);
    let b = Address::generate(&env);
    assert_eq!(token.balance(&a), 10);
    assert_eq!(token.balance(&b), 0);

    token.mock_all_auths().transfer(&a, &b, &2);

    assert_eq!(
        env.auths(),
        [(
            a.clone(),
            AuthorizedInvocation {
                function: AuthorizedFunction::Contract((
                    token.address.clone(),
                    symbol_short!("transfer"),
                    (&a, &b, 2i128).into_val(&env),
                )),
                sub_invocations: [].into(),
            }
        ),]
    );

    assert_eq!(token.balance(&a), 8);
    assert_eq!(token.balance(&b), 2);

    assert_eq!(
        env.events().all(),
        vec![
            &env,
            (
                token.address.clone(),
                (symbol_short!("transfer"), &a, &b).into_val(&env),
                2i128.into_val(&env),
            )
        ],
    );
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
fn test_mock_false() {
    let env = Env::default();
    let admin = Address::generate(&env);
    let pause = env.register(Pause, ());
    let id = env.register(Token, (&admin, &pause));
    let token = TokenClient::new(&env, &id);

    let a = Address::generate(&env);
    token.mock_all_auths().mint(&a, &10);
    let b = Address::generate(&env);

    assert_eq!(token.mock_all_auths().try_transfer(&a, &b, &2), Ok(Ok(())));

    assert_eq!(token.balance(&a), 8);
    assert_eq!(token.balance(&b), 2);
}

#[test]
fn test_mock_true() {
    let env = Env::default();
    let admin = Address::generate(&env);
    let pause = env.register(Pause, ());
    let id = env.register(Token, (&admin, &pause));
    let token = TokenClient::new(&env, &id);

    let a = Address::generate(&env);
    token.mock_all_auths().mint(&a, &10);
    let b = Address::generate(&env);

    assert_eq!(
        token.mock_all_auths().try_transfer(&a, &b, &2),
        Err(Ok(Error::Paused))
    );

    assert_eq!(token.balance(&a), 8);
    assert_eq!(token.balance(&b), 2);
}

// mod pause {
//     soroban_sdk::contractimport!(file = "pause.wasm");
// }

// #[test]
// fn test_import() {
//     let env = Env::default();
//     let admin = Address::generate(&env);
//     let pause = env.register(pause::WASM, ());
//     let id = env.register(Token, (&admin, &pause));
//     let token = TokenClient::new(&env, &id);

//     // ...
// }

// #[contract]
// struct Token;
// #[contractimpl]
// impl Token {
//     // ...
// }

mod token {
    soroban_sdk::contractimport!(file = "target/wasm32-unknown-unknown/release/token.wasm");
}

#[test]
fn test_differential() {
    assert_eq!({
        let env = Env::default();
        let id = env.register(Token, (&Address::generate(&env),));
        let token = TokenClient::new(&env, &id);
        let a = Address::generate(&env);
        let b = Address::generate(&env);
        token.mock_all_auths().mint(&a, &10);
        token.mock_all_auths().transfer(&a, &b, &2);
        (token.balance(&a), token.balance(&b))
    }, {
        let env = Env::default();
        let id = env.register(token::WASM, (&Address::generate(&env),));
        let token = TokenClient::new(&env, &id);
        let a = Address::generate(&env);
        let b = Address::generate(&env);
        token.mock_all_auths().mint(&a, &10);
        token.mock_all_auths().transfer(&a, &b, &2);
        (token.balance(&a), token.balance(&b))
    });
}
