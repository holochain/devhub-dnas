use crate::{
    hdk,
    hdk_extensions,
    hdi_extensions,
    MY_APPS_ANCHOR,
};

use hdk::prelude::*;
use hdk_extensions::{
    must_get,
};
use hdi_extensions::{
    ScopedTypeConnector,
};
use apphub::{
    EntryTypes,
    LinkTypes,
    AppEntry,
    hc_crud::{
        Entity,
        EntryModel,
        create_entity, delete_entity,
    },
};
use apphub_sdk::{
    LinkBase,
    AppEntryInput,
    CreateAppInput,
};


fn create_app_entry_handler(entry: AppEntry) -> ExternResult<Entity<AppEntry>> {
    let entity = create_entity( &entry )?;

    MY_APPS_ANCHOR.create_link_if_not_exists( &entity.address, () )?;

    Ok( entity )
}


#[hdk_extern]
fn create_app_entry(input: AppEntryInput) -> ExternResult<Entity<AppEntry>> {
    create_app_entry_handler( input.into() )
}


#[hdk_extern]
pub fn create_app(input: CreateAppInput) -> ExternResult<Entity<AppEntry>> {
    create_app_entry_handler( input.try_into()? )
}


#[hdk_extern]
pub fn get_app_entry(addr: AnyDhtHash) -> ExternResult<Entity<AppEntry>> {
    let record = must_get( &addr )?;
    let content = AppEntry::try_from_record( &record )?;
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
pub fn get_app_entries_for_agent(maybe_agent_id: Option<AgentPubKey>) ->
    ExternResult<Vec<Entity<AppEntry>>>
{
    let agent_id = match maybe_agent_id {
        Some(agent_id) => agent_id,
        None => hdk_extensions::agent_id()?,
    };
    let agent_anchor = LinkBase::new( agent_id, LinkTypes::App );

    let apps = agent_anchor.get_links( None )?.into_iter()
        .filter_map(|link| {
            let addr = link.target.into_entry_hash()?;
            get_app_entry( addr.into() ).ok()
        })
        .collect();

    Ok( apps )
}


#[hdk_extern]
fn delete_app(addr: ActionHash) -> ExternResult<ActionHash> {
    Ok( delete_entity::<AppEntry,EntryTypes>( &addr )? )
}
