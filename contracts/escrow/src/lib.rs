#![no_std]

mod errors;
mod storage;
mod types;

use errors::EscrowError;
use types::{Escrow, EscrowStatus};

use soroban_sdk::{contract, contractimpl, token::TokenClient, Address, Env, Symbol};

#[contract]
pub struct EscrowContract;

#[contractimpl]
impl EscrowContract {
    /// Initialize the contract. Can only be called once.
    pub fn initialize(env: Env, admin: Address, token: Address) {
        if env.storage().instance().has(&Symbol::new(&env, "admin")) {
            panic!("{:?}", EscrowError::AlreadyInitialized);
        }
        env.storage()
            .instance()
            .set(&Symbol::new(&env, "admin"), &admin);
        env.storage()
            .instance()
            .set(&Symbol::new(&env, "token"), &token);
        env.storage()
            .instance()
            .set(&Symbol::new(&env, "escrow_ctr"), &0u32);
    }

    /// Create an escrow. Buyer deposits tokens.
    pub fn create_escrow(env: Env, buyer: Address, farmer: Address, order_id: u32, amount: i128) {
        buyer.require_auth();

        let token: Address = env
            .storage()
            .instance()
            .get(&Symbol::new(&env, "token"))
            .unwrap();
        let contract_addr = env.current_contract_address();
        let token_client = TokenClient::new(&env, &token);
        token_client.transfer_from(&contract_addr, &buyer, &contract_addr, &amount);

        let mut escrow_counter: u32 = env
            .storage()
            .instance()
            .get(&Symbol::new(&env, "escrow_ctr"))
            .unwrap_or(0);
        escrow_counter += 1;

        let escrow = Escrow {
            buyer: buyer.clone(),
            farmer: farmer.clone(),
            amount,
            status: EscrowStatus::AwaitingDelivery,
            order_id,
        };

        let escrow_key = (Symbol::new(&env, "escrow"), escrow_counter);
        env.storage().persistent().set(&escrow_key, &escrow);
        env.storage()
            .instance()
            .set(&Symbol::new(&env, "escrow_ctr"), &escrow_counter);

        env.events().publish(
            (Symbol::new(&env, "escrow"), Symbol::new(&env, "created")),
            (escrow_counter, buyer, farmer, order_id, amount),
        );
    }

    /// Confirm delivery. Only buyer can call. Transitions AwaitingDelivery -> AwaitingApproval.
    pub fn confirm_delivery(env: Env, caller: Address, escrow_id: u32) {
        caller.require_auth();

        let escrow_key = (Symbol::new(&env, "escrow"), escrow_id);
        let mut escrow: Escrow = env
            .storage()
            .persistent()
            .get(&escrow_key)
            .unwrap_or_else(|| panic!("{:?}", EscrowError::EscrowNotFound));

        if escrow.buyer != caller {
            panic!("{:?}", EscrowError::OnlyBuyerCanConfirm);
        }
        if escrow.status != EscrowStatus::AwaitingDelivery {
            panic!("{:?}", EscrowError::InvalidStatus);
        }

        escrow.status = EscrowStatus::AwaitingApproval;
        env.storage().persistent().set(&escrow_key, &escrow);

        env.events().publish(
            (
                Symbol::new(&env, "escrow"),
                Symbol::new(&env, "delivery_confirmed"),
            ),
            escrow_id,
        );
    }

    /// Approve delivery. Only buyer can call.
    pub fn approve_delivery(env: Env, caller: Address, escrow_id: u32) {
        caller.require_auth();

        let escrow_key = (Symbol::new(&env, "escrow"), escrow_id);
        let mut escrow: Escrow = env
            .storage()
            .persistent()
            .get(&escrow_key)
            .unwrap_or_else(|| panic!("{:?}", EscrowError::EscrowNotFound));

        if escrow.buyer != caller {
            panic!("{:?}", EscrowError::OnlyBuyerCanApprove);
        }
        if escrow.status != EscrowStatus::AwaitingApproval {
            panic!("{:?}", EscrowError::InvalidStatus);
        }

        let token: Address = env
            .storage()
            .instance()
            .get(&Symbol::new(&env, "token"))
            .unwrap();
        let contract_addr = env.current_contract_address();
        let token_client = TokenClient::new(&env, &token);
        token_client.transfer(&contract_addr, &escrow.farmer, &escrow.amount);

        escrow.status = EscrowStatus::Complete;
        env.storage().persistent().set(&escrow_key, &escrow);

        env.events().publish(
            (
                Symbol::new(&env, "escrow"),
                Symbol::new(&env, "delivery_approved"),
            ),
            escrow_id,
        );
        env.events().publish(
            (Symbol::new(&env, "escrow"), Symbol::new(&env, "completed")),
            escrow_id,
        );
    }

    /// Raise a dispute. Buyer or farmer can call.
    pub fn raise_dispute(env: Env, caller: Address, escrow_id: u32) {
        caller.require_auth();

        let escrow_key = (Symbol::new(&env, "escrow"), escrow_id);
        let mut escrow: Escrow = env
            .storage()
            .persistent()
            .get(&escrow_key)
            .unwrap_or_else(|| panic!("{:?}", EscrowError::EscrowNotFound));

        if caller != escrow.buyer && caller != escrow.farmer {
            panic!("{:?}", EscrowError::OnlyParticipantsCanRaise);
        }

        escrow.status = EscrowStatus::Dispute;
        env.storage().persistent().set(&escrow_key, &escrow);

        env.events().publish(
            (
                Symbol::new(&env, "escrow"),
                Symbol::new(&env, "dispute_raised"),
            ),
            escrow_id,
        );
    }

    /// Resolve a dispute. Only admin can call.
    pub fn resolve_dispute(env: Env, admin: Address, escrow_id: u32, winner: Address) {
        admin.require_auth();

        let stored_admin: Address = env
            .storage()
            .instance()
            .get(&Symbol::new(&env, "admin"))
            .unwrap();
        if admin != stored_admin {
            panic!("{:?}", EscrowError::OnlyAdminCanResolve);
        }

        let escrow_key = (Symbol::new(&env, "escrow"), escrow_id);
        let mut escrow: Escrow = env
            .storage()
            .persistent()
            .get(&escrow_key)
            .unwrap_or_else(|| panic!("{:?}", EscrowError::EscrowNotFound));

        if escrow.status != EscrowStatus::Dispute {
            panic!("{:?}", EscrowError::InvalidStatus);
        }

        if winner != escrow.buyer && winner != escrow.farmer {
            panic!("{:?}", EscrowError::InvalidWinner);
        }

        let token: Address = env
            .storage()
            .instance()
            .get(&Symbol::new(&env, "token"))
            .unwrap();
        let contract_addr = env.current_contract_address();
        let token_client = TokenClient::new(&env, &token);
        token_client.transfer(&contract_addr, &winner, &escrow.amount);

        escrow.status = EscrowStatus::Complete;
        env.storage().persistent().set(&escrow_key, &escrow);

        env.events().publish(
            (
                Symbol::new(&env, "escrow"),
                Symbol::new(&env, "dispute_resolved"),
            ),
            (escrow_id, winner),
        );
    }

    /// Get escrow details.
    pub fn get_escrow_details(env: Env, escrow_id: u32) -> Escrow {
        let escrow_key = (Symbol::new(&env, "escrow"), escrow_id);
        env.storage()
            .persistent()
            .get(&escrow_key)
            .unwrap_or_else(|| panic!("{:?}", EscrowError::EscrowNotFound))
    }
}

#[cfg(test)]
mod test;
