
# 🚀 EchoLedger WSL Deployment Guide - Step by Step
# //THIS WAS A CHECKLIST USED BY MY TEAM// #
# //NOT FOR SUBMISSION// #

## 📋 Complete Guide: From WSL Setup to ICP Mainnet Deployment

This guide provides detailed step-by-step instructions for deploying EchoLedger to ICP mainnet using Windows Subsystem for Linux (WSL).

---

## 🔧 Step 1: WSL Setup and Prerequisites

### **1.1 Install WSL (if not already installed)**
```powershell
# Run in Windows PowerShell as Administrator
wsl --install
# Restart your computer when prompted
```

### **1.2 Open WSL Terminal**
```bash
# Open WSL Ubuntu terminal
# You should see something like: user@DESKTOP-XXXXX:~$
```

### **1.3 Update WSL System**
```bash
sudo apt update && sudo apt upgrade -y
```

---

## 🛠️ Step 2: Install Required Dependencies

### **2.1 Install Node.js and npm**
```bash
# Install Node.js 18.x (required for dfx)
curl -fsSL https://deb.nodesource.com/setup_18.x | sudo -E bash -
sudo apt-get install -y nodejs

# Verify installation
node --version  # Should show v18.x.x
npm --version   # Should show 9.x.x or higher
```

### **2.2 Install Rust**
```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Source the environment
source ~/.cargo/env

# Add WebAssembly target
rustup target add wasm32-unknown-unknown

# Verify installation
cargo --version  # Should show cargo 1.x.x
rustc --version  # Should show rustc 1.x.x
```

### **2.3 Install DFX (DFINITY Canister SDK)**
```bash
# Install dfx
sh -ci "$(curl -fsSL https://internetcomputer.org/install.sh)"

# Add dfx to PATH
echo 'export PATH="$HOME/bin:$PATH"' >> ~/.bashrc
source ~/.bashrc

# Verify installation
dfx --version  # Should show dfx 0.15.x or higher
```

---

## 📁 Step 3: Navigate to EchoLedger Project

### **3.1 Navigate to Project Directory**
```bash
# Navigate to your EchoLedger project
# Replace with your actual path - the Windows path is accessible via /mnt/c/
cd /mnt/c/Users/YuvanShankar/Downloads/EchoLedger-2.0-main/EchoLedger-2.0-main

# Verify you're in the right directory
ls -la
# You should see: dfx.json, Cargo.toml, src/, frontend/, etc.
```

### **3.2 Set Permissions for Scripts**
```bash
# Make deployment script executable
chmod +x deploy_enhanced.sh
chmod +x deploy_mainnet.sh

# Verify permissions
ls -la *.sh
```

---

## 🔑 Step 4: Setup DFX Identity and Wallet

### **4.1 Create DFX Identity**
```bash
# Create a new identity for EchoLedger
dfx identity new echoledger
dfx identity use echoledger

# Verify identity
dfx identity whoami
# Should show: echoledger
```

### **4.2 Get Your Principal ID**
```bash
# Get your principal ID (you'll need this)
dfx identity get-principal
# Save this principal ID - you'll need it for cycles
```

### **4.3 Get Cycles for Deployment**

**Option A: Use Cycles Faucet (Recommended for Testing)**
```bash
# Visit the cycles faucet in your browser
echo "Visit: https://faucet.dfinity.org/"
echo "Use your principal ID: $(dfx identity get-principal)"
# Follow the instructions on the website to get free cycles
```

**Option B: Convert ICP to Cycles**
```bash
# If you have ICP tokens, you can convert them to cycles
# First, get your account ID
dfx ledger account-id

# Check your ICP balance
dfx ledger balance

# Create a canister with cycles (requires ICP)
dfx ledger create-canister --amount 10.0
```

### **4.4 Verify Wallet Setup**
```bash
# Check your wallet balance
dfx wallet balance --network ic
# Should show something like: 15.000 TC (trillion cycles)
```

---

## 🏗️ Step 5: Build the Project

### **5.1 Install Frontend Dependencies**
```bash
# Navigate to frontend directory
cd frontend

# Install npm dependencies
npm install

# Build frontend
npm run build

# Go back to root directory
cd ..
```

### **5.2 Build Rust Canisters**
```bash
# Build all Rust canisters
cargo build --release --target wasm32-unknown-unknown

# Verify build success
ls -la target/wasm32-unknown-unknown/release/
# Should see: emergency_bridge.wasm, executor_ai.wasm, llm_canister.wasm
```

### **5.3 Test Local Deployment (Optional but Recommended)**
```bash
# Start local dfx replica
dfx start --clean --background

# Deploy locally first to test
dfx deploy --network local

# Test basic functionality
dfx canister call directive_manager get_system_info --network local

# Stop local replica
dfx stop
```

---

## 🌐 Step 6: Deploy to ICP Mainnet

### **6.1 Run Enhanced Deployment Script**
```bash
# Make sure you're in the EchoLedger-2.0-main directory
pwd
# Should show: /mnt/c/Users/YuvanShankar/Downloads/EchoLedger-2.0-main/EchoLedger-2.0-main

# Run the enhanced deployment script
./deploy_enhanced.sh
```

### **6.2 Follow Deployment Prompts**
The script will ask several questions:

```bash
# Question 1: Test local deployment first?
# Answer: n (we already tested if you did step 5.3)

# Question 2: Continue with mainnet deployment?
# Answer: y

# The script will then:
# 1. Check prerequisites ✅
# 2. Verify wallet balance ✅
# 3. Build all canisters ✅
# 4. Deploy to mainnet ✅
# 5. Get canister IDs ✅
# 6. Test functionality ✅
```

### **6.3 Expected Output**
```bash
🎉 EchoLedger 2.0 Live on ICP Mainnet:
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
🚨 Emergency Bridge:    https://[CANISTER-ID].icp0.io
📋 Directive Manager:   https://[CANISTER-ID].icp0.io
🤖 Executor AI:         https://[CANISTER-ID].icp0.io
🧠 LLM Canister:        https://[CANISTER-ID].icp0.io
🌐 Frontend dApp:       https://[CANISTER-ID].icp0.io
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
```

---

## 🧪 Step 7: Test Live Deployment

### **7.1 Test Emergency Scenario**
```bash
# Replace [EMERGENCY-BRIDGE-ID] with your actual canister ID
dfx canister call [EMERGENCY-BRIDGE-ID] emergency_check '(record {
  patient_id = "cardiac_patient_001";
  hospital_id = "MAYO_EMERGENCY_001";
  situation = "cardiac_arrest";
  vitals = opt "{\"blood_pressure\": \"60/40\", \"pulse\": 0, \"respiratory_rate\": 0}";
  access_token = opt "emergency_access_token_123"
})' --network ic
```

### **7.2 Test AI Processing**
```bash
# Replace [LLM-CANISTER-ID] with your actual canister ID
dfx canister call [LLM-CANISTER-ID] process_medical_directive '(
  "sarah_chen_001",
  "I, Sarah Chen, being of sound mind, do not want resuscitation if I have less than 5% chance of meaningful recovery. Donate my kidneys and corneas. Share anonymized data with cancer research institutions."
)' --network ic
```

### **7.3 Test Organ Coordination**
```bash
# Replace [EXECUTOR-AI-ID] with your actual canister ID
dfx canister call [EXECUTOR-AI-ID] execute_death_directives '("organ_donor_sarah_chen_001")' --network ic
```

---

## 🔍 Step 8: Verify Deployment Success

### **8.1 Check Canister Status**
```bash
# Check all canister statuses
dfx canister status directive_manager --network ic
dfx canister status emergency_bridge --network ic
dfx canister status executor_ai --network ic
dfx canister status llm_canister --network ic
```

### **8.2 Check Cycle Balances**
```bash
# Verify canisters have sufficient cycles
dfx canister status directive_manager --network ic | grep "Balance"
dfx canister status emergency_bridge --network ic | grep "Balance"
dfx canister status executor_ai --network ic | grep "Balance"
dfx canister status llm_canister --network ic | grep "Balance"
```

### **8.3 Test Frontend Access**
```bash
# Get frontend URL
dfx canister id frontend --network ic
echo "Frontend URL: https://$(dfx canister id frontend --network ic).icp0.io"

# Open in browser to test Internet Identity login
```

---

## 🚨 Troubleshooting Common Issues

### **Issue 1: "No wallet configured"**
```bash
# Solution: Set up wallet
dfx identity deploy-wallet --network ic
dfx wallet balance --network ic
```

### **Issue 2: "Insufficient cycles"**
```bash
# Solution: Get more cycles from faucet or convert ICP
# Visit: https://faucet.dfinity.org/
# Or: dfx ledger create-canister --amount 5.0
```

### **Issue 3: "Build failed"**
```bash
# Solution: Check Rust installation
rustup target add wasm32-unknown-unknown
cargo clean
cargo build --release --target wasm32-unknown-unknown
```

### **Issue 4: "Canister not found"**
```bash
# Solution: Verify canister was deployed
dfx canister id [CANISTER_NAME] --network ic
dfx canister status [CANISTER_NAME] --network ic
```

### **Issue 5: "Frontend build failed"**
```bash
# Solution: Install frontend dependencies
cd frontend
npm install
npm run build
cd ..
```

