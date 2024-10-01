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
    pub fn __constructor(env: &Env, admin: Address) {
        env.storage().persistent().set(&"ADMIN", &admin);
    }

    pub fn mint(env: &Env, to: Address, amount: i128) -> Result<(), Error> {
        env.storage()
            .persistent()
            .get::<_, Address>(&"ADMIN")
            .unwrap()
            .require_auth();
        env.storage().persistent().try_update(&to, |bal| {
            let bal = bal.unwrap_or(0i128);
            bal.checked_add(amount).ok_or(Error::Overflow)
        })?;
        Ok(())
    }

    pub fn balance(env: &Env, addr: Address) -> i128 {
        env.storage().persistent().get(&addr).unwrap_or(0)
    }

    pub fn transfer(env: &Env, from: Address, to: Address, amount: i128) -> Result<(), Error> {
        from.require_auth();
        if amount < 0 {
            return Err(Error::NegativeAmount);
        }

        env.storage().persistent().try_update(&from, |bal| {
            let bal = bal.unwrap_or(0i128);
            if bal < amount {
                return Err(Error::InsufficientBalance);
            }
            bal.checked_sub(amount).ok_or(Error::Overflow)
        })?;

        env.storage().persistent().try_update(&to, |bal| {
            let bal = bal.unwrap_or(0i128);
            bal.checked_add(amount).ok_or(Error::Overflow)
        })?;

        env.events()
            .publish((symbol_short!("transfer"), &from, &to), amount);

        Self::invariant_balance_gte_zero(env, &[from, to]);

        Ok(())
    }

    fn invariant_balance_gte_zero(env: &Env, addrs: &[Address]) {
        for addr in addrs {
            assert!(Self::balance(env, addr.clone()) >= 0);
        }
    }
}

mod tests;
