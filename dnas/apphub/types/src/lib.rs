mod holochain_types;
mod app_entry;
mod ui_entry;
mod webapp_entry;
mod webapp_package_entry;
mod webapp_package_version_entry;

pub use hdi_extensions;
pub use hdi_extensions::hdi;
pub use holochain_types::*;

pub use app_entry::*;
pub use ui_entry::*;

pub use webapp_entry::*;
pub use webapp_package_entry::*;
pub use webapp_package_version_entry::*;

use rmp_serde;
use hdi::prelude::*;
use hdi_extensions::{
    guest_error,
};
use sha2::{ Digest, Sha256 };



pub type EntityId = ActionHash;
pub type BundleAddr = EntryHash;
pub type MemoryAddr = EntryHash;



#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct AppToken {
    pub integrity_hash: Vec<u8>,
    pub roles_token_hash: Vec<u8>,
}


#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct WebAppAssetsToken {
    pub ui_hash: Vec<u8>,
    pub roles_token_hash: Vec<u8>,
}


#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct WebAppToken {
    pub integrity_hash: Vec<u8>,
    pub assets_token_hash: Vec<u8>,
}



#[derive(Serialize, Deserialize, Clone, Debug)]
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
