use crate::hdi;

use std::collections::BTreeMap;
use hdi::prelude::*;
use crate::{
    MemoryAddr,
    Authority,
    DeprecationNotice,
};



//
// WebApp Package Entry
//
#[hdk_entry_helper]
#[derive(Clone)]
pub struct WebAppPackageEntry {
    /// Public facing common name for this app
    pub title: String,
    /// Public facing subtitle for this app
    pub subtitle: String,
    /// Information regarding the purpose and usage of this package
    pub description: String,
    /// Authority for modifying this entry
    pub maintainer: Authority,
    /// Mere Memory address of image bytes
    pub icon: MemoryAddr,
    /// Link to project code repository
    pub source_code_url: Option<String>,

    // State
    /// Set when this package has been deprecated
    pub deprecation: Option<DeprecationNotice>,

    // Common fields
    pub metadata: BTreeMap<String, rmpv::Value>,
}
