# AgroVest-Contract Agent Guide

## What this is

Stellar Soroban smart contracts (Rust, `no_std`). Four contracts in a Cargo workspace:

- **farm** — marketplace (register farms, list products, cart, purchases, reviews)
- **escrow** — buyer-farmer escrow with dispute resolution
- **investment** — time-locked farm investment opportunities
- **dao** — governance (proposals, voting, delegation, challenges, disputes)

## Build & test commands

```bash
# Build WASM artifacts (NOT cargo build)
soroban contract build

# Run all tests
cargo test --workspace

# Single contract tests
cargo test -p agrovest-farm
cargo test -p agrovest-escrow
cargo test -p agrovest-investment
cargo test -p agrovest-dao

# CI checks (must all pass)
cargo fmt --all -- --check
cargo clippy --workspace -- -D warnings

# Fix formatting
cargo fmt --all

# Makefile shortcut for full check
make check   # runs fmt-check + clippy + test
```

## Prerequisites

- Rust stable + `wasm32-unknown-unknown` target: `rustup target add wasm32-unknown-unknown`
- Soroban CLI: `cargo install --locked soroban-cli`

## Contract structure (each contract follows this)

```
contracts/<name>/src/
├── lib.rs       — #[contract] struct + #[contractimpl] methods
├── types.rs     — data structs with #[contracttype]
├── errors.rs    — error enum with #[contracterror]
├── storage.rs   — documentation of storage key patterns
└── test.rs      — unit tests (included via `#[cfg(test)] mod test;` at bottom of lib.rs)
```

## Soroban patterns (not obvious from Rust conventions)

- **Auth**: Every mutating fn takes `caller: Address` and calls `caller.require_auth()`. There is no `msg.sender`.
- **Storage backends**: `instance` (contract config/admin), `persistent` (core state), `temporary` (ephemeral, e.g. carts).
- **Storage keys**: Inline `Symbol::new(&env, "key")` or tuple composites like `(Symbol::new(&env, "farmer"), address)`. No enum-based keys.
- **No custom token**: Uses Stellar Asset Contract (SAC) for the AVT token. Token transfers would call SAC `transfer_from`/`transfer`.
- **Events**: `env.events().publish((namespace_symbol, event_symbol), data_tuple)`.
- **Errors**: Contracts `panic!("{:?}", ErrorVariant)` rather than returning `Result`. Error enums use `#[contracterror]` with `#[repr(u32)]`.
- **Strings**: Use `soroban_sdk::String`, not `std::string::String` (this is `no_std`).
- **Testing**: `env.mock_all_auths()` to skip auth in tests. `Address::generate(&env)` for test addresses. Generated `<Contract>Client` (e.g. `FarmContractClient`) wraps contract calls.
- **Test snapshots**: Soroban generates `test_snapshots/` JSON files alongside tests. These are auto-generated; commit them but don't edit manually.

## WASM output

Built artifacts go to `target/wasm32-unknown-unknown/release/` with names like `agrovest_farm.wasm` (underscores, not hyphens).

## Deploy

```bash
./scripts/deploy.sh --network testnet --source <secret-key>
# Or via env vars: NETWORK=testnet SOURCE=<secret-key> ./scripts/deploy.sh
```

Each contract must be initialized after deployment with appropriate token/escrow addresses.
