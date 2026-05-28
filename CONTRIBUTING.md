# Contributing to AgroVest Contracts

Thank you for your interest in contributing to AgroVest! This document provides guidelines and steps for contributing.

## Getting Started

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) (stable)
- [Soroban CLI](https://soroban.stellar.org/docs/getting-started/setup#install-soroban-cli)
- Git

### Setup

```bash
# Clone the repository
git clone https://github.com/AgroVestOfficial/AgroVest-Contract.git
cd AgroVest-Contract

# Add the wasm target
rustup target add wasm32-unknown-unknown

# Build contracts
soroban contract build

# Run tests
cargo test --workspace
```

## Development Workflow

1. Fork the repository
2. Create a feature branch from `main`
3. Make your changes
4. Ensure all checks pass:
   ```bash
   make check  # runs fmt, clippy, and tests
   ```
5. Commit your changes with a clear message
6. Push to your fork and open a Pull Request

## Code Standards

- **Formatting**: Run `cargo fmt --all` before committing
- **Linting**: All code must pass `cargo clippy --workspace -- -D warnings`
- **Tests**: Add tests for new functionality. All existing tests must pass
- **No warnings**: The build must be warning-free

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

## Adding a New Contract

1. Create a new directory under `contracts/`
2. Add a `Cargo.toml` following the existing pattern
3. Add the crate to the workspace `Cargo.toml` members list
4. Implement the contract following the existing patterns

## Reporting Issues

- Use the [Bug Report](https://github.com/AgroVestOfficial/AgroVest-Contract/issues/new?template=bug_report.md) template for bugs
- Use the [Feature Request](https://github.com/AgroVestOfficial/AgroVest-Contract/issues/new?template=feature_request.md) template for new ideas

## Pull Request Guidelines

- Keep PRs focused on a single change
- Include tests for new functionality
- Update documentation if needed
- Reference related issues in the PR description
- Ensure CI passes before requesting review

## License

By contributing, you agree that your contributions will be licensed under the MIT License.
