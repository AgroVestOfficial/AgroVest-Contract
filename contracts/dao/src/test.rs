use soroban_sdk::{Address, Env, String};
use soroban_sdk::testutils::Address as _;

use crate::{DaoContract, DaoContractClient};
use crate::types::VoteData;

fn setup_env<'a>() -> (Env, DaoContractClient<'a>) {
    let env = Env::default();
    env.mock_all_auths();
    let contract_id = env.register_contract(None, DaoContract);
    let client = DaoContractClient::new(&env, &contract_id);

    let admin = Address::generate(&env);
    let token = Address::generate(&env);
    let investment = Address::generate(&env);

    client.initialize(&admin, &token, &investment);

    (env, client)
}

#[test]
fn test_initialize() {
    let (_env, _client) = setup_env();
}

#[test]
fn test_lock_and_unlock_tokens() {
    let (env, client) = setup_env();

    let user = Address::generate(&env);

    client.lock_tokens(&user, &1000i128);

    let balance = client.get_token_balance(&user);
    assert_eq!(balance, 1000);

    client.unlock_tokens(&user);

    let balance = client.get_token_balance(&user);
    assert_eq!(balance, 0);
}

#[test]
fn test_create_proposal() {
    let (env, client) = setup_env();

    let proposer = Address::generate(&env);
    let title = String::from_str(&env, "Fund Rice Farm");
    let desc = String::from_str(&env, "Proposal to fund rice farming operations");
    let ends_at = env.ledger().timestamp() + 86400;

    client.create_proposal(&proposer, &title, &desc, &100i128, &ends_at);

    let proposal = client.get_proposal(&1u32);
    assert_eq!(proposal.title, title);
    assert_eq!(proposal.required_votes, 100);
}

#[test]
fn test_vote_proposal() {
    let (env, client) = setup_env();

    let proposer = Address::generate(&env);
    let voter = Address::generate(&env);
    let title = String::from_str(&env, "Fund Rice Farm");
    let desc = String::from_str(&env, "Proposal to fund rice farming operations");
    let ends_at = env.ledger().timestamp() + 86400;

    client.lock_tokens(&voter, &1000i128);
    client.create_proposal(&proposer, &title, &desc, &100i128, &ends_at);
    client.vote_proposal(&voter, &1u32, &VoteData::Accept);

    let proposal = client.get_proposal(&1u32);
    assert!(proposal.accept_votes > 0);
}

#[test]
fn test_delegate() {
    let (env, client) = setup_env();

    let delegator = Address::generate(&env);
    let delegatee = Address::generate(&env);

    client.delegate(&delegator, &delegatee);

    let d = client.get_delegate(&delegator);
    assert_eq!(d, delegatee);
}

#[test]
fn test_create_challenge() {
    let (env, client) = setup_env();

    let proposer = Address::generate(&env);
    let challenger = Address::generate(&env);
    let title = String::from_str(&env, "Fund Rice Farm");
    let desc = String::from_str(&env, "Proposal to fund rice farming operations");
    let ends_at = env.ledger().timestamp() + 86400;

    client.create_proposal(&proposer, &title, &desc, &100i128, &ends_at);

    let chall_desc = String::from_str(&env, "This proposal is invalid");
    client.create_challenge(&challenger, &1u32, &chall_desc);

    let challenge = client.get_challenge(&1u32);
    assert_eq!(challenge.proposal_id, 1);
    assert!(!challenge.resolved);
}

#[test]
fn test_dispute_flow() {
    let (env, client) = setup_env();

    let proposer = Address::generate(&env);
    let challenger = Address::generate(&env);
    let arbitrator = Address::generate(&env);
    let title = String::from_str(&env, "Fund Rice Farm");
    let desc = String::from_str(&env, "Proposal to fund rice farming operations");
    let ends_at = env.ledger().timestamp() + 86400;

    client.create_proposal(&proposer, &title, &desc, &100i128, &ends_at);

    let chall_desc = String::from_str(&env, "Invalid proposal");
    client.create_challenge(&challenger, &1u32, &chall_desc);
    client.initiate_dispute(&challenger, &1u32, &arbitrator);

    let dispute = client.get_dispute(&1u32);
    assert_eq!(dispute.arbitrator, arbitrator);
    assert!(!dispute.resolved);

    client.resolve_dispute(&arbitrator, &1u32, &true);

    let dispute = client.get_dispute(&1u32);
    assert!(dispute.resolved);
    assert!(dispute.ruling);
}
