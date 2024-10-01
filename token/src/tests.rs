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
fn test_mock() {
    let env = Env::default();
    let admin = Address::generate(&env);
    let pause = env.register(Pause, ());
    let id = env.register(Token, (&admin, &pause));
    let _token = TokenClient::new(&env, &id);

    // ...
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
