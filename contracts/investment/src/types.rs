use soroban_sdk::contracttype;

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FarmInvestmentDetails {
    pub id: u32,
    pub farm_id: u32,
    pub image: soroban_sdk::String,
    pub name: soroban_sdk::String,
    pub about: soroban_sdk::String,
    pub owner: soroban_sdk::Address,
    pub min_amount: i128,
    pub amount_raised: i128,
    pub start_date: u64,
    pub end_date: u64,
    pub farm_investor_count: u32,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Investor {
    pub id: u32,
    pub farm_id: u32,
    pub investor_address: soroban_sdk::Address,
    pub amount: i128,
}
