use crate::{
    hdi,
    EntityId, BundleAddr,
    Authority,
    WebAppToken,
};

use hdi::prelude::*;



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
    /// Copy of [`WebAppToken`] from the [`WebAppEntry`]
    pub webapp_token: WebAppToken,

    // Optional
    pub source_code_revision_url: Option<String>,
}
