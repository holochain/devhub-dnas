pub use zomehub_types;
pub use devhub_sdk;
pub use devhub_sdk::*;

use std::collections::BTreeMap;
use hdk::prelude::*;
use hdk_extensions::{
    agent_id,
    must_get,
};
use zomehub_types::{
    Authority,
    RmpvValue,

    ZomeType,
    ZomeEntry,
    ZomePackageEntry,
    ZomePackageVersionEntry,
    ApiCompatibility,

    mere_memory_types,
};
use mere_memory_types::{
    MemoryEntry,
};
use hc_crud::{
    Entity, EntityId,
};


pub type EntityMap<T> = BTreeMap<String, Entity<T>>;
pub type EntityPointerMap = BTreeMap<String, EntityId>;

pub type ZomeMap = EntityMap<ZomeEntry>;
pub type ZomePackageMap = EntityMap<ZomePackageEntry>;
pub type ZomePackageVersionMap = EntityMap<ZomePackageVersionEntry>;


#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MemoryWithBytes(
    MemoryEntry,
    Vec<u8>
);


#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ZomeAsset {
    pub zome_entry: ZomeEntry,
    pub memory_entry: MemoryEntry,
    pub bytes: Vec<u8>,
}

impl TryInto<ZomeAsset> for EntryHash {
    type Error = WasmError;
    fn try_into(self) -> ExternResult<ZomeAsset> {
        let zome_entry : ZomeEntry = must_get( &self )?.try_into()?;

        let memory_with_bytes : MemoryWithBytes = call_zome(
            "mere_memory_api",
            "get_memory_with_bytes",
            zome_entry.mere_memory_addr.clone(),
            (),
        )?;

        Ok(
            ZomeAsset {
                zome_entry: zome_entry,
                memory_entry: memory_with_bytes.0,
                bytes: memory_with_bytes.1.to_vec(),

            }
        )
    }
}


#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CreateZomePackageInput {
    pub name: String,
    pub title: String,
    pub description: String,
    pub zome_type: ZomeType,

    // optional
    pub maintainer: Option<Authority>,
    pub tags: Option<Vec<String>>,

    // Common fields
    #[serde(default)]
    pub metadata: BTreeMap<String, RmpvValue>,
}

impl TryFrom<CreateZomePackageInput> for ZomePackageEntry {
    type Error = WasmError;

    fn try_from(input: CreateZomePackageInput) -> ExternResult<Self> {
        Ok(
            Self {
                name: input.name,
                title: input.title,
                description: input.description,
                zome_type: input.zome_type,
                maintainer: input.maintainer
                    .unwrap_or( agent_id()?.into() ),
                metadata: input.metadata,

                // optional
                tags: input.tags,
            }
        )
    }
}


#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CreateZomePackageVersionInput {
    pub for_package: EntityId,
    pub version: String,
    pub zome_entry: EntryHash,

    // optional
    pub maintainer: Option<Authority>,
    pub changelog: Option<String>,
    pub source_code_revision_uri: Option<String>,
    pub api_compatibility: ApiCompatibility,

    // Common fields
    #[serde(default)]
    pub metadata: BTreeMap<String, RmpvValue>,
}

impl TryFrom<CreateZomePackageVersionInput> for ZomePackageVersionEntry {
    type Error = WasmError;

    fn try_from(input: CreateZomePackageVersionInput) -> ExternResult<Self> {
        Ok(
            Self {
                for_package: input.for_package.clone(),
                zome_entry: input.zome_entry,
                maintainer: match input.maintainer {
                    None => {
                        let zome_package : ZomePackageEntry = must_get( &input.for_package )?.try_into()?;

                        zome_package.maintainer
                    },
                    Some(auth) => auth,
                },

                changelog: input.changelog,
                source_code_revision_uri: input.source_code_revision_uri,
                api_compatibility: input.api_compatibility,
                metadata: input.metadata,
            }
        )
    }
}
