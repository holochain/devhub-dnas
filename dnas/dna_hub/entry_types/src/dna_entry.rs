use crate::hdi;

use std::{
    collections::BTreeMap,
    path::PathBuf,
};
use hdi::prelude::*;
pub use crate::holochain_types::{
    DnaManifestV1,
};


pub type ResourceMap = BTreeMap<PathBuf, ActionHash>;


//
// Dna Entry
//
#[hdk_entry_helper]
#[derive(Clone)]
pub struct DnaEntry {
    pub manifest: DnaManifestV1,
    pub resources: ResourceMap,
}
