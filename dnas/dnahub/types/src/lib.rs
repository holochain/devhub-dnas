mod holochain_types;
mod dna_entry;

pub use hdi_extensions;
pub use hdi_extensions::hdi;
pub use holochain_types::*;

pub use dna_entry::*;

use std::collections::BTreeMap;
use rmp_serde;
use hdi::prelude::*;
use hdi_extensions::{
    guest_error,
};
use sha2::{ Digest, Sha256 };


// TODO: this might be redundant because the zome name is already included in the integrity hash
pub type IntegritiesToken = Vec<(String, Vec<u8>)>;
pub type CoordinatorsToken = Vec<(String, Vec<u8>)>;
pub type AssetHashes = BTreeMap<String, String>;
pub type ResourcesMap = BTreeMap<String, HRL>;



#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct HRL {
    pub dna: DnaHash,
    pub target: AnyDhtHash,
}


#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct DnaToken {
    pub integrity_hash: Vec<u8>,
    pub integrities_token_hash: Vec<u8>,
    pub coordinators_token_hash: Vec<u8>,
}



pub fn serialize<T>(target: &T) -> ExternResult<Vec<u8>>
where
    T: Serialize + ?Sized,
{
    rmp_serde::encode::to_vec( target )
        .map_err( |err| guest_error!(format!(
            "Failed to serialize target: {:?}", err
        )) )
}


pub fn hash<T>(target: &T) -> ExternResult<Vec<u8>>
where
    T: Serialize + ?Sized,
{
    Ok(
        Sha256::digest( &serialize( target )? ).to_vec()
    )
}
