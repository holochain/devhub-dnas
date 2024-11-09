mod zome_entry;
mod zome_package_entry;
mod zome_package_version_entry;

pub use hdi_extensions;
pub use hdi_extensions::hdi;
pub use mere_memory_types;

pub use zome_entry::*;
pub use zome_package_entry::*;
pub use zome_package_version_entry::*;

use hdi::prelude::*;



pub type EntityId = ActionHash;
pub type RmpvValue = rmpv::Value;



#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq)]
#[serde(tag = "type", content = "content")]
#[serde(rename_all = "snake_case")]
pub enum Authority {
    Group(ActionHash, ActionHash),
    Agent(AgentPubKey),
}

impl From<AgentPubKey> for Authority {
    fn from(agent_pub_key: AgentPubKey) -> Self {
        Authority::Agent(agent_pub_key)
    }
}

impl From<(ActionHash, ActionHash)> for Authority {
    fn from((group_id, group_addr): (ActionHash, ActionHash)) -> Self {
        Authority::Group(group_id, group_addr)
    }
}
