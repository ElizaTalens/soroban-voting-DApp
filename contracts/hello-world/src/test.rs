#![cfg(test)]

use super::*;
use soroban_sdk::{testutils::Address as _, Address, Env, String};

#[test]
fn test_initialize() {
    let env = Env::default();
    env.mock_all_auths();

    let contract_id = env.register(VotingContract, ());
    let client = VotingContractClient::new(&env, &contract_id);

    let admin = Address::generate(&env);

    client.initialize(&admin);

    assert_eq!(client.get_admin(), admin);
    assert_eq!(client.is_voting_open(), true);
    assert_eq!(client.get_candidates().len(), 0);
}

#[test]
fn test_add_candidate() {
    let env = Env::default();
    env.mock_all_auths();

    let contract_id = env.register(VotingContract, ());
    let client = VotingContractClient::new(&env, &contract_id);

    let admin = Address::generate(&env);

    client.initialize(&admin);
    client.add_candidate(&admin, &String::from_str(&env, "Alice"));
    client.add_candidate(&admin, &String::from_str(&env, "Bob"));

    let candidates = client.get_candidates();

    assert_eq!(candidates.len(), 2);
    assert_eq!(candidates.get(0).unwrap().name, String::from_str(&env, "Alice"));
    assert_eq!(candidates.get(1).unwrap().name, String::from_str(&env, "Bob"));
}

#[test]
fn test_vote() {
    let env = Env::default();
    env.mock_all_auths();

    let contract_id = env.register(VotingContract, ());
    let client = VotingContractClient::new(&env, &contract_id);

    let admin = Address::generate(&env);
    let voter = Address::generate(&env);

    client.initialize(&admin);
    client.add_candidate(&admin, &String::from_str(&env, "Alice"));

    client.vote(&voter, &0);

    let candidate = client.get_candidate(&0);

    assert_eq!(candidate.vote_count, 1);
    assert_eq!(client.has_voted(&voter), true);
}