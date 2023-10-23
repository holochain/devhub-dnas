use crate::hdk;
use crate::hdk_extensions;
use crate::hdi_extensions;

use hdk::prelude::*;
use hdk_extensions::{
    must_get,
};
use hdi_extensions::{
    ScopedTypeConnector,
};
use apphub::{
    LinkTypes,
    UiEntry,
};



#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CreateUiEntryInput {
    pub mere_memory_addr: EntryHash,
}

#[hdk_extern]
fn create_ui_entry(input: CreateUiEntryInput) -> ExternResult<ActionHash> {
    let agent_id = hdk_extensions::agent_id()?;
    let entry = UiEntry::new( input.mere_memory_addr )?;

    let action_hash = create_entry( entry.to_input() )?;

    create_link( agent_id, action_hash.clone(), LinkTypes::Ui, () )?;

    Ok( action_hash )
}

#[hdk_extern]
fn get_ui_entry(addr: ActionHash) -> ExternResult<UiEntry> {
    let record = must_get( &addr )?;

    Ok( UiEntry::try_from_record( &record )? )
}

#[hdk_extern]
fn get_ui_entries_for_agent(maybe_agent_id: Option<AgentPubKey>) -> ExternResult<Vec<UiEntry>> {
    let agent_id = match maybe_agent_id {
        Some(agent_id) => agent_id,
        None => hdk_extensions::agent_id()?,
    };
    let uis = get_links( agent_id, LinkTypes::Ui, None )?.into_iter()
        .filter_map(|link| {
            link.target.into_action_hash()
        })
        .map(|ui_addr| {
            get_ui_entry( ui_addr )
        })
        .collect::<ExternResult<Vec<UiEntry>>>()?;

    Ok( uis )
}
