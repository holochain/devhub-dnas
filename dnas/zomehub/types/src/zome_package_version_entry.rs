use crate::{
    hdi,
    EntityId,
};

use std::collections::BTreeMap;
use hdi::prelude::*;



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

    // Common fields
    pub metadata: BTreeMap<String, rmpv::Value>,
}
