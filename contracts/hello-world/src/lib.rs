#![no_std]

use soroban_sdk::{
    contract, contractimpl, contracttype, Address, Env, String, Vec,
};

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Candidate {
    pub id: u32,
    pub name: String,
    pub vote_count: u32,
}

#[contracttype]
#[derive(Clone)]
pub enum DataKey {
    Admin,
    VotingOpen,
    Candidates,
    Voted(Address),
}

#[contract]
pub struct VotingContract;

#[contractimpl]
impl VotingContract {
    pub fn initialize(env: Env, admin: Address) -> String {
        if env.storage().instance().has(&DataKey::Admin) {
            panic!("Contract already initialized");
        }

        admin.require_auth();

        let candidates: Vec<Candidate> = Vec::new(&env);

        env.storage().instance().set(&DataKey::Admin, &admin);
        env.storage().instance().set(&DataKey::VotingOpen, &true);
        env.storage().instance().set(&DataKey::Candidates, &candidates);

        String::from_str(&env, "Voting contract initialized")
    }

    pub fn add_candidate(env: Env, admin: Address, name: String) -> String {
        admin.require_auth();

        let saved_admin: Address = env
            .storage()
            .instance()
            .get(&DataKey::Admin)
            .expect("Admin not set");

        if admin != saved_admin {
            panic!("Only admin can add candidate");
        }

        let mut candidates: Vec<Candidate> = env
            .storage()
            .instance()
            .get(&DataKey::Candidates)
            .unwrap_or(Vec::new(&env));

        let candidate = Candidate {
            id: candidates.len(),
            name,
            vote_count: 0,
        };

        candidates.push_back(candidate);
        env.storage().instance().set(&DataKey::Candidates, &candidates);

        String::from_str(&env, "Candidate added")
    }

    pub fn vote(env: Env, voter: Address, candidate_id: u32) -> String {
        voter.require_auth();

        let voting_open: bool = env
            .storage()
            .instance()
            .get(&DataKey::VotingOpen)
            .unwrap_or(false);

        if !voting_open {
            panic!("Voting is closed");
        }

        let voted_key = DataKey::Voted(voter.clone());

        let already_voted: bool = env
            .storage()
            .instance()
            .get(&voted_key)
            .unwrap_or(false);

        if already_voted {
            panic!("You have already voted");
        }

        let mut candidates: Vec<Candidate> = env
            .storage()
            .instance()
            .get(&DataKey::Candidates)
            .unwrap_or(Vec::new(&env));

        if candidate_id >= candidates.len() {
            panic!("Candidate not found");
        }

        let mut candidate = candidates.get(candidate_id).unwrap();
        candidate.vote_count += 1;

        candidates.set(candidate_id, candidate);

        env.storage().instance().set(&DataKey::Candidates, &candidates);
        env.storage().instance().set(&voted_key, &true);

        String::from_str(&env, "Vote submitted")
    }

    pub fn get_candidates(env: Env) -> Vec<Candidate> {
        env.storage()
            .instance()
            .get(&DataKey::Candidates)
            .unwrap_or(Vec::new(&env))
    }

    pub fn get_candidate(env: Env, candidate_id: u32) -> Candidate {
        let candidates: Vec<Candidate> = env
            .storage()
            .instance()
            .get(&DataKey::Candidates)
            .unwrap_or(Vec::new(&env));

        if candidate_id >= candidates.len() {
            panic!("Candidate not found");
        }

        candidates.get(candidate_id).unwrap()
    }

    pub fn has_voted(env: Env, voter: Address) -> bool {
        env.storage()
            .instance()
            .get(&DataKey::Voted(voter))
            .unwrap_or(false)
    }

    pub fn set_voting_open(env: Env, admin: Address, is_open: bool) -> String {
        admin.require_auth();

        let saved_admin: Address = env
            .storage()
            .instance()
            .get(&DataKey::Admin)
            .expect("Admin not set");

        if admin != saved_admin {
            panic!("Only admin can change voting status");
        }

        env.storage().instance().set(&DataKey::VotingOpen, &is_open);

        String::from_str(&env, "Voting status updated")
    }

    pub fn is_voting_open(env: Env) -> bool {
        env.storage()
            .instance()
            .get(&DataKey::VotingOpen)
            .unwrap_or(false)
    }

    pub fn get_admin(env: Env) -> Address {
        env.storage()
            .instance()
            .get(&DataKey::Admin)
            .expect("Admin not set")
    }
}

mod test;