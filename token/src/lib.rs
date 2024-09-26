#![no_std]
use soroban_sdk::*;

#[contracterror]
pub enum Error {
    Overflow = 1,
    InsufficientBalance = 2,
    NegativeAmount = 3,
}

#[contract]
pub struct Token;

#[contractimpl]
impl Token {
    // Initialize the token with an initial mint.
    pub fn __constructor(env: &Env, mint: Vec<(Address, i128)>) {
        for (id, bal) in mint {
            env.storage().persistent().set(&id, &bal);
        }
    }

    // Get the balance held by the address.
    pub fn balance(env: &Env, addr: Address) -> i128 {
        env.storage().persistent().get(&addr).unwrap_or(0)
    }

    // Transfer `amount` from `from` to `to`.
    // Requires auth from `from`.
    pub fn transfer(env: &Env, from: Address, to: Address, amount: i128) -> Result<(), Error> {
        from.require_auth();
        if amount < 0 {
            return Err(Error::NegativeAmount);
        }

        let from_bal: i128 = env.storage().persistent().get(&from).unwrap_or(0);
        if from_bal < amount {
            return Err(Error::InsufficientBalance);
        }
        let from_bal = from_bal.checked_sub(amount).ok_or(Error::Overflow)?;
        env.storage().persistent().set(&from, &from_bal);

        let to_bal: i128 = env.storage().persistent().get(&to).unwrap_or(0);
        let to_bal = to_bal.checked_add(amount).ok_or(Error::Overflow)?;
        env.storage().persistent().set(&to, &to_bal);

        env.events()
            .publish((symbol_short!("transfer"), &from, &to), amount);

        Ok(())
    }
}

mod tests;
