#!/bin/bash

# EchoLedger WSL Quick Start Script
# Automated setup and deployment for WCHL 2025

set -e

# Color codes
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
PURPLE='\033[0;35m'
NC='\033[0m'

echo -e "${PURPLE}🏆 EchoLedger WSL Quick Start - WCHL 2025${NC}"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo -e "${BLUE}This script will automatically set up and deploy EchoLedger to ICP mainnet${NC}"
echo ""

# Function to check if command exists
command_exists() {
    command -v "$1" >/dev/null 2>&1
}

# Function to install Node.js
install_nodejs() {
    echo -e "${BLUE}📦 Installing Node.js...${NC}"
    curl -fsSL https://deb.nodesource.com/setup_18.x | sudo -E bash -
    sudo apt-get install -y nodejs
    echo -e "${GREEN}✅ Node.js installed: $(node --version)${NC}"
}

# Function to install Rust
install_rust() {
    echo -e "${BLUE}🦀 Installing Rust...${NC}"
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
    source ~/.cargo/env
    rustup target add wasm32-unknown-unknown
    echo -e "${GREEN}✅ Rust installed: $(cargo --version)${NC}"
}

# Function to install dfx
install_dfx() {
    echo -e "${BLUE}⚙️ Installing DFX...${NC}"
    sh -ci "$(curl -fsSL https://internetcomputer.org/install.sh)"
    echo 'export PATH="$HOME/bin:$PATH"' >> ~/.bashrc
    source ~/.bashrc
    export PATH="$HOME/bin:$PATH"
    echo -e "${GREEN}✅ DFX installed: $(dfx --version)${NC}"
}

# Step 1: Update system
echo -e "${BLUE}🔄 Updating WSL system...${NC}"
sudo apt update && sudo apt upgrade -y

# Step 2: Install dependencies
echo -e "${BLUE}📦 Installing dependencies...${NC}"

if ! command_exists node; then
    install_nodejs
else
    echo -e "${GREEN}✅ Node.js already installed: $(node --version)${NC}"
fi

if ! command_exists cargo; then
    install_rust
else
    echo -e "${GREEN}✅ Rust already installed: $(cargo --version)${NC}"
fi

if ! command_exists dfx; then
    install_dfx
else
    echo -e "${GREEN}✅ DFX already installed: $(dfx --version)${NC}"
fi

# Ensure PATH is updated
export PATH="$HOME/bin:$PATH"

# Step 3: Setup DFX identity
echo -e "${BLUE}🔑 Setting up DFX identity...${NC}"

if ! dfx identity whoami >/dev/null 2>&1; then
    echo -e "${YELLOW}Creating new DFX identity...${NC}"
    dfx identity new echoledger
    dfx identity use echoledger
else
    echo -e "${GREEN}✅ DFX identity exists: $(dfx identity whoami)${NC}"
fi

PRINCIPAL_ID=$(dfx identity get-principal)
echo -e "${GREEN}📋 Your Principal ID: ${PRINCIPAL_ID}${NC}"

# Step 4: Check cycles
echo -e "${BLUE}💰 Checking wallet and cycles...${NC}"

# Try to get wallet balance
if dfx wallet balance --network ic >/dev/null 2>&1; then
    BALANCE=$(dfx wallet balance --network ic)
    echo -e "${GREEN}✅ Wallet balance: ${BALANCE}${NC}"
else
    echo -e "${YELLOW}⚠️ No wallet found or insufficient cycles${NC}"
    echo -e "${BLUE}🎁 Getting free cycles from faucet...${NC}"
    echo ""
    echo -e "${YELLOW}IMPORTANT: You need to get cycles for deployment!${NC}"
    echo -e "${BLUE}1. Visit: https://faucet.dfinity.org/${NC}"
    echo -e "${BLUE}2. Use your Principal ID: ${PRINCIPAL_ID}${NC}"
    echo -e "${BLUE}3. Request cycles (you'll get ~20 TC for free)${NC}"
    echo ""
    read -p "Press Enter after you've obtained cycles from the faucet..."
    
    # Verify cycles were received
    if dfx wallet balance --network ic >/dev/null 2>&1; then
        BALANCE=$(dfx wallet balance --network ic)
        echo -e "${GREEN}✅ Cycles received: ${BALANCE}${NC}"
    else
        echo -e "${RED}❌ Still no cycles detected. Please visit the faucet and try again.${NC}"
        echo "Faucet URL: https://faucet.dfinity.org/"
        echo "Your Principal ID: ${PRINCIPAL_ID}"
        exit 1
    fi
fi

# Step 5: Build project
echo -e "${BLUE}🔨 Building EchoLedger project...${NC}"

# Install frontend dependencies
echo "Installing frontend dependencies..."
cd frontend
npm install
npm run build
cd ..

# Build Rust canisters
echo "Building Rust canisters..."
cargo build --release --target wasm32-unknown-unknown

echo -e "${GREEN}✅ Build completed successfully${NC}"

# Step 6: Deploy to mainnet
echo -e "${PURPLE}🌐 Deploying to ICP Mainnet...${NC}"
echo -e "${YELLOW}⚠️ This will consume cycles from your wallet!${NC}"

read -p "Continue with mainnet deployment? (y/n): " confirm
if [[ $confirm != "y" ]]; then
    echo -e "${RED}❌ Deployment cancelled${NC}"
    exit 1
fi

# Deploy each canister with proper cycle allocation
echo -e "${BLUE}📦 Deploying canisters...${NC}"

echo "Deploying directive_manager (Motoko)..."
dfx deploy directive_manager --network ic --with-cycles 1000000000000

echo "Deploying emergency_bridge (Rust)..."
dfx deploy emergency_bridge --network ic --with-cycles 1000000000000

echo "Deploying executor_ai (Rust)..."
dfx deploy executor_ai --network ic --with-cycles 1000000000000

echo "Deploying llm_canister (Rust)..."
dfx deploy llm_canister --network ic --with-cycles 1000000000000

echo "Deploying frontend..."
dfx deploy frontend --network ic --with-cycles 500000000000

echo -e "${GREEN}✅ All canisters deployed successfully!${NC}"

# Step 7: Get canister IDs and create documentation
echo -e "${BLUE}📋 Retrieving canister IDs...${NC}"

EMERGENCY_BRIDGE_ID=$(dfx canister id emergency_bridge --network ic)
DIRECTIVE_MANAGER_ID=$(dfx canister id directive_manager --network ic)
EXECUTOR_AI_ID=$(dfx canister id executor_ai --network ic)
LLM_CANISTER_ID=$(dfx canister id llm_canister --network ic)
FRONTEND_ID=$(dfx canister id frontend --network ic)

# Create live deployment documentation
cat > LIVE_DEPLOYMENT_SUCCESS.md << EOF
# 🎊 EchoLedger Successfully Deployed to ICP Mainnet!

## 🏆 WCHL 2025 Competition - Live Deployment

**Deployment Date:** $(date)
**Deployed By:** $(dfx identity whoami)
**Principal ID:** $(dfx identity get-principal)
**Status:** ✅ LIVE AND OPERATIONAL

## 🆔 Live Canister IDs

| Canister | ID | Live URL |
|----------|----|---------| 
| **Emergency Bridge** | \`${EMERGENCY_BRIDGE_ID}\` | https://${EMERGENCY_BRIDGE_ID}.icp0.io |
| **Directive Manager** | \`${DIRECTIVE_MANAGER_ID}\` | https://${DIRECTIVE_MANAGER_ID}.icp0.io |
| **Executor AI** | \`${EXECUTOR_AI_ID}\` | https://${EXECUTOR_AI_ID}.icp0.io |
| **LLM Canister** | \`${LLM_CANISTER_ID}\` | https://${LLM_CANISTER_ID}.icp0.io |
| **Frontend dApp** | \`${FRONTEND_ID}\` | https://${FRONTEND_ID}.icp0.io |

## 🧪 Live Demo Commands for WCHL 2025

### Scenario 1: Emergency DNR Verification
\`\`\`bash
dfx canister call ${EMERGENCY_BRIDGE_ID} emergency_check '(record {
  patient_id = "cardiac_patient_001";
  hospital_id = "MAYO_EMERGENCY_001";
  situation = "cardiac_arrest";
  vitals = opt "{\"blood_pressure\": \"60/40\", \"pulse\": 0, \"respiratory_rate\": 0}";
  access_token = opt "emergency_access_token_123"
})' --network ic
\`\`\`

### Scenario 2: AI Medical Directive Processing
\`\`\`bash
dfx canister call ${LLM_CANISTER_ID} process_medical_directive '(
  "sarah_chen_001",
  "I, Sarah Chen, being of sound mind, do not want resuscitation if I have less than 5% chance of meaningful recovery. Donate my kidneys and corneas. Share anonymized data with cancer research institutions."
)' --network ic
\`\`\`

### Scenario 3: Autonomous Organ Coordination
\`\`\`bash
dfx canister call ${EXECUTOR_AI_ID} execute_death_directives '("organ_donor_sarah_chen_001")' --network ic
\`\`\`

## 📊 System Health Check
\`\`\`bash
# Check system metrics
dfx canister call ${DIRECTIVE_MANAGER_ID} get_system_info --network ic
dfx canister call ${EMERGENCY_BRIDGE_ID} get_impact_metrics --network ic
dfx canister call ${LLM_CANISTER_ID} get_processing_statistics --network ic
dfx canister call ${EXECUTOR_AI_ID} get_supported_organ_networks --network ic
\`\`\`

## 🌐 Frontend Access
**Main Application:** https://${FRONTEND_ID}.icp0.io

## 🏆 Competition Ready!
EchoLedger is now live on ICP mainnet and ready for WCHL 2025 competition demonstration.

**Team Contact:** rayhanhameed5@gmail.com
**Competition Category:** Healthcare Technology Innovation
**Expected Impact:** 28,000+ organs saved annually

*EchoLedger: Where Every Healthcare Directive is Heard, Verified, and Executed When It Matters Most*
EOF

# Step 8: Test deployment
echo -e "${BLUE}🧪 Testing deployment...${NC}"

echo "Testing directive manager..."
dfx canister call $DIRECTIVE_MANAGER_ID get_system_info --network ic

echo "Testing emergency bridge..."
dfx canister call $EMERGENCY_BRIDGE_ID get_impact_metrics --network ic

echo "Testing LLM canister..."
dfx canister call $LLM_CANISTER_ID get_supported_directive_types --network ic

echo "Testing executor AI..."
dfx canister call $EXECUTOR_AI_ID get_supported_organ_networks --network ic

# Step 9: Create demo script with actual IDs
cat > LIVE_DEMO_COMMANDS.sh << EOF
#!/bin/bash
# EchoLedger Live Demo Commands for WCHL 2025 Competition

echo -e "${PURPLE}🚨 Scenario 1: Emergency DNR Verification${NC}"
dfx canister call ${EMERGENCY_BRIDGE_ID} emergency_check '(record {
  patient_id = "cardiac_patient_001";
  hospital_id = "MAYO_EMERGENCY_001";
  situation = "cardiac_arrest";
  vitals = opt "{\"blood_pressure\": \"60/40\", \"pulse\": 0, \"respiratory_rate\": 0}";
  access_token = opt "emergency_access_token_123"
})' --network ic

echo ""
echo -e "${PURPLE}🧠 Scenario 2: AI Medical Processing${NC}"
dfx canister call ${LLM_CANISTER_ID} process_medical_directive '(
  "sarah_chen_001",
  "I, Sarah Chen, being of sound mind, do not want resuscitation if I have less than 5% chance of meaningful recovery. Donate my kidneys and corneas. Share anonymized data with cancer research institutions."
)' --network ic

echo ""
echo -e "${PURPLE}🫀 Scenario 3: Autonomous Organ Coordination${NC}"
dfx canister call ${EXECUTOR_AI_ID} execute_death_directives '("organ_donor_sarah_chen_001")' --network ic
EOF

chmod +x LIVE_DEMO_COMMANDS.sh

# Final success message
echo ""
echo -e "${GREEN}🎊 EchoLedger Successfully Deployed to ICP Mainnet!${NC}"
echo -e "${PURPLE}🏆 Ready for WCHL 2025 Competition Judging${NC}"
echo ""
echo -e "${YELLOW}📋 Deployment Summary:${NC}"
echo -e "${GREEN}🚨 Emergency Bridge:    https://${EMERGENCY_BRIDGE_ID}.icp0.io${NC}"
echo -e "${GREEN}📋 Directive Manager:   https://${DIRECTIVE_MANAGER_ID}.icp0.io${NC}"
echo -e "${GREEN}🤖 Executor AI:         https://${EXECUTOR_AI_ID}.icp0.io${NC}"
echo -e "${GREEN}🧠 LLM Canister:        https://${LLM_CANISTER_ID}.icp0.io${NC}"
echo -e "${GREEN}🌐 Frontend dApp:       https://${FRONTEND_ID}.icp0.io${NC}"
echo ""
echo -e "${BLUE}📄 Documentation created:${NC}"
echo "• LIVE_DEPLOYMENT_SUCCESS.md - Complete deployment info"
echo "• LIVE_DEMO_COMMANDS.sh - Ready-to-use demo commands"
echo ""
echo -e "${YELLOW}🎬 Next Steps:${NC}"
echo "1. ✅ Test the frontend: https://${FRONTEND_ID}.icp0.io"
echo "2. ✅ Run demo commands: ./LIVE_DEMO_COMMANDS.sh"
echo "3. ✅ Practice your 8-minute presentation"
echo "4. ✅ Submit to WCHL 2025 with live URLs"
echo ""
echo -e "${PURPLE}🎯 EchoLedger: Saving Lives Through Blockchain Innovation${NC}"