use crate::hdi;

use hdi::prelude::*;
pub use crate::holochain_types::{
    AppManifestV1,
    ResourceMap,
};



//
// App Entry
//
#[hdk_entry_helper]
#[derive(Clone)]
pub struct AppEntry {
    pub manifest: AppManifestV1,
    pub resources: ResourceMap,
}
