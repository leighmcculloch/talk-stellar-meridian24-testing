#![no_std]
use soroban_sdk::{contract, contracterror, contractimpl, Address, Env};

#[contracterror]
#[derive(Clone, Copy)]
pub enum Error {
    NotInit = 1,
    AlreadyInit = 2,
    Insufficient = 3,
    Overflow = 4,
}

pub const ADMIN: soroban_sdk::Symbol = soroban_sdk::symbol_short!("ADMIN");

#[contract]
pub struct Token;

#[contractimpl]
impl Token {
    pub fn init(env: &Env, admin: Address) -> Result<(), Error> {
        if env.storage().instance().has(&ADMIN) {
            Err(Error::AlreadyInit)
        } else {
            env.storage().instance().set(&ADMIN, &admin);
            Ok(())
        }
    }

    pub fn mint(env: &Env, to: Address, amount: i128) -> Result<(), Error> {
        env.storage()
            .instance()
            .get::<_, Address>(&ADMIN)
            .ok_or(Error::NotInit)?
            .require_auth();
        env.storage()
            .persistent()
            .try_update(&to, |bal: Option<i128>| {
                bal.unwrap_or(0).checked_add(amount).ok_or(Error::Overflow)
            })?;
        Ok(())
    }

    pub fn balance(env: &Env, id: Address) -> i128 {
        env.storage().persistent().get(&id).unwrap_or(0)
    }

    pub fn transfer(env: &Env, from: Address, to: Address, amount: i128) -> Result<(), Error> {
        from.require_auth();
        env.storage()
            .persistent()
            .try_update(&from, |bal: Option<i128>| {
                bal.unwrap_or(0).checked_sub(amount).ok_or(Error::Overflow)
            })?;
        env.storage()
            .persistent()
            .try_update(&to, |bal: Option<i128>| {
                bal.unwrap_or(0).checked_add(amount).ok_or(Error::Overflow)
            })?;
        Ok(())
    }
}

mod tests_integration;
mod tests_unit;
