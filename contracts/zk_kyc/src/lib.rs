#![no_std]
use soroban_sdk::{contract, contractimpl, contracterror, contracttype, Address, Env};

#[contracterror]
#[derive(Clone, Copy, Eq, PartialEq, Debug)]
#[repr(u32)]
pub enum KycError {
    NotInitialized = 1,
    AlreadyInitialized = 2,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum DataKey {
    Verifier,
    KycStatus(Address),
}

#[contract]
pub struct ZKKYCContract;

#[contractimpl]
impl ZKKYCContract {
    pub fn init(env: Env, verifier: Address) -> Result<(), KycError> {
        if env.storage().instance().has(&DataKey::Verifier) {
            return Err(KycError::AlreadyInitialized);
        }
        env.storage().instance().set(&DataKey::Verifier, &verifier);
        Ok(())
    }

    pub fn verify_user(env: Env, user: Address) -> Result<(), KycError> {
        let verifier: Address = env.storage().instance().get(&DataKey::Verifier).ok_or(KycError::NotInitialized)?;
        verifier.require_auth();
        env.storage().persistent().set(&DataKey::KycStatus(user), &true);
        Ok(())
    }

    pub fn revoke_user(env: Env, user: Address) -> Result<(), KycError> {
        let verifier: Address = env.storage().instance().get(&DataKey::Verifier).ok_or(KycError::NotInitialized)?;
        verifier.require_auth();
        env.storage().persistent().remove(&DataKey::KycStatus(user));
        Ok(())
    }

    pub fn is_verified(env: Env, user: Address) -> bool {
        env.storage().persistent().get(&DataKey::KycStatus(user)).unwrap_or(false)
    }
}