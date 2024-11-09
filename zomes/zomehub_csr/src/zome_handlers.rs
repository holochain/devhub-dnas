use crate::{
    hdk,
    hdk_extensions,
    MY_ZOMES_ANCHOR,
};
use hdk::prelude::*;
use hdk_extensions::{
    must_get,
    hdi_extensions::{
        ScopedTypeConnector,
    },
};
use zomehub::{
    EntryTypes,
    LinkTypes,

    ZomeType,
    ZomeEntry,
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
    let agent_anchor = LinkBase::new( agent_id, LinkTypes::AgentToZome );

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
