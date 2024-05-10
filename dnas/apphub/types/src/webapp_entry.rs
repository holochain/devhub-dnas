use crate::{
    hdi,
    hdi_extensions,
    hash,
    AppEntry,
    WebAppToken,
    WebAppResourcesMap,
};

use hdi::prelude::*;
use hdi_extensions::{
    guest_error,
};
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
    pub resources: WebAppResourcesMap,

    pub webapp_token: WebAppToken,
}

impl WebAppEntry {
    pub fn new(
        manifest: WebAppManifestV1,
        resources: WebAppResourcesMap,
    ) -> ExternResult<Self> {
        let app_entry_addr = resources.get( &manifest.happ_manifest.bundled )
            .ok_or(guest_error!(format!(
                "WebAppEntry does not have resource for path '{}'",
                manifest.happ_manifest.bundled,
            )))?;
        let ui_entry_addr = resources.get( &manifest.ui.bundled )
            .ok_or(guest_error!(format!(
                "WebAppEntry does not have resource for path '{}'",
                manifest.ui.bundled,
            )))?;

        let webapp_token = Self::create_webapp_token( &app_entry_addr, &ui_entry_addr )?;

        Ok(
            Self {
                manifest,
                resources,
                webapp_token,
            }
        )
    }

    pub fn app_entry_addr(&self) -> ExternResult<EntryHash> {
        Ok(
            self.resources.get( &self.manifest.happ_manifest.bundled )
                .ok_or(guest_error!(format!(
                    "WebAppEntry does not have resource for path '{}'",
                    self.manifest.happ_manifest.bundled,
                )))?
                .to_owned()
        )
    }

    pub fn ui_entry_addr(&self) -> ExternResult<EntryHash> {
        Ok(
            self.resources.get( &self.manifest.ui.bundled )
                .ok_or(guest_error!(format!(
                    "WebAppEntry does not have resource for path '{}'",
                    self.manifest.ui.bundled,
                )))?
                .to_owned()
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
        let app_entry_addr = self.app_entry_addr()?;

        WebAppEntry::create_integrity_hash( &app_entry_addr )
    }

    pub fn calculate_webapp_token(&self) -> ExternResult<WebAppToken> {
        let app_entry_addr = self.app_entry_addr()?;
        let ui_entry_addr = self.ui_entry_addr()?;

        WebAppEntry::create_webapp_token(
            &app_entry_addr,
            &ui_entry_addr
        )
    }
}
