use candid::{CandidType, Deserialize};
use ic_cdk::api::time;
use serde::Serialize;
use std::collections::BTreeMap;

#[derive(CandidType, Deserialize, Serialize, Clone, Debug)]
pub struct PHIMetadata {
    pub patient_id_hash: Vec<u8>,
    pub directive_type: String,
    pub version: u64,
    pub created_at: u64,
    pub updated_at: u64,
    pub off_chain_ref: String,
    pub retention_period: u64,
}

#[derive(CandidType, Deserialize, Serialize, Clone, Debug)]
pub struct ConsentDirective {
    pub patient_id: String,
    pub directive_type: String,
    pub status: String,
    pub consent_items: Vec<String>,
    pub timestamp: u64,
    pub signature: Vec<u8>,
}

thread_local! {
    static PHI_METADATA: std::cell::RefCell<BTreeMap<Vec<u8>, PHIMetadata>> = 
        std::cell::RefCell::new(BTreeMap::new());
    
    static CONSENT_DIRECTIVES: std::cell::RefCell<BTreeMap<String, ConsentDirective>> = 
        std::cell::RefCell::new(BTreeMap::new());
}

#[ic_cdk::update]
async fn store_directive_metadata(metadata: PHIMetadata) -> Result<(), String> {
    if metadata.retention_period > 50 * 365 * 24 * 60 * 60 * 1000 {
        return Err("Retention period exceeds HIPAA limits".to_string());
    }

    PHI_METADATA.with(|phi_map| {
        phi_map.borrow_mut().insert(metadata.patient_id_hash.clone(), metadata);
    });

    Ok(())
}

#[ic_cdk::update]
fn update_consent_directive(directive: ConsentDirective) -> Result<(), String> {
    CONSENT_DIRECTIVES.with(|directives| {
        directives.borrow_mut().insert(directive.patient_id.clone(), directive);
    });

    Ok(())
}

#[ic_cdk::query]
fn get_consent_status(patient_id: String) -> Option<ConsentDirective> {
    CONSENT_DIRECTIVES.with(|directives| {
        directives.borrow().get(&patient_id).cloned()
    })
}