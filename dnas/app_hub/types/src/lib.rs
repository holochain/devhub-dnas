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

use hdi::prelude::*;


pub type EntityId = ActionHash;
pub type BundleAddr = ActionHash; // TODO: should this be EntryHash?
pub type MemoryAddr = EntryHash;


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
