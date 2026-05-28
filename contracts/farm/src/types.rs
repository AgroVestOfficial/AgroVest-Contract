use soroban_sdk::contracttype;

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Farmer {
    pub farm_id: u32,
    pub business_name: soroban_sdk::String,
    pub business_image: soroban_sdk::String,
    pub business_location: soroban_sdk::String,
    pub business_contact: soroban_sdk::String,
    pub business_email: soroban_sdk::String,
    pub farmer_address: soroban_sdk::Address,
    pub is_registered: bool,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FarmProduct {
    pub product_name: soroban_sdk::String,
    pub product_image: soroban_sdk::String,
    pub product_description: soroban_sdk::String,
    pub product_price: i128,
    pub product_owner: soroban_sdk::Address,
    pub product_id: u32,
    pub sold: bool,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Review {
    pub reviewer: soroban_sdk::Address,
    pub review: soroban_sdk::String,
}
