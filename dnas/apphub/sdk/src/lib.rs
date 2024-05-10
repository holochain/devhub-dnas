pub use apphub_types;
pub use devhub_sdk;
pub use devhub_sdk::*;

use std::collections::BTreeMap;
use hdk::prelude::*;
use hdk_extensions::{
    agent_id,
    must_get,
    hdi_extensions::{
        guest_error,
    },
};
use serde_bytes::ByteBuf;
use apphub_types::{
    Authority,
    MemoryAddr,
    BundleAddr,
    DeprecationNotice,
    RmpvValue,
    ResourcesMap,

    RoleToken,
    AppManifestV1,
    AppToken,
    AppEntry,

    RolesToken,
    RolesDnaTokens,
    WebAppManifestV1,
    WebAppToken,
    WebAppEntry,

    WebAppPackageEntry,
    WebAppPackageVersionEntry,

    UiEntry,
    mere_memory_types,
};
use mere_memory_types::{
    MemoryEntry,
};
use dnahub_sdk::{
    DnaTokenInput,
    DnaAsset,
};
use hc_crud::{
    Entity, EntityId,
};


pub type EntityMap<T> = BTreeMap<String, Entity<T>>;
pub type EntityPointerMap = BTreeMap<String, EntityId>;

pub type WebAppMap = EntityMap<WebAppEntry>;
pub type WebAppPackageMap = EntityMap<WebAppPackageEntry>;
pub type WebAppPackageVersionMap = EntityMap<WebAppPackageVersionEntry>;


#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct RolesTokenInput(pub Vec<(String, RoleTokenInput)>);

impl From<RolesTokenInput> for RolesToken {
    fn from(input: RolesTokenInput) -> Self {
        Self(
            input.0.into_iter()
                .map( |(role_name, role_token_input)| (role_name, role_token_input.into()) )
                .collect()
        )
    }
}


#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct RolesDnaTokensInput(pub BTreeMap<String, DnaTokenInput>);

impl From<RolesDnaTokensInput> for RolesDnaTokens {
    fn from(input: RolesDnaTokensInput) -> Self {
        Self(
            input.0.into_iter()
                .map( |(role_name, dna_token_input)| (role_name, dna_token_input.into()) )
                .collect()
        )
    }
}


#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct RoleTokenInput {
    pub integrity_hash: ByteBuf,
    pub integrities_token_hash: ByteBuf,
    pub coordinators_token_hash: ByteBuf,
    pub modifiers_hash: ByteBuf,
}

impl From<RoleTokenInput> for RoleToken {
    fn from(input: RoleTokenInput) -> Self {
        Self {
            integrity_hash: input.integrity_hash.to_vec(),
            integrities_token_hash: input.integrities_token_hash.to_vec(),
            coordinators_token_hash: input.coordinators_token_hash.to_vec(),
            modifiers_hash: input.modifiers_hash.to_vec(),
        }
    }
}


#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct AppTokenInput {
    pub integrity_hash: ByteBuf,
    pub roles_token_hash: ByteBuf,
    pub roles_token: RolesTokenInput,
}

impl From<AppTokenInput> for AppToken {
    fn from(input: AppTokenInput) -> Self {
        Self {
            integrity_hash: input.integrity_hash.to_vec(),
            roles_token_hash: input.roles_token_hash.to_vec(),
            roles_token: input.roles_token.into(),
        }
    }
}


#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct AppEntryInput {
    pub manifest: AppManifestV1,
    pub resources: ResourcesMap,
    pub app_token: AppTokenInput,
    pub claimed_file_size: u64,
}

impl From<AppEntryInput> for AppEntry {
    fn from(input: AppEntryInput) -> Self {
        Self {
            manifest: input.manifest,
            resources: input.resources,
            app_token: input.app_token.into(),
            claimed_file_size: input.claimed_file_size,
        }
    }
}


#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CreateAppInput {
    pub manifest: AppManifestV1,
    pub resources: ResourcesMap,
    pub roles_dna_tokens: RolesDnaTokensInput,
    pub claimed_file_size: u64,
}

impl TryFrom<CreateAppInput> for AppEntry {
    type Error = WasmError;

    fn try_from(input: CreateAppInput) -> ExternResult<Self> {
        Self::new(
            input.manifest,
            input.resources,
            input.roles_dna_tokens.into(),
            input.claimed_file_size
        )
    }
}


#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct WebAppTokenInput {
    pub ui_hash: ByteBuf,
    pub app_token: AppTokenInput,
}

impl From<WebAppTokenInput> for WebAppToken {
    fn from(input: WebAppTokenInput) -> Self {
        Self {
            ui_hash: input.ui_hash.to_vec(),
            app_token: input.app_token.into(),
        }
    }
}


#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct WebAppEntryInput {
    pub manifest: WebAppManifestV1,

    pub webapp_token: WebAppTokenInput,
}

impl From<WebAppEntryInput> for WebAppEntry {
    fn from(input: WebAppEntryInput) -> Self {
        Self {
            manifest: input.manifest,
            webapp_token: input.webapp_token.into(),
        }
    }
}


#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CreateWebAppInput {
    pub manifest: WebAppManifestV1,
}

impl TryFrom<CreateWebAppInput> for WebAppEntry {
    type Error = WasmError;

    fn try_from(input: CreateWebAppInput) -> ExternResult<Self> {
        Self::new( input.manifest )
    }
}


#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct WebAppPackageEntryInput {
    pub title: String,
    pub subtitle: String,
    pub description: String,
    pub maintainer: Authority,
    pub icon: MemoryAddr,
    pub source_code_uri: Option<String>,
    #[serde(default)]
    pub deprecation: Option<DeprecationNotice>,
    pub metadata: BTreeMap<String, RmpvValue>,
}

impl From<WebAppPackageEntryInput> for WebAppPackageEntry {
    fn from(input: WebAppPackageEntryInput) -> Self {
        Self {
            title: input.title,
            subtitle: input.subtitle,
            description: input.description,
            maintainer: input.maintainer,
            icon: input.icon,
            source_code_uri: input.source_code_uri,
            deprecation: input.deprecation,
            metadata: input.metadata,
        }
    }
}


#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CreateWebAppPackageInput {
    pub title: String,
    pub subtitle: String,
    pub description: String,
    pub icon: MemoryAddr,
    #[serde(default)]
    pub metadata: BTreeMap<String, RmpvValue>,
    pub maintainer: Option<Authority>,
    pub source_code_uri: Option<String>,
}

impl TryFrom<CreateWebAppPackageInput> for WebAppPackageEntry {
    type Error = WasmError;

    fn try_from(input: CreateWebAppPackageInput) -> ExternResult<Self> {
        Ok(
            Self {
                title: input.title,
                subtitle: input.subtitle,
                description: input.description,
                maintainer: input.maintainer
                    .unwrap_or( agent_id()?.into() ),
                icon: input.icon,
                source_code_uri: input.source_code_uri,
                deprecation: None,
                metadata: input.metadata,
            }
        )
    }
}


#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct WebAppPackageVersionEntryInput {
    pub for_package: EntityId,
    pub maintainer: Authority,
    pub webapp: BundleAddr,
    pub webapp_token: WebAppTokenInput,
    pub changelog: Option<String>,
    pub source_code_revision_uri: Option<String>,
    #[serde(default)]
    pub metadata: BTreeMap<String, RmpvValue>,
}

impl From<WebAppPackageVersionEntryInput> for WebAppPackageVersionEntry {
    fn from(input: WebAppPackageVersionEntryInput) -> Self {
        Self {
            for_package: input.for_package,
            webapp: input.webapp,
            webapp_token: input.webapp_token.into(),
            changelog: input.changelog,
            maintainer: input.maintainer,
            source_code_revision_uri: input.source_code_revision_uri,
            metadata: input.metadata,
        }
    }
}


#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CreateWebAppPackageVersionInput {
    pub for_package: EntityId,
    pub version: String,
    pub webapp: BundleAddr,
    #[serde(default)]
    pub metadata: BTreeMap<String, RmpvValue>,

    // Optional
    pub changelog: Option<String>,
    pub maintainer: Option<Authority>,
    pub source_code_revision_uri: Option<String>,
}

impl TryFrom<CreateWebAppPackageVersionInput> for WebAppPackageVersionEntry {
    type Error = WasmError;

    fn try_from(input: CreateWebAppPackageVersionInput) -> ExternResult<Self> {
        let webapp_entry : WebAppEntry = must_get( &input.webapp )?.try_into()?;

        Ok(
            Self {
                for_package: input.for_package,
                webapp: input.webapp,
                webapp_token: webapp_entry.webapp_token,
                changelog: input.changelog,
                maintainer: input.maintainer
                    .unwrap_or( agent_id()?.into() ),
                source_code_revision_uri: input.source_code_revision_uri,
                metadata: input.metadata,
            }
        )
    }
}


#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AppAsset {
    pub app_entry: AppEntry,
    pub dna_assets: BTreeMap<RoleName, DnaAsset>,
}

impl TryInto<AppAsset> for EntryHash {
    type Error = WasmError;
    fn try_into(self) -> ExternResult<AppAsset> {
        let app_entry : AppEntry = must_get( &self )?.try_into()?;
        let mut dna_assets = BTreeMap::new();

        for role_manifest in app_entry.manifest.roles.iter() {
            let hrl = app_entry.resources.get( &role_manifest.dna.bundled )
                .ok_or(guest_error!(format!(
                    "DnaEntry does not have resources with path '{}'",
                    role_manifest.dna.bundled,
                )))?;
            let dna_asset : DnaAsset = call_cell(
                hrl.dna.clone(),
                "dnahub_csr",
                "get_dna_asset",
                hrl.target.clone(),
                (),
            )?;

            dna_assets.insert( role_manifest.name.clone(), dna_asset );
        }

        Ok(
            AppAsset {
                app_entry,
                dna_assets,
            }
        )
    }
}


#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct WebAppAsset {
    pub webapp_entry: WebAppEntry,
    pub app_asset: AppAsset,
    pub ui_asset: UiAsset,
}

impl TryInto<WebAppAsset> for EntryHash {
    type Error = WasmError;
    fn try_into(self) -> ExternResult<WebAppAsset> {
        let webapp_entry : WebAppEntry = must_get( &self )?.try_into()?;
        let app_asset : AppAsset = call_zome(
            "apphub_csr",
            "get_app_asset",
            webapp_entry.manifest.happ_manifest.app_entry.clone(),
            (),
        )?;
        let ui_asset : UiAsset = call_zome(
            "apphub_csr",
            "get_ui_asset",
            webapp_entry.manifest.ui.ui_entry.clone(),
            (),
        )?;

        Ok(
            WebAppAsset {
                webapp_entry,
                app_asset,
                ui_asset,
            }
        )
    }
}


#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MemoryWithBytes(
    MemoryEntry,
    Vec<u8>
);

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UiAsset {
    pub ui_entry: UiEntry,
    pub memory_entry: MemoryEntry,
    pub bytes: Vec<u8>,
}

impl TryInto<UiAsset> for EntryHash {
    type Error = WasmError;
    fn try_into(self) -> ExternResult<UiAsset> {
        let ui_entry : UiEntry = must_get( &self )?.try_into()?;

        let memory_with_bytes : MemoryWithBytes = call_zome(
            "mere_memory_api",
            "get_memory_with_bytes",
            ui_entry.mere_memory_addr.clone(),
            (),
        )?;

        Ok(
            UiAsset {
                ui_entry: ui_entry,
                memory_entry: memory_with_bytes.0,
                bytes: memory_with_bytes.1.to_vec(),
            }
        )
    }
}
