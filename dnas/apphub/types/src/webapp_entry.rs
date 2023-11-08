use crate::{
    hdi,
    hash,
    AppEntry,
    WebAppToken,
    WebAppAssetsToken,
};

use hdi::prelude::*;
pub use crate::holochain_types::{
    WebAppManifestV1,
};



//
// WebApp Entry
//
#[hdk_entry_helper]
#[derive(Clone)]
pub struct WebAppEntry {
    pub manifest: WebAppManifestV1,

    pub webapp_token: WebAppToken,
}

impl WebAppEntry {
    pub fn new(manifest: WebAppManifestV1) -> ExternResult<Self> {
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

    pub fn create_integrity_hash(app_entry_addr: &EntryHash) -> ExternResult<Vec<u8>> {
        let app_entry : AppEntry = must_get_entry( app_entry_addr.to_owned() )?
            .try_into()?;

        Ok( app_entry.integrity_hash() )
    }

    pub fn create_webapp_token(app_entry_addr: &EntryHash, ui_entry: &EntryHash) -> ExternResult<WebAppToken> {
        let app_entry : AppEntry = must_get_entry( app_entry_addr.to_owned() )?
            .try_into()?;
        let assets_token = &WebAppEntry::create_webapp_assets_token( app_entry_addr, ui_entry )?;

        Ok(
            WebAppToken {
                integrity_hash: app_entry.integrity_hash(),
                assets_token_hash: hash( &assets_token )?,
            }
        )
    }

    pub fn create_webapp_assets_token(app_entry_addr: &EntryHash, ui_entry: &EntryHash) -> ExternResult<WebAppAssetsToken> {
        let app_entry : AppEntry = must_get_entry( app_entry_addr.to_owned() )?
            .try_into()?;

        Ok(
            WebAppAssetsToken {
                ui_hash: hash( &ui_entry )?,
                roles_token_hash: app_entry.roles_token_hash(),
            }
        )
    }

    pub fn integrity_hash(&self) -> Vec<u8> {
        self.webapp_token.integrity_hash.clone()
    }

    pub fn calculate_integrity_hash(&self) -> ExternResult<Vec<u8>> {
        WebAppEntry::create_integrity_hash( &self.manifest.happ_manifest.app_entry )
    }

    pub fn calculate_webapp_token(&self) -> ExternResult<WebAppToken> {
        WebAppEntry::create_webapp_token(
            &self.manifest.happ_manifest.app_entry,
            &self.manifest.ui.ui_entry
        )
    }

    pub fn calculate_webapp_assets_token(&self) -> ExternResult<WebAppAssetsToken> {
        WebAppEntry::create_webapp_assets_token(
            &self.manifest.happ_manifest.app_entry,
            &self.manifest.ui.ui_entry
        )
    }
}
