pub use dnahub_types;
pub use zomehub_sdk;
pub use zomehub_sdk::zomehub_types;
pub use devhub_sdk;
pub use devhub_sdk::*;

use std::collections::BTreeMap;
use hdk::prelude::*;
use hdk_extensions::{
    must_get,
    hdi_extensions::{
        guest_error,
    },
};
use serde_bytes::*;
use zomehub_sdk::{
    ZomeAsset,
};
use dnahub_types::{
    RmpvValue,
    DnaEntry,
    DnaToken,
    DnaManifestV1,
    DnaAssetHashes,
    ResourcesMap,
};


pub type IntegritiesTokenInput = Vec<(String, ByteBuf)>;
pub type CoordinatorsTokenInput = Vec<(String, ByteBuf)>;


#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct DnaTokenInput {
    pub integrity_hash: ByteBuf,
    pub integrities_token_hash: ByteBuf,
    pub coordinators_token_hash: ByteBuf,
}

impl From<DnaTokenInput> for DnaToken {
    fn from(dna_token_input: DnaTokenInput) -> Self {
        Self {
            integrity_hash: dna_token_input.integrity_hash.to_vec(),
            integrities_token_hash: dna_token_input.integrities_token_hash.to_vec(),
            coordinators_token_hash: dna_token_input.coordinators_token_hash.to_vec(),
        }
    }
}


#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct DnaEntryInput {
    pub manifest: RmpvValue,
    pub resources: ResourcesMap,
    pub dna_token: DnaTokenInput,
    pub integrities_token: IntegritiesTokenInput,
    pub coordinators_token: CoordinatorsTokenInput,
    pub claimed_file_size: u64,
    pub asset_hashes: DnaAssetHashes,
}

impl From<DnaEntryInput> for DnaEntry {
    fn from(dna_entry_input: DnaEntryInput) -> Self {
        Self {
            manifest: dna_entry_input.manifest,
            resources: dna_entry_input.resources,
            dna_token: dna_entry_input.dna_token.into(),
            integrities_token: dna_entry_input.integrities_token.into_iter()
                .map( |(zome_name, bytes_input)| (zome_name, bytes_input.to_vec()) )
                .collect(),
            coordinators_token: dna_entry_input.coordinators_token.into_iter()
                .map( |(zome_name, bytes_input)| (zome_name, bytes_input.to_vec()) )
                .collect(),
            claimed_file_size: dna_entry_input.claimed_file_size,
            asset_hashes: dna_entry_input.asset_hashes,
        }
    }
}


#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CreateDnaInput {
    pub manifest: RmpvValue,
    pub resources: ResourcesMap,
    pub claimed_file_size: u64,
    pub asset_hashes: DnaAssetHashes,
}

impl CreateDnaInput {
    pub fn deserialized_manifest(&self) -> ExternResult<DnaManifestV1> {
        DnaEntry::deserialize_manifest( &self.manifest )
    }
}

impl TryFrom<CreateDnaInput> for DnaEntry {
    type Error = WasmError;

    fn try_from(create_dna_input: CreateDnaInput) -> ExternResult<Self> {
        let manifest = create_dna_input.deserialized_manifest()?;

        let dna_token = manifest.dna_token( &create_dna_input.asset_hashes )?;
        let integrities_token = manifest.integrities_token( &create_dna_input.asset_hashes )?;
        let coordinators_token = manifest.coordinators_token()?;

        Ok(
            Self {
                manifest: create_dna_input.manifest,
                resources: create_dna_input.resources,
                dna_token,
                integrities_token,
                coordinators_token,
                claimed_file_size: create_dna_input.claimed_file_size,
                asset_hashes: create_dna_input.asset_hashes,
            }
        )
    }
}


pub type ZomeAssetMap = BTreeMap<ZomeName, ZomeAsset>;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DnaAsset {
    pub dna_entry: DnaEntry,
    pub zome_assets: ZomeAssetMap,
}

impl TryInto<DnaAsset> for EntryHash {
    type Error = WasmError;
    fn try_into(self) -> ExternResult<DnaAsset> {
        let dna_entry : DnaEntry = must_get( &self )?.try_into()?;
        let mut zome_assets = BTreeMap::new();

        for zome_manifest in dna_entry.deserialized_manifest()?.integrity.zomes.iter() {
            let hrl = dna_entry.resources.get( &zome_manifest.bundled )
                .ok_or(guest_error!(format!(
                    "DnaEntry does not have resources with path '{}'",
                    zome_manifest.bundled,
                )))?;
            let zome_asset : ZomeAsset = call_cell(
                hrl.dna.clone(),
                "zomehub_csr",
                "get_zome_asset",
                hrl.target.clone(),
                (),
            )?;

            zome_assets.insert( zome_manifest.name.clone(), zome_asset );
        }

        for zome_manifest in dna_entry.deserialized_manifest()?.coordinator.zomes.iter() {
            let hrl = dna_entry.resources.get( &zome_manifest.bundled )
                .ok_or(guest_error!(format!(
                    "DnaEntry does not have resources with path '{}'",
                    zome_manifest.bundled,
                )))?;
            let zome_asset : ZomeAsset = call_cell(
                hrl.dna.clone(),
                "zomehub_csr",
                "get_zome_asset",
                hrl.target.clone(),
                (),
            )?;

            zome_assets.insert( zome_manifest.name.clone(), zome_asset );
        }

        Ok(
            DnaAsset {
                dna_entry,
                zome_assets,
            }
        )
    }
}
