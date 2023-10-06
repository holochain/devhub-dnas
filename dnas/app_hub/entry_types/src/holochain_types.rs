use crate::hdi;

use std::{
    collections::BTreeMap,
    path::PathBuf,
};
use hdi::prelude::*;
use holo_hash::DnaHashB64;
use holochain_zome_types::{
    DnaModifiersOpt,
    YamlProperties,
};


pub type RoleName = String;
pub type ResourceMap = BTreeMap<PathBuf, ActionHash>;


#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "snake_case")]
pub struct AppManifestV1 {
    pub name: String,
    pub description: Option<String>,
    pub roles: Vec<AppRoleManifest>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "snake_case")]
pub struct AppRoleManifest {
    pub name: RoleName,
    #[serde(default)]
    pub provisioning: Option<CellProvisioning>,
    pub dna: AppRoleDnaManifest,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct AppRoleDnaManifest {
    pub bundled: String,
    #[serde(default)]
    pub modifiers: DnaModifiersOpt<YamlProperties>,
    #[serde(default)]
    pub installed_hash: Option<DnaHashB64>,
    #[serde(default)]
    pub clone_limit: u32,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum CellProvisioning {
    Create { deferred: bool },
    CloneOnly,
}

impl Default for CellProvisioning {
    fn default() -> Self {
        Self::Create { deferred: false }
    }
}
