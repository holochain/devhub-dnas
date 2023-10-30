use crate::{
    hdi,
    hdi_extensions,
    hash,
    AppToken,
    holochain_types::{
        AppManifestV1,
    },
};

use std::collections::{
    BTreeMap,
};
use hdi::prelude::*;
use hdi_extensions::{
    guest_error,
};
use holochain_zome_types::{
    DnaModifiersOpt,
    YamlProperties,
};
use dnahub_types::{
    DnaToken,
};



pub type RolesToken = Vec<(String, RoleToken)>;
pub type RolesDnaTokens = BTreeMap<String, DnaToken>;


#[derive(Clone, Debug, Serialize, Deserialize, PartialOrd, PartialEq, Ord, Eq)]
pub struct RoleToken {
    pub integrity_hash: Vec<u8>,
    pub integrities_token_hash: Vec<u8>,
    pub coordinators_token_hash: Vec<u8>,
    pub modifiers_hash: Vec<u8>,
}

impl RoleToken {
    pub fn new(dna_token: &DnaToken, modifiers: &DnaModifiersOpt<YamlProperties>) -> ExternResult<Self> {
        Ok(
            Self {
                integrity_hash: dna_token.integrity_hash.to_owned(),
                integrities_token_hash: dna_token.integrities_token_hash.to_owned(),
                coordinators_token_hash: dna_token.coordinators_token_hash.to_owned(),
                modifiers_hash: hash( modifiers )?,
            }
        )
    }
}



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
    pub fn new(
        manifest: AppManifestV1,
        roles_dna_tokens: RolesDnaTokens,
    ) -> ExternResult<Self> {
        if manifest.roles.len() != roles_dna_tokens.len() {
            return Err(guest_error!(format!(
                "Wrong number of DNA Tokens provided ({}); must match manifest roles length ({})",
                roles_dna_tokens.len(),
                manifest.roles.len(),
            )));
        }
        let mut roles_token = Vec::new();

        // Calculate integrity hash by sorting and hashing the DNA token integrity hashes
        let mut dnas_integrity_hashes = manifest.roles.iter()
            .map( |role_manifest| {
                let dna_token = roles_dna_tokens.get( &role_manifest.name )
                    .ok_or(guest_error!(format!(
                        "Missing DNA Token for role name '{}'", role_manifest.name,
                    )))?;

                roles_token.push((
                    role_manifest.name.clone(),
                    RoleToken::new( &dna_token, &role_manifest.dna.modifiers )?
                ));

                Ok( dna_token.integrity_hash.to_vec() )
            })
            .collect::<ExternResult<Vec<Vec<u8>>>>()?;
        dnas_integrity_hashes.sort();
        roles_token.sort();

        let integrity_hash = hash( &dnas_integrity_hashes )?;
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
}
