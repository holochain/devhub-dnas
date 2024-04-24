use crate::{
    hdi,
    hash,
    AppToken,
    RolesToken,
    RolesDnaTokens,
    holochain_types::{
        AppManifestV1,
    },
};

use hdi::prelude::*;


//
// App Entry
//
#[hdk_entry_helper]
#[derive(Clone)]
pub struct AppEntry {
    pub manifest: AppManifestV1,

    // This cannot be used for validation as it is solely provided by the client-side and cannot be
    // proven to belong to the corresponding `DnaEntry`
    pub app_token: AppToken,
    pub claimed_file_size: u64,
}

impl AppEntry {
    pub fn new(
        manifest: AppManifestV1,
        roles_dna_tokens: RolesDnaTokens,
        claimed_file_size: u64,
    ) -> ExternResult<Self> {
        // This manifest method will ensure all DNA tokens are present
        let roles_token = manifest.roles_token( roles_dna_tokens )?;

        Ok(
            Self {
                manifest,
                app_token: AppEntry::create_app_token( &roles_token )?,
                claimed_file_size,
            }
        )
    }

    pub fn create_integrity_hash(roles_token: &RolesToken) -> ExternResult<Vec<u8>> {
        roles_token.integrity_hash()
    }

    pub fn create_roles_token_hash(roles_token: &RolesToken) -> ExternResult<Vec<u8>> {
        hash( roles_token )
    }

    pub fn create_app_token(roles_token: &RolesToken) -> ExternResult<AppToken> {
        Ok(
            AppToken {
                integrity_hash: AppEntry::create_integrity_hash( roles_token )?,
                roles_token_hash: AppEntry::create_roles_token_hash( roles_token )?,
                roles_token: roles_token.to_owned(),
            }
        )
    }

    pub fn integrity_hash(&self) -> Vec<u8> {
        self.app_token.integrity_hash.clone()
    }

    pub fn roles_token_hash(&self) -> Vec<u8> {
        self.app_token.roles_token_hash.clone()
    }

    pub fn calculate_integrity_hash(&self) -> ExternResult<Vec<u8>> {
        AppEntry::create_integrity_hash( &self.app_token.roles_token )
    }

    pub fn calculate_roles_token_hash(&self) -> ExternResult<Vec<u8>> {
        AppEntry::create_roles_token_hash( &self.app_token.roles_token )
    }

    pub fn calculate_app_token(&self) -> ExternResult<AppToken> {
        AppEntry::create_app_token( &self.app_token.roles_token )
    }

    pub fn validate_roles_token(&self) -> ExternResult<()> {
        self.manifest.validate_roles_token( &self.app_token.roles_token )
    }
}
