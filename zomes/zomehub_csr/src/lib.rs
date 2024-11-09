mod group_link_handlers;
mod zome_handlers;
mod zome_package_handlers;
mod zome_package_version_handlers;
mod zome_package_base;

pub use zomehub::hdi;
pub use zomehub::hdi_extensions;
pub use zomehub::hc_crud;
pub use zomehub_sdk::hdk;
pub use zomehub_sdk::hdk_extensions;
pub use zome_package_base::*;

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

    pub static ref ALL_ORGS_ANCHOR_HASH : EntryHash = Path::from( vec![ Component::from("all_orgs".as_bytes().to_vec()) ] )
        .path_entry_hash()
        .expect("Unable to derive all_orgs anchor");
    pub static ref ALL_ORGS_ANCHOR : TypedLinkBase = LinkBase::new( ALL_ORGS_ANCHOR_HASH.clone(), LinkTypes::AllOrgsToGroup );

    pub static ref ALL_AGENTS_ANCHOR_HASH : EntryHash = Path::from( vec![ Component::from("all_agents".as_bytes().to_vec()) ] )
        .path_entry_hash()
        .expect("Unable to derive all_agents anchor");
    pub static ref ALL_AGENTS_ANCHOR : TypedLinkBase = LinkBase::new( ALL_AGENTS_ANCHOR_HASH.clone(), LinkTypes::AllAgentsToAgent );

    pub static ref ALL_ZOME_PACKS_ANCHOR_HASH : EntryHash = Path::from( vec![ Component::from("all_zome_packages".as_bytes().to_vec()) ] )
        .path_entry_hash()
        .expect("Unable to derive all_agents anchor");
    pub static ref ALL_ZOME_PACKS_ANCHOR : TypedLinkBase = LinkBase::new( ALL_ZOME_PACKS_ANCHOR_HASH.clone(), LinkTypes::ZomePackage );

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

    ALL_AGENTS_ANCHOR.create_link_if_not_exists( &agent_info()?.agent_initial_pubkey, () )?;

    Ok(InitCallbackResult::Pass)
}


#[hdk_extern]
fn whoami(_: ()) -> ExternResult<AgentInfo> {
    Ok( agent_info()? )
}


#[hdk_extern]
pub fn query_whole_chain() -> ExternResult<Vec<Record>> {
    Ok(
        query(
            ChainQueryFilter::new()
                .include_entries(true)
        )?
    )
}


#[hdk_extern]
pub fn list_all_agents() -> ExternResult<Vec<AgentPubKey>> {
    let agents = ALL_AGENTS_ANCHOR.get_links( None )?.into_iter()
        .filter_map(|link| {
            link.target.into_agent_pub_key()
        })
        .collect();

    Ok(agents)
}
