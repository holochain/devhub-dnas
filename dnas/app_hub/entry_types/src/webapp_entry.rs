use crate::hdi;

use std::collections::BTreeMap;
use hdi::prelude::*;
pub use crate::holochain_types::{
    WebAppManifestV1,
    ResourceMap,
};
use crate::{
    EntityId, BundleAddr, MemoryAddr,
    Authority,
    DeprecationNotice,
};


//
// WebApp Entry
//
#[hdk_entry_helper]
#[derive(Clone)]
pub struct WebAppEntry {
    pub manifest: WebAppManifestV1,
    pub resources: ResourceMap,
}



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



//
// WebApp Package Version Entry
//
#[hdk_entry_helper]
#[derive(Clone)]
pub struct WebAppPackageVersionEntry {
    // Context
    pub for_package: EntityId,
    pub maintainer: Authority,

    // Properties
    /// Pointer to the uploaded bundle WebAppEntry
    pub webapp: BundleAddr,

    // Optional
    pub source_code_url: Option<String>,
}
