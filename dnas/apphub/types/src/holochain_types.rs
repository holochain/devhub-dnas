use crate::{
    hdi,
    hdi_extensions,
    hash,
    RolesToken,
    RoleToken,
    RolesDnaTokens,
};

use std::collections::BTreeMap;
use hdi::prelude::*;
use hdi_extensions::{
    guest_error,
};
use holo_hash::DnaHashB64;
use holochain_zome_types::{
    prelude::DnaModifiersOpt,
    properties::YamlProperties,
};



pub type RoleName = String;



#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct HRL {
    pub dna: DnaHash,
    pub target: AnyDhtHash,
}


#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "snake_case")]
pub struct AppManifestV1 {
    pub name: String,
    pub description: Option<String>,
    pub roles: Vec<AppRoleManifest>,
}

impl AppManifestV1 {
    pub fn roles_token(&self, roles_dna_tokens: RolesDnaTokens) -> ExternResult<RolesToken> {
        let roles_token = self.roles.iter()
            .map( |role_manifest| {
                let dna_token = roles_dna_tokens.dna_token( &role_manifest.name )?;

                Ok((
                    role_manifest.name.clone(),
                    RoleToken::new( &dna_token, &role_manifest.dna.modifiers )?
                ))
            })
            .collect::<ExternResult<RolesToken>>()?;

        Ok( roles_token )
    }

    pub fn roles_token_hash(&self, roles_dna_tokens: RolesDnaTokens) -> ExternResult<Vec<u8>> {
        hash( &self.roles_token( roles_dna_tokens )? )
    }

    /// A [`RolesToken`] is valid when:
    ///
    /// - The length matches the manifest.roles length
    /// - There is a [`RoleToken`] for each manifest role
    /// - The `modifiers_hash` for each [`RoleToken`] matches the corresponding role modifiers
    ///
    pub fn validate_roles_token(&self, roles_token: &RolesToken) -> ExternResult<()> {
        if roles_token.0.len() != self.roles.len() {
            return Err(guest_error!(format!(
                "Invalid RolesToken length ({}); must match the manifest's roles length ({})",
                roles_token.0.len(), self.roles.len()
            )));
        }

        let roles_token_map : BTreeMap<String, RoleToken> = roles_token.clone().0.into_iter().collect();

        for role_manifest in self.roles.iter() {
            let role_token = roles_token_map.get( &role_manifest.name )
                .ok_or(guest_error!(format!(
                    "Missing RoleToken for role '{}'", role_manifest.name
                )))?;

            if role_token.modifiers_hash != hash( &role_manifest.dna.modifiers )? {
                return Err(guest_error!(format!(
                    "Missing RoleToken for role '{}'", role_manifest.name
                )));
            }
        }

        Ok(())
    }
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
    pub dna_hrl: HRL,
    #[serde(default)]
    pub modifiers: DnaModifiersOpt<YamlProperties>,
    #[serde(default)]
    pub installed_hash: Option<DnaHashB64>,
    #[serde(default)]
    pub clone_limit: u32,
}


#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "snake_case")]
#[serde(tag = "strategy")]
pub enum CellProvisioning {
    Create { deferred: bool },
    CloneOnly,
}

impl Default for CellProvisioning {
    fn default() -> Self {
        Self::Create { deferred: false }
    }
}


#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct WebAppManifestV1 {
    pub name: String,
    pub ui: WebUI,
    pub happ_manifest: AppManifestLocation,
}


#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct WebUI {
    pub ui_entry: EntryHash,
}


#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct AppManifestLocation {
    pub app_entry: EntryHash,
}
