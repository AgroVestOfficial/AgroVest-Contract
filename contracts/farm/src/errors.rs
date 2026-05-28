use soroban_sdk::contracterror;

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum FarmError {
    NameCannotBeEmpty = 1,
    NameAlreadyRegistered = 2,
    InvalidFarmIndex = 3,
    NotRegistered = 4,
    FarmDoesNotBelongToYou = 5,
    FarmNotFound = 6,
    InvalidProductIndex = 7,
    ProductDoesNotExist = 8,
    OnlyBuyersCanReview = 9,
    AlreadyReviewed = 10,
    AlreadyPurchased = 11,
    ProductAlreadySold = 12,
    PriceMismatch = 13,
    ProductNotInCart = 14,
    AlreadyInitialized = 15,
    NotInitialized = 16,
}
