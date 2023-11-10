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
    WasmEntry,
    WasmType,
    hc_crud::{
        Entity,
        EntryModel,
        create_entity, delete_entity,
    },
};
use zomehub_sdk::{
    LinkBase,
};


pub type TypedLinkBase = LinkBase<LinkTypes>;

lazy_static! {
    pub static ref AGENT_ID : AgentPubKey = agent_id().expect("Unable to obtain current Agent context");

    pub static ref MY_ZOMES_ANCHOR : TypedLinkBase = LinkBase::new( AGENT_ID.clone(), LinkTypes::Wasm );
}


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


#[hdk_extern]
fn create_wasm_entry(input: WasmEntry) -> ExternResult<Entity<WasmEntry>> {
    let entity = create_entity( &input )?;

    MY_ZOMES_ANCHOR.create_link_if_not_exists( &entity.address, () )?;

    Ok( entity )
}


#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CreateWasmEntryInput {
    pub wasm_type: WasmType,
    pub mere_memory_addr: EntryHash,
}

#[hdk_extern]
fn create_wasm(input: CreateWasmEntryInput) -> ExternResult<Entity<WasmEntry>> {
    let entry = WasmEntry::new( input.wasm_type, input.mere_memory_addr )?;

    create_wasm_entry( entry )
}


#[hdk_extern]
fn get_wasm_entry(addr: AnyDhtHash) -> ExternResult<Entity<WasmEntry>> {
    let record = must_get( &addr )?;
    let content = WasmEntry::try_from_record( &record )?;
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
fn get_wasm_entries_for_agent(maybe_agent_id: Option<AgentPubKey>) ->
    ExternResult<Vec<Entity<WasmEntry>>>
{
    let agent_id = match maybe_agent_id {
        Some(agent_id) => agent_id,
        None => hdk_extensions::agent_id()?,
    };
    let agent_anchor = LinkBase::new( agent_id, LinkTypes::Wasm );

    let wasms = agent_anchor.get_links( None )?.into_iter()
        .filter_map(|link| {
            let addr = link.target.into_entry_hash()?;
            get_wasm_entry( addr.into() ).ok()
        })
        .collect();

    Ok( wasms )
}


#[hdk_extern]
fn delete_wasm(addr: ActionHash) -> ExternResult<ActionHash> {
    Ok( delete_entity::<WasmEntry,EntryTypes>( &addr )? )
}
