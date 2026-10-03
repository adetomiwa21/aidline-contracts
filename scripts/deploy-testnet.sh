#!/usr/bin/env bash
# Builds and deploys the Aidline contract to Stellar testnet.
#
# Usage: ./scripts/deploy-testnet.sh <identity> [token_contract_id]
#
# <identity> must exist in the Stellar CLI keystore. It becomes the admin and
# is registered as the first verifier so you can test the full flow.
# If no token is given, the native XLM asset contract is used.
set -euo pipefail

IDENTITY="${1:?usage: deploy-testnet.sh <identity> [token_contract_id]}"
NETWORK=testnet

ADMIN=$(stellar keys address "$IDENTITY")
TOKEN="${2:-$(stellar contract id asset --asset native --network "$NETWORK")}"

echo "Building contract..."
stellar contract build

echo "Deploying with admin $ADMIN and token $TOKEN..."
CONTRACT_ID=$(stellar contract deploy \
  --wasm target/wasm32v1-none/release/aidline.wasm \
  --source "$IDENTITY" \
  --network "$NETWORK" \
  -- \
  --admin "$ADMIN" \
  --token "$TOKEN")

echo "Registering $ADMIN as a verifier..."
stellar contract invoke --id "$CONTRACT_ID" --source "$IDENTITY" --network "$NETWORK" \
  -- add_verifier --verifier "$ADMIN" >/dev/null

cat > .env.testnet <<ENV
AIDLINE_CONTRACT_ID=$CONTRACT_ID
AIDLINE_TOKEN_ID=$TOKEN
AIDLINE_ADMIN=$ADMIN
STELLAR_NETWORK=$NETWORK
ENV

echo
echo "Deployed: $CONTRACT_ID"
echo "Saved to .env.testnet"
