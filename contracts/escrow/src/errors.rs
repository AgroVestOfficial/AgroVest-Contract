use soroban_sdk::contracterror;

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum EscrowError {
    AlreadyInitialized = 1,
    NotInitialized = 2,
    EscrowNotFound = 3,
    InvalidStatus = 4,
    OnlyBuyerCanApprove = 5,
    OnlyParticipantsCanRaise = 6,
    OnlyAdminCanResolve = 7,
    TransferFailed = 8,
    AlreadyResolved = 9,
    InvalidWinner = 10,
    OnlyBuyerCanConfirm = 11,
}
