use devhub_sdk::hdk;
use devhub_sdk::hdk_extensions;

use hdk::prelude::*;
use hdk_extensions::{
    must_get,
};
use zome_hub::hdi_extensions::{
    ScopedTypeConnector,
    // AnyLinkableHashTransformer,
};
use zome_hub::{
    WasmEntry,
    LinkTypes,
};



#[hdk_extern]
fn init(_: ()) -> ExternResult<InitCallbackResult> {
    Ok(InitCallbackResult::Pass)
}


#[hdk_extern]
fn whoami(_: ()) -> ExternResult<AgentInfo> {
    Ok( agent_info()? )
}


#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CreateWasmEntryInput {
    pub mere_memory_addr: EntryHash,
}

#[hdk_extern]
fn create_wasm_entry(input: CreateWasmEntryInput) -> ExternResult<ActionHash> {
    let agent_id = hdk_extensions::agent_id()?;
    let entry = WasmEntry::new_integrity( input.mere_memory_addr )?;

    let action_hash = create_entry( entry.to_input() )?;

    create_link( agent_id, action_hash.clone(), LinkTypes::Wasm, () )?;

    Ok( action_hash )
}

#[hdk_extern]
fn get_wasm_entry(addr: ActionHash) -> ExternResult<WasmEntry> {
    let record = must_get( &addr )?;

    Ok( WasmEntry::try_from_record( &record )? )
}

#[hdk_extern]
fn get_wasm_entries_for_agent(maybe_agent_id: Option<AgentPubKey>) -> ExternResult<Vec<WasmEntry>> {
    let agent_id = match maybe_agent_id {
        Some(agent_id) => agent_id,
        None => hdk_extensions::agent_id()?,
    };
    let wasms = get_links( agent_id, LinkTypes::Wasm, None )?.into_iter()
        .filter_map(|link| {
            link.target.into_action_hash()
        })
        .map(|wasm_addr| {
            get_wasm_entry( wasm_addr )
        })
        .collect::<ExternResult<Vec<WasmEntry>>>()?;

    Ok( wasms )
}
