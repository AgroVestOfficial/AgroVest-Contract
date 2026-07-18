use soroban_sdk::testutils::Address as _;
use soroban_sdk::token::{StellarAssetClient, TokenClient};
use soroban_sdk::{Address, Env, String};

use crate::{FarmContract, FarmContractClient};

struct TestCtx<'a> {
    env: Env,
    admin: Address,
    token_addr: Address,
    contract_addr: Address,
    client: FarmContractClient<'a>,
}

fn setup<'a>() -> TestCtx<'a> {
    let env = Env::default();
    env.mock_all_auths();

    let token_admin = Address::generate(&env);
    let token_addr = env.register_stellar_asset_contract(token_admin.clone());

    let admin = Address::generate(&env);
    let escrow = Address::generate(&env);
    let contract_addr = env.register_contract(None, FarmContract);
    let client = FarmContractClient::new(&env, &contract_addr);

    client.initialize(&admin, &token_addr, &escrow);

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

fn register_farmer(ctx: &TestCtx, farmer: &Address) {
    let name = String::from_str(&ctx.env, "AbelFarm");
    let image = String::from_str(&ctx.env, "img.png");
    let location = String::from_str(&ctx.env, "Lagos");
    let contact = String::from_str(&ctx.env, "7012345678");
    let email = String::from_str(&ctx.env, "abel@test.com");

    ctx.client
        .register_farm(farmer, &name, &image, &location, &contact, farmer, &email);
}

#[test]
fn test_initialize() {
    let _ctx = setup();
}

#[test]
#[should_panic]
fn test_double_initialize() {
    let ctx = setup();
    let admin = Address::generate(&ctx.env);
    let token = Address::generate(&ctx.env);
    let escrow = Address::generate(&ctx.env);
    ctx.client.initialize(&admin, &token, &escrow);
}

#[test]
fn test_register_farm() {
    let ctx = setup();

    let farmer_addr = Address::generate(&ctx.env);
    let name = String::from_str(&ctx.env, "AbelFarm");

    register_farmer(&ctx, &farmer_addr);

    let farm_index = ctx.client.get_farm_index(&name);
    assert_eq!(farm_index, 1);

    let user = ctx.client.get_user(&farmer_addr);
    assert_eq!(user.business_name, name);
    assert!(user.is_registered);
}

#[test]
#[should_panic]
fn test_register_farm_empty_name() {
    let ctx = setup();

    let farmer_addr = Address::generate(&ctx.env);
    let name = String::from_str(&ctx.env, "");
    let image = String::from_str(&ctx.env, "img.png");
    let location = String::from_str(&ctx.env, "Lagos");
    let contact = String::from_str(&ctx.env, "7012345678");
    let email = String::from_str(&ctx.env, "abel@test.com");

    ctx.client.register_farm(
        &farmer_addr,
        &name,
        &image,
        &location,
        &contact,
        &farmer_addr,
        &email,
    );
}

#[test]
fn test_add_farm_product() {
    let ctx = setup();

    let farmer_addr = Address::generate(&ctx.env);
    register_farmer(&ctx, &farmer_addr);

    let prod_name = String::from_str(&ctx.env, "Rice");
    let prod_image = String::from_str(&ctx.env, "rice.png");
    let prod_desc = String::from_str(&ctx.env, "Fresh rice");
    let prod_price = 100i128;

    ctx.client.add_farm_product(
        &farmer_addr,
        &prod_name,
        &prod_image,
        &prod_desc,
        &prod_price,
    );

    let products = ctx.client.get_farm_products(&farmer_addr);
    assert_eq!(products.len(), 1);
    assert_eq!(products.get(0).unwrap().product_name, prod_name);
    assert_eq!(products.get(0).unwrap().product_price, 100);
}

#[test]
fn test_get_total_sales() {
    let ctx = setup();
    let sales = ctx.client.get_total_sales();
    assert_eq!(sales, 0);
}

#[test]
fn test_purchase_transfers_tokens() {
    let ctx = setup();

    let farmer_addr = Address::generate(&ctx.env);
    register_farmer(&ctx, &farmer_addr);

    let prod_name = String::from_str(&ctx.env, "Rice");
    let prod_image = String::from_str(&ctx.env, "rice.png");
    let prod_desc = String::from_str(&ctx.env, "Fresh rice");
    let prod_price = 500i128;

    ctx.client.add_farm_product(
        &farmer_addr,
        &prod_name,
        &prod_image,
        &prod_desc,
        &prod_price,
    );

    let buyer = Address::generate(&ctx.env);
    mint(&ctx, &buyer, 1000);
    approve(&ctx, &buyer, 1000);

    ctx.client.purchase_product(&buyer, &1u32, &500i128);

    let token = TokenClient::new(&ctx.env, &ctx.token_addr);
    assert_eq!(token.balance(&buyer), 500);
    assert_eq!(token.balance(&farmer_addr), 500);

    let product = ctx.client.get_farm_products(&farmer_addr).get(0).unwrap();
    assert!(product.sold);

    assert_eq!(ctx.client.get_total_sales(), 500);
}

#[test]
#[should_panic]
fn test_purchase_already_sold() {
    let ctx = setup();

    let farmer_addr = Address::generate(&ctx.env);
    register_farmer(&ctx, &farmer_addr);

    let prod_name = String::from_str(&ctx.env, "Rice");
    let prod_image = String::from_str(&ctx.env, "rice.png");
    let prod_desc = String::from_str(&ctx.env, "Fresh rice");
    let prod_price = 500i128;

    ctx.client.add_farm_product(
        &farmer_addr,
        &prod_name,
        &prod_image,
        &prod_desc,
        &prod_price,
    );

    let buyer = Address::generate(&ctx.env);
    mint(&ctx, &buyer, 2000);
    approve(&ctx, &buyer, 2000);

    ctx.client.purchase_product(&buyer, &1u32, &500i128);
    ctx.client.purchase_product(&buyer, &1u32, &500i128);
}

#[test]
#[should_panic]
fn test_purchase_price_mismatch() {
    let ctx = setup();

    let farmer_addr = Address::generate(&ctx.env);
    register_farmer(&ctx, &farmer_addr);

    let prod_name = String::from_str(&ctx.env, "Rice");
    let prod_image = String::from_str(&ctx.env, "rice.png");
    let prod_desc = String::from_str(&ctx.env, "Fresh rice");
    let prod_price = 500i128;

    ctx.client.add_farm_product(
        &farmer_addr,
        &prod_name,
        &prod_image,
        &prod_desc,
        &prod_price,
    );

    let buyer = Address::generate(&ctx.env);
    mint(&ctx, &buyer, 2000);
    approve(&ctx, &buyer, 2000);

    ctx.client.purchase_product(&buyer, &1u32, &100i128);
}
