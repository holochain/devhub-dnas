use crate::hdi;

use hdi::prelude::*;
use crate::{
    EntityId, BundleAddr,
    Authority,
};



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
