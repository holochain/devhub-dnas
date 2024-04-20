mod zome_handlers;
mod zome_package_handlers;

use zomehub_sdk::hdk;
use zomehub_sdk::hdk_extensions;

use lazy_static::lazy_static;
use hdk::prelude::*;
use hdk_extensions::{
    agent_id,
};
use zomehub::{
    LinkTypes,
};
use zomehub_sdk::{
    LinkBase,
};


pub type TypedLinkBase = LinkBase<LinkTypes>;

lazy_static! {
    pub static ref AGENT_ID : AgentPubKey = agent_id().expect("Unable to obtain current Agent context");

    pub static ref MY_ZOMES_ANCHOR : TypedLinkBase = LinkBase::new( AGENT_ID.clone(), LinkTypes::AgentToZome );
    pub static ref MY_ZOME_PACKS_ANCHOR : TypedLinkBase = LinkBase::new( AGENT_ID.clone(), LinkTypes::AgentToZomePackage );
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
        ( "mere_memory_api", "get_memory_bytes" ),
        ( "mere_memory_api", "get_memory_with_bytes" ),
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
