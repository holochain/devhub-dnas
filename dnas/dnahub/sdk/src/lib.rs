pub use dnahub_types;
pub use devhub_sdk;
pub use devhub_sdk::*;

use hdk::prelude::*;
use serde_bytes::*;
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
        }
    }
}


#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CreateDnaInput {
    pub manifest: DnaManifestV1,
}

impl TryFrom<CreateDnaInput> for DnaEntry {
    type Error = WasmError;

    fn try_from(create_dna_input: CreateDnaInput) -> ExternResult<Self> {
        Self::try_from( create_dna_input.manifest )
    }
}
