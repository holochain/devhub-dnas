pub use apphub_types;
pub use devhub_sdk;
pub use devhub_sdk::*;

use std::collections::BTreeMap;
use hdk::prelude::*;
use hdk_extensions::{
    agent_id,
    must_get,
};
use apphub_types::{
    Authority,
    MemoryAddr,
    BundleAddr,
    DeprecationNotice,
    RmpvValue,

    RoleToken,
    AppManifestV1,
    AppToken,
    AppEntry,

    RolesToken,
    RolesDnaTokens,
    WebAppManifestV1,
    WebAppToken,
    WebAppAssetsToken,
    WebAppEntry,

    WebAppPackageEntry,
    WebAppPackageVersionEntry,
};
use dnahub_sdk::{
    DnaTokenInput,
};
use hc_crud::{
    Entity, EntityId,
};
use serde_bytes::ByteBuf;


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
}

impl From<AppTokenInput> for AppToken {
    fn from(input: AppTokenInput) -> Self {
        Self {
            integrity_hash: input.integrity_hash.to_vec(),
            roles_token_hash: input.roles_token_hash.to_vec(),
        }
    }
}


#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct AppEntryInput {
    pub manifest: AppManifestV1,
    pub app_token: AppTokenInput,
    pub roles_token: RolesTokenInput,
}

impl From<AppEntryInput> for AppEntry {
    fn from(input: AppEntryInput) -> Self {
        Self {
            manifest: input.manifest,
            app_token: input.app_token.into(),
            roles_token: input.roles_token.into(),
        }
    }
}


#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CreateAppInput {
    pub manifest: AppManifestV1,
    pub roles_dna_tokens: RolesDnaTokensInput,
}

impl TryFrom<CreateAppInput> for AppEntry {
    type Error = WasmError;

    fn try_from(input: CreateAppInput) -> ExternResult<Self> {
        Self::new( input.manifest, input.roles_dna_tokens.into() )
    }
}


#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct WebAppAssetsTokenInput {
    pub ui_hash: ByteBuf,
    pub roles_token_hash: ByteBuf,
}

impl From<WebAppAssetsTokenInput> for WebAppAssetsToken {
    fn from(input: WebAppAssetsTokenInput) -> Self {
        Self {
            ui_hash: input.ui_hash.to_vec(),
            roles_token_hash: input.roles_token_hash.to_vec(),
        }
    }
}


#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct WebAppTokenInput {
    pub integrity_hash: ByteBuf,
    pub assets_token_hash: ByteBuf,
}

impl From<WebAppTokenInput> for WebAppToken {
    fn from(input: WebAppTokenInput) -> Self {
        Self {
            integrity_hash: input.integrity_hash.to_vec(),
            assets_token_hash: input.assets_token_hash.to_vec(),
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
