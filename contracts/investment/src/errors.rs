use soroban_sdk::contracterror;

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum InvestmentError {
    AlreadyInitialized = 1,
    NotInitialized = 2,
    InvestmentNotFound = 3,
    InvestmentNotActive = 4,
    EndDateNotReached = 5,
    AmountBelowMinimum = 6,
    NotFarmOwner = 7,
    NothingToClaim = 8,
}
