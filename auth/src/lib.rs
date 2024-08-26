#![no_std]
use soroban_sdk::{
    auth::{Context, CustomAccountInterface},
    contract, contracterror, contractimpl, contractmeta,
    crypto::Hash,
    symbol_short,
    token::TokenClient,
    unwrap::UnwrapOptimized,
    Address, Env, IntoVal as _, Map, Symbol, Vec,
};

contractmeta!(key = "github", val = "stellar/soroban-examples");

mod test_fail;
mod test_pass;

#[contracterror]
#[derive(Clone, Copy)]
pub enum Error {
    Insufficient = 1,
}

#[contract]
pub struct Auth;

pub const CFG: Symbol = symbol_short!("CFG");

#[contractimpl]
impl Auth {
    pub fn __constructor(env: &Env, token: Address, threshold: i128) {
        env.storage().instance().set(&CFG, &(token, threshold));
    }
}

#[contractimpl]
impl CustomAccountInterface for Auth {
    type Signature = Map<Address, ()>;
    type Error = Error;
    fn __check_auth(
        env: Env,
        signature_payload: Hash<32>,
        signatures: Map<Address, ()>,
        _auth_contexts: Vec<Context>,
    ) -> Result<(), Error> {
        // Check all signers signed.
        for (addr, ()) in signatures.iter() {
            addr.require_auth_for_args((signature_payload.to_val(),).into_val(&env));
        }

        // Check that all signer weights (as defined by their holding balance of
        // the token) sum to the minimum required to auth.
        let (token, threshold) = env
            .storage()
            .instance()
            .get::<_, (Address, i128)>(&CFG)
            .unwrap_optimized();
        let token = TokenClient::new(&env, &token);
        let weight = signatures.iter().fold(0i128, |weight, (addr, ())| {
            let bal = token.balance(&addr);
            weight.saturating_add(bal)
        });
        if weight >= threshold {
            Ok(())
        } else {
            Err(Error::Insufficient)
        }
    }
}
