# AgroVest Contracts

Stellar Soroban smart contracts for the AgroVest agricultural investment platform.

## Contracts

- **Farm** — Marketplace for farmers to register businesses, list products, and for buyers to purchase and review
- **FarmEscrow** — Escrow for buyer-farmer transactions with dispute resolution
- **Investment** — Farm investment opportunities with time-locked funding
- **DAO** — Governance with token locking, proposals, voting, delegation, challenges, and disputes

## Build

```bash
# Install Soroban CLI
cargo install --locked soroban-cli

# Build all contracts
soroban contract build
```

## Test

```bash
cargo test --workspace
```

## Deploy

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

## Token

The AVT token uses Stellar's native Asset Contract (SAC). Create the asset on Stellar and pass the SAC contract address to downstream contracts during initialization.
