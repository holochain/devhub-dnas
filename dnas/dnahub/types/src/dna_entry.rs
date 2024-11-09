pub use crate::{
    hdi,
    hdi_extensions,
    hash,
    DnaToken,
    IntegritiesToken,
    CoordinatorsToken,
    AssetHashes,
    ResourcesMap,
    holochain_types::{
        DnaManifestV1,
    },
};

use std::io::Cursor;
use hdi::prelude::*;
use hdi_extensions::{
    guest_error,
};


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
    /// Original bundle manifest content
    pub manifest: rmpv::Value,
    /// HRLs for zome dependencies
    pub resources: ResourcesMap,
    /// Tokens for identifying the state of integrity and coordinator configurations
    pub dna_token: DnaToken,
    /// Map of integrity zomes and and their tokens (hash identifier)
    pub integrities_token: IntegritiesToken,
    /// Map of coordinator zomes and and their tokens (hash identifier)
    pub coordinators_token: CoordinatorsToken,
    /// DNA bundle size declared by creator
    pub claimed_file_size: u64,
    /// Map of zome (WASM) assets and their content hashes
    pub asset_hashes: DnaAssetHashes,
}

impl DnaEntry {
    pub fn deserialize_manifest(manifest: &rmpv::Value) -> ExternResult<DnaManifestV1> {
        let mut buf = Vec::new();
        rmpv::encode::write_value(&mut buf, manifest)
            .map_err(|e| guest_error!(format!(
                "Failed to encode manifest value: {:?}", e
            )))?;

        let cursor = Cursor::new(buf);
        rmp_serde::from_read(cursor)
            .map_err(|e| guest_error!(format!(
                "Failed to deserialize manifest: {:?}", e
            )))
    }

    pub fn deserialized_manifest(&self) -> ExternResult<DnaManifestV1> {
        DnaEntry::deserialize_manifest( &self.manifest )
    }

    pub fn integrity_hash(&self) -> Vec<u8> {
        self.dna_token.integrity_hash.clone()
    }

    pub fn calculate_integrity_hash(&self) -> ExternResult<Vec<u8>> {
        hash( &self.deserialized_manifest()?.integrity )
    }

    pub fn calc_dna_token(&self) -> ExternResult<DnaToken> {
        let manifest = self.deserialized_manifest()?;

        manifest.dna_token( &self.asset_hashes )
    }

    pub fn calc_integrities_token(&self) -> ExternResult<IntegritiesToken> {
        let manifest = self.deserialized_manifest()?;

        manifest.integrities_token( &self.asset_hashes )
    }

    pub fn calc_coordinators_token(&self) -> ExternResult<CoordinatorsToken> {
        let manifest = self.deserialized_manifest()?;

        manifest.coordinators_token()
    }
}
