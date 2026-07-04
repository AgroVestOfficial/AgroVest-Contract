use soroban_sdk::testutils::{Address as _, Ledger};
use soroban_sdk::token::{StellarAssetClient, TokenClient};
use soroban_sdk::{Address, Env, String};

use crate::{InvestmentContract, InvestmentContractClient};

struct TestCtx<'a> {
    env: Env,
    token_addr: Address,
    contract_addr: Address,
    client: InvestmentContractClient<'a>,
}

fn setup<'a>() -> TestCtx<'a> {
    let env = Env::default();
    env.mock_all_auths();

    let token_admin = Address::generate(&env);
    let token_addr = env.register_stellar_asset_contract(token_admin.clone());

    let contract_addr = env.register_contract(None, InvestmentContract);
    let client = InvestmentContractClient::new(&env, &contract_addr);

    client.initialize(&token_addr);

    TestCtx {
        env,
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
fn test_create_investment() {
    let ctx = setup();
    let owner = Address::generate(&ctx.env);
    let name = String::from_str(&ctx.env, "Rice Farm Investment");
    let about = String::from_str(&ctx.env, "Invest in rice farming");
    let image = String::from_str(&ctx.env, "rice.png");
    let end_date = ctx.env.ledger().timestamp() + 86400;

    ctx.client
        .create_investment(&1u32, &image, &name, &about, &100i128, &end_date, &owner);

    let farms = ctx.client.get_all_investable_farms();
    assert_eq!(farms.len(), 1);
    assert_eq!(farms.get(0).unwrap().name, name);
    assert_eq!(farms.get(0).unwrap().min_amount, 100);
}

#[test]
fn test_invest_transfers_tokens() {
    let ctx = setup();
    let owner = Address::generate(&ctx.env);
    let investor = Address::generate(&ctx.env);
    let name = String::from_str(&ctx.env, "Rice Farm Investment");
    let about = String::from_str(&ctx.env, "Invest in rice farming");
    let image = String::from_str(&ctx.env, "rice.png");
    let end_date = ctx.env.ledger().timestamp() + 86400;

    ctx.client
        .create_investment(&1u32, &image, &name, &about, &100i128, &end_date, &owner);

    mint(&ctx, &investor, 1000);
    approve(&ctx, &investor, 1000);

    ctx.client.invest(&investor, &1u32, &500i128);

    let total = ctx.client.get_total_investment();
    assert_eq!(total, 500);

    let farm_investors = ctx.client.get_farm_investors(&1u32);
    assert_eq!(farm_investors.len(), 1);
    assert_eq!(farm_investors.get(0).unwrap().amount, 500);
}

#[test]
#[should_panic]
fn test_invest_below_minimum() {
    let ctx = setup();
    let owner = Address::generate(&ctx.env);
    let investor = Address::generate(&ctx.env);
    let name = String::from_str(&ctx.env, "Rice Farm Investment");
    let about = String::from_str(&ctx.env, "Invest in rice farming");
    let image = String::from_str(&ctx.env, "rice.png");
    let end_date = ctx.env.ledger().timestamp() + 86400;

    ctx.client
        .create_investment(&1u32, &image, &name, &about, &100i128, &end_date, &owner);

    mint(&ctx, &investor, 1000);
    approve(&ctx, &investor, 1000);

    ctx.client.invest(&investor, &1u32, &50i128);
}

#[test]
#[should_panic]
fn test_claim_before_end_date() {
    let ctx = setup();
    let owner = Address::generate(&ctx.env);
    let investor = Address::generate(&ctx.env);
    let name = String::from_str(&ctx.env, "Rice Farm Investment");
    let about = String::from_str(&ctx.env, "Invest in rice farming");
    let image = String::from_str(&ctx.env, "rice.png");
    let end_date = ctx.env.ledger().timestamp() + 86400;

    ctx.client
        .create_investment(&1u32, &image, &name, &about, &100i128, &end_date, &owner);

    mint(&ctx, &investor, 1000);
    approve(&ctx, &investor, 1000);

    ctx.client.invest(&investor, &1u32, &500i128);
    ctx.client.claim_investment(&owner, &1u32);
}

#[test]
fn test_claim_after_end_date_transfers_to_owner() {
    let ctx = setup();
    let owner = Address::generate(&ctx.env);
    let investor = Address::generate(&ctx.env);
    let name = String::from_str(&ctx.env, "Rice Farm Investment");
    let about = String::from_str(&ctx.env, "Invest in rice farming");
    let image = String::from_str(&ctx.env, "rice.png");
    let end_date = ctx.env.ledger().timestamp() + 86400;

    ctx.client
        .create_investment(&1u32, &image, &name, &about, &100i128, &end_date, &owner);

    mint(&ctx, &investor, 1000);
    approve(&ctx, &investor, 1000);

    ctx.client.invest(&investor, &1u32, &500i128);

    // Advance ledger past end_date
    ctx.env.ledger().with_mut(|l| {
        l.timestamp = end_date + 1;
    });

    ctx.client.claim_investment(&owner, &1u32);

    let investment = ctx.client.get_all_investable_farms().get(0).unwrap();
    assert_eq!(investment.amount_raised, 0);
}
