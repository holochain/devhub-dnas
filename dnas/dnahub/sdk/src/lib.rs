pub use dnahub_types;
pub use devhub_sdk;
pub use devhub_sdk::*;

use std::collections::BTreeMap;
use hdk::prelude::*;
use hdk_extensions::{
    must_get,
};
use serde_bytes::*;
use zomehub_sdk::{
    WasmPackage,
};
use dnahub_types::{
    DnaEntry,
    DnaToken,
    DnaManifestV1,
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
    pub manifest: DnaManifestV1,
    pub dna_token: DnaTokenInput,
    pub integrities_token: IntegritiesTokenInput,
    pub coordinators_token: CoordinatorsTokenInput,
    pub claimed_file_size: u64,
}

impl From<DnaEntryInput> for DnaEntry {
    fn from(dna_entry_input: DnaEntryInput) -> Self {
        Self {
            manifest: dna_entry_input.manifest,
            dna_token: dna_entry_input.dna_token.into(),
            integrities_token: dna_entry_input.integrities_token.into_iter()
                .map( |(zome_name, bytes_input)| (zome_name, bytes_input.to_vec()) )
                .collect(),
            coordinators_token: dna_entry_input.coordinators_token.into_iter()
                .map( |(zome_name, bytes_input)| (zome_name, bytes_input.to_vec()) )
                .collect(),
            claimed_file_size: dna_entry_input.claimed_file_size,
        }
    }
}


#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CreateDnaInput {
    pub manifest: DnaManifestV1,
    pub claimed_file_size: u64,
}

impl TryFrom<CreateDnaInput> for DnaEntry {
    type Error = WasmError;

    fn try_from(create_dna_input: CreateDnaInput) -> ExternResult<Self> {
        let dna_token = create_dna_input.manifest.dna_token()?;
        let integrities_token = create_dna_input.manifest.integrities_token()?;
        let coordinators_token = create_dna_input.manifest.coordinators_token()?;

        Ok(
            Self {
                manifest: create_dna_input.manifest,
                dna_token,
                integrities_token,
                coordinators_token,
                claimed_file_size: create_dna_input.claimed_file_size,
            }
        )
    }
}


#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DnaPackage {
    pub dna_entry: DnaEntry,
    pub wasm_packages: BTreeMap<ZomeName, Vec<u8>>,
}

impl TryInto<DnaPackage> for EntryHash {
    type Error = WasmError;
    fn try_into(self) -> ExternResult<DnaPackage> {
        let dna_entry : DnaEntry = must_get( &self )?.try_into()?;
        let mut wasm_packages = BTreeMap::new();

        for zome_manifest in dna_entry.manifest.integrity.zomes.iter() {
            let wasm_package : WasmPackage = call_role(
                "zomehub",
                "zomehub_csr",
                "get_wasm_package",
                zome_manifest.wasm_hrl.target.clone(),
                (),
            )?;

            wasm_packages.insert( zome_manifest.name.clone(), wasm_package.bytes );
        }

        for zome_manifest in dna_entry.manifest.coordinator.zomes.iter() {
            let wasm_package : WasmPackage = call_role(
                "zomehub",
                "zomehub_csr",
                "get_wasm_package",
                zome_manifest.wasm_hrl.target.clone(),
                (),
            )?;

            wasm_packages.insert( zome_manifest.name.clone(), wasm_package.bytes );
        }

        Ok(
            DnaPackage {
                dna_entry,
                wasm_packages,
            }
        )
    }
}
