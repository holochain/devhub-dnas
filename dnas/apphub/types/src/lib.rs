mod holochain_types;
mod app_entry;
mod ui_entry;
mod webapp_entry;
mod webapp_package_entry;
mod webapp_package_version_entry;

pub use hdi_extensions;
pub use hdi_extensions::hdi;
pub use holochain_types::*;
pub use mere_memory_types;

pub use app_entry::*;
pub use ui_entry::*;

pub use webapp_entry::*;
pub use webapp_package_entry::*;
pub use webapp_package_version_entry::*;

use std::{
    iter::{
        FromIterator,
    },
    collections::{
        BTreeMap,
    },
};
use hdi::prelude::*;
use hdi_extensions::{
    guest_error,
};
use holochain_zome_types::{
    DnaModifiersOpt,
    YamlProperties,
};
use rmp_serde;
use sha2::{ Digest, Sha256 };
use dnahub_types::{
    DnaToken,
};



pub type EntityId = ActionHash;
pub type BundleAddr = EntryHash;
pub type MemoryAddr = EntryHash;
pub type RmpvValue = rmpv::Value;


#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct RolesDnaTokens(pub BTreeMap<String, DnaToken>);

impl RolesDnaTokens {
    pub fn integrity_hashes(&self) -> Vec<Vec<u8>> {
        let mut dnas_integrity_hashes = self.0.iter()
            .map( |(_, dna_token)| dna_token.integrity_hash.clone() )
            .collect::<Vec<Vec<u8>>>();

        dnas_integrity_hashes.sort();

        dnas_integrity_hashes
    }

    pub fn integrity_hash(&self) -> ExternResult<Vec<u8>> {
        hash( &self.integrity_hashes() )
    }

    pub fn dna_token(&self, role_name: &str) -> ExternResult<DnaToken> {
        match self.0.iter().find( |(name, _)| name.as_str() == role_name ) {
            Some((_, dna_token)) => Ok( dna_token.clone() ),
            None => Err(guest_error!(format!(
                "Missing DnaToken for role '{}'", role_name
            ))),
        }
    }
}

impl FromIterator<(String, DnaToken)> for RolesDnaTokens {
    fn from_iter<I: IntoIterator<Item = (String, DnaToken)>>(iter: I) -> Self {
        let map: BTreeMap<String, DnaToken> = iter.into_iter().collect();
        RolesDnaTokens(map)
    }
}


#[derive(Clone, Debug, Serialize, Deserialize, PartialOrd, PartialEq, Ord, Eq)]
pub struct RoleToken {
    pub integrity_hash: Vec<u8>,
    pub integrities_token_hash: Vec<u8>,
    pub coordinators_token_hash: Vec<u8>,
    pub modifiers_hash: Vec<u8>,
}

impl RoleToken {
    pub fn new(dna_token: &DnaToken, modifiers: &DnaModifiersOpt<YamlProperties>) ->
        ExternResult<Self>
    {
        Ok(
            Self {
                integrity_hash: dna_token.integrity_hash.to_owned(),
                integrities_token_hash: dna_token.integrities_token_hash.to_owned(),
                coordinators_token_hash: dna_token.coordinators_token_hash.to_owned(),
                modifiers_hash: hash( modifiers )?,
            }
        )
    }

    pub fn dna_token(&self) -> DnaToken {
        DnaToken {
            integrity_hash: self.integrity_hash.to_owned(),
            integrities_token_hash: self.integrities_token_hash.to_owned(),
            coordinators_token_hash: self.coordinators_token_hash.to_owned(),
        }
    }
}


#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct RolesToken(pub Vec<(String, RoleToken)>);

impl RolesToken {
    pub fn integrity_hashes(&self) -> Vec<Vec<u8>> {
        let mut dnas_integrity_hashes = self.0.iter()
            .map( |(_, role_token)| role_token.integrity_hash.clone() )
            .collect::<Vec<Vec<u8>>>();

        dnas_integrity_hashes.sort();

        dnas_integrity_hashes
    }

    pub fn integrity_hash(&self) -> ExternResult<Vec<u8>> {
        hash( &self.integrity_hashes() )
    }

    pub fn role_token(&self, role_name: &str) -> ExternResult<RoleToken> {
        match self.0.iter()
            .find( |(name, _)| name.as_str() == role_name ) {
                Some((_, role_token)) => Ok( role_token.clone() ),
                None => Err(guest_error!(format!(
                    "Missing RoleToken for role '{}'", role_name
                ))),
            }
    }

    pub fn dna_token(&self, role_name: &str) -> ExternResult<DnaToken> {
        match self.0.iter()
            .find( |(name, _)| name.as_str() == role_name ) {
                Some((_, role_token)) => Ok( role_token.dna_token() ),
                None => Err(guest_error!(format!(
                    "Missing DnaToken for role '{}'", role_name
                ))),
            }
    }
}

impl FromIterator<(String, RoleToken)> for RolesToken {
    fn from_iter<I: IntoIterator<Item = (String, RoleToken)>>(iter: I) -> Self {
        let vec: Vec<(String, RoleToken)> = iter.into_iter().collect();
        RolesToken(vec)
    }
}


#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct AppToken {
    pub integrity_hash: Vec<u8>,
    pub roles_token_hash: Vec<u8>,
}


#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct WebAppAssetsToken {
    pub ui_hash: Vec<u8>,
    pub roles_token_hash: Vec<u8>,
}


#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct WebAppToken {
    pub integrity_hash: Vec<u8>,
    pub assets_token_hash: Vec<u8>,
}



#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(tag = "type", content = "content")]
#[serde(rename_all = "snake_case")]
pub enum Authority {
    // Group(ActionHash, ActionHash),
    Agent(AgentPubKey),
}

impl From<AgentPubKey> for Authority {
    fn from(agent_pub_key: AgentPubKey) -> Self {
        Authority::Agent(agent_pub_key)
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct DeprecationNotice {
    pub message: String,
    #[serde(default)]
    pub recommended_alternatives: Vec<ActionHash>,
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
