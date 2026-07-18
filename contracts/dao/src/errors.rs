use soroban_sdk::contracterror;

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum DaoError {
    AlreadyInitialized = 1,
    NotInitialized = 2,
    NoTokenLocked = 3,
    CannotDelegateToSelf = 4,
    AlreadyDelegated = 5,
    NotDelegated = 6,
    AlreadyVoted = 7,
    VotesNotTallied = 8,
    ProposalNotEnded = 9,
    ProposalAlreadyExecuted = 10,
    ChallengeAlreadyResolved = 11,
    DisputeAlreadyResolved = 12,
    InvalidArbitrator = 13,
    ChallengeNotFound = 14,
    DisputeNotFound = 15,
    ProposalNotFound = 16,
    DescriptionCannotBeEmpty = 17,
    TransferFailed = 18,
    InsufficientVotes = 19,
    NotAdmin = 20,
}
