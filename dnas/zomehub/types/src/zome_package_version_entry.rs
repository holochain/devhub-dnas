use crate::{
    hdi,
    EntityId,
};

use std::collections::BTreeMap;
use hdi::prelude::*;


#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct ApiCompatibilityBuiltWith {
    pub hdi_version: String,
    pub hdk_version: Option<String>, // Only required for coordinator zomes
}

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct ApiCompatibility {
    pub build_with: ApiCompatibilityBuiltWith,
    pub tested_with: String,
}


//
// Zome Package Version Entry
//
#[hdk_entry_helper]
#[derive(Clone)]
pub struct ZomePackageVersionEntry {
    pub for_package: EntityId,
    pub zome_entry: EntryHash,

    // Optional
    pub changelog: Option<String>,
    pub source_code_revision_uri: Option<String>,
    pub api_compatibility: ApiCompatibility,

    // Common fields
    pub metadata: BTreeMap<String, rmpv::Value>,
}