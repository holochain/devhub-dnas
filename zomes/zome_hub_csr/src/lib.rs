use std::collections::BTreeMap;

use devhub_sdk::hdk;
use devhub_sdk::hdk_extensions;

use hdk::prelude::*;
use hdk_extensions::{
    must_get,
};
use devhub_sdk::{
    timestamp,
};
use zome_hub::hdi_extensions::ScopedTypeConnector;
use zome_hub::{
    WasmEntry,
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
    let now = timestamp()?;
    let entry = WasmEntry {
        author: hdk_extensions::agent_id()?,
        mere_memory_addr: input.mere_memory_addr,
        published_at: now,
        last_updated: now,
        metadata: BTreeMap::new(),
    };

    let action_hash = create_entry( entry.to_input() )?;

    Ok( action_hash )
}

#[hdk_extern]
fn get_wasm_entry(addr: ActionHash) -> ExternResult<WasmEntry> {
    let record = must_get( &addr )?;

    Ok( WasmEntry::try_from_record( &record )? )
}
