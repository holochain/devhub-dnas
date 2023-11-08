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
    pub roles_token: RolesToken,
}

impl AppEntry {
    pub fn new(manifest: AppManifestV1, roles_dna_tokens: RolesDnaTokens) -> ExternResult<Self> {
        let integrity_hash = roles_dna_tokens.integrity_hash()?;

        // This manifest method will ensure all DNA tokens are present
        let roles_token = manifest.roles_token( roles_dna_tokens )?;
        let roles_token_hash = hash( &roles_token )?;

        let app_token = AppToken {
            integrity_hash,
            roles_token_hash,
        };

        Ok(
            Self {
                manifest,
                app_token,
                roles_token,
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
        self.roles_token.integrity_hash()
    }

    pub fn calculate_roles_token_hash(&self) -> ExternResult<Vec<u8>> {
        hash( &self.roles_token )
    }

    pub fn calculate_app_token(&self) -> ExternResult<AppToken> {
        Ok(
            AppToken {
                integrity_hash: self.calculate_integrity_hash()?,
                roles_token_hash: self.calculate_roles_token_hash()?,
            }
        )
    }

    pub fn validate_roles_token(&self) -> ExternResult<()> {
        self.manifest.validate_roles_token( &self.roles_token )
    }
}
