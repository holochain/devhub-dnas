use devhub_types::{
    DevHubResponse, Entity, EntityResponse, EntityCollectionResponse, GetEntityInput, FilterInput,
    constants::{ VALUE_MD, ENTITY_MD, ENTITY_COLLECTION_MD },
    happ_entry_types::{
	HappEntry, HappInfo,
	HappReleaseEntry, HappReleaseInfo,
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


use constants::{
    TAG_HAPP,
    TAG_HAPP_RELEASE,
    ANCHOR_HAPPS,
};


entry_defs![
    PathEntry::entry_def(),
    HappEntry::entry_def(),
    HappReleaseEntry::entry_def()
];


#[derive(Debug, Deserialize)]
pub struct GetAgentItemsInput {
    pub agent: Option<AgentPubKey>,
}


pub fn agent_path_base(pubkey: Option<AgentPubKey>) -> String {
    match agent_info() {
	Ok(agent_info) => format!("{}", pubkey.unwrap_or( agent_info.agent_initial_pubkey ) ),
	Err(_) => String::from("unknown_agent"),
    }
}


#[hdk_extern]
fn init(_: ()) -> ExternResult<InitCallbackResult> {
    let agent_path = agent_path_base( None );
    debug!("Ensure the agent '{}' root path exists", agent_path );
    devhub_types::ensure_path( &agent_path, Vec::<String>::new() )?;

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
fn get_happs(input: GetAgentItemsInput) -> ExternResult<EntityCollectionResponse<HappEntry>> {
    let (base_path, _) = devhub_types::create_path( &agent_path_base( input.agent ), vec![ ANCHOR_HAPPS ] );
    let collection = catch!( devhub_types::get_entities_for_path_filtered( TAG_HAPP.into(), base_path, |items : Vec<Entity<HappEntry>>| {
	Ok( items.into_iter()
	    .filter(|entity| {
		entity.content.deprecation.is_none()
	    })
	    .collect() )
    }) );

    Ok(composition( collection, ENTITY_COLLECTION_MD ))
}

#[hdk_extern]
fn get_my_happs(_:()) -> ExternResult<EntityCollectionResponse<HappEntry>> {
    get_happs( GetAgentItemsInput {
	agent: None
    })
}

#[hdk_extern]
fn get_happs_by_filter( input: FilterInput ) -> ExternResult<EntityCollectionResponse<HappEntry>> {
    let collection = catch!( devhub_types::get_by_filter( TAG_HAPP.into(), input.filter, input.keyword ) );

    Ok(composition( collection, ENTITY_COLLECTION_MD ))
}

#[hdk_extern]
fn get_happs_by_tags( input: Vec<String> ) -> ExternResult<DevHubResponse<Vec<Entity<HappEntry>>>> {
    let list = catch!( devhub_types::get_by_tags( TAG_HAPP.into(), input ) );

    Ok(composition( list, VALUE_MD ))
}

#[hdk_extern]
fn get_all_happs(_:()) -> ExternResult<EntityCollectionResponse<HappEntry>> {
    let (base_path, _) = devhub_types::create_path( ANCHOR_HAPPS, Vec::<String>::new() );
    let collection = catch!( devhub_types::get_entities_for_path( TAG_HAPP.into(), base_path ) );

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
fn get_happ_releases(input: happ_release::GetHappReleasesInput) -> ExternResult<EntityCollectionResponse<HappReleaseEntry>> {
    let collection = catch!( happ_release::get_happ_releases( input ) );

    Ok(composition( collection, ENTITY_COLLECTION_MD ))
}

#[hdk_extern]
fn get_happ_releases_by_filter( input: FilterInput ) -> ExternResult<EntityCollectionResponse<HappReleaseEntry>> {
    let collection = catch!( devhub_types::get_by_filter( TAG_HAPP_RELEASE.into(), input.filter, input.keyword ) );

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

#[hdk_extern]
fn get_webhapp_package(input: packaging::GetWebHappPackageInput) -> ExternResult<DevHubResponse<Vec<u8>>> {
    let value = catch!( packaging::get_webhapp_package( input ) );

    Ok(composition( value, VALUE_MD ))
}
