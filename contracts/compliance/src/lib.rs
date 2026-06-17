#![no_std]
use soroban_sdk::{contract, contractimpl, contracterror, contracttype, Address, Env};

#[contracterror]
#[derive(Clone, Copy, Eq, PartialEq, Debug)]
#[repr(u32)]
pub enum ComplianceError {
    NotInitialized = 1,
    AlreadyInitialized = 2,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum DataKey {
    Officer,
    Sanctioned(Address),
    Flagged(Address),
}

#[contract]
pub struct ComplianceContract;

#[contractimpl]
impl ComplianceContract {
    pub fn init(env: Env, officer: Address) -> Result<(), ComplianceError> {
        if env.storage().instance().has(&DataKey::Officer) {
            return Err(ComplianceError::AlreadyInitialized);
        }
        env.storage().instance().set(&DataKey::Officer, &officer);
        Ok(())
    }

    pub fn sanction(env: Env, target: Address) -> Result<(), ComplianceError> {
        let officer: Address = env.storage().instance().get(&DataKey::Officer).ok_or(ComplianceError::NotInitialized)?;
        officer.require_auth();
        env.storage().persistent().set(&DataKey::Sanctioned(target), &true);
        Ok(())
    }

    pub fn unsanction(env: Env, target: Address) -> Result<(), ComplianceError> {
        let officer: Address = env.storage().instance().get(&DataKey::Officer).ok_or(ComplianceError::NotInitialized)?;
        officer.require_auth();
        env.storage().persistent().remove(&DataKey::Sanctioned(target));
        Ok(())
    }

    pub fn is_sanctioned(env: Env, target: Address) -> bool {
        env.storage().persistent().get(&DataKey::Sanctioned(target)).unwrap_or(false)
    }

    pub fn flag_address(env: Env, target: Address) -> Result<(), ComplianceError> {
        let officer: Address = env.storage().instance().get(&DataKey::Officer).ok_or(ComplianceError::NotInitialized)?;
        officer.require_auth();
        env.storage().persistent().set(&DataKey::Flagged(target), &true);
        Ok(())
    }

    pub fn is_flagged(env: Env, target: Address) -> bool {
        env.storage().persistent().get(&DataKey::Flagged(target)).unwrap_or(false)
    }
}