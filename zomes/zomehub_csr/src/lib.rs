use zomehub_sdk::hdk;
use zomehub_sdk::hdk_extensions;

use lazy_static::lazy_static;
use hdk::prelude::*;
use hdk_extensions::{
    agent_id,
    must_get,
    hdi_extensions::{
        guest_error,
        ScopedTypeConnector,
    },
};
use zomehub::{
    EntryTypes,
    LinkTypes,
    ZomeEntry,
    ZomeType,
    hc_crud::{
        Entity,
        EntryModel,
        create_entity, delete_entity,
    },
};
use zomehub_sdk::{
    LinkBase,
    ZomeAsset,
};


pub type TypedLinkBase = LinkBase<LinkTypes>;

lazy_static! {
    pub static ref AGENT_ID : AgentPubKey = agent_id().expect("Unable to obtain current Agent context");

    pub static ref MY_ZOMES_ANCHOR : TypedLinkBase = LinkBase::new( AGENT_ID.clone(), LinkTypes::Zome );
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


#[hdk_extern]
fn create_zome_entry(input: ZomeEntry) -> ExternResult<Entity<ZomeEntry>> {
    let entity = create_entity( &input )?;

    MY_ZOMES_ANCHOR.create_link_if_not_exists( &entity.address, () )?;

    Ok( entity )
}


#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CreateZomeEntryInput {
    pub zome_type: ZomeType,
    pub mere_memory_addr: EntryHash,
}

#[hdk_extern]
fn create_zome(input: CreateZomeEntryInput) -> ExternResult<Entity<ZomeEntry>> {
    let entry = ZomeEntry::new( input.zome_type, input.mere_memory_addr )?;

    create_zome_entry( entry )
}


#[hdk_extern]
fn get_zome_entry(addr: AnyDhtHash) -> ExternResult<Entity<ZomeEntry>> {
    let record = must_get( &addr )?;
    let content = ZomeEntry::try_from_record( &record )?;
    let id = record.action_address().to_owned();
    let addr = hash_entry( content.clone() )?;

    Ok(
        Entity {
            id: id.clone(),
            action: id,
	    address: addr,
	    ctype: content.get_type(),
	    content: content,
        }
    )
}


#[hdk_extern]
fn get_zome_asset(addr: EntryHash) -> ExternResult<ZomeAsset> {
    Ok( addr.try_into()? )
}


#[hdk_extern]
fn get_zome_entries_for_agent(maybe_agent_id: Option<AgentPubKey>) ->
    ExternResult<Vec<Entity<ZomeEntry>>>
{
    let agent_id = match maybe_agent_id {
        Some(agent_id) => agent_id,
        None => hdk_extensions::agent_id()?,
    };
    let agent_anchor = LinkBase::new( agent_id, LinkTypes::Zome );

    let zomes = agent_anchor.get_links( None )?.into_iter()
        .filter_map(|link| {
            let addr = link.target.into_entry_hash()?;
            get_zome_entry( addr.into() ).ok()
        })
        .collect();

    Ok( zomes )
}


#[hdk_extern]
fn delete_zome(addr: ActionHash) -> ExternResult<ActionHash> {
    Ok( delete_entity::<ZomeEntry,EntryTypes>( &addr )? )
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
