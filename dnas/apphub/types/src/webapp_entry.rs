use crate::{
    hdi,
    hash,
    AppEntry,
    WebAppToken,
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
        let app_entry_addr = manifest.happ_manifest.app_entry.clone();
        let ui_entry_addr = manifest.ui.ui_entry.clone();

        let webapp_token = Self::create_webapp_token( &app_entry_addr, &ui_entry_addr )?;

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

        Ok(
            WebAppToken {
                ui_hash: hash( &ui_entry )?,
                app_token: app_entry.calculate_app_token()?,
            }
        )
    }

    pub fn integrity_hash(&self) -> Vec<u8> {
        self.webapp_token.app_token.integrity_hash.clone()
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
}
