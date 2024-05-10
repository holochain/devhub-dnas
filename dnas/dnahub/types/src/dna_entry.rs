pub use crate::{
    hdi,
    hdi_extensions,
    DnaToken,
    IntegritiesToken,
    CoordinatorsToken,
    AssetHashes,
    ResourcesMap,
    holochain_types::{
        DnaManifestV1,
    },
};

use hdi::prelude::*;


#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct DnaAssetHashes {
    pub integrity: AssetHashes,
    pub coordinator: AssetHashes,
}


//
// Dna Entry
//
#[hdk_entry_helper]
#[derive(Clone)]
pub struct DnaEntry {
    pub manifest: DnaManifestV1,
    pub resources: ResourcesMap,
    pub dna_token: DnaToken,
    pub integrities_token: IntegritiesToken,
    pub coordinators_token: CoordinatorsToken,
    pub claimed_file_size: u64,
    pub asset_hashes: DnaAssetHashes,
}

impl DnaEntry {
    pub fn integrity_hash(&self) -> Vec<u8> {
        self.dna_token.integrity_hash.clone()
    }

    pub fn calculate_integrity_hash(&self) -> ExternResult<Vec<u8>> {
        self.manifest.integrity_hash()
    }
}
