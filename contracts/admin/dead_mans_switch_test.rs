#[cfg(test)]
mod tests {
    use super::*;
    use soroban_sdk::{
        testutils::{Address as _, Ledger, LedgerInfo, Events},
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

    #[test]
    fn test_heartbeat_emits_event() {
        let (env, client, admin, _vault) = setup();
        let now = env.ledger().timestamp();
        client.heartbeat(&admin).unwrap();
        let events = env.events().all();
        let last = events.last().unwrap();
        assert_eq!(last.0, (Symbol::new(&env, "heartbeat"),));
        let (emitted_admin, emitted_ts): (Address, u64) = last.2.clone().try_into().unwrap();
        assert_eq!(emitted_admin, admin);
        assert_eq!(emitted_ts, now);
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
    fn test_claim_at_exactly_180_days_succeeds() {
        let (env, client, _admin, vault) = setup();
        env.ledger().set(LedgerInfo {
            timestamp: env.ledger().timestamp() + (180 * 24 * 60 * 60),
            ..env.ledger().get()
        });
        client.claim_admin(&vault).unwrap();
        assert_eq!(client.get_admin().unwrap(), vault);
    }

    #[test]
    fn test_claim_emits_recovery_event() {
        let (env, client, _admin, vault) = setup();
        env.ledger().set(LedgerInfo {
            timestamp: env.ledger().timestamp() + (181 * 24 * 60 * 60),
            ..env.ledger().get()
        });
        client.claim_admin(&vault).unwrap();
        let events = env.events().all();
        let recovery_event = events.iter().find(|e| e.0 == (Symbol::new(&env, "recovered"),));
        assert!(recovery_event.is_some());
        let (emitted_vault, emitted_ts): (Address, u64) = recovery_event.unwrap().2.clone().try_into().unwrap();
        assert_eq!(emitted_vault, vault);
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

    // ── Recovery Executed Flag ───────────────────────────────────────────────

    #[test]
    fn test_recovery_executed_flag_persists() {
        let (env, client, _admin, vault) = setup();
        env.ledger().set(LedgerInfo {
            timestamp: env.ledger().timestamp() + (181 * 24 * 60 * 60),
            ..env.ledger().get()
        });
        client.claim_admin(&vault).unwrap();
        // After recovery, another claim from any vault must still fail
        let second_vault = Address::generate(&env);
        let result = client.try_claim_admin(&second_vault);
        assert_eq!(result, Err(Ok(SwitchError::RecoveryAlreadyExecuted)));
    }

    #[test]
    fn test_recovery_vault_becomes_new_admin() {
        let (env, client, _admin, vault) = setup();
        env.ledger().set(LedgerInfo {
            timestamp: env.ledger().timestamp() + (181 * 24 * 60 * 60),
            ..env.ledger().get()
        });
        client.claim_admin(&vault).unwrap();
        // The recovery vault is now the primary admin
        assert_eq!(client.get_admin().unwrap(), vault);
        // The recovery vault can now heartbeat as the new admin
        client.heartbeat(&vault).unwrap();
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

    #[test]
    fn test_time_until_recovery_at_initialization() {
        let (env, client, _admin, _vault) = setup();
        assert_eq!(client.time_until_recovery(), INACTIVITY_PERIOD);
    }

    #[test]
    fn test_time_until_recovery_after_heartbeat() {
        let (env, client, admin, _vault) = setup();
        env.ledger().set(LedgerInfo {
            timestamp: env.ledger().timestamp() + (50 * 24 * 60 * 60),
            ..env.ledger().get()
        });
        client.heartbeat(&admin).unwrap();
        assert!(client.time_until_recovery() > INACTIVITY_PERIOD - 60);
    }

    #[test]
    fn test_get_admin_fails_for_uninitialized() {
        let env = Env::default();
        env.mock_all_auths();
        let contract_id = env.register_contract(None, DeadMansSwitchContract);
        let client = DeadMansSwitchContractClient::new(&env, &contract_id);
        let result = client.try_get_admin();
        assert_eq!(result, Err(Ok(SwitchError::NotInitialized)));
    }
}