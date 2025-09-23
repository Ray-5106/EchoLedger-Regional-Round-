use ic_cdk::{call, caller, Principal};
use ic_cdk_macros::{update, query, init};
use candid::{CandidType, Deserialize};
use serde::Serialize;
use std::collections::{BTreeMap, HashMap};
use std::cell::RefCell;

#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
pub struct OrganAvailability {
    pub organ_type: String,
    pub blood_type: String,
    pub hla_typing: Vec<String>,
    pub organ_condition: String,
    pub time_since_harvest: u64,
    pub location: String,
    pub viability_score: f32,
}

#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
pub struct RecipientMatch {
    pub recipient_id: String,
    pub organ: String,
    pub compatibility_score: f32,
    pub urgency_level: u8, // 1 = Critical, 2 = High, 3 = Medium
    pub distance_km: u32,
    pub transplant_center: String,
    pub notification_sent: bool,
    pub estimated_survival_benefit: f32,
}

#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
pub struct ExecutionResult {
    pub execution_id: String,
    pub patient_id: String,
    pub directives_executed: Vec<DirectiveExecution>,
    pub total_execution_time_ms: u64,
    pub blockchain_verification: String,
    pub audit_log_created: bool,
    pub compliance_verified: bool,
}

#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
pub struct DirectiveExecution {
    pub directive_type: String,
    pub execution_status: String,
    pub organs_processed: Vec<String>,
    pub recipient_matches: Vec<RecipientMatch>,
    pub total_recipients_notified: u32,
    pub estimated_lives_saved: u32,
    pub data_shared_with: Vec<String>,
    pub anonymization_verified: bool,
    pub research_impact_score: f32,
}

#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
pub struct OrganNetworkAlert {
    pub alert_id: String,
    pub network: String,
    pub transplant_center: String,
    pub organ: String,
    pub recipient: String,
    pub alert_time: String,
    pub delivery_status: String,
    pub response_time_ms: u32,
}

#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
pub struct FHIRPatientRecord {
    pub resource_type: String,
    pub id: String,
    pub active: bool,
    pub name: Vec<FHIRName>,
    pub gender: String,
    pub birth_date: String,
    pub medical_record_number: String,
}

#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
pub struct FHIRName {
    pub use_type: String,
    pub family: String,
    pub given: Vec<String>,
}

#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
pub struct DirectiveUpdate {
    pub directive_type: String,
    pub status: String,
    pub last_updated: u64,
    pub blockchain_reference: String,
}

thread_local! {
    static EXECUTION_HISTORY: RefCell<BTreeMap<String, ExecutionResult>> = RefCell::new(BTreeMap::new());
    static ORGAN_NETWORKS: RefCell<HashMap<String, Vec<String>>> = RefCell::new({
        let mut networks = HashMap::new();
        networks.insert("UNOS".to_string(), vec![
            "Mayo Clinic Transplant Center".to_string(),
            "Johns Hopkins Transplant Center".to_string(),
            "Cleveland Clinic".to_string(),
            "UCLA Medical Center".to_string(),
        ]);
        networks.insert("Eurotransplant".to_string(), vec![
            "Charité Berlin".to_string(),
            "University Hospital Zurich".to_string(),
            "Academic Medical Center Amsterdam".to_string(),
        ]);
        networks.insert("ANZOD".to_string(), vec![
            "Royal Melbourne Hospital".to_string(),
            "Sydney Children's Hospital".to_string(),
        ]);
        networks
    });
    static RESEARCH_INSTITUTIONS: RefCell<Vec<String>> = RefCell::new(vec![
        "National Cancer Institute".to_string(),
        "Memorial Sloan Kettering Cancer Center".to_string(),
        "MD Anderson Cancer Center".to_string(),
        "Dana-Farber Cancer Institute".to_string(),
        "Fred Hutchinson Cancer Research Center".to_string(),
    ]);
}

#[init]
fn init() {
    ic_cdk::println!("🤖 Executor AI initialized - Ready for autonomous directive execution");
}

// Main function for autonomous death directive execution
#[update]
async fn execute_death_directives(patient_id: String) -> Result<ExecutionResult, String> {
    let start_time = ic_cdk::api::time();
    let execution_id = format!("EXEC_{}_{}", patient_id, start_time);
    
    ic_cdk::println!("🚀 Starting autonomous execution for patient: {}", patient_id);
    
    // 1. Verify death certificate (simulated)
    let death_verified = verify_death_certificate(&patient_id).await?;
    if !death_verified {
        return Err("Death certificate verification failed".to_string());
    }
    
    // 2. Retrieve all patient directives
    let directives = get_all_patient_directives(&patient_id).await?;
    
    let mut executed_directives = Vec::new();
    
    // 3. Execute organ donation if consented
    if directives.contains(&"ORGAN_DONATION".to_string()) {
        let organ_execution = execute_organ_donation(&patient_id).await?;
        executed_directives.push(organ_execution);
    }
    
    // 4. Execute data sharing if consented
    if directives.contains(&"DATA_CONSENT".to_string()) {
        let data_execution = execute_data_sharing(&patient_id).await?;
        executed_directives.push(data_execution);
    }
    
    let total_execution_time = ((ic_cdk::api::time() - start_time) / 1_000_000) as u64; // Convert to ms
    
    // 5. Create execution result
    let execution_result = ExecutionResult {
        execution_id: execution_id.clone(),
        patient_id: patient_id.clone(),
        directives_executed: executed_directives,
        total_execution_time_ms: total_execution_time,
        blockchain_verification: format!("0x{:x}", ic_cdk::api::sha256(execution_id.as_bytes())[0..8].iter().fold(0u64, |acc, &b| acc << 8 | b as u64)),
        audit_log_created: true,
        compliance_verified: true,
    };
    
    // 6. Store execution result for audit
    EXECUTION_HISTORY.with(|history| {
        history.borrow_mut().insert(execution_id.clone(), execution_result.clone());
    });
    
    // 7. Create immutable audit log
    create_execution_audit_log(&patient_id, &execution_result).await?;
    
    ic_cdk::println!("✅ Autonomous execution completed: {} in {}ms", execution_id, total_execution_time);
    
    Ok(execution_result)
}

// Execute organ donation with network coordination
async fn execute_organ_donation(patient_id: &str) -> Result<DirectiveExecution, String> {
    ic_cdk::println!("🫀 Executing organ donation for patient: {}", patient_id);
    
    // 1. Assess organ viability
    let available_organs = assess_organ_viability(patient_id).await?;
    
    // 2. Find optimal recipients
    let recipient_matches = find_optimal_recipients(&available_organs).await?;
    
    // 3. Send notifications to transplant centers
    let mut notification_count = 0;
    let mut updated_matches = Vec::new();
    
    for mut recipient_match in recipient_matches {
        let notification_result = notify_transplant_center(&recipient_match).await;
        recipient_match.notification_sent = notification_result.is_ok();
        if recipient_match.notification_sent {
            notification_count += 1;
        }
        updated_matches.push(recipient_match);
    }
    
    // 4. Calculate estimated lives saved
    let estimated_lives_saved = updated_matches.iter()
        .filter(|m| m.notification_sent && m.urgency_level <= 2)
        .count() as u32;
    
    Ok(DirectiveExecution {
        directive_type: "ORGAN_DONATION".to_string(),
        execution_status: "COMPLETED".to_string(),
        organs_processed: available_organs.iter().map(|o| o.organ_type.clone()).collect(),
        recipient_matches: updated_matches,
        total_recipients_notified: notification_count,
        estimated_lives_saved,
        data_shared_with: vec![],
        anonymization_verified: true,
        research_impact_score: 0.0,
    })
}

// Execute data sharing for research
async fn execute_data_sharing(patient_id: &str) -> Result<DirectiveExecution, String> {
    ic_cdk::println!("📊 Executing data sharing for patient: {}", patient_id);
    
    // 1. Anonymize patient data
    let anonymized_data = anonymize_patient_data(patient_id).await?;
    
    // 2. Share with consented research institutions
    let research_institutions = RESEARCH_INSTITUTIONS.with(|institutions| {
        institutions.borrow().clone()
    });
    
    // 3. Calculate research impact score
    let research_impact_score = calculate_research_impact(&anonymized_data);
    
    Ok(DirectiveExecution {
        directive_type: "DATA_CONSENT".to_string(),
        execution_status: "COMPLETED".to_string(),
        organs_processed: vec![],
        recipient_matches: vec![],
        total_recipients_notified: 0,
        estimated_lives_saved: 0,
        data_shared_with: research_institutions,
        anonymization_verified: true,
        research_impact_score,
    })
}

// Assess organ viability for donation
async fn assess_organ_viability(patient_id: &str) -> Result<Vec<OrganAvailability>, String> {
    // Simulate organ assessment based on patient data
    let organs = vec![
        OrganAvailability {
            organ_type: "kidney_left".to_string(),
            blood_type: "O+".to_string(),
            hla_typing: vec!["A*02:01".to_string(), "B*07:02".to_string()],
            organ_condition: "Excellent".to_string(),
            time_since_harvest: 0,
            location: "Mayo Clinic".to_string(),
            viability_score: 0.95,
        },
        OrganAvailability {
            organ_type: "kidney_right".to_string(),
            blood_type: "O+".to_string(),
            hla_typing: vec!["A*02:01".to_string(), "B*07:02".to_string()],
            organ_condition: "Excellent".to_string(),
            time_since_harvest: 0,
            location: "Mayo Clinic".to_string(),
            viability_score: 0.94,
        },
        OrganAvailability {
            organ_type: "liver".to_string(),
            blood_type: "O+".to_string(),
            hla_typing: vec!["A*02:01".to_string(), "B*07:02".to_string()],
            organ_condition: "Good".to_string(),
            time_since_harvest: 0,
            location: "Mayo Clinic".to_string(),
            viability_score: 0.91,
        },
        OrganAvailability {
            organ_type: "corneas".to_string(),
            blood_type: "O+".to_string(),
            hla_typing: vec![],
            organ_condition: "Excellent".to_string(),
            time_since_harvest: 0,
            location: "Mayo Clinic".to_string(),
            viability_score: 0.98,
        },
    ];
    
    ic_cdk::println!("🔬 Assessed {} organs for patient: {}", organs.len(), patient_id);
    Ok(organs)
}

// Find optimal recipients using AI matching
async fn find_optimal_recipients(available_organs: &[OrganAvailability]) -> Result<Vec<RecipientMatch>, String> {
    let mut matches = Vec::new();
    
    for organ in available_organs {
        match organ.organ_type.as_str() {
            "kidney_left" => {
                matches.push(RecipientMatch {
                    recipient_id: "R_001_kidney".to_string(),
                    organ: organ.organ_type.clone(),
                    compatibility_score: 0.97,
                    urgency_level: 1,
                    distance_km: 45,
                    transplant_center: "Mayo Clinic Transplant Center".to_string(),
                    notification_sent: false,
                    estimated_survival_benefit: 0.92,
                });
            },
            "kidney_right" => {
                matches.push(RecipientMatch {
                    recipient_id: "R_002_kidney".to_string(),
                    organ: organ.organ_type.clone(),
                    compatibility_score: 0.94,
                    urgency_level: 1,
                    distance_km: 78,
                    transplant_center: "Johns Hopkins Transplant Center".to_string(),
                    notification_sent: false,
                    estimated_survival_benefit: 0.89,
                });
            },
            "liver" => {
                matches.push(RecipientMatch {
                    recipient_id: "R_003_liver".to_string(),
                    organ: organ.organ_type.clone(),
                    compatibility_score: 0.91,
                    urgency_level: 2,
                    distance_km: 120,
                    transplant_center: "Cleveland Clinic".to_string(),
                    notification_sent: false,
                    estimated_survival_benefit: 0.85,
                });
            },
            "corneas" => {
                matches.push(RecipientMatch {
                    recipient_id: "R_004_corneas".to_string(),
                    organ: organ.organ_type.clone(),
                    compatibility_score: 0.99,
                    urgency_level: 3,
                    distance_km: 25,
                    transplant_center: "Mayo Clinic Eye Center".to_string(),
                    notification_sent: false,
                    estimated_survival_benefit: 0.95,
                });
            },
            _ => {}
        }
    }
    
    // Sort by compatibility score and urgency
    matches.sort_by(|a, b| {
        (b.compatibility_score * (4 - b.urgency_level) as f32)
            .partial_cmp(&(a.compatibility_score * (4 - a.urgency_level) as f32))
            .unwrap_or(std::cmp::Ordering::Equal)
    });
    
    Ok(matches)
}

// Notify transplant centers
async fn notify_transplant_center(recipient_match: &RecipientMatch) -> Result<(), String> {
    ic_cdk::println!(
        "🚨 ORGAN AVAILABLE: Center: {} - Recipient: {} - Organ: {} - Compatibility: {:.2}",
        recipient_match.transplant_center,
        recipient_match.recipient_id,
        recipient_match.organ,
        recipient_match.compatibility_score
    );
    
    // In a real implementation, this would send actual notifications
    // via secure channels to the transplant centers
    
    Ok(())
}

// Get organ network alerts for monitoring
#[query]
fn get_organ_network_alerts(execution_id: String) -> Result<Vec<OrganNetworkAlert>, String> {
    // Return mock alerts for demo purposes
    Ok(vec![
        OrganNetworkAlert {
            alert_id: "ALERT_kidney_left_001".to_string(),
            network: "UNOS".to_string(),
            transplant_center: "Mayo Clinic Transplant Center".to_string(),
            organ: "kidney_left".to_string(),
            recipient: "R_001_kidney".to_string(),
            alert_time: "2024-12-21T02:31:15Z".to_string(),
            delivery_status: "DELIVERED".to_string(),
            response_time_ms: 234,
        },
        OrganNetworkAlert {
            alert_id: "ALERT_kidney_right_002".to_string(),
            network: "UNOS".to_string(),
            transplant_center: "Johns Hopkins Transplant Center".to_string(),
            organ: "kidney_right".to_string(),
            recipient: "R_002_kidney".to_string(),
            alert_time: "2024-12-21T02:31:16Z".to_string(),
            delivery_status: "DELIVERED".to_string(),
            response_time_ms: 189,
        },
        OrganNetworkAlert {
            alert_id: "ALERT_liver_003".to_string(),
            network: "UNOS".to_string(),
            transplant_center: "Cleveland Clinic".to_string(),
            organ: "liver".to_string(),
            recipient: "R_003_liver".to_string(),
            alert_time: "2024-12-21T02:31:17Z".to_string(),
            delivery_status: "DELIVERED".to_string(),
            response_time_ms: 156,
        },
    ])
}

// EHR Integration functions
async fn fetch_patient_emergency_data(
    patient_id: &str,
    ehr_system: &str,
    emergency_token: &str
) -> Result<FHIRPatientRecord, String> {
    ic_cdk::println!(
        "🏥 Fetching emergency data: Patient {} from {} using token {}",
        patient_id, ehr_system, emergency_token
    );
    
    // Mock FHIR patient record
    Ok(FHIRPatientRecord {
        resource_type: "Patient".to_string(),
        id: patient_id.to_string(),
        active: true,
        name: vec![FHIRName {
            use_type: "official".to_string(),
            family: "Emergency".to_string(),
            given: vec!["Patient".to_string()],
        }],
        gender: "unknown".to_string(),
        birth_date: "1980-01-01".to_string(),
        medical_record_number: format!("MRN_{}", patient_id),
    })
}

async fn update_directive_in_ehr(
    patient_id: &str,
    directive_update: &DirectiveUpdate,
    ehr_system: &str
) -> Result<(), String> {
    ic_cdk::println!(
        "📋 EHR Update: Patient {} - System {} - Directive {} - Status {}",
        patient_id,
        ehr_system,
        directive_update.directive_type,
        directive_update.status
    );
    
    Ok(())
}

// Helper functions
async fn verify_death_certificate(patient_id: &str) -> Result<bool, String> {
    ic_cdk::println!("📜 Verifying death certificate for patient: {}", patient_id);
    // In a real implementation, this would verify with official death registries
    Ok(true)
}

async fn get_all_patient_directives(patient_id: &str) -> Result<Vec<String>, String> {
    ic_cdk::println!("📋 Retrieving all directives for patient: {}", patient_id);
    // Mock directives for demo
    Ok(vec!["ORGAN_DONATION".to_string(), "DATA_CONSENT".to_string()])
}

async fn anonymize_patient_data(patient_id: &str) -> Result<String, String> {
    ic_cdk::println!("🔒 Anonymizing data for patient: {}", patient_id);
    // Create anonymized data hash
    let anonymized_hash = format!("ANON_{:x}", ic_cdk::api::sha256(patient_id.as_bytes())[0..8].iter().fold(0u64, |acc, &b| acc << 8 | b as u64));
    Ok(anonymized_hash)
}

fn calculate_research_impact(anonymized_data: &str) -> f32 {
    // Calculate research impact score based on data quality and relevance
    0.88 // Mock score
}

async fn create_execution_audit_log(
    patient_id: &str,
    execution_result: &ExecutionResult
) -> Result<(), String> {
    ic_cdk::println!(
        "📝 AUDIT: Execution completed - Patient: {} - Execution ID: {} - Time: {} - Lives saved: {}",
        patient_id,
        execution_result.execution_id,
        execution_result.total_execution_time_ms,
        execution_result.directives_executed.iter().map(|d| d.estimated_lives_saved).sum::<u32>()
    );
    
    Ok(())
}

// Query functions for monitoring
#[query]
fn get_execution_history() -> Vec<ExecutionResult> {
    EXECUTION_HISTORY.with(|history| {
        history.borrow().values().cloned().collect()
    })
}

#[query]
fn get_supported_organ_networks() -> Vec<String> {
    ORGAN_NETWORKS.with(|networks| {
        networks.borrow().keys().cloned().collect()
    })
}

#[query]
fn get_research_institutions() -> Vec<String> {
    RESEARCH_INSTITUTIONS.with(|institutions| {
        institutions.borrow().clone()
    })
}