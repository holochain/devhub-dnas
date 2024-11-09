use crate::{
    hdi,
    hash,
    AppToken,
    RolesToken,
    RolesDnaTokens,
    ResourcesMap,
    holochain_types::{
        AppManifestV1,
    },
};

use std::io::Cursor;
use hdi::prelude::*;
use hdi_extensions::{
    guest_error,
};


//
// App Entry
//
#[hdk_entry_helper]
#[derive(Clone)]
pub struct AppEntry {
    pub manifest: rmpv::Value,
    pub resources: ResourcesMap,

    // This cannot be used for validation as it is solely provided by the client-side and cannot be
    // proven to belong to the corresponding `DnaEntry`
    pub app_token: AppToken,
    pub claimed_file_size: u64,
}

impl AppEntry {
    pub fn new(
        manifest: rmpv::Value,
        resources: ResourcesMap,
        roles_dna_tokens: RolesDnaTokens,
        claimed_file_size: u64,
    ) -> ExternResult<Self> {
        // This manifest method will ensure all DNA tokens are present
        let app_manifest = AppEntry::deserialize_manifest( &manifest )?;
        let roles_token = app_manifest.roles_token( roles_dna_tokens )?;

        Ok(
            Self {
                manifest,
                resources,
                app_token: AppEntry::create_app_token( &roles_token )?,
                claimed_file_size,
            }
        )
    }

    pub fn deserialize_manifest(manifest: &rmpv::Value) -> ExternResult<AppManifestV1> {
        let mut buf = Vec::new();
        rmpv::encode::write_value(&mut buf, manifest)
            .map_err(|e| guest_error!(format!(
                "Failed to encode manifest value: {:?}", e
            )))?;

        let cursor = Cursor::new(buf);
        rmp_serde::from_read(cursor)
            .map_err(|e| guest_error!(format!(
                "Failed to deserialize manifest: {:?}", e
            )))
    }

    pub fn deserialized_manifest(&self) -> ExternResult<AppManifestV1> {
        AppEntry::deserialize_manifest( &self.manifest )
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
        self.deserialized_manifest()?.validate_roles_token( &self.app_token.roles_token )
    }
}
