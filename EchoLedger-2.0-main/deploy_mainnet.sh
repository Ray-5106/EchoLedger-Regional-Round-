#!/bin/bash

echo "🚀 Deploying EchoLedger to ICP Mainnet"

# Set environment
dfx identity use echoledger
export NETWORK=ic

# Deploy canisters with sufficient cycles
dfx deploy --network $NETWORK emergency_bridge --with-cycles 1000000000000
dfx deploy --network $NETWORK directive_manager --with-cycles 1000000000000
dfx deploy --network $NETWORK executor_ai --with-cycles 2000000000000

# Get canister IDs
echo "📦 Canister IDs:"
echo "Emergency Bridge: $(dfx canister id emergency_bridge --network $NETWORK)"
echo "Directive Manager: $(dfx canister id directive_manager --network $NETWORK)"
echo "Executor AI: $(dfx canister id executor_ai --network $NETWORK)"

# Verify deployment
echo "✅ Verification:"
dfx canister call emergency_bridge health_check --network $NETWORK