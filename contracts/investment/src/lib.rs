#![no_std]

mod errors;
mod storage;
mod types;

use errors::InvestmentError;
use types::{FarmInvestmentDetails, Investor};

use soroban_sdk::{contract, contractimpl, token::TokenClient, Address, Env, String, Symbol, Vec};

#[contract]
pub struct InvestmentContract;

#[contractimpl]
impl InvestmentContract {
    /// Initialize the contract.
    pub fn initialize(env: Env, token: Address) {
        if env.storage().instance().has(&Symbol::new(&env, "token")) {
            panic!("{:?}", InvestmentError::AlreadyInitialized);
        }
        env.storage()
            .instance()
            .set(&Symbol::new(&env, "token"), &token);
        env.storage()
            .instance()
            .set(&Symbol::new(&env, "total"), &0i128);
        env.storage()
            .instance()
            .set(&Symbol::new(&env, "inv_ctr"), &0u32);
    }

    /// Create a new investment opportunity for a farm.
    pub fn create_investment(
        env: Env,
        farm_id: u32,
        image: String,
        name: String,
        about: String,
        min_amount: i128,
        end_date: u64,
        owner: Address,
    ) {
        let mut inv_counter: u32 = env
            .storage()
            .instance()
            .get(&Symbol::new(&env, "inv_ctr"))
            .unwrap_or(0);
        inv_counter += 1;

        let start_date = env.ledger().timestamp();

        let investment = FarmInvestmentDetails {
            id: inv_counter,
            farm_id,
            image,
            name,
            about,
            owner,
            min_amount,
            amount_raised: 0,
            start_date,
            end_date,
            farm_investor_count: 0,
        };

        let inv_key = (Symbol::new(&env, "inv"), inv_counter);
        env.storage().persistent().set(&inv_key, &investment);

        let active_key = (Symbol::new(&env, "active"), inv_counter);
        env.storage().persistent().set(&active_key, &true);

        env.storage()
            .instance()
            .set(&Symbol::new(&env, "inv_ctr"), &inv_counter);

        env.events().publish(
            (
                Symbol::new(&env, "investment"),
                Symbol::new(&env, "created"),
            ),
            (inv_counter, farm_id),
        );
    }

    /// Invest in a farm.
    pub fn invest(env: Env, caller: Address, farm_id: u32, amount: i128) {
        caller.require_auth();

        let inv_key = (Symbol::new(&env, "inv"), farm_id);
        let mut investment: FarmInvestmentDetails = env
            .storage()
            .persistent()
            .get(&inv_key)
            .unwrap_or_else(|| panic!("{:?}", InvestmentError::InvestmentNotFound));

        let active_key = (Symbol::new(&env, "active"), farm_id);
        let active: bool = env.storage().persistent().get(&active_key).unwrap_or(false);
        if !active {
            panic!("{:?}", InvestmentError::InvestmentNotActive);
        }

        if amount < investment.min_amount {
            panic!("{:?}", InvestmentError::AmountBelowMinimum);
        }

        let token: Address = env
            .storage()
            .instance()
            .get(&Symbol::new(&env, "token"))
            .unwrap();
        let contract_addr = env.current_contract_address();
        let token_client = TokenClient::new(&env, &token);
        token_client.transfer_from(&contract_addr, &caller, &contract_addr, &amount);

        // Record investor
        let inv_count: u32 = env
            .storage()
            .persistent()
            .get(&Symbol::new(&env, "inv_count"))
            .unwrap_or(0);
        let investor = Investor {
            id: inv_count + 1,
            farm_id,
            investor_address: caller.clone(),
            amount,
        };

        let investor_key = (Symbol::new(&env, "investor"), farm_id, inv_count);
        env.storage().persistent().set(&investor_key, &investor);

        // Farm-specific investor
        let f_inv_count_key = (Symbol::new(&env, "f_inv_c"), farm_id);
        let f_count: u32 = env
            .storage()
            .persistent()
            .get(&f_inv_count_key)
            .unwrap_or(0);
        let f_inv_key = (Symbol::new(&env, "f_inv"), farm_id, f_count);
        env.storage().persistent().set(&f_inv_key, &investor);
        env.storage()
            .persistent()
            .set(&f_inv_count_key, &(f_count + 1));

        // Update investment
        investment.amount_raised += amount;
        investment.farm_investor_count += 1;
        env.storage().persistent().set(&inv_key, &investment);

        // Update total
        let mut total: i128 = env
            .storage()
            .instance()
            .get(&Symbol::new(&env, "total"))
            .unwrap_or(0);
        total += amount;
        env.storage()
            .instance()
            .set(&Symbol::new(&env, "total"), &total);

        env.storage()
            .persistent()
            .set(&Symbol::new(&env, "inv_count"), &(inv_count + 1));

        env.events().publish(
            (
                Symbol::new(&env, "investment"),
                Symbol::new(&env, "new_investment"),
            ),
            (farm_id, caller, amount),
        );
    }

    /// Claim investment funds. Only farm owner can claim after end_date.
    pub fn claim_investment(env: Env, caller: Address, id: u32) {
        caller.require_auth();

        let inv_key = (Symbol::new(&env, "inv"), id);
        let mut investment: FarmInvestmentDetails = env
            .storage()
            .persistent()
            .get(&inv_key)
            .unwrap_or_else(|| panic!("{:?}", InvestmentError::InvestmentNotFound));

        let active_key = (Symbol::new(&env, "active"), id);
        let active: bool = env.storage().persistent().get(&active_key).unwrap_or(false);
        if !active {
            panic!("{:?}", InvestmentError::InvestmentNotActive);
        }

        if caller != investment.owner {
            panic!("{:?}", InvestmentError::NotFarmOwner);
        }

        let current_time = env.ledger().timestamp();
        if current_time < investment.end_date {
            panic!("{:?}", InvestmentError::EndDateNotReached);
        }

        if investment.amount_raised == 0 {
            panic!("{:?}", InvestmentError::NothingToClaim);
        }

        let token: Address = env
            .storage()
            .instance()
            .get(&Symbol::new(&env, "token"))
            .unwrap();
        let contract_addr = env.current_contract_address();
        let token_client = TokenClient::new(&env, &token);
        token_client.transfer(&contract_addr, &caller, &investment.amount_raised);

        let claimed = investment.amount_raised;
        investment.amount_raised = 0;
        env.storage().persistent().set(&inv_key, &investment);
        env.storage().persistent().set(&active_key, &false);

        env.events().publish(
            (
                Symbol::new(&env, "investment"),
                Symbol::new(&env, "withdrawn"),
            ),
            (id, caller, claimed),
        );
    }

    /// Get all investors.
    pub fn get_all_investors(env: Env) -> Vec<Investor> {
        let _inv_count: u32 = env
            .storage()
            .persistent()
            .get(&Symbol::new(&env, "inv_count"))
            .unwrap_or(0);
        // TODO: iterate all farms to collect investors
        Vec::new(&env)
    }

    /// Get all investable farms.
    pub fn get_all_investable_farms(env: Env) -> Vec<FarmInvestmentDetails> {
        let inv_counter: u32 = env
            .storage()
            .instance()
            .get(&Symbol::new(&env, "inv_ctr"))
            .unwrap_or(0);
        let mut farms = Vec::new(&env);
        for i in 1..=inv_counter {
            let inv_key = (Symbol::new(&env, "inv"), i);
            if let Some(inv) = env
                .storage()
                .persistent()
                .get::<_, FarmInvestmentDetails>(&inv_key)
            {
                farms.push_back(inv);
            }
        }
        farms
    }

    /// Get total investment amount.
    pub fn get_total_investment(env: Env) -> i128 {
        env.storage()
            .instance()
            .get(&Symbol::new(&env, "total"))
            .unwrap_or(0)
    }

    /// Get all investors for a specific farm.
    pub fn get_farm_investors(env: Env, farm_id: u32) -> Vec<Investor> {
        let f_count_key = (Symbol::new(&env, "f_inv_c"), farm_id);
        let count: u32 = env.storage().persistent().get(&f_count_key).unwrap_or(0);
        let mut investors = Vec::new(&env);
        for i in 0..count {
            let f_inv_key = (Symbol::new(&env, "f_inv"), farm_id, i);
            if let Some(inv) = env.storage().persistent().get::<_, Investor>(&f_inv_key) {
                investors.push_back(inv);
            }
        }
        investors
    }
}

#[cfg(test)]
mod test;
