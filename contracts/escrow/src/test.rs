use soroban_sdk::{Address, Env};
use soroban_sdk::testutils::Address as _;

use crate::{EscrowContract, EscrowContractClient};
use crate::types::EscrowStatus;

fn setup_env<'a>() -> (Env, Address, EscrowContractClient<'a>) {
    let env = Env::default();
    env.mock_all_auths();
    let contract_id = env.register_contract(None, EscrowContract);
    let client = EscrowContractClient::new(&env, &contract_id);

    let admin = Address::generate(&env);
    let token = Address::generate(&env);

    client.initialize(&admin, &token);

    (env, admin, client)
}

#[test]
fn test_initialize() {
    let (_env, _admin, _client) = setup_env();
}

#[test]
fn test_create_escrow() {
    let (env, _admin, client) = setup_env();

    let buyer = Address::generate(&env);
    let farmer = Address::generate(&env);

    client.create_escrow(&buyer, &farmer, &1u32, &500i128);

    let escrow = client.get_escrow_details(&1u32);
    assert_eq!(escrow.buyer, buyer);
    assert_eq!(escrow.farmer, farmer);
    assert_eq!(escrow.amount, 500);
    assert_eq!(escrow.status, EscrowStatus::AwaitingDelivery);
}

#[test]
#[should_panic]
fn test_approve_delivery_not_buyer() {
    let (env, _admin, client) = setup_env();

    let buyer = Address::generate(&env);
    let farmer = Address::generate(&env);
    let other = Address::generate(&env);

    client.create_escrow(&buyer, &farmer, &1u32, &500i128);
    client.approve_delivery(&other, &1u32);
}

#[test]
fn test_raise_dispute() {
    let (env, _admin, client) = setup_env();

    let buyer = Address::generate(&env);
    let farmer = Address::generate(&env);

    client.create_escrow(&buyer, &farmer, &1u32, &500i128);
    client.raise_dispute(&farmer, &1u32);

    let escrow = client.get_escrow_details(&1u32);
    assert_eq!(escrow.status, EscrowStatus::Dispute);
}

#[test]
fn test_resolve_dispute() {
    let (env, admin, client) = setup_env();

    let buyer = Address::generate(&env);
    let farmer = Address::generate(&env);

    client.create_escrow(&buyer, &farmer, &1u32, &500i128);
    client.raise_dispute(&buyer, &1u32);
    client.resolve_dispute(&admin, &1u32, &farmer);

    let escrow = client.get_escrow_details(&1u32);
    assert_eq!(escrow.status, EscrowStatus::Complete);
}
