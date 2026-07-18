use soroban_sdk::contracttype;

#[contracttype]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum VoteData {
    Null = 0,
    Accept = 1,
    Reject = 2,
    Undecided = 3,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ProposalData {
    pub is_challenged: bool,
    pub proposal_id: u32,
    pub title: soroban_sdk::String,
    pub description: soroban_sdk::String,
    pub created_at: u64,
    pub ends_at: u64,
    pub required_votes: i128,
    pub proposer: soroban_sdk::Address,
    pub executed: bool,
    pub accept_votes: i128,
    pub reject_votes: i128,
    pub undecided_votes: i128,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Votes {
    pub proposal_id: u32,
    pub voter: soroban_sdk::Address,
    pub voting_power: i128,
    pub vote_type: VoteData,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ChallengeData {
    pub proposal_id: u32,
    pub description: soroban_sdk::String,
    pub resolved: bool,
    pub valid: bool,
    pub challenger: soroban_sdk::Address,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DisputeData {
    pub challenge_id: u32,
    pub arbitrator: soroban_sdk::Address,
    pub resolved: bool,
    pub ruling: bool,
}
