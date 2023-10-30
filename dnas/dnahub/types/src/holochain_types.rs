use crate::{
    hdi,
    hash,
    IntegritiesToken,
    CoordinatorsToken,
    DnaToken,
};

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

    pub fn integrity_hash(&self) -> ExternResult<Vec<u8>> {
        hash( &self.integrity )
    }

    pub fn integrities_token(&self) -> ExternResult<IntegritiesToken> {
        let mut integrities_token = self.integrity.zomes.iter()
            .map( |zome_manifest| {
                Ok((
                    zome_manifest.name.0.clone().into(),
                    hash( &zome_manifest )?,
                ))
            })
            .collect::<ExternResult<IntegritiesToken>>()?;

        integrities_token.sort();

        Ok( integrities_token )
    }

    pub fn integrities_token_hash(&self) -> ExternResult<Vec<u8>> {
        hash( &self.integrities_token()? )
    }

    pub fn coordinators_token(&self) -> ExternResult<CoordinatorsToken> {
        let mut coordinators_token = self.coordinator.zomes.iter()
            .map( |zome_manifest| {
                Ok((
                    zome_manifest.name.0.clone().into(),
                    hash( &zome_manifest )?,
                ))
            })
            .collect::<ExternResult<CoordinatorsToken>>()?;

        coordinators_token.sort();

        Ok( coordinators_token )
    }

    pub fn coordinators_token_hash(&self) -> ExternResult<Vec<u8>> {
        hash( &self.coordinators_token()? )
    }

    pub fn dna_token(&self) -> ExternResult<DnaToken> {
        Ok(
            DnaToken {
                integrity_hash: self.integrity_hash()?,
                integrities_token_hash: self.integrities_token_hash()?,
                coordinators_token_hash: self.coordinators_token_hash()?,
            }
        )
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
    pub wasm_entry: EntryHash,
    pub dependencies: Option<Vec<ZomeDependency>>,
    #[serde(default)]
    pub dylib: Option<PathBuf>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "snake_case")]
pub struct ZomeDependency {
    pub name: ZomeName,
}
