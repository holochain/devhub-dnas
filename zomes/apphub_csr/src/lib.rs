mod app_handlers;
mod ui_handlers;
mod webapp_handlers;
mod webapp_package_handlers;
mod webapp_package_version_handlers;
mod webapp_package_anchor;


pub use apphub::hdi;
pub use apphub::hdi_extensions;
pub use apphub::hc_crud;
pub use apphub_sdk::hdk;
pub use apphub_sdk::hdk_extensions;
pub use webapp_package_anchor::*;

use lazy_static::lazy_static;
use hdk::prelude::*;
use apphub_sdk::{
    path,
};


const ALL_APPS_PATH : &str = "apps";


lazy_static! {
    pub static ref ALL_APPS_ANCHOR : EntryHash = path( ALL_APPS_PATH ).unwrap().path_entry_hash().unwrap();
}



#[hdk_extern]
fn init(_: ()) -> ExternResult<InitCallbackResult> {
    let zome_name = zome_info()?.name;
    debug!("'{}' init", zome_name );

    portal_sdk::register_if_exists!({
        dna: dna_info()?.hash,
        granted_functions: vec![
            ( zome_name.0.as_ref(), "get_app_entry" ),
            ( zome_name.0.as_ref(), "get_app_entries_for_agent" ),

            ( zome_name.0.as_ref(), "get_ui_entry" ),
            ( zome_name.0.as_ref(), "get_ui_entries_for_agent" ),

            ( zome_name.0.as_ref(), "get_webapp_entry" ),
            ( zome_name.0.as_ref(), "get_webapp_entries_for_agent" ),

            ( zome_name.0.as_ref(), "get_webapp_package_entry" ),
            ( zome_name.0.as_ref(), "get_webapp_package_versions" ),
            ( zome_name.0.as_ref(), "get_all_webapp_package_entries" ),

            ( zome_name.0.as_ref(), "get_webapp_package_version_entry" ),
        ],
    })?;

    Ok(InitCallbackResult::Pass)
}


#[hdk_extern]
fn whoami(_: ()) -> ExternResult<AgentInfo> {
    Ok( agent_info()? )
}
