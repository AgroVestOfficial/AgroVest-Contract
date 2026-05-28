use soroban_sdk::contracttype;

#[contracttype]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum EscrowStatus {
    AwaitingDelivery = 0,
    AwaitingApproval = 1,
    Complete = 2,
    Dispute = 3,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Escrow {
    pub buyer: soroban_sdk::Address,
    pub farmer: soroban_sdk::Address,
    pub amount: i128,
    pub status: EscrowStatus,
    pub order_id: u32,
}
