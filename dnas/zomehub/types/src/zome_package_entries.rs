use std::collections::BTreeMap;
use hdi::prelude::*;


#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Maintainer {
    Group(ActionHash),
    Agent(AgentPubKey),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Authority {
    Group(ActionHash, ActionHash),
    Agent(AgentPubKey),
}



//
// Integrity Zome Package Version Entry
//
#[hdk_entry_helper]
#[derive(Clone)]
pub struct IntegrityZomePackageEntry {
    pub name: String,
    pub description: String,
    pub maintainer: Maintainer,

    // optional
    pub tags: Option<Vec<String>>,

    // Common fields
    pub metadata: BTreeMap<String, rmpv::Value>,
}



//
// Integrity Zome Package Version Entry
//
#[hdk_entry_helper]
#[derive(Clone)]
pub struct IntegrityZomePackageVersionEntry {
    pub version: String,
    pub wasm_entry: EntryHash,
    pub source_code_url: Option<String>,
    pub for_package: EntryHash,
    pub authority: Authority,
}
