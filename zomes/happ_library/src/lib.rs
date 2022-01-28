use devhub_types::{
    DevHubResponse, EntityResponse, EntityCollectionResponse, GetEntityInput, FilterInput,
    constants::{ VALUE_MD, ENTITY_MD, ENTITY_COLLECTION_MD },
    happ_entry_types::{
	HappEntry, HappInfo, HappSummary,
	HappReleaseEntry, HappReleaseInfo, HappReleaseSummary,
    },
    web_asset_entry_types::{ FileInfo },
    composition,
    catch,
};
use hdk::prelude::*;

mod happ;
mod happ_release;
mod packaging;
mod constants;



entry_defs![
    PathEntry::entry_def(),
    HappEntry::entry_def(),
    HappReleaseEntry::entry_def()
];


pub fn root_path(pubkey: Option<AgentPubKey>) -> ExternResult<Path> {
    let pubkey = pubkey
	.unwrap_or( agent_info()?.agent_initial_pubkey );
    let path = Path::from( format!("{:?}", pubkey ) );

    debug!("Agent ({:?}) root path is: {:?}", pubkey, path.path_entry_hash()? );
    Ok( path )
}
pub fn root_path_hash(pubkey: Option<AgentPubKey>) -> ExternResult<EntryHash> {
    Ok( root_path( pubkey )?.path_entry_hash()? )
}


#[hdk_extern]
fn init(_: ()) -> ExternResult<InitCallbackResult> {
    let agent = agent_info()?.agent_initial_pubkey;
    let path = root_path( Some(agent.to_owned()) )?;

    debug!("Ensure the agent ({:?}) root path is there: {:?}", agent, path.path_entry_hash()? );
    path.ensure()?;

    Ok(InitCallbackResult::Pass)
}


#[hdk_extern]
fn whoami(_: ()) -> ExternResult<DevHubResponse<AgentInfo>> {
    Ok(composition( agent_info()?, VALUE_MD ))
}


// hApps
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
fn get_happs(input: happ::GetHappsInput) -> ExternResult<EntityCollectionResponse<HappSummary>> {
    let collection = catch!( happ::get_happs( input ) );

    Ok(composition( collection, ENTITY_COLLECTION_MD ))
}

#[hdk_extern]
fn get_my_happs(_:()) -> ExternResult<EntityCollectionResponse<HappSummary>> {
    let collection = catch!( happ::get_my_happs() );

    Ok(composition( collection, ENTITY_COLLECTION_MD ))
}

#[hdk_extern]
fn get_happs_by_filter( input: FilterInput ) -> ExternResult<EntityCollectionResponse<HappSummary>> {
    let collection = catch!( happ::get_happs_by_filter( input.filter, input.keyword ) );

    Ok(composition( collection, ENTITY_COLLECTION_MD ))
}


// hApp Releases
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

#[hdk_extern]
fn update_happ_release(input: happ_release::HappReleaseUpdateInput) -> ExternResult<EntityResponse<HappReleaseInfo>> {
    let entity = catch!( happ_release::update_happ_release( input ) );

    Ok(composition( entity, ENTITY_MD ))
}

#[hdk_extern]
fn delete_happ_release(input: happ_release::DeleteInput) -> ExternResult<DevHubResponse<HeaderHash>> {
    let value = catch!( happ_release::delete_happ_release( input ) );

    Ok(composition( value, VALUE_MD ))
}

#[hdk_extern]
fn get_happ_releases(input: happ_release::GetHappReleasesInput) -> ExternResult<EntityCollectionResponse<HappReleaseSummary>> {
    let collection = catch!( happ_release::get_happ_releases( input ) );

    Ok(composition( collection, ENTITY_COLLECTION_MD ))
}

#[hdk_extern]
fn get_happ_releases_by_filter( input: FilterInput ) -> ExternResult<EntityCollectionResponse<HappReleaseSummary>> {
    let collection = catch!( happ_release::get_happ_releases_by_filter( input.filter, input.keyword ) );

    Ok(composition( collection, ENTITY_COLLECTION_MD ))
}


// Packaging
#[hdk_extern]
fn get_gui(input: packaging::GetGUIInput) -> ExternResult<EntityResponse<FileInfo>> {
    let entity = catch!( packaging::get_gui( input ) );

    Ok(composition( entity, ENTITY_MD ))
}

#[hdk_extern]
fn get_release_package(input: packaging::GetReleasePackageInput) -> ExternResult<DevHubResponse<Vec<u8>>> {
    let value = catch!( packaging::get_release_package( input ) );

    Ok(composition( value, VALUE_MD ))
}
