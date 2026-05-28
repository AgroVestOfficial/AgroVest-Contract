#!/bin/bash
set -euo pipefail

# AgroVest Contract Deployment Script
# Usage: ./scripts/deploy.sh --network testnet --source <secret-key>

NETWORK="${NETWORK:-testnet}"
SOURCE="${SOURCE:-}"

while [[ $# -gt 0 ]]; do
    case "$1" in
        --network) NETWORK="$2"; shift 2 ;;
        --source) SOURCE="$2"; shift 2 ;;
        *) echo "Unknown arg: $1"; exit 1 ;;
    esac
done

if [ -z "$SOURCE" ]; then
    echo "Error: --source <secret-key> is required"
    exit 1
fi

echo "Building contracts..."
make build

WASM_DIR="target/wasm32-unknown-unknown/release"

echo "Deploying to $NETWORK..."

# Deploy Escrow
echo "Deploying FarmEscrow..."
ESCROW_ID=$(soroban contract deploy \
    --wasm "$WASM_DIR/agrovest_escrow.wasm" \
    --source "$SOURCE" \
    --network "$NETWORK")
echo "FarmEscrow deployed: $ESCROW_ID"

# Deploy Investment
echo "Deploying Investment..."
INVESTMENT_ID=$(soroban contract deploy \
    --wasm "$WASM_DIR/agrovest_investment.wasm" \
    --source "$SOURCE" \
    --network "$NETWORK")
echo "Investment deployed: $INVESTMENT_ID"

# Deploy Farm
echo "Deploying Farm..."
FARM_ID=$(soroban contract deploy \
    --wasm "$WASM_DIR/agrovest_farm.wasm" \
    --source "$SOURCE" \
    --network "$NETWORK")
echo "Farm deployed: $FARM_ID"

# Deploy DAO
echo "Deploying DAO..."
DAO_ID=$(soroban contract deploy \
    --wasm "$WASM_DIR/agrovest_dao.wasm" \
    --source "$SOURCE" \
    --network "$NETWORK")
echo "DAO deployed: $DAO_ID"

echo ""
echo "All contracts deployed!"
echo "  FarmEscrow: $ESCROW_ID"
echo "  Investment: $INVESTMENT_ID"
echo "  Farm:       $FARM_ID"
echo "  DAO:        $DAO_ID"
echo ""
echo "Next: Initialize each contract with the appropriate token/escrow addresses."
