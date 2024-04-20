use crate::{
    hdi,
    EntityId,
};
use hdi::prelude::*;


//
// Zome Package Version Entry
//
#[hdk_entry_helper]
#[derive(Clone)]
pub struct ZomePackageVersionEntry {
    pub for_package: EntityId,
    pub zome_entry: EntryHash,
}
