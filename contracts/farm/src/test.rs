use soroban_sdk::testutils::Address as _;
use soroban_sdk::{Address, Env, String};

use crate::{FarmContract, FarmContractClient};

fn setup_env<'a>() -> (Env, FarmContractClient<'a>) {
    let env = Env::default();
    env.mock_all_auths();
    let contract_id = env.register_contract(None, FarmContract);
    let client = FarmContractClient::new(&env, &contract_id);

    let admin = Address::generate(&env);
    let token = Address::generate(&env);
    let escrow = Address::generate(&env);

    client.initialize(&admin, &token, &escrow);

    (env, client)
}

#[test]
fn test_initialize() {
    let (_env, _client) = setup_env();
}

#[test]
#[should_panic]
fn test_double_initialize() {
    let (env, client) = setup_env();
    let admin = Address::generate(&env);
    let token = Address::generate(&env);
    let escrow = Address::generate(&env);
    client.initialize(&admin, &token, &escrow);
}

#[test]
fn test_register_farm() {
    let (env, client) = setup_env();

    let farmer_addr = Address::generate(&env);
    let name = String::from_str(&env, "AbelFarm");
    let image = String::from_str(&env, "img.png");
    let location = String::from_str(&env, "Lagos");
    let contact = String::from_str(&env, "7012345678");
    let email = String::from_str(&env, "abel@test.com");

    client.register_farm(
        &farmer_addr,
        &name,
        &image,
        &location,
        &contact,
        &farmer_addr,
        &email,
    );

    let farm_index = client.get_farm_index(&name);
    assert_eq!(farm_index, 1);

    let user = client.get_user(&farmer_addr);
    assert_eq!(user.business_name, name);
    assert!(user.is_registered);
}

#[test]
#[should_panic]
fn test_register_farm_empty_name() {
    let (env, client) = setup_env();

    let farmer_addr = Address::generate(&env);
    let name = String::from_str(&env, "");
    let image = String::from_str(&env, "img.png");
    let location = String::from_str(&env, "Lagos");
    let contact = String::from_str(&env, "7012345678");
    let email = String::from_str(&env, "abel@test.com");

    client.register_farm(
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
    let (env, client) = setup_env();

    let farmer_addr = Address::generate(&env);
    let name = String::from_str(&env, "AbelFarm");
    let image = String::from_str(&env, "img.png");
    let location = String::from_str(&env, "Lagos");
    let contact = String::from_str(&env, "7012345678");
    let email = String::from_str(&env, "abel@test.com");

    client.register_farm(
        &farmer_addr,
        &name,
        &image,
        &location,
        &contact,
        &farmer_addr,
        &email,
    );

    let prod_name = String::from_str(&env, "Rice");
    let prod_image = String::from_str(&env, "rice.png");
    let prod_desc = String::from_str(&env, "Fresh rice");
    let prod_price = 100i128;

    client.add_farm_product(
        &farmer_addr,
        &prod_name,
        &prod_image,
        &prod_desc,
        &prod_price,
    );

    let products = client.get_farm_products(&farmer_addr);
    assert_eq!(products.len(), 1);
    assert_eq!(products.get(0).unwrap().product_name, prod_name);
    assert_eq!(products.get(0).unwrap().product_price, 100);
}

#[test]
fn test_get_total_sales() {
    let (_env, client) = setup_env();
    let sales = client.get_total_sales();
    assert_eq!(sales, 0);
}
