.PHONY: build test fmt clippy clean deploy check

build:
	soroban contract build

test:
	cargo test --workspace

fmt:
	cargo fmt --all

fmt-check:
	cargo fmt --all -- --check

clippy:
	cargo clippy --workspace -- -D warnings

check: fmt-check clippy test

clean:
	cargo clean

deploy:
	@echo "Usage: NETWORK=testnet SOURCE=<secret-key> ./scripts/deploy.sh"
