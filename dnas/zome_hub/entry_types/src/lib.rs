pub use hdi;

use std::collections::BTreeMap;
use hdi::prelude::*;


//
// Wasm Entry
//
#[hdk_entry_helper]
#[derive(Clone)]
pub struct WasmEntry {
    pub author: AgentPubKey,
    pub mere_memory_addr: EntryHash,
    // pub mere_memory_hash: String,

    // Common fields
    pub published_at: u64,
    pub last_updated: u64,
    pub metadata: BTreeMap<String, rmpv::Value>,
}


//
// Zome Entry
//
#[hdk_entry_helper]
#[derive(Clone)]
pub struct ZomeEntry {
    pub name: String,
    pub zome_type: u8,
    pub description: String,
    pub published_at: u64,
    pub last_updated: u64,
    pub developer: AgentPubKey,
    pub metadata: BTreeMap<String, rmpv::Value>,

    // optional
    pub display_name: Option<String>,
    pub tags: Option<Vec<String>>,
    pub source_code_url: Option<String>,
    // pub deprecation: Option<DeprecationNotice>,
}



//
// Zome Version Entry
//
#[hdk_entry_helper]
#[derive(Clone, PartialEq)]
pub struct ZomeVersionEntry {
    pub for_zome: ActionHash,
    pub version: String,
    pub ordering: u64,
    // pub properties: Option<serde_yaml::Value>,
    pub published_at: u64,
    pub last_updated: u64,
    pub changelog: String,
    pub mere_memory_addr: EntryHash,
    pub mere_memory_hash: String,
    pub hdk_version: String,
    pub metadata: BTreeMap<String, rmpv::Value>,

    // optional
    pub review_summary: Option<ActionHash>,
    pub source_code_commit_url: Option<String>,
}
