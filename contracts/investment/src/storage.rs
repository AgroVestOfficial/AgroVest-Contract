// Instance storage:
//   "token"  => Address
//   "total"  => i128

// Persistent storage:
//   ("inv", u32)                => FarmInvestmentDetails
//   "inv_ctr"                   => u32
//   ("active", u32)             => bool
//   ("investor", u32, u32)      => Investor (farm_id, investor_idx)
//   "inv_count"                 => u32
//   ("f_inv", u32, u32)         => Investor (farm_id, local_idx)
//   ("f_inv_c", u32)            => u32
