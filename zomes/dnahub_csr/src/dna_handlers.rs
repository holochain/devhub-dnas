use crate::{
    hdk,
    hdk_extensions,
    hdi_extensions,
    MY_DNAS_ANCHOR,
};

use hdk::prelude::*;
use hdk_extensions::{
    must_get,
};
use hdi_extensions::{
    ScopedTypeConnector,
};
use dnahub::{
    EntryTypes,
    LinkTypes,
    DnaEntry,
    DnaToken,
    IntegritiesToken,
    CoordinatorsToken,
    hc_crud::{
        Entity,
        EntryModel,
        create_entity, delete_entity,
    },
};
use dnahub_sdk::{
    LinkBase,
    DnaEntryInput,
    CreateDnaInput,
    DnaAsset,
};



fn create_dna_entry_handler(entry: DnaEntry) -> ExternResult<Entity<DnaEntry>> {
    let entity = create_entity( &entry )?;

    MY_DNAS_ANCHOR.create_link_if_not_exists( &entity.address, () )?;

    Ok( entity )
}


#[hdk_extern]
fn create_dna_entry(input: DnaEntryInput) -> ExternResult<Entity<DnaEntry>> {
    create_dna_entry_handler( input.into() )
}


#[hdk_extern]
fn create_dna(input: CreateDnaInput) -> ExternResult<Entity<DnaEntry>> {
    create_dna_entry_handler( input.try_into()? )
}


#[hdk_extern]
fn derive_dna_token(input: CreateDnaInput) -> ExternResult<DnaToken> {
    input.deserialized_manifest()?.dna_token( &input.asset_hashes )
}


#[hdk_extern]
fn derive_integrities_token(input: CreateDnaInput) -> ExternResult<IntegritiesToken> {
    input.deserialized_manifest()?.integrities_token( &input.asset_hashes )
}


#[hdk_extern]
fn derive_coordinators_token(input: CreateDnaInput) -> ExternResult<CoordinatorsToken> {
    input.deserialized_manifest()?.coordinators_token()
}


#[hdk_extern]
fn get_dna_entry(addr: AnyDhtHash) -> ExternResult<Entity<DnaEntry>> {
    let record = must_get( &addr )?;
    let content = DnaEntry::try_from_record( &record )?;
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
fn get_dna_asset(addr: EntryHash) -> ExternResult<DnaAsset> {
    Ok( addr.try_into()? )
}


#[hdk_extern]
fn get_dna_entries_for_agent(maybe_agent_id: Option<AgentPubKey>) ->
    ExternResult<Vec<Entity<DnaEntry>>>
{
    let agent_id = match maybe_agent_id {
        Some(agent_id) => agent_id,
        None => hdk_extensions::agent_id()?,
    };
    let agent_anchor = LinkBase::new( agent_id, LinkTypes::Dna );

    let dnas = agent_anchor.get_links( None )?.into_iter()
        .filter_map(|link| {
            let addr = link.target.into_entry_hash()?;
            get_dna_entry( addr.into() ).ok()
        })
        .collect();

    Ok( dnas )
}


#[hdk_extern]
fn delete_dna(addr: ActionHash) -> ExternResult<ActionHash> {
    Ok( delete_entity::<DnaEntry,EntryTypes>( &addr )? )
}
