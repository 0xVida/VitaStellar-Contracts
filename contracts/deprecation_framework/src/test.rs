#[cfg(test)]
mod tests {
    use crate::{DeprecationFramework, DeprecationFrameworkClient, DeprecationPhase};
    use soroban_sdk::{testutils::Address as _, testutils::Ledger as _, Address, Env, String, Vec};

    #[test]
    fn test_initialize() {
        let env = Env::default();
        env.mock_all_auths();

        let admin = Address::generate(&env);
        let contract_id = env.register_contract(None, DeprecationFramework);
        let client = DeprecationFrameworkClient::new(&env, &contract_id);

        client.initialize(&admin);
    }

    #[test]
    fn test_mark_for_deprecation() {
        let env = Env::default();
        env.mock_all_auths();

        let admin = Address::generate(&env);
        let contract_id = env.register_contract(None, DeprecationFramework);
        let client = DeprecationFrameworkClient::new(&env, &contract_id);

        client.initialize(&admin);

        let contract_to_deprecate = String::from_str(&env, "old_contract");
        let contract_name = String::from_str(&env, "Old Contract");
        let reason = String::from_str(&env, "Replaced by new version");

        client.mark_for_deprecation(
            &admin,
            &contract_to_deprecate,
            &contract_name,
            &reason,
            &None,
        );
    }

    #[test]
    #[should_panic]
    fn test_mark_duplicate_deprecation() {
        let env = Env::default();
        env.mock_all_auths();

        let admin = Address::generate(&env);
        let contract_id = env.register_contract(None, DeprecationFramework);
        let client = DeprecationFrameworkClient::new(&env, &contract_id);

        client.initialize(&admin);

        let contract_to_deprecate = String::from_str(&env, "old_contract");
        let contract_name = String::from_str(&env, "Old Contract");
        let reason = String::from_str(&env, "Replaced by new version");

        client.mark_for_deprecation(
            &admin,
            &contract_to_deprecate,
            &contract_name,
            &reason,
            &None,
        );

        client.mark_for_deprecation(
            &admin,
            &contract_to_deprecate,
            &contract_name,
            &reason,
            &None,
        );
    }

    #[test]
    fn test_set_sunset_timeline() {
        let env = Env::default();
        env.mock_all_auths();

        let admin = Address::generate(&env);
        let contract_id = env.register_contract(None, DeprecationFramework);
        let client = DeprecationFrameworkClient::new(&env, &contract_id);

        client.initialize(&admin);

        let contract_to_deprecate = String::from_str(&env, "old_contract");
        let contract_name = String::from_str(&env, "Old Contract");
        let reason = String::from_str(&env, "Replaced by new version");

        client.mark_for_deprecation(
            &admin,
            &contract_to_deprecate,
            &contract_name,
            &reason,
            &None,
        );

        client.set_sunset_timeline(&admin, &contract_to_deprecate, &1000, &2000, &3000);
    }

    #[test]
    #[should_panic]
    fn test_set_invalid_sunset_timeline() {
        let env = Env::default();
        env.mock_all_auths();

        let admin = Address::generate(&env);
        let contract_id = env.register_contract(None, DeprecationFramework);
        let client = DeprecationFrameworkClient::new(&env, &contract_id);

        client.initialize(&admin);

        let contract_to_deprecate = String::from_str(&env, "old_contract");
        let contract_name = String::from_str(&env, "Old Contract");
        let reason = String::from_str(&env, "Replaced by new version");

        client.mark_for_deprecation(
            &admin,
            &contract_to_deprecate,
            &contract_name,
            &reason,
            &None,
        );

        // Invalid: dates not in chronological order
        client.set_sunset_timeline(&admin, &contract_to_deprecate, &3000, &2000, &1000);
    }

    #[test]
    fn test_add_migration_guide() {
        let env = Env::default();
        env.mock_all_auths();

        let admin = Address::generate(&env);
        let contract_id = env.register_contract(None, DeprecationFramework);
        let client = DeprecationFrameworkClient::new(&env, &contract_id);

        client.initialize(&admin);

        let contract_to_deprecate = String::from_str(&env, "old_contract");
        let contract_name = String::from_str(&env, "Old Contract");
        let reason = String::from_str(&env, "Replaced by new version");

        client.mark_for_deprecation(
            &admin,
            &contract_to_deprecate,
            &contract_name,
            &reason,
            &None,
        );

        let guide_title = String::from_str(&env, "Migration Guide");
        let guide_content = String::from_str(&env, "Follow these steps...");
        let examples = Vec::new(&env);

        client.add_migration_guide(
            &admin,
            &contract_to_deprecate,
            &guide_title,
            &guide_content,
            &examples,
        );
    }

    #[test]
    fn test_update_deprecation_phase() {
        let env = Env::default();
        env.mock_all_auths();

        let admin = Address::generate(&env);
        let contract_id = env.register_contract(None, DeprecationFramework);
        let client = DeprecationFrameworkClient::new(&env, &contract_id);

        client.initialize(&admin);

        let contract_to_deprecate = String::from_str(&env, "old_contract");
        let contract_name = String::from_str(&env, "Old Contract");
        let reason = String::from_str(&env, "Replaced by new version");

        client.mark_for_deprecation(
            &admin,
            &contract_to_deprecate,
            &contract_name,
            &reason,
            &None,
        );

        client.update_deprecation_phase(
            &admin,
            &contract_to_deprecate,
            &DeprecationPhase::Supported,
        );
    }

    #[test]
    fn test_publish_user_communication() {
        let env = Env::default();
        env.mock_all_auths();

        let admin = Address::generate(&env);
        let contract_id = env.register_contract(None, DeprecationFramework);
        let client = DeprecationFrameworkClient::new(&env, &contract_id);

        client.initialize(&admin);

        let contract_to_deprecate = String::from_str(&env, "old_contract");
        let contract_name = String::from_str(&env, "Old Contract");
        let reason = String::from_str(&env, "Replaced by new version");

        client.mark_for_deprecation(
            &admin,
            &contract_to_deprecate,
            &contract_name,
            &reason,
            &None,
        );

        let message = String::from_str(&env, "This contract is deprecated");
        let comm_type = String::from_str(&env, "announcement");

        let result =
            client.publish_user_communication(&admin, &contract_to_deprecate, &message, &comm_type);
        assert_eq!(result, 0);
    }

    #[test]
    fn test_create_removal_checklist() {
        let env = Env::default();
        env.mock_all_auths();

        let admin = Address::generate(&env);
        let contract_id = env.register_contract(None, DeprecationFramework);
        let client = DeprecationFrameworkClient::new(&env, &contract_id);

        client.initialize(&admin);

        let contract_to_deprecate = String::from_str(&env, "old_contract");
        let contract_name = String::from_str(&env, "Old Contract");
        let reason = String::from_str(&env, "Replaced by new version");

        client.mark_for_deprecation(
            &admin,
            &contract_to_deprecate,
            &contract_name,
            &reason,
            &None,
        );

        let mut items = Vec::new(&env);
        items.push_back(String::from_str(&env, "Migrate data"));
        items.push_back(String::from_str(&env, "Update documentation"));

        client.create_removal_checklist(&admin, &contract_to_deprecate, &items);
    }

    #[test]
    fn test_is_deprecated() {
        let env = Env::default();
        env.mock_all_auths();

        let admin = Address::generate(&env);
        let contract_id = env.register_contract(None, DeprecationFramework);
        let client = DeprecationFrameworkClient::new(&env, &contract_id);

        client.initialize(&admin);

        let contract_to_deprecate = String::from_str(&env, "old_contract");
        let contract_name = String::from_str(&env, "Old Contract");
        let reason = String::from_str(&env, "Replaced by new version");

        assert!(!client.is_deprecated(&contract_to_deprecate));

        client.mark_for_deprecation(
            &admin,
            &contract_to_deprecate,
            &contract_name,
            &reason,
            &None,
        );

        assert!(client.is_deprecated(&contract_to_deprecate));
    }

    #[test]
    fn test_pin_version_success() {
        let env = Env::default();
        env.mock_all_auths();

        let admin = Address::generate(&env);
        let contract_id = env.register_contract(None, DeprecationFramework);
        let client = DeprecationFrameworkClient::new(&env, &contract_id);

        client.initialize(&admin);

        // Default min pinnable version is 1
        assert_eq!(client.get_min_pinnable_version(), 1);

        // Pin version 1 (same as floor)
        client.pin_version(&admin, &1);
        let current = client.get_current_pinned_version().unwrap();
        assert_eq!(current.0, 1);

        // Pin version 2 (higher than floor)
        client.pin_version(&admin, &2);
        let current = client.get_current_pinned_version().unwrap();
        assert_eq!(current.0, 2);
    }

    #[test]
    fn test_pin_version_downgrade_rejected() {
        let env = Env::default();
        env.mock_all_auths();

        let admin = Address::generate(&env);
        let contract_id = env.register_contract(None, DeprecationFramework);
        let client = DeprecationFrameworkClient::new(&env, &contract_id);

        client.initialize(&admin);

        // Attempting to pin 0 (less than default floor 1) should be rejected
        let result = client.try_pin_version(&admin, &0);
        assert_eq!(result, Err(Ok(crate::errors::Error::DowngradeNotAllowed)));
    }

    #[test]
    fn test_raise_min_version_flow() {
        let env = Env::default();
        env.mock_all_auths();

        let admin = Address::generate(&env);
        let contract_id = env.register_contract(None, DeprecationFramework);
        let client = DeprecationFrameworkClient::new(&env, &contract_id);

        client.initialize(&admin);

        // Propose raising min pinnable version to 3
        client.propose_raise_min_version(&admin, &3);

        // Verify proposed details
        let proposed = client.get_proposed_min_version().unwrap();
        assert_eq!(proposed.0, 3);
        assert_eq!(proposed.1, 86400); // 24 hours timelock

        // Trying to execute raise immediately should fail
        let result = client.try_raise_min_version(&admin);
        assert_eq!(result, Err(Ok(crate::errors::Error::TimeLockNotExpired)));

        // Advance ledger time by 24 hours (86400 seconds)
        env.ledger().set_timestamp(86400);

        // Now execute should succeed
        client.raise_min_version(&admin);

        // Verify new floor
        assert_eq!(client.get_min_pinnable_version(), 3);
        assert!(client.get_proposed_min_version().is_none());

        // Pinning 2 (less than new floor 3) should now fail
        let result = client.try_pin_version(&admin, &2);
        assert_eq!(result, Err(Ok(crate::errors::Error::DowngradeNotAllowed)));

        // Pinning 3 should succeed
        client.pin_version(&admin, &3);
        let current = client.get_current_pinned_version().unwrap();
        assert_eq!(current.0, 3);
    }
}
