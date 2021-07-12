use devhub_types::{
    DevHubResponse, EntityResponse,
    constants::{ VALUE_MD, ENTITY_MD },
    happ_entry_types::{
	HappEntry, HappInfo,
	HappReleaseEntry, HappReleaseInfo,
    },
    web_asset_entry_types::{ FileInfo },
    composition,
    catch,
};
use hc_entities::{ GetEntityInput };
use hdk::prelude::*;

mod happ;
mod happ_release;
mod constants;



entry_defs![
    HappEntry::entry_def(),
    HappReleaseEntry::entry_def()
];



#[hdk_extern]
fn init(_: ()) -> ExternResult<InitCallbackResult> {
    Ok(InitCallbackResult::Pass)
}


#[hdk_extern]
fn whoami(_: ()) -> ExternResult<DevHubResponse<AgentInfo>> {
    Ok(composition( agent_info()?, VALUE_MD ))
}


#[hdk_extern]
fn create_happ(input: happ::CreateInput) -> ExternResult<EntityResponse<HappInfo>> {
    let entity = catch!( happ::create_happ( input ) );

    Ok(composition( entity, ENTITY_MD ))
}

#[hdk_extern]
fn get_happ(input: GetEntityInput) -> ExternResult<EntityResponse<HappInfo>> {
    let entity = catch!( happ::get_happ( input ) );

    Ok(composition( entity, ENTITY_MD ))
}

#[hdk_extern]
fn update_happ(input: happ::HappUpdateInput) -> ExternResult<EntityResponse<HappInfo>> {
    let entity = catch!( happ::update_happ( input ) );

    Ok(composition( entity, ENTITY_MD ))
}

#[hdk_extern]
fn deprecate_happ(input: happ::HappDeprecateInput) -> ExternResult<EntityResponse<HappInfo>> {
    let entity = catch!( happ::deprecate_happ( input ) );

    Ok(composition( entity, ENTITY_MD ))
}

#[hdk_extern]
fn get_gui(input: happ::GetGUIInput) -> ExternResult<EntityResponse<FileInfo>> {
    let entity = catch!( happ::get_gui( input ) );

    Ok(composition( entity, ENTITY_MD ))
}

#[hdk_extern]
fn create_happ_release(input: happ_release::CreateInput) -> ExternResult<EntityResponse<HappReleaseInfo>> {
    let entity = catch!( happ_release::create_happ_release( input ) );

    Ok(composition( entity, ENTITY_MD ))
}

#[hdk_extern]
fn get_happ_release(input: GetEntityInput) -> ExternResult<EntityResponse<HappReleaseInfo>> {
    let entity = catch!( happ_release::get_happ_release( input ) );

    Ok(composition( entity, ENTITY_MD ))
}
