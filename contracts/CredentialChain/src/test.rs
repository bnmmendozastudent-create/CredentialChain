#[cfg(test)]
mod tests {
    use super::*;
    use soroban_sdk::Env;

    #[test]
    fn test_create_credential_happy_path() {
        let env = Env::default();
        let worker = Address::random(&env);

        CredentialChainContract::initialize(env.clone());

        let credential_id = CredentialChainContract::create_credential(
            env.clone(),
            worker.clone(),
            String::from_str(&env, "Welding Level 2"),
            String::from_str(&env, "MIG welding ISO 6947 certified"),
        );

        assert_eq!(credential_id, 1);

        let credential = CredentialChainContract::get_credential(env.clone(), credential_id);
        assert_eq!(credential.worker, worker);
        assert_eq!(credential.status, CredentialStatus::Pending);
        assert_eq!(credential.verified, false);
    }

    #[test]
    fn test_employer_verify_credential() {
        let env = Env::default();
        let worker = Address::random(&env);
        let employer = Address::random(&env);

        CredentialChainContract::initialize(env.clone());

        let credential_id = CredentialChainContract::create_credential(
            env.clone(),
            worker.clone(),
            String::from_str(&env, "English Construction"),
            String::from_str(&env, "English proficiency for construction sites"),
        );

        let result = CredentialChainContract::employer_verify_credential(
            env.clone(),
            employer.clone(),
            credential_id,
            String::from_str(&env, "Acme Construction"),
            500_0000000,
        );

        assert_eq!(result, true);

        let credential = CredentialChainContract::get_credential(env.clone(), credential_id);
        assert_eq!(credential.verified, true);
        assert_eq!(credential.status, CredentialStatus::Verified);
        assert_eq!(credential.issuer, employer);
    }

    #[test]
    #[should_panic]
    fn test_employer_verify_insufficient_stake() {
        let env = Env::default();
        let worker = Address::random(&env);
        let employer = Address::random(&env);

        CredentialChainContract::initialize(env.clone());

        let credential_id = CredentialChainContract::create_credential(
            env.clone(),
            worker.clone(),
            String::from_str(&env, "Safety Certification"),
            String::from_str(&env, "Basic safety training"),
        );

        CredentialChainContract::employer_verify_credential(
            env.clone(),
            employer,
            credential_id,
            String::from_str(&env, "Bad Employer"),
            100_0000000,
        );
    }

    #[test]
    fn test_worker_portfolio_and_rating() {
        let env = Env::default();
        let worker = Address::random(&env);
        let employer = Address::random(&env);

        CredentialChainContract::initialize(env.clone());

        let credential_id = CredentialChainContract::create_credential(
            env.clone(),
            worker.clone(),
            String::from_str(&env, "Equipment Operation"),
            String::from_str(&env, "Operates heavy machinery safely"),
        );

        CredentialChainContract::employer_verify_credential(
            env.clone(),
            employer.clone(),
            credential_id,
            String::from_str(&env, "Builders Inc"),
            500_0000000,
        );

        CredentialChainContract::add_to_portfolio(
            env.clone(),
            worker.clone(),
            credential_id,
        );

        let portfolio = CredentialChainContract::get_worker_portfolio(env.clone(), worker.clone());
        assert_eq!(portfolio.total_verified, 1);

        let new_reputation = CredentialChainContract::worker_rate_employer(
            env.clone(),
            worker.clone(),
            employer.clone(),
            900,
        );

        assert!(new_reputation > 0);

        let employer_record = CredentialChainContract::get_employer(env.clone(), employer);
        assert!(employer_record.reputation_score > 0);
    }

    #[test]
    fn test_full_workflow_end_to_end() {
        let env = Env::default();
        let worker = Address::random(&env);
        let employer1 = Address::random(&env);
        let employer2 = Address::random(&env);

        CredentialChainContract::initialize(env.clone());

        let cred1 = CredentialChainContract::create_credential(
            env.clone(),
            worker.clone(),
            String::from_str(&env, "Welding L2"),
            String::from_str(&env, "ISO 6947 certified"),
        );

        let cred2 = CredentialChainContract::create_credential(
            env.clone(),
            worker.clone(),
            String::from_str(&env, "English Advanced"),
            String::from_str(&env, "Fluent English for work"),
        );

        assert_eq!(cred1, 1);
        assert_eq!(cred2, 2);

        CredentialChainContract::employer_verify_credential(
            env.clone(),
            employer1.clone(),
            cred1,
            String::from_str(&env, "Acme Construction"),
            500_0000000,
        );

        CredentialChainContract::employer_verify_credential(
            env.clone(),
            employer2.clone(),
            cred2,
            String::from_str(&env, "Elite Staffing"),
            500_0000000,
        );

        CredentialChainContract::add_to_portfolio(env.clone(), worker.clone(), cred1);
        CredentialChainContract::add_to_portfolio(env.clone(), worker.clone(), cred2);

        let portfolio = CredentialChainContract::get_worker_portfolio(env.clone(), worker.clone());
        assert_eq!(portfolio.total_verified, 2);

        CredentialChainContract::worker_rate_employer(
            env.clone(),
            worker.clone(),
            employer1.clone(),
            850,
        );

        CredentialChainContract::worker_rate_employer(
            env.clone(),
            worker.clone(),
            employer2.clone(),
            920,
        );

        let updated_portfolio = CredentialChainContract::view_portfolio(env.clone(), worker.clone());
        assert_eq!(updated_portfolio.portfolio_views, 1);
        assert!(updated_portfolio.earnings_from_views > 0);

        let counter = CredentialChainContract::get_credential_counter(env.clone());
        assert_eq!(counter, 2);
    }
}
