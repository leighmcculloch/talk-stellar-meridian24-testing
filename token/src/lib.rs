#![no_std]
use soroban_sdk::{contract, contracterror, contractimpl, Address, Env};

#[contracterror]
pub enum Error {
    Overflow = 1,
}

#[contract]
pub struct Token;

#[contractimpl]
impl Token {
    // Initialize the token with an initial mint.
    pub fn __constructor(env: &Env, id: Address, bal: i128) {
        env.storage().persistent().set(&id, &bal);
    }

    // Get the balance held by the address.
    pub fn balance(env: &Env, addr: Address) -> i128 {
        env.storage().persistent().get(&addr).unwrap_or(0)
    }

    // Transfer `amount` from `from` to `to`.
    // Requires auth from `from`.
    pub fn transfer(env: &Env, from: Address, to: Address, amount: i128) -> Result<(), Error> {
        from.require_auth();
        env.storage().persistent().try_update(&from, |bal: Option<i128>| {
            // 👇 WARNING: BUGS LIVE HERE 👇
            bal.unwrap_or(0).checked_sub(amount).ok_or(Error::Overflow)
        })?;
        env.storage().persistent().try_update(&to, |bal: Option<i128>| {
            bal.unwrap_or(0).checked_add(amount).ok_or(Error::Overflow)
        })?;
        Ok(())
    }
}

mod tests;
