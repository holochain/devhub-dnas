
mod happ;

mod errors;
mod constants;
mod entry_types;

use hdk::prelude::*;
use entry_types::{ HappEntry };
use devhub_types::{ DevHubResponse, EntityResponse, VALUE_MD, ENTITY_MD };
use hc_dna_utils::catch;


entry_defs![
    HappEntry::entry_def()
];


#[hdk_extern]
fn init(_: ()) -> ExternResult<InitCallbackResult> {
    Ok(InitCallbackResult::Pass)
}


#[hdk_extern]
fn whoami(_: ()) -> ExternResult<DevHubResponse<AgentInfo>> {
    Ok( DevHubResponse::success( agent_info()?, VALUE_MD ) )
}


#[hdk_extern]
fn create_happ(input: happ::CreateInput) -> ExternResult<EntityResponse<entry_types::HappInfo>> {
    let entity = catch!( happ::create_happ( input ) );

    Ok(EntityResponse::success( entity, ENTITY_MD ))
}

#[hdk_extern]
fn update_happ(input: happ::HappUpdateInput) -> ExternResult<EntityResponse<entry_types::HappInfo>> {
    let entity = catch!( happ::update_happ( input ) );

    Ok(EntityResponse::success( entity, ENTITY_MD ))
}
