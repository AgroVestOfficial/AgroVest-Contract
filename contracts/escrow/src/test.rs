use soroban_sdk::testutils::Address as _;
use soroban_sdk::token::{StellarAssetClient, TokenClient};
use soroban_sdk::{Address, Env};

use crate::types::EscrowStatus;
use crate::{EscrowContract, EscrowContractClient};

struct TestCtx<'a> {
    env: Env,
    admin: Address,
    token_addr: Address,
    contract_addr: Address,
    client: EscrowContractClient<'a>,
}

fn setup<'a>() -> TestCtx<'a> {
    let env = Env::default();
    env.mock_all_auths();

    let token_admin = Address::generate(&env);
    let token_addr = env.register_stellar_asset_contract(token_admin.clone());

    let admin = Address::generate(&env);
    let contract_addr = env.register_contract(None, EscrowContract);
    let client = EscrowContractClient::new(&env, &contract_addr);

    client.initialize(&admin, &token_addr);

    TestCtx {
        env,
        admin,
        token_addr,
        contract_addr,
        client,
    }
}

fn mint(ctx: &TestCtx, to: &Address, amount: i128) {
    let sac = StellarAssetClient::new(&ctx.env, &ctx.token_addr);
    sac.mock_all_auths().mint(to, &amount);
}

fn approve(ctx: &TestCtx, from: &Address, amount: i128) {
    let token = TokenClient::new(&ctx.env, &ctx.token_addr);
    let max_ttl = ctx.env.ledger().sequence() + 100;
    token
        .mock_all_auths()
        .approve(from, &ctx.contract_addr, &amount, &max_ttl);
}

#[test]
fn test_initialize() {
    let _ctx = setup();
}

#[test]
fn test_create_escrow_transfers_from_buyer() {
    let ctx = setup();
    let buyer = Address::generate(&ctx.env);
    let farmer = Address::generate(&ctx.env);

    mint(&ctx, &buyer, 1000);
    approve(&ctx, &buyer, 1000);

    ctx.client.create_escrow(&buyer, &farmer, &1u32, &500i128);

    let escrow = ctx.client.get_escrow_details(&1u32);
    assert_eq!(escrow.buyer, buyer);
    assert_eq!(escrow.farmer, farmer);
    assert_eq!(escrow.amount, 500);
    assert_eq!(escrow.status, EscrowStatus::AwaitingDelivery);
}

#[test]
fn test_confirm_delivery_transitions_status() {
    let ctx = setup();
    let buyer = Address::generate(&ctx.env);
    let farmer = Address::generate(&ctx.env);

    mint(&ctx, &buyer, 1000);
    approve(&ctx, &buyer, 1000);
    ctx.client.create_escrow(&buyer, &farmer, &1u32, &500i128);

    let escrow = ctx.client.get_escrow_details(&1u32);
    assert_eq!(escrow.status, EscrowStatus::AwaitingDelivery);

    ctx.client.confirm_delivery(&buyer, &1u32);

    let escrow = ctx.client.get_escrow_details(&1u32);
    assert_eq!(escrow.status, EscrowStatus::AwaitingApproval);
}

#[test]
#[should_panic]
fn test_confirm_delivery_not_buyer() {
    let ctx = setup();
    let buyer = Address::generate(&ctx.env);
    let farmer = Address::generate(&ctx.env);
    let other = Address::generate(&ctx.env);

    mint(&ctx, &buyer, 1000);
    approve(&ctx, &buyer, 1000);
    ctx.client.create_escrow(&buyer, &farmer, &1u32, &500i128);
    ctx.client.confirm_delivery(&other, &1u32);
}

#[test]
#[should_panic]
fn test_confirm_delivery_wrong_status() {
    let ctx = setup();
    let buyer = Address::generate(&ctx.env);
    let farmer = Address::generate(&ctx.env);

    mint(&ctx, &buyer, 1000);
    approve(&ctx, &buyer, 1000);
    ctx.client.create_escrow(&buyer, &farmer, &1u32, &500i128);
    ctx.client.confirm_delivery(&buyer, &1u32);
    ctx.client.confirm_delivery(&buyer, &1u32);
}

#[test]
#[should_panic]
fn test_approve_delivery_not_buyer() {
    let ctx = setup();
    let buyer = Address::generate(&ctx.env);
    let farmer = Address::generate(&ctx.env);
    let other = Address::generate(&ctx.env);

    mint(&ctx, &buyer, 1000);
    approve(&ctx, &buyer, 1000);
    ctx.client.create_escrow(&buyer, &farmer, &1u32, &500i128);
    ctx.client.confirm_delivery(&buyer, &1u32);
    ctx.client.approve_delivery(&other, &1u32);
}

#[test]
fn test_raise_dispute() {
    let ctx = setup();
    let buyer = Address::generate(&ctx.env);
    let farmer = Address::generate(&ctx.env);

    mint(&ctx, &buyer, 1000);
    approve(&ctx, &buyer, 1000);
    ctx.client.create_escrow(&buyer, &farmer, &1u32, &500i128);
    ctx.client.raise_dispute(&farmer, &1u32);

    let escrow = ctx.client.get_escrow_details(&1u32);
    assert_eq!(escrow.status, EscrowStatus::Dispute);
}

#[test]
fn test_resolve_dispute_transfers_to_winner() {
    let ctx = setup();
    let buyer = Address::generate(&ctx.env);
    let farmer = Address::generate(&ctx.env);

    mint(&ctx, &buyer, 1000);
    approve(&ctx, &buyer, 1000);
    ctx.client.create_escrow(&buyer, &farmer, &1u32, &500i128);
    ctx.client.raise_dispute(&buyer, &1u32);
    ctx.client.resolve_dispute(&ctx.admin, &1u32, &farmer);

    let escrow = ctx.client.get_escrow_details(&1u32);
    assert_eq!(escrow.status, EscrowStatus::Complete);
}

#[test]
fn test_full_escrow_lifecycle() {
    let ctx = setup();
    let buyer = Address::generate(&ctx.env);
    let farmer = Address::generate(&ctx.env);

    mint(&ctx, &buyer, 2000);
    approve(&ctx, &buyer, 2000);

    ctx.client.create_escrow(&buyer, &farmer, &1u32, &500i128);
    let escrow = ctx.client.get_escrow_details(&1u32);
    assert_eq!(escrow.status, EscrowStatus::AwaitingDelivery);

    ctx.client.confirm_delivery(&buyer, &1u32);
    let escrow = ctx.client.get_escrow_details(&1u32);
    assert_eq!(escrow.status, EscrowStatus::AwaitingApproval);

    ctx.client.approve_delivery(&buyer, &1u32);
    let escrow = ctx.client.get_escrow_details(&1u32);
    assert_eq!(escrow.status, EscrowStatus::Complete);

    let token = TokenClient::new(&ctx.env, &ctx.token_addr);
    let contract_balance = token.balance(&ctx.contract_addr);
    assert_eq!(contract_balance, 0);
    let farmer_balance = token.balance(&farmer);
    assert_eq!(farmer_balance, 500);
    let buyer_balance = token.balance(&buyer);
    assert_eq!(buyer_balance, 1500);
}
