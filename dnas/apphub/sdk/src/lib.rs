pub use apphub_types;
pub use devhub_sdk;
pub use devhub_sdk::*;

use std::collections::BTreeMap;
use hdk::prelude::*;
use apphub_types::{
    hash,
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
    fn from(roles_token_input: RolesTokenInput) -> Self {
        Self(
            roles_token_input.0.into_iter()
                .map( |(role_name, role_token_input)| (role_name, role_token_input.into()) )
                .collect()
        )
    }
}


#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct RolesDnaTokensInput(pub BTreeMap<String, DnaTokenInput>);

impl From<RolesDnaTokensInput> for RolesDnaTokens {
    fn from(roles_dna_tokens_input: RolesDnaTokensInput) -> Self {
        Self(
            roles_dna_tokens_input.0.into_iter()
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
    fn from(role_token_input: RoleTokenInput) -> Self {
        RoleToken {
            integrity_hash: role_token_input.integrity_hash.to_vec(),
            integrities_token_hash: role_token_input.integrities_token_hash.to_vec(),
            coordinators_token_hash: role_token_input.coordinators_token_hash.to_vec(),
            modifiers_hash: role_token_input.modifiers_hash.to_vec(),
        }
    }
}


#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct AppTokenInput {
    pub integrity_hash: ByteBuf,
    pub roles_token_hash: ByteBuf,
}

impl From<AppTokenInput> for AppToken {
    fn from(app_token_input: AppTokenInput) -> Self {
        AppToken {
            integrity_hash: app_token_input.integrity_hash.to_vec(),
            roles_token_hash: app_token_input.roles_token_hash.to_vec(),
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
    fn from(app_entry_input: AppEntryInput) -> Self {
        AppEntry {
            manifest: app_entry_input.manifest,
            app_token: app_entry_input.app_token.into(),
            roles_token: app_entry_input.roles_token.into(),
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

    fn try_from(create_app_input: CreateAppInput) -> ExternResult<Self> {
        Self::new( create_app_input.manifest, create_app_input.roles_dna_tokens.into() )
    }
}


#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct WebAppAssetsTokenInput {
    pub ui_hash: ByteBuf,
    pub roles_token_hash: ByteBuf,
}

impl From<WebAppAssetsTokenInput> for WebAppAssetsToken {
    fn from(webapp_assets_token_input: WebAppAssetsTokenInput) -> Self {
        WebAppAssetsToken {
            ui_hash: webapp_assets_token_input.ui_hash.to_vec(),
            roles_token_hash: webapp_assets_token_input.roles_token_hash.to_vec(),
        }
    }
}


#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct WebAppTokenInput {
    pub integrity_hash: ByteBuf,
    pub assets_token_hash: ByteBuf,
}

impl From<WebAppTokenInput> for WebAppToken {
    fn from(webapp_token_input: WebAppTokenInput) -> Self {
        WebAppToken {
            integrity_hash: webapp_token_input.integrity_hash.to_vec(),
            assets_token_hash: webapp_token_input.assets_token_hash.to_vec(),
        }
    }
}


#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct WebAppEntryInput {
    pub manifest: WebAppManifestV1,

    pub webapp_token: WebAppTokenInput,
}

impl From<WebAppEntryInput> for WebAppEntry {
    fn from(webapp_entry_input: WebAppEntryInput) -> Self {
        WebAppEntry {
            manifest: webapp_entry_input.manifest,
            webapp_token: webapp_entry_input.webapp_token.into(),
        }
    }
}


#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CreateWebAppInput {
    pub manifest: WebAppManifestV1,
}

impl TryFrom<CreateWebAppInput> for WebAppEntry {
    type Error = WasmError;

    fn try_from(create_webapp_input: CreateWebAppInput) -> ExternResult<Self> {
        let manifest = create_webapp_input.manifest;
        let ui_entry_addr = manifest.ui.ui_entry.clone();
        let app_entry_addr = manifest.happ_manifest.app_entry.clone();

        let app_entry : AppEntry = must_get_entry( app_entry_addr )?.try_into()?;

        let webapp_assets_token = WebAppAssetsToken {
            ui_hash: hash( &ui_entry_addr )?,
            roles_token_hash: app_entry.app_token.roles_token_hash,
        };
        let webapp_token = WebAppToken {
            integrity_hash:  app_entry.app_token.integrity_hash,
            assets_token_hash: hash( &webapp_assets_token )?,
        };

        Ok(
            Self {
                manifest,
                webapp_token,
            }
        )
    }
}
