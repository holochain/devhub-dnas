use crate::hdi;

use hdi::prelude::*;
pub use crate::holochain_types::{
    DnaManifestV1,
    ResourceMap,
};



//
// Dna Entry
//
#[hdk_entry_helper]
#[derive(Clone)]
pub struct DnaEntry {
    pub manifest: DnaManifestV1,
    pub resources: ResourceMap,
}
