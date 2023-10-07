use crate::hdi;

use hdi::prelude::*;
pub use crate::holochain_types::{
    WebAppManifestV1,
    ResourceMap,
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
