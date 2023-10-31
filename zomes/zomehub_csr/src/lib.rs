use devhub_sdk::hdk;
use devhub_sdk::hdk_extensions;

use hdk::prelude::*;
use hdk_extensions::{
    must_get,
};
use zomehub::hdi_extensions::{
    ScopedTypeConnector,
};
use zomehub::{
    WasmEntry,
    WasmType,
    LinkTypes,
};



#[hdk_extern]
fn init(_: ()) -> ExternResult<InitCallbackResult> {
    let zome_name = zome_info()?.name;
    debug!("'{}' init", zome_name );

    portal_sdk::register_if_exists!({
        dna: dna_info()?.hash,
        granted_functions: vec![
            ( zome_name.0.as_ref(), "get_wasm_entry" ),
            ( zome_name.0.as_ref(), "get_wasm_entries_for_agent" ),
        ],
    })?;

    Ok(InitCallbackResult::Pass)
}


#[hdk_extern]
fn whoami(_: ()) -> ExternResult<AgentInfo> {
    Ok( agent_info()? )
}


#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CreateWasmEntryInput {
    pub wasm_type: WasmType,
    pub mere_memory_addr: EntryHash,
}

#[hdk_extern]
fn create_wasm_entry(input: CreateWasmEntryInput) -> ExternResult<EntryHash> {
    let agent_id = hdk_extensions::agent_id()?;
    let entry = WasmEntry::new( input.wasm_type, input.mere_memory_addr )?;

    let entry_hash = hash_entry( entry.clone() )?;
    create_entry( entry.to_input() )?;

    create_link( agent_id, entry_hash.clone(), LinkTypes::Wasm, () )?;

    Ok( entry_hash )
}

#[hdk_extern]
fn get_wasm_entry(addr: AnyDhtHash) -> ExternResult<WasmEntry> {
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
            let addr = link.target.into_entry_hash()?;
            get_wasm_entry( addr.into() ).ok()
        })
        .collect();

    Ok( wasms )
}
