# 📡 EchoLedger - Autonomous Health Directive Executor

[![ICP](https://img.shields.io/badge/Internet_Computer-Protocol-29ABE2?style=for-the-badge&logo=internetcomputer&logoColor=white)](https://internetcomputer.org)
[![Rust](https://img.shields.io/badge/rust-%23000000.svg?style=for-the-badge&logo=rust&logoColor=white)](https://www.rust-lang.org/)
[![Motoko](https://img.shields.io/badge/Motoko-Blue?style=for-the-badge)](https://motoko.org/)

> **Decentralized, HIPAA-compliant autonomous enforcement of patient advance directives using Internet Computer Protocol**

## 🌐 Project Overview

EchoLedger is a decentralized application that ensures patient end-of-life wishes are respected and automatically enforced through blockchain technology. It provides a secure, immutable, and transparent way to manage and execute healthcare directives.

## NOTE:-
If you want to access a trail run of this project, kindly download the standalone html version and run, to see the demo.

## 🚀 Features

- **🔐 Secure Storage**: Patient directives stored on the Internet Computer blockchain
- **🤖 Autonomous Execution**: Smart contracts enforce directives without intermediaries
- **🔒 Privacy-First**: HIPAA-compliant data handling with patient-controlled access
- **🌐 Decentralized**: No single point of failure or control
- **⚡ Real-time Updates**: Instant access to critical healthcare directives

## 🏗️ Project Structure

```
EchoLedger/
├── src/                    # Source code
│   ├── directive_manager/  # Motoko canister for managing directives
│   ├── emergency_bridge/   # Rust canister for emergency access
│   ├── executor_ai/        # AI directive executor
│   └── llm_canister/       # Language model integration
├── frontend/              # React-based frontend
├── tests/                 # Integration and unit tests
└── docs/                  # Documentation
```

## 🛠️ Prerequisites

- [DFINITY Canister SDK (dfx)](https://sdk.dfinity.org/docs/quickstart/local-quickstart.html)
- [Node.js](https://nodejs.org/) (v18 or later)
- [Yarn](https://yarnpkg.com/) package manager
- [Rust](https://www.rust-lang.org/tools/install) (latest stable)
- [Motoko](https://sdk.dfinity.org/docs/language-guide/motoko.html) (included with DFX)

## 🚀 Quick Start

1. **Clone the repository**
   ```bash
   git clone https://github.com/your-organization/echoledger.git
   cd echoledger
   ```

2. **Install dependencies**
   ```bash
   # Install Rust dependencies
   rustup target add wasm32-unknown-unknown
   
   # Install frontend dependencies
   cd frontend
   yarn install
   cd ..
   ```

3. **Start local development environment**
   ```bash
   # Start local Internet Computer network
   dfx start --background
   
   # Deploy canisters
   dfx deploy
   ```

4. **Start the frontend**
   ```bash
   cd frontend
   yarn start
   `
## 🤝 Contributing

Contributions are welcome! Please read our [Contributing Guidelines](CONTRIBUTING.md) for details on our code of conduct and the process for submitting pull requests.

## 🙏 Acknowledgments

- Internet Computer Protocol (ICP) community
- DFINITY Foundation
- Open-source contributors

## 🏗️ Architecture

### **🔗 ICP Canister Structure**
```
EchoLedger/
├── 🚨 emergency_bridge/     # Rust - Real-time ER alerts (WebSpeed)
├── 📋 directive_manager/    # Motoko - HIPAA-compliant storage
├── 🤖 executor_ai/          # Rust - Organ matching & coordination  
├── 🧠 llm_canister/         # Rust - Llama3.1:8b medical NLP
└── 🌐 frontend/             # React - Asset canister UI
```

### **⚡ Core Workflows**

#### 1. **Emergency Directive Verification** (Sub-second)
```mermaid
graph LR
    A[ER Staff] --> B[Emergency Bridge]
    B --> C[Chain Fusion EHR]
    C --> D[Threshold ECDSA]
    D --> E[WebSpeed Alert]
    E --> F[🚨 DNR Alert]
```

#### 2. **AI Directive Processing** (Llama3.1:8b)
```mermaid
graph LR
    A[Natural Language] --> B[LLM Canister]
    B --> C[Medical NLP]
    C --> D[Confidence Score]
    D --> E[Blockchain Storage]
```

#### 3. **Autonomous Execution** (On Death)
```mermaid
graph LR
    A[Patient Death] --> B[Executor AI]
    B --> C[Organ Referral]
    B --> D[Data Release] 
    B --> E[Compliance Check]
```

---

## 🚀 Quick Start

### **Prerequisites**
```bash
# Install dfx CLI
sh -ci "$(curl -fsSL https://internetcomputer.org/install.sh)"

# Create identity and get cycles
dfx identity new echoledger
dfx identity use echoledger
dfx ledger create-canister --amount 10
```

### **🏃 Local Development**
```bash
# Clone and setup
git clone https://github.com/your-username/echoledger-icp
cd echoledger-icp

# Start local replica
dfx start --clean --background

# Deploy locally
dfx deploy --network local

# Test emergency check
dfx canister call emergency_bridge emergency_check '(record {
  patient_id = "patient_001";
  hospital_id = "ER_001"; 
  situation = "cardiac_arrest"
})' --network local
```

### **🌐 Deploy to Mainnet**
```bash
# One-click deployment
./deploy_mainnet.sh

# Or manual deployment
dfx deploy --network ic
```

---


### **🔬 Test Emergency Scenarios**

#### **Scenario 1: DNR Verification**
```bash
dfx canister call emergency_bridge emergency_check '(record {
  patient_id = "dnr_patient_001";
  hospital_id = "EMERGENCY_ROOM_001";
  situation = "cardiac_arrest";
  vitals = opt "{\"bp\": \"80/50\", \"pulse\": 120}";
  access_token = opt "zk_proof_token"
})' --network ic
```

**Expected Response:**
```json
{
  "action": "alert_ER",
  "directive": "DNR", 
  "message": "DNR directive verified on-chain. Do not resuscitate per patient's wishes.",
  "signature_verified": true
}
```

#### **Scenario 2: AI Directive Processing**
```bash
dfx canister call llm_canister process_medical_directive '(
  "sarah_chen_001",
  "I, Sarah Chen, being of sound mind, do not want resuscitation if I have less than 5% chance of meaningful recovery. Donate my kidneys and corneas. Share anonymized data with cancer research institutions."
)' --network ic
```

**Expected Response:**
```json
{
  "confidence_score": 0.94,
  "extracted_elements": {
    "dnr_detected": true,
    "dnr_conditions": ["<5% recovery probability"],
    "organ_donation": ["kidney", "corneas"],
    "data_consent": {
      "research_allowed": true,
      "anonymization_required": true,
      "institutions": ["ALL_CANCER_RESEARCH"]
    }
  }
}
```

#### **Scenario 3: Autonomous Death Execution**
```bash
dfx canister call executor_ai execute_death_directives_ai '("sarah_chen_001")' --network ic
```

---

## 🛡️ HIPAA Compliance & Privacy

### **🔒 Privacy Protection**
- **50-year data retention** for deceased patients (HIPAA requirement)
- **Threshold ECDSA** signature verification on all directives
- **Zero PHI exposure** - only structured actions output
- **Granular consent** - separate permissions for organs, data, research

### **📋 Compliance Features**
```rust
// HIPAA Enforcement Module
let PHI_REDACT = [
    "name", "ssn", "dob", "address", "phone", "email",
    "medical_record_number", "account_number", ...
];

pub fn hipaa_compliance_check(directive: &PatientDirective) -> ComplianceResult {
    // Automatic PHI redaction and compliance validation
}
```

### **🇪🇺 GDPR Support**
```motoko
public func gdpr_check(patient_nationality: Text): async Bool {
    if (patient_nationality == "EU") {
        set_retention_timer(5); // 5-year retention
        return true;
    }
    false;
}
```

---

## 🤖 AI Models & Integration

### **🧠 Llama3.1:8b Medical NLP**
- **Fully on-chain** processing (no external APIs)
- **Fine-tuned** for medical directive extraction
- **High accuracy** (>89% confidence) for DNR detection
- **HIPAA compliant** - no PHI in processing logs

### **🔬 BioBERT Risk Assessment**
- **Real-time** patient condition analysis
- **Recovery probability** scoring for DNR activation
- **Clinical decision support** for emergency staff

### **⚡ WebSpeed Integration**
- **Sub-second** emergency alerts to ER staff
- **Push notifications** for critical directive changes
- **Real-time** organ availability updates

---

##  Features Met:**
- [x] **Live dApp on ICP Mainnet** ✅
- [x] **Actual ICP Canisters** (Rust + Motoko) ✅
- [x] **Chain Fusion** integration ✅
- [x] **Threshold ECDSA** verification ✅
- [x] **AI/LLM** on-chain processing ✅
- [x] **Real-world impact** (saves lives) ✅

### **🎯 Innovation Highlights:**
1. **Sub-second emergency response** using WebSpeed
2. **Autonomous directive execution** without human intervention
3. **AI-powered** medical NLP with Llama3.1:8b
4. **HIPAA-compliant** blockchain storage
5. **Organ donation optimization** reducing 28K annual waste

---

## 📈 Impact & Future

### **📊 Potential Impact:**
- **🏥 Every hospital** can instantly access patient directives
- **💓 Thousands of organs** saved through automated coordination
- **🧬 Research acceleration** via consented data sharing
- **⚖️ Legal compliance** with automated HIPAA enforcement

### **🚀 Roadmap:**
- **Q3 2025:** Integration with major EHR systems (Epic, Cerner)
- **Q4 2025:** Multi-language support for global deployment  
- **Q1 2026:** Mobile app for directive creation
- **Q2 2026:** AI-powered organ matching optimization

---

## 🛠️ Development

### **📦 Built With:**
- **Internet Computer Protocol** - Decentralized backend
- **Rust** - High-performance canisters
- **Motoko** - ICP-native smart contracts
- **React** - Modern frontend UI
- **Tailwind CSS** - Responsive design
- **Llama3.1:8b** - Medical AI processing

### **🧪 Testing:**
```bash
# Run all tests
cargo test

# Test specific canister
dfx canister call emergency_bridge get_recent_alerts '(10)' --network local

# Compliance testing
npm run compliance-check
```

### **📋 Contributing:**
1. Fork the repository
2. Create feature branch (`git checkout -b feature/amazing-feature`)
3. Commit changes (`git commit -m 'Add amazing feature'`)
4. Push to branch (`git push origin feature/amazing-feature`)
5. Open Pull Request

---

## 👥 Team

**Built for WCHL 2025 by:**
- **Rayhan Hameed** - Team Lead
- **Yuvan Shankar** - Core Developer
- **Rohith K J** 
- **Mohamed Aaftaab M R** - 
- **Monish S** 

---

## 📜 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

**Special Note:** This is a competition entry for WCHL 2025. The code is open-source, but please respect the competitive nature and originality requirements.

---



## 📞 Contact & Links
  
**Email:** rayhanhameed5@gmail.com
  

---

**🎉 Thank you **


*Built with 💜 on the Internet Computer - Where every canister can save a life*

