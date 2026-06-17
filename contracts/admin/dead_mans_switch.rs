#![no_std]
use soroban_sdk::{
    contract, contractimpl, contracterror, contracttype, Address, Env,
};

const INACTIVITY_PERIOD: u64 = 180 * 24 * 60 * 60; // 180 days in seconds

// ── Storage Keys ─────────────────────────────────────────────────────────────

#[contracttype]
pub enum SwitchKey {
    PrimaryAdmin,       // Address
    RecoveryVault,      // Address
    LastActivityAt,     // u64 timestamp — reset on every admin action
    RecoveryExecuted,   // bool
}

// ── Errors ───────────────────────────────────────────────────────────────────

/// All error conditions the Dead Man's Switch contract can return.
#[contracterror]
#[derive(Clone, Copy, Eq, PartialEq, Debug)]
#[repr(u32)]
pub enum SwitchError {
    /// Contract has not been initialized yet, or storage is corrupt.
    NotInitialized = 1,
    /// Caller is not the primary admin.
    UnauthorizedAdmin = 2,
    /// Caller is not the recovery vault.
    UnauthorizedVault = 3,
    /// Recovery has already been executed once.
    RecoveryAlreadyExecuted = 4,
    /// The inactivity period has not elapsed yet.
    InactivityPeriodNotElapsed = 5,
}

// ── Contract ──────────────────────────────────────────────────────────────────

#[contract]
pub struct DeadMansSwitchContract;

#[contractimpl]
impl DeadMansSwitchContract {

    /// Initialize with a primary admin and a recovery vault address.
    /// Returns an error if already initialized.
    pub fn initialize(env: Env, primary_admin: Address, recovery_vault: Address) -> Result<(), SwitchError> {
        if env.storage().instance().has(&SwitchKey::PrimaryAdmin) {
            return Err(SwitchError::NotInitialized);
        }
        primary_admin.require_auth();
        env.storage().instance().set(&SwitchKey::PrimaryAdmin, &primary_admin);
        env.storage().instance().set(&SwitchKey::RecoveryVault, &recovery_vault);
        env.storage().instance().set(&SwitchKey::LastActivityAt, &env.ledger().timestamp());
        env.storage().instance().set(&SwitchKey::RecoveryExecuted, &false);
        Ok(())
    }

    /// Heartbeat — primary admin calls this to prove liveness and reset the countdown.
    pub fn heartbeat(env: Env, admin: Address) -> Result<(), SwitchError> {
        admin.require_auth();
        Self::assert_primary_admin(&env, &admin)?;
        let now = env.ledger().timestamp();
        env.storage().instance().set(&SwitchKey::LastActivityAt, &now);
        env.events().publish(
            (soroban_sdk::symbol_short!("heartbeat"),),
            (admin, now),
        );
        Ok(())
    }

    /// Any admin action should call this internally to reset the countdown.
    /// Call at the start of every privileged function.
    pub fn record_activity(env: &Env) {
        env.storage()
            .instance()
            .set(&SwitchKey::LastActivityAt, &env.ledger().timestamp());
    }

    /// Recovery vault claims admin rights after 180 days of inactivity.
    pub fn claim_admin(env: Env, recovery_vault: Address) -> Result<(), SwitchError> {
        recovery_vault.require_auth();
        Self::assert_recovery_vault(&env, &recovery_vault)?;

        let already_executed: bool = env
            .storage()
            .instance()
            .get(&SwitchKey::RecoveryExecuted)
            .unwrap_or(false);
        if already_executed {
            return Err(SwitchError::RecoveryAlreadyExecuted);
        }

        let last_activity: u64 = env
            .storage()
            .instance()
            .get(&SwitchKey::LastActivityAt)
            .unwrap_or(0);
        let now = env.ledger().timestamp();
        let elapsed = now.saturating_sub(last_activity);

        if elapsed < INACTIVITY_PERIOD {
            return Err(SwitchError::InactivityPeriodNotElapsed);
        }

        env.storage().instance().set(&SwitchKey::PrimaryAdmin, &recovery_vault);
        env.storage().instance().set(&SwitchKey::RecoveryExecuted, &true);

        env.events().publish(
            (soroban_sdk::symbol_short!("recovered"),),
            (recovery_vault, now),
        );
        Ok(())
    }

    /// Update the recovery vault address (primary admin only).
    pub fn update_recovery_vault(env: Env, admin: Address, new_vault: Address) -> Result<(), SwitchError> {
        admin.require_auth();
        Self::assert_primary_admin(&env, &admin)?;
        Self::record_activity(&env);
        env.storage().instance().set(&SwitchKey::RecoveryVault, &new_vault);
        Ok(())
    }

    /// View how many seconds remain before the recovery vault can claim admin.
    pub fn time_until_recovery(env: Env) -> u64 {
        let last_activity: u64 = env
            .storage()
            .instance()
            .get(&SwitchKey::LastActivityAt)
            .unwrap_or(0);
        let elapsed = env.ledger().timestamp().saturating_sub(last_activity);
        INACTIVITY_PERIOD.saturating_sub(elapsed)
    }

    /// View current primary admin.
    pub fn get_admin(env: Env) -> Result<Address, SwitchError> {
        env.storage()
            .instance()
            .get(&SwitchKey::PrimaryAdmin)
            .ok_or(SwitchError::NotInitialized)
    }

    /// View recovery vault address.
    pub fn get_recovery_vault(env: Env) -> Result<Address, SwitchError> {
        env.storage()
            .instance()
            .get(&SwitchKey::RecoveryVault)
            .ok_or(SwitchError::NotInitialized)
    }

    // ── Internal ─────────────────────────────────────────────────────────────

    fn assert_primary_admin(env: &Env, caller: &Address) -> Result<(), SwitchError> {
        let admin: Address = env
            .storage()
            .instance()
            .get(&SwitchKey::PrimaryAdmin)
            .ok_or(SwitchError::NotInitialized)?;
        if *caller != admin {
            return Err(SwitchError::UnauthorizedAdmin);
        }
        Ok(())
    }

    fn assert_recovery_vault(env: &Env, caller: &Address) -> Result<(), SwitchError> {
        let vault: Address = env
            .storage()
            .instance()
            .get(&SwitchKey::RecoveryVault)
            .ok_or(SwitchError::NotInitialized)?;
        if *caller != vault {
            return Err(SwitchError::UnauthorizedVault);
        }
        Ok(())
    }
}