use crate::hdi;

use std::path::PathBuf;
use hdi::prelude::*;
use holo_hash::WasmHashB64;
use holochain_integrity_types::ZomeName;
use holochain_zome_types::properties::YamlProperties;


#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "snake_case")]
pub struct DnaManifestV1 {
    pub name: String,
    pub integrity: IntegrityManifest,
    #[serde(default)]
    pub coordinator: CoordinatorManifest,
}

impl DnaManifestV1 {
    pub fn all_zomes(&self) -> impl Iterator<Item = &ZomeManifest> {
        self.integrity
            .zomes
            .iter()
            .chain(self.coordinator.zomes.iter())
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct IntegrityManifest {
    pub network_seed: Option<String>,
    pub properties: Option<YamlProperties>,
    pub origin_time: HumanTimestamp,
    pub zomes: Vec<ZomeManifest>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
#[serde(rename_all = "snake_case")]
pub struct CoordinatorManifest {
    pub zomes: Vec<ZomeManifest>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "snake_case")]
pub struct ZomeManifest {
    pub name: ZomeName,
    pub hash: Option<WasmHashB64>,
    pub bundled: String,
    pub dependencies: Option<Vec<ZomeDependency>>,
    #[serde(default)]
    pub dylib: Option<PathBuf>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "snake_case")]
pub struct ZomeDependency {
    pub name: ZomeName,
}
