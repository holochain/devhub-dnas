use crate::{
    hdi,
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
