#![no_std]
use soroban_sdk::{
    contract, contractimpl, contracttype, Address, Env, String, Vec,
};

#[derive(Clone, Debug)]
#[contracttype]
pub enum DataKey {
    CredentialCounter,
    Credential(u64),
    Employer(Address),
    WorkerPortfolio(Address),
}

#[derive(Clone, Debug, PartialEq)]
#[contracttype]
pub enum CredentialStatus {
    Pending,
    Verified,
    Active,
    Expired,
    Revoked,
}

#[derive(Clone, Debug)]
#[contracttype]
pub struct Credential {
    pub id: u64,
    pub worker: Address,
    pub credential_type: String,
    pub issued_at: u64,
    pub issuer: Address,
    pub issuer_name: String,
    pub skill_description: String,
    pub status: CredentialStatus,
    pub verified: bool,
    pub staked_amount: i128,
}

#[derive(Clone, Debug)]
#[contracttype]
pub struct Employer {
    pub address: Address,
    pub name: String,
    pub staked_amount: i128,
    pub credentials_issued: u32,
    pub reputation_score: u32,
    pub banned: bool,
}

#[derive(Clone, Debug)]
#[contracttype]
pub struct WorkerPortfolio {
    pub worker: Address,
    pub credentials: Vec<u64>,
    pub total_verified: u32,
    pub average_employer_rating: u32,
    pub portfolio_views: u32,
    pub earnings_from_views: i128,
}

#[contract]
pub struct CredentialChainContract;

#[contractimpl]
impl CredentialChainContract {
    pub fn initialize(env: Env) {
        let storage = env.storage().persistent();
        if storage.has(&DataKey::CredentialCounter) {
            panic!("Already initialized");
        }
        storage.set(&DataKey::CredentialCounter, &0u64);
    }

    pub fn create_credential(
        env: Env,
        worker: Address,
        credential_type: String,
        skill_description: String,
    ) -> u64 {
        worker.require_auth();

        let storage = env.storage().persistent();
        let mut counter: u64 = storage.get(&DataKey::CredentialCounter).unwrap_or(0);
        counter += 1;

        let credential = Credential {
            id: counter,
            worker: worker.clone(),
            credential_type,
            issued_at: env.ledger().timestamp(),
            issuer: worker.clone(),
            issuer_name: String::from_str(&env, "Unverified"),
            skill_description,
            status: CredentialStatus::Pending,
            verified: false,
            staked_amount: 0,
        };

        storage.set(&DataKey::Credential(counter), &credential);
        storage.set(&DataKey::CredentialCounter, &counter);

        counter
    }

    pub fn employer_verify_credential(
        env: Env,
        employer: Address,
        credential_id: u64,
        issuer_name: String,
        stake_amount: i128,
    ) -> bool {
        employer.require_auth();

        if stake_amount < 500_0000000 {
            panic!("Minimum stake is 500 USDC");
        }

        let storage = env.storage().persistent();
        match storage.get::<DataKey, Credential>(&DataKey::Credential(credential_id)) {
            Some(mut credential) => {
                if credential.verified {
                    panic!("Credential already verified");
                }

                credential.issuer = employer.clone();
                credential.issuer_name = issuer_name;
                credential.status = CredentialStatus::Verified;
                credential.verified = true;
                credential.staked_amount = stake_amount;

                storage.set(&DataKey::Credential(credential_id), &credential);

                let mut employer_record: Employer = storage
                    .get(&DataKey::Employer(employer.clone()))
                    .unwrap_or(Employer {
                        address: employer.clone(),
                        name: String::from_str(&env, "Unknown"),
                        staked_amount: 0,
                        credentials_issued: 0,
                        reputation_score: 750,
                        banned: false,
                    });

                employer_record.staked_amount += stake_amount;
                employer_record.credentials_issued += 1;

                storage.set(&DataKey::Employer(employer.clone()), &employer_record);

                true
            }
            None => {
                panic!("Credential not found");
            }
        }
    }

    pub fn worker_rate_employer(
        env: Env,
        worker: Address,
        employer: Address,
        rating: u32,
    ) -> u32 {
        worker.require_auth();

        if rating > 1000 {
            panic!("Rating must be between 0 and 1000");
        }

        let storage = env.storage().persistent();
        match storage.get::<DataKey, Employer>(&DataKey::Employer(employer.clone())) {
            Some(mut employer_record) => {
                let prev_total = employer_record.reputation_score * employer_record.credentials_issued as u32;
                let new_total = prev_total + rating;
                let new_count = employer_record.credentials_issued as u32 + 1;
                employer_record.reputation_score = new_total / new_count;

                storage.set(&DataKey::Employer(employer.clone()), &employer_record);

                employer_record.reputation_score
            }
            None => {
                panic!("Employer not found");
            }
        }
    }

    pub fn get_worker_portfolio(env: Env, worker: Address) -> WorkerPortfolio {
        let storage = env.storage().persistent();
        
        match storage.get::<DataKey, WorkerPortfolio>(&DataKey::WorkerPortfolio(worker.clone())) {
            Some(portfolio) => portfolio,
            None => {
                let portfolio = WorkerPortfolio {
                    worker: worker.clone(),
                    credentials: Vec::new(&env),
                    total_verified: 0,
                    average_employer_rating: 750,
                    portfolio_views: 0,
                    earnings_from_views: 0,
                };

                storage.set(&DataKey::WorkerPortfolio(worker), &portfolio);
                portfolio
            }
        }
    }

    pub fn add_to_portfolio(
        env: Env,
        worker: Address,
        credential_id: u64,
    ) {
        worker.require_auth();

        let storage = env.storage().persistent();
        
        match storage.get::<DataKey, Credential>(&DataKey::Credential(credential_id)) {
            Some(credential) => {
                if credential.worker != worker {
                    panic!("Credential does not belong to this worker");
                }

                if !credential.verified {
                    panic!("Credential is not yet verified");
                }

                let mut portfolio: WorkerPortfolio = storage
                    .get(&DataKey::WorkerPortfolio(worker.clone()))
                    .unwrap_or(WorkerPortfolio {
                        worker: worker.clone(),
                        credentials: Vec::new(&env),
                        total_verified: 0,
                        average_employer_rating: 750,
                        portfolio_views: 0,
                        earnings_from_views: 0,
                    });

                let mut found = false;
                for cred_id in portfolio.credentials.iter() {
                    if cred_id == credential_id {
                        found = true;
                        break;
                    }
                }

                if !found {
                    portfolio.credentials.push_back(credential_id);
                    portfolio.total_verified += 1;
                }

                storage.set(&DataKey::WorkerPortfolio(worker), &portfolio);
            }
            None => {
                panic!("Credential not found");
            }
        }
    }

    pub fn get_credential(env: Env, credential_id: u64) -> Credential {
        let storage = env.storage().persistent();
        match storage.get::<DataKey, Credential>(&DataKey::Credential(credential_id)) {
            Some(credential) => credential,
            None => {
                panic!("Credential not found");
            }
        }
    }

    pub fn get_employer(env: Env, employer: Address) -> Employer {
        let storage = env.storage().persistent();
        match storage.get::<DataKey, Employer>(&DataKey::Employer(employer)) {
            Some(employer_record) => employer_record,
            None => {
                panic!("Employer not found");
            }
        }
    }

    pub fn view_portfolio(env: Env, worker: Address) -> WorkerPortfolio {
        let storage = env.storage().persistent();
        
        match storage.get::<DataKey, WorkerPortfolio>(&DataKey::WorkerPortfolio(worker.clone())) {
            Some(mut portfolio) => {
                portfolio.portfolio_views += 1;
                portfolio.earnings_from_views += 400000;

                storage.set(&DataKey::WorkerPortfolio(worker), &portfolio);

                portfolio
            }
            None => {
                panic!("Portfolio not found");
            }
        }
    }

    pub fn get_credential_counter(env: Env) -> u64 {
        let storage = env.storage().persistent();
        storage.get(&DataKey::CredentialCounter).unwrap_or(0)
    }
}

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
    #[should_panic(expected = "Minimum stake is 500 USDC")]
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