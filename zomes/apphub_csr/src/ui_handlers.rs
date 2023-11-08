use crate::{
    hdk,
    hdk_extensions,
    hdi_extensions,
    MY_UIS_ANCHOR,
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
    UiEntry,
    hc_crud::{
        Entity,
        EntryModel,
        create_entity, delete_entity,
    },
};


fn create_ui_entry_handler(entry: UiEntry) -> ExternResult<Entity<UiEntry>> {
    let entity = create_entity( &entry )?;

    MY_UIS_ANCHOR.create_link_if_not_exists( &entity.address, () )?;

    Ok( entity )
}


#[hdk_extern]
fn create_ui_entry(input: UiEntry) -> ExternResult<Entity<UiEntry>> {
    create_ui_entry_handler( input )
}


#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CreateUiEntryInput {
    pub mere_memory_addr: EntryHash,
}

#[hdk_extern]
pub fn create_ui(input: CreateUiEntryInput) -> ExternResult<Entity<UiEntry>> {
    let entry = UiEntry::new( input.mere_memory_addr )?;

    create_ui_entry_handler( entry )
}


#[hdk_extern]
pub fn get_ui_entry(addr: AnyDhtHash) -> ExternResult<Entity<UiEntry>> {
    let record = must_get( &addr )?;
    let content = UiEntry::try_from_record( &record )?;
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
pub fn get_ui_entries_for_agent(maybe_agent_id: Option<AgentPubKey>) -> ExternResult<Vec<Entity<UiEntry>>> {
    let agent_id = match maybe_agent_id {
        Some(agent_id) => agent_id,
        None => hdk_extensions::agent_id()?,
    };
    let uis = get_links( agent_id, LinkTypes::Ui, None )?.into_iter()
        .filter_map(|link| {
            let addr = link.target.into_entry_hash()?;
            get_ui_entry( addr.into() ).ok()
        })
        .collect();

    Ok( uis )
}


#[hdk_extern]
fn delete_ui(addr: ActionHash) -> ExternResult<ActionHash> {
    Ok( delete_entity::<UiEntry,EntryTypes>( &addr )? )
}
