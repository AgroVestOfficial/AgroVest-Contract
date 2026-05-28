# AgroVest Contracts

[![CI](https://github.com/AgroVestOfficial/AgroVest-Contract/actions/workflows/ci.yml/badge.svg)](https://github.com/AgroVestOfficial/AgroVest-Contract/actions/workflows/ci.yml)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](LICENSE)
[![Soroban](https://img.shields.io/badge/Soroban-v22-blue.svg)](https://soroban.stellar.org)

Stellar Soroban smart contracts for the AgroVest agricultural investment platform.

## Contracts

| Contract | Description | Functions |
|----------|-------------|-----------|
| **Farm** | Marketplace for farmers to register businesses, list products, and for buyers to purchase and review | 19 |
| **FarmEscrow** | Escrow for buyer-farmer transactions with dispute resolution | 6 |
| **Investment** | Farm investment opportunities with time-locked funding | 8 |
| **DAO** | Governance with token locking, proposals, voting, delegation, challenges, and disputes | 17 |

## Quick Start

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) (stable)
- [Soroban CLI](https://soroban.stellar.org/docs/getting-started/setup#install-soroban-cli)

### Build

```bash
# Install Soroban CLI
cargo install --locked soroban-cli

# Add WASM target
rustup target add wasm32-unknown-unknown

# Build all contracts
soroban contract build
```

### Test

```bash
cargo test --workspace
```

### Deploy

```bash
# Deploy to testnet
./scripts/deploy.sh --network testnet --source <your-secret-key>

# Or use Makefile
make build
```

## Project Structure

```
contracts/
├── farm/          # Marketplace contract
├── escrow/        # Escrow contract
├── investment/    # Investment contract
└── dao/           # DAO governance contract
```

Each contract follows this structure:
- `lib.rs` — Contract logic and public functions
- `types.rs` — Data structures (`#[contracttype]`)
- `errors.rs` — Error definitions (`#[contracterror]`)
- `storage.rs` — Storage key documentation
- `test.rs` — Unit tests

## Architecture

- **Token**: Uses Stellar's native Asset Contract (SAC) for the AVT token — no custom token contract needed
- **Payments**: XLM transfers via SAC `transfer_from`/`transfer`
- **Auth**: `require_auth()` replaces `msg.sender` — every mutating function takes an explicit `caller: Address`
- **Storage**: Soroban's three backends — `instance` (config), `persistent` (core state), `temporary` (ephemeral data like carts)

## Contributing

We welcome contributions! Please see [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines.

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

Please read our [Code of Conduct](CODE_OF_CONDUCT.md) before contributing.

## License

This project is licensed under the MIT License — see [LICENSE](LICENSE) for details.
