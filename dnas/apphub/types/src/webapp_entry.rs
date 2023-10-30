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
    pub fn new(
        manifest: WebAppManifestV1,
    ) -> ExternResult<Self> {
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
