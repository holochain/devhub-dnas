use crate::{
    hdk,
    hdk_extensions,
    hdi_extensions,
    MY_WEBAPPS_ANCHOR,
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
    WebAppEntry,
    hc_crud::{
        Entity,
        EntryModel,
        create_entity, delete_entity,
    },
};
use apphub_sdk::{
    LinkBase,
    WebAppEntryInput,
    CreateWebAppInput,
};


fn create_webapp_entry_handler(entry: WebAppEntry) -> ExternResult<Entity<WebAppEntry>> {
    let entity = create_entity( &entry )?;

    MY_WEBAPPS_ANCHOR.create_link_if_not_exists( &entity.address, () )?;

    Ok( entity )
}


#[hdk_extern]
fn create_webapp_entry(input: WebAppEntryInput) -> ExternResult<Entity<WebAppEntry>> {
    create_webapp_entry_handler( input.into() )
}


#[hdk_extern]
pub fn create_webapp(input: CreateWebAppInput) -> ExternResult<Entity<WebAppEntry>> {
    create_webapp_entry_handler( input.try_into()? )
}

#[hdk_extern]
pub fn get_webapp_entry(addr: AnyDhtHash) -> ExternResult<Entity<WebAppEntry>> {
    let record = must_get( &addr )?;
    let content = WebAppEntry::try_from_record( &record )?;
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
pub fn get_webapp_entries_for_agent(maybe_agent_id: Option<AgentPubKey>) ->
    ExternResult<Vec<Entity<WebAppEntry>>>
{
    let agent_id = match maybe_agent_id {
        Some(agent_id) => agent_id,
        None => hdk_extensions::agent_id()?,
    };
    let agent_anchor = LinkBase::new( agent_id, LinkTypes::WebApp );
    let webapps = agent_anchor.get_links( None )?.into_iter()
        .filter_map(|link| {
            let addr = link.target.into_entry_hash()?;
            get_webapp_entry( addr.into() ).ok()
        })
        .collect();

    Ok( webapps )
}


#[hdk_extern]
fn delete_webapp(addr: ActionHash) -> ExternResult<ActionHash> {
    Ok( delete_entity::<WebAppEntry,EntryTypes>( &addr )? )
}
