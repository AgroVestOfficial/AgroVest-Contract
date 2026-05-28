use soroban_sdk::{Address, Env, String};
use soroban_sdk::testutils::Address as _;

use crate::{InvestmentContract, InvestmentContractClient};

fn setup_env<'a>() -> (Env, InvestmentContractClient<'a>) {
    let env = Env::default();
    env.mock_all_auths();
    let contract_id = env.register_contract(None, InvestmentContract);
    let client = InvestmentContractClient::new(&env, &contract_id);

    let token = Address::generate(&env);
    client.initialize(&token);

    (env, client)
}

#[test]
fn test_initialize() {
    let (_env, _client) = setup_env();
}

#[test]
fn test_create_investment() {
    let (env, client) = setup_env();

    let owner = Address::generate(&env);
    let name = String::from_str(&env, "Rice Farm Investment");
    let about = String::from_str(&env, "Invest in rice farming");
    let image = String::from_str(&env, "rice.png");
    let end_date = env.ledger().timestamp() + 86400;

    client.create_investment(
        &1u32, &image, &name, &about, &100i128, &end_date, &owner,
    );

    let farms = client.get_all_investable_farms();
    assert_eq!(farms.len(), 1);
    assert_eq!(farms.get(0).unwrap().name, name);
    assert_eq!(farms.get(0).unwrap().min_amount, 100);
}

#[test]
fn test_invest() {
    let (env, client) = setup_env();

    let owner = Address::generate(&env);
    let investor = Address::generate(&env);
    let name = String::from_str(&env, "Rice Farm Investment");
    let about = String::from_str(&env, "Invest in rice farming");
    let image = String::from_str(&env, "rice.png");
    let end_date = env.ledger().timestamp() + 86400;

    client.create_investment(
        &1u32, &image, &name, &about, &100i128, &end_date, &owner,
    );

    client.invest(&investor, &1u32, &500i128);

    let total = client.get_total_investment();
    assert_eq!(total, 500);

    let farm_investors = client.get_farm_investors(&1u32);
    assert_eq!(farm_investors.len(), 1);
    assert_eq!(farm_investors.get(0).unwrap().amount, 500);
}

#[test]
#[should_panic]
fn test_invest_below_minimum() {
    let (env, client) = setup_env();

    let owner = Address::generate(&env);
    let investor = Address::generate(&env);
    let name = String::from_str(&env, "Rice Farm Investment");
    let about = String::from_str(&env, "Invest in rice farming");
    let image = String::from_str(&env, "rice.png");
    let end_date = env.ledger().timestamp() + 86400;

    client.create_investment(
        &1u32, &image, &name, &about, &100i128, &end_date, &owner,
    );

    client.invest(&investor, &1u32, &50i128);
}

#[test]
#[should_panic]
fn test_claim_before_end_date() {
    let (env, client) = setup_env();

    let owner = Address::generate(&env);
    let investor = Address::generate(&env);
    let name = String::from_str(&env, "Rice Farm Investment");
    let about = String::from_str(&env, "Invest in rice farming");
    let image = String::from_str(&env, "rice.png");
    let end_date = env.ledger().timestamp() + 86400;

    client.create_investment(
        &1u32, &image, &name, &about, &100i128, &end_date, &owner,
    );

    client.invest(&investor, &1u32, &500i128);
    client.claim_investment(&owner, &1u32);
}
