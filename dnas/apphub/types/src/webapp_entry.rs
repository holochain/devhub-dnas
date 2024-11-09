use crate::{
    hdi,
    hdi_extensions,
    hash,
    AppEntry,
    WebAppToken,
    WebAppResourcesMap,
    holochain_types::{
        WebAppManifestV1,
    },
};

use std::io::Cursor;
use hdi::prelude::*;
use hdi_extensions::{
    guest_error,
};


//
// WebApp Entry
//
#[hdk_entry_helper]
#[derive(Clone)]
pub struct WebAppEntry {
    pub manifest: rmpv::Value,
    pub resources: WebAppResourcesMap,

    pub webapp_token: WebAppToken,
}

impl WebAppEntry {
    pub fn new(
        manifest: rmpv::Value,
        resources: WebAppResourcesMap,
    ) -> ExternResult<Self> {
        let webapp_manifest = WebAppEntry::deserialize_manifest( &manifest )?;
        let app_entry_addr = resources.get( &webapp_manifest.happ_manifest.bundled )
            .ok_or(guest_error!(format!(
                "WebAppEntry does not have resource for path '{}'",
                webapp_manifest.happ_manifest.bundled,
            )))?;
        let ui_entry_addr = resources.get( &webapp_manifest.ui.bundled )
            .ok_or(guest_error!(format!(
                "WebAppEntry does not have resource for path '{}'",
                webapp_manifest.ui.bundled,
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

    pub fn deserialize_manifest(manifest: &rmpv::Value) -> ExternResult<WebAppManifestV1> {
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

    pub fn deserialized_manifest(&self) -> ExternResult<WebAppManifestV1> {
        WebAppEntry::deserialize_manifest( &self.manifest )
    }

    pub fn app_entry_addr(&self) -> ExternResult<EntryHash> {
        let manifest = self.deserialized_manifest()?;

        Ok(
            self.resources.get( &manifest.happ_manifest.bundled )
                .ok_or(guest_error!(format!(
                    "WebAppEntry does not have resource for path '{}'",
                    manifest.happ_manifest.bundled,
                )))?
                .to_owned()
        )
    }

    pub fn ui_entry_addr(&self) -> ExternResult<EntryHash> {
        let manifest = self.deserialized_manifest()?;

        Ok(
            self.resources.get( &manifest.ui.bundled )
                .ok_or(guest_error!(format!(
                    "WebAppEntry does not have resource for path '{}'",
                    manifest.ui.bundled,
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
