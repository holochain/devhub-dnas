mod app_handlers;
mod ui_handlers;
mod webapp_handlers;
mod webapp_package_handlers;
mod webapp_package_version_handlers;
mod webapp_package_base;


pub use apphub::hdi;
pub use apphub::hdi_extensions;
pub use apphub::hc_crud;
pub use apphub_sdk::hdk;
pub use apphub_sdk::hdk_extensions;
pub use webapp_package_base::*;

use lazy_static::lazy_static;
use hdk::prelude::*;
use hdk_extensions::{
    agent_id,
};
use apphub::{
    LinkTypes,
};
use apphub_sdk::{
    PathInput,
    LinkBase,
};


pub type TypedLinkBase = LinkBase<LinkTypes>;

lazy_static! {
    pub static ref AGENT_ID : AgentPubKey = agent_id().expect("Unable to obtain current Agent context");

    pub static ref ALL_WEBAPP_PACKS_ANCHOR	: TypedLinkBase = LinkBase::try_from(
        ( PathInput::from("all.webapp_packages"), LinkTypes::WebAppPackage )
    ).unwrap();

    pub static ref MY_APPS_ANCHOR		: TypedLinkBase = LinkBase::new( AGENT_ID.clone(), LinkTypes::App );
    pub static ref MY_UIS_ANCHOR		: TypedLinkBase = LinkBase::new( AGENT_ID.clone(), LinkTypes::Ui );
    pub static ref MY_WEBAPPS_ANCHOR		: TypedLinkBase = LinkBase::new( AGENT_ID.clone(), LinkTypes::WebApp );
    pub static ref MY_WEBAPP_PACKS_ANCHOR	: TypedLinkBase = LinkBase::new( AGENT_ID.clone(), LinkTypes::WebAppPackage );
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
