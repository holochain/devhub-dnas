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

    pub static ref MY_APPS_ANCHOR			: TypedLinkBase = LinkBase::new( AGENT_ID.clone(), LinkTypes::AgentToApp );
    pub static ref MY_UIS_ANCHOR			: TypedLinkBase = LinkBase::new( AGENT_ID.clone(), LinkTypes::AgentToUi );
    pub static ref MY_WEBAPPS_ANCHOR			: TypedLinkBase = LinkBase::new( AGENT_ID.clone(), LinkTypes::AgentToWebApp );
    pub static ref MY_WEBAPP_PACKS_ANCHOR		: TypedLinkBase = LinkBase::new( AGENT_ID.clone(), LinkTypes::AgentToWebAppPackage );
    pub static ref MY_WEBAPP_PACK_VERSIONS_ANCHOR	: TypedLinkBase = LinkBase::new( AGENT_ID.clone(), LinkTypes::AgentToWebAppPackageVersion );
}


#[hdk_extern]
fn init(_: ()) -> ExternResult<InitCallbackResult> {
    let zome_settings = zome_info()?;
    let zome_name = zome_settings.name;
    debug!("'{}' init", zome_name );

    let main_functions : Vec<(&str, &str)> = zome_settings.extern_fns.iter()
        .filter_map(|fn_name| match fn_name.as_ref().starts_with("get_") {
            true => Some(( zome_name.0.as_ref(), fn_name.0.as_str() )),
            false => None,
        })
        .collect();
    let mere_memory_functions = vec![
        ( "mere_memory_api", "get_memory_entry" ),
        ( "mere_memory_api", "get_memory_block_entry" ),
        ( "mere_memory_api", "memory_exists" ),
    ];

    portal_sdk::register_if_exists!({
        dna: dna_info()?.hash,
        granted_functions: vec![ main_functions, mere_memory_functions ]
            .into_iter().flatten().collect(),
    })?;

    Ok(InitCallbackResult::Pass)
}


#[hdk_extern]
fn whoami(_: ()) -> ExternResult<AgentInfo> {
    Ok( agent_info()? )
}
