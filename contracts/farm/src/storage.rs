// Farm Contract Storage Key Documentation
// ========================================
//
// All storage keys are created inline in lib.rs using Symbol::new(&env, "key").
// This file documents the key patterns used throughout the contract.
//
// Instance storage (config/admin):
//   "admin"       => Address
//   "token"       => Address
//   "escrow"      => Address
//   "farm_ctr"    => u32
//   "prod_ctr"    => u32
//   "total_sales" => i128
//
// Persistent storage (composite keys as tuples):
//   ("farmer", Address)              => Farmer
//   ("farm_id", u32)                 => Farmer
//   ("f_n", String)                  => u32 (farm_id)
//   ("f_a", Address)                 => u32 (farm_id)
//   ("prod", Address, u32)           => FarmProduct (owner, local_idx)
//   ("prod_g", u32)                  => (Address, u32) (global_id => owner, local_idx)
//   ("purch", Address, u32)          => bool (buyer, product_id)
//   ("p_prod", Address, u32)         => FarmProduct (buyer, index)
//   ("p_cnt", Address)               => u32
//   ("rev", u32, u32)                => Review (product_id, review_idx)
//   ("rev_c", u32)                   => u32
//   ("has_rev", u32, Address)        => bool
//
// Temporary storage (cart, TTL ~24h):
//   ("cart", Address, u32)           => FarmProduct (buyer, cart_idx)
//   ("cart_c", Address)              => u32
