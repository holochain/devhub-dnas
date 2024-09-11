use crate::{
    hdi,
    ZomeType,
    Authority,
};
use std::collections::BTreeMap;
use hdi::prelude::*;



//
// Zome Package Entry
//
#[hdk_entry_helper]
#[derive(Clone)]
pub struct ZomePackageEntry {
    pub name: String,
    pub title: String,
    pub description: String,
    pub zome_type: ZomeType,
    pub maintainer: Authority,

    // optional
    pub tags: Option<Vec<String>>,

    // Common fields
    pub metadata: BTreeMap<String, rmpv::Value>,
}
