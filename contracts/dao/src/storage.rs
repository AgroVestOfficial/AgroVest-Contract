// Instance storage:
//   "admin"      => Address
//   "token"      => Address
//   "investment" => Address

// Persistent storage:
//   ("locked", Address)           => i128
//   ("prop", u32)                 => ProposalData
//   "prop_ctr"                    => u32
//   ("voted", u32, Address)       => bool
//   ("vote", u32, Address)        => Votes
//   ("total_v", u32)              => i128
//   ("tally", u32)                => bool
//   ("deleg", Address)            => Address
//   ("chall", u32)                => ChallengeData
//   "chall_ctr"                   => u32
//   ("disp", u32)                 => DisputeData
//   "disp_ctr"                    => u32
