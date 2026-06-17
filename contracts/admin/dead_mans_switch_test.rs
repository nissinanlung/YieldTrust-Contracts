#[cfg(test)]
mod tests {
    use super::*;
    use soroban_sdk::{
        testutils::{Address as _, Ledger, LedgerInfo},
        Env,
    };

    fn setup() -> (Env, DeadMansSwitchContractClient<'static>, Address, Address) {
        let env = Env::default();
        env.mock_all_auths();
        let contract_id = env.register_contract(None, DeadMansSwitchContract);
        let client = DeadMansSwitchContractClient::new(&env, &contract_id);
        let admin = Address::generate(&env);
        let vault = Address::generate(&env);
        client.initialize(&admin, &vault).unwrap();
        (env, client, admin, vault)
    }

    // ── Initialization ───────────────────────────────────────────────────────

    #[test]
    fn test_initialize_succeeds() {
        let env = Env::default();
        env.mock_all_auths();
        let contract_id = env.register_contract(None, DeadMansSwitchContract);
        let client = DeadMansSwitchContractClient::new(&env, &contract_id);
        let admin = Address::generate(&env);
        let vault = Address::generate(&env);
        assert!(client.initialize(&admin, &vault).is_ok());
    }

    #[test]
    fn test_double_initialize_fails() {
        let env = Env::default();
        env.mock_all_auths();
        let contract_id = env.register_contract(None, DeadMansSwitchContract);
        let client = DeadMansSwitchContractClient::new(&env, &contract_id);
        let admin = Address::generate(&env);
        let vault = Address::generate(&env);
        client.initialize(&admin, &vault).unwrap();
        let result = client.try_initialize(&admin, &vault);
        assert_eq!(result, Err(Ok(SwitchError::NotInitialized)));
    }

    // ── Heartbeat ────────────────────────────────────────────────────────────

    #[test]
    fn test_heartbeat_resets_countdown() {
        let (env, client, admin, _vault) = setup();
        env.ledger().set(LedgerInfo {
            timestamp: env.ledger().timestamp() + (100 * 24 * 60 * 60),
            ..env.ledger().get()
        });
        client.heartbeat(&admin).unwrap();
        assert!(client.time_until_recovery() > INACTIVITY_PERIOD - 10);
    }

    #[test]
    fn test_heartbeat_unauthorized_user_fails() {
        let (env, client, _admin, _vault) = setup();
        let stranger = Address::generate(&env);
        let result = client.try_heartbeat(&stranger);
        assert_eq!(result, Err(Ok(SwitchError::UnauthorizedAdmin)));
    }

    // ── Claim Admin ──────────────────────────────────────────────────────────

    #[test]
    fn test_claim_before_180_days_fails_with_correct_error() {
        let (env, client, _admin, vault) = setup();
        env.ledger().set(LedgerInfo {
            timestamp: env.ledger().timestamp() + (90 * 24 * 60 * 60),
            ..env.ledger().get()
        });
        let result = client.try_claim_admin(&vault);
        assert_eq!(result, Err(Ok(SwitchError::InactivityPeriodNotElapsed)));
    }

    #[test]
    fn test_claim_after_180_days_succeeds() {
        let (env, client, _admin, vault) = setup();
        env.ledger().set(LedgerInfo {
            timestamp: env.ledger().timestamp() + (181 * 24 * 60 * 60),
            ..env.ledger().get()
        });
        client.claim_admin(&vault).unwrap();
        assert_eq!(client.get_admin().unwrap(), vault);
    }

    #[test]
    fn test_claim_unauthorized_user_fails() {
        let (env, client, _admin, _vault) = setup();
        let stranger = Address::generate(&env);
        env.ledger().set(LedgerInfo {
            timestamp: env.ledger().timestamp() + (181 * 24 * 60 * 60),
            ..env.ledger().get()
        });
        let result = client.try_claim_admin(&stranger);
        assert_eq!(result, Err(Ok(SwitchError::UnauthorizedVault)));
    }

    #[test]
    fn test_double_claim_fails_with_correct_error() {
        let (env, client, _admin, vault) = setup();
        env.ledger().set(LedgerInfo {
            timestamp: env.ledger().timestamp() + (181 * 24 * 60 * 60),
            ..env.ledger().get()
        });
        client.claim_admin(&vault).unwrap();
        let result = client.try_claim_admin(&vault);
        assert_eq!(result, Err(Ok(SwitchError::RecoveryAlreadyExecuted)));
    }

    // ── Update Recovery Vault ────────────────────────────────────────────────

    #[test]
    fn test_update_recovery_vault_by_admin_succeeds() {
        let (env, client, admin, _vault) = setup();
        let new_vault = Address::generate(&env);
        client.update_recovery_vault(&admin, &new_vault).unwrap();
        assert_eq!(client.get_recovery_vault().unwrap(), new_vault);
    }

    #[test]
    fn test_update_recovery_vault_unauthorized_fails() {
        let (env, client, _admin, _vault) = setup();
        let stranger = Address::generate(&env);
        let new_vault = Address::generate(&env);
        let result = client.try_update_recovery_vault(&stranger, &new_vault);
        assert_eq!(result, Err(Ok(SwitchError::UnauthorizedAdmin)));
    }

    // ── Views ────────────────────────────────────────────────────────────────

    #[test]
    fn test_get_admin_succeeds() {
        let (_, client, admin, _vault) = setup();
        assert_eq!(client.get_admin().unwrap(), admin);
    }

    #[test]
    fn test_get_recovery_vault_succeeds() {
        let (_, client, _admin, vault) = setup();
        assert_eq!(client.get_recovery_vault().unwrap(), vault);
    }
}