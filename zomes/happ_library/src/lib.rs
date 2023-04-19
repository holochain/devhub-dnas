use happs_core::{
    LinkTypes,
};
use devhub_types::{
    DevHubResponse, AppResult, ErrorKinds,
    Entity, EntityResponse, GetEntityInput, FilterInput,
    constants::{ VALUE_MD, ENTITY_MD, ENTITY_COLLECTION_MD },
    happ_entry_types::{
	HappEntry,
	HappReleaseEntry,

	GUIEntry,
	GUIReleaseEntry,
    },
    web_asset_entry_types::{
	FilePackage,
    },
    call_local_dna_zome,
    composition,
    catch,
};
use hdk::prelude::*;
use holo_hash::DnaHash;

mod happ;
mod happ_release;
mod gui;
mod gui_release;

mod packaging;
mod constants;


use constants::{
    ANCHOR_HAPPS,
    ANCHOR_GUIS,
};



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

#[derive(Debug, Serialize, Deserialize)]
#[allow(non_snake_case)]
pub struct ListedFunctions {
    Listed: Vec<(String, String)>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RegisterHostInput {
    pub dna: DnaHash,
    pub granted_functions: ListedFunctions,
}

#[hdk_extern]
fn init(_: ()) -> ExternResult<InitCallbackResult> {
    let info = dna_info()?;
    let result : AppResult<serde_yaml::Value> = call_local_dna_zome( "portal", "portal_api", "register_host", RegisterHostInput {
	dna: info.hash,
	granted_functions: ListedFunctions {
	    Listed: vec![
		( "happ_library".to_string(), "get_release_package".to_string() ),
		( "happ_library".to_string(), "get_webhapp_package".to_string() ),
		( "happ_library".to_string(), "get_happ".to_string() ),
		( "happ_library".to_string(), "get_happ_release".to_string() ),
		( "happ_library".to_string(), "get_happ_releases".to_string() ),
		( "happ_library".to_string(), "get_gui".to_string() ),
		( "happ_library".to_string(), "get_gui_release".to_string() ),
		( "happ_library".to_string(), "get_gui_releases".to_string() ),
		( "happ_library".to_string(), "get_webasset".to_string() ),
	    ],
	},
    });

    if let Err(err) = result {
	if let ErrorKinds::HDKError(WasmError { error, .. }) = &err {
	    match error {
		WasmErrorInner::Host(msg) => match msg.contains("Role not found") {
		    true => (),
		    false => Err(err)?,
		},
		_ => Err(err)?,
	    }
	}
    }

    Ok(InitCallbackResult::Pass)
}


#[hdk_extern]
fn whoami(_: ()) -> ExternResult<DevHubResponse<AgentInfo>> {
    Ok(composition( agent_info()?, VALUE_MD ))
}


// hApps
#[hdk_extern]
fn create_happ(input: happ::CreateInput) -> ExternResult<EntityResponse<HappEntry>> {
    let entity = catch!( happ::create_happ( input ) );

    Ok(composition( entity, ENTITY_MD ))
}

#[hdk_extern]
fn get_happ(input: GetEntityInput) -> ExternResult<EntityResponse<HappEntry>> {
    let entity = catch!( happ::get_happ( input ) );

    Ok(composition( entity, ENTITY_MD ))
}

#[hdk_extern]
fn update_happ(input: happ::HappUpdateInput) -> ExternResult<EntityResponse<HappEntry>> {
    let entity = catch!( happ::update_happ( input ) );

    Ok(composition( entity, ENTITY_MD ))
}

#[hdk_extern]
fn deprecate_happ(input: happ::HappDeprecateInput) -> ExternResult<EntityResponse<HappEntry>> {
    let entity = catch!( happ::deprecate_happ( input ) );

    Ok(composition( entity, ENTITY_MD ))
}

#[hdk_extern]
fn get_happs(input: GetAgentItemsInput) -> ExternResult<DevHubResponse<Vec<Entity<HappEntry>>>> {
    let (base_path, _) = devhub_types::create_path( &agent_path_base( input.agent ), vec![ ANCHOR_HAPPS ] );
    let collection = catch!( devhub_types::get_entities_for_path_filtered( base_path, LinkTypes::Happ, None, |items : Vec<Entity<HappEntry>>| {
	Ok( items.into_iter()
	    .filter(|entity| {
		entity.content.deprecation.is_none()
	    })
	    .collect() )
    }) );

    Ok(composition( collection, ENTITY_COLLECTION_MD ))
}

#[hdk_extern]
fn get_my_happs(_:()) -> ExternResult<DevHubResponse<Vec<Entity<HappEntry>>>> {
    get_happs( GetAgentItemsInput {
	agent: None
    })
}

#[hdk_extern]
fn get_happs_by_filter( input: FilterInput ) -> ExternResult<DevHubResponse<Vec<Entity<HappEntry>>>> {
    let collection = catch!( devhub_types::get_by_filter( LinkTypes::Happ, input.filter, input.keyword ) );

    Ok(composition(
	collection.into_iter()
	    .filter(|entity: &Entity<HappEntry>| {
		entity.content.deprecation.is_none()
	    })
	    .collect(),
	ENTITY_COLLECTION_MD
    ))
}

#[hdk_extern]
fn get_happs_by_tags( input: Vec<String> ) -> ExternResult<DevHubResponse<Vec<Entity<HappEntry>>>> {
    let list = catch!( devhub_types::get_by_tags( LinkTypes::Happ, input ) );

    Ok(composition( list.into_iter()
		    .filter(|entity: &Entity<HappEntry>| {
			entity.content.deprecation.is_none()
		    })
		    .collect(), VALUE_MD ))
}

#[hdk_extern]
fn get_all_happs(_:()) -> ExternResult<DevHubResponse<Vec<Entity<HappEntry>>>> {
    let (base_path, _) = devhub_types::create_path( ANCHOR_HAPPS, Vec::<String>::new() );
    let collection = catch!( devhub_types::get_entities_for_path_filtered( base_path, LinkTypes::Happ, None, |items : Vec<Entity<HappEntry>>| {
	Ok( items.into_iter()
	    .filter(|entity| {
		entity.content.deprecation.is_none()
	    })
	    .collect() )
    }) );

    Ok(composition( collection, ENTITY_COLLECTION_MD ))
}


// hApp Releases
#[hdk_extern]
fn create_happ_release(input: happ_release::CreateInput) -> ExternResult<EntityResponse<HappReleaseEntry>> {
    let entity = catch!( happ_release::create_happ_release( input ) );

    Ok(composition( entity, ENTITY_MD ))
}

#[hdk_extern]
fn get_happ_release(input: GetEntityInput) -> ExternResult<EntityResponse<HappReleaseEntry>> {
    let entity = catch!( happ_release::get_happ_release( input ) );

    Ok(composition( entity, ENTITY_MD ))
}

#[hdk_extern]
fn update_happ_release(input: happ_release::HappReleaseUpdateInput) -> ExternResult<EntityResponse<HappReleaseEntry>> {
    let entity = catch!( happ_release::update_happ_release( input ) );

    Ok(composition( entity, ENTITY_MD ))
}

#[hdk_extern]
fn delete_happ_release(input: happ_release::DeleteInput) -> ExternResult<DevHubResponse<ActionHash>> {
    let value = catch!( happ_release::delete_happ_release( input ) );

    Ok(composition( value, VALUE_MD ))
}

#[hdk_extern]
fn get_happ_releases(input: happ_release::GetHappReleasesInput) -> ExternResult<DevHubResponse<Vec<Entity<HappReleaseEntry>>>> {
    let collection = catch!( happ_release::get_happ_releases( input ) );

    Ok(composition( collection, ENTITY_COLLECTION_MD ))
}

#[hdk_extern]
fn get_happ_releases_by_filter( input: FilterInput ) -> ExternResult<DevHubResponse<Vec<Entity<HappReleaseEntry>>>> {
    let collection = catch!( devhub_types::get_by_filter( LinkTypes::HappRelease, input.filter, input.keyword ) );

    Ok(composition( collection, ENTITY_COLLECTION_MD ))
}


// GUI
#[hdk_extern]
fn create_gui(input: gui::CreateInput) -> ExternResult<EntityResponse<GUIEntry>> {
    let entity = catch!( gui::create_gui( input ) );

    Ok(composition( entity, ENTITY_MD ))
}

#[hdk_extern]
fn get_gui(input: GetEntityInput) -> ExternResult<EntityResponse<GUIEntry>> {
    let entity = catch!( gui::get_gui( input ) );

    Ok(composition( entity, ENTITY_MD ))
}

#[hdk_extern]
fn get_guis(input: GetAgentItemsInput) -> ExternResult<DevHubResponse<Vec<Entity<GUIEntry>>>> {
    let (base_path, _) = devhub_types::create_path( &agent_path_base( input.agent ), vec![ ANCHOR_GUIS ] );
    let collection = catch!( devhub_types::get_entities_for_path_filtered( base_path, LinkTypes::GUI, None, |items : Vec<Entity<GUIEntry>>| {
	Ok( items.into_iter()
	    .filter(|entity| {
		entity.content.deprecation.is_none()
	    })
	    .collect() )
    }) );

    Ok(composition( collection, ENTITY_COLLECTION_MD ))
}

#[hdk_extern]
fn get_my_guis(_:()) -> ExternResult<DevHubResponse<Vec<Entity<GUIEntry>>>> {
    get_guis( GetAgentItemsInput {
	agent: None
    })
}

#[hdk_extern]
fn update_gui(input: gui::GUIUpdateInput) -> ExternResult<EntityResponse<GUIEntry>> {
    let entity = catch!( gui::update_gui( input ) );

    Ok(composition( entity, ENTITY_MD ))
}

#[hdk_extern]
fn deprecate_gui(input: gui::GUIDeprecateInput) -> ExternResult<EntityResponse<GUIEntry>> {
    let value = catch!( gui::deprecate_gui( input ) );

    Ok(composition( value, ENTITY_MD ))
}

#[hdk_extern]
fn get_guis_by_tags( input: Vec<String> ) -> ExternResult<DevHubResponse<Vec<Entity<GUIEntry>>>> {
    let list = catch!( devhub_types::get_by_tags( LinkTypes::GUI, input ) );

    Ok(composition( list.into_iter()
		    .filter(|entity: &Entity<GUIEntry>| {
			entity.content.deprecation.is_none()
		    })
		    .collect(), VALUE_MD ))
}

#[hdk_extern]
fn get_all_guis(_:()) -> ExternResult<DevHubResponse<Vec<Entity<GUIEntry>>>> {
    let (base_path, _) = devhub_types::create_path( ANCHOR_GUIS, Vec::<String>::new() );
    let collection = catch!( devhub_types::get_entities_for_path_filtered( base_path, LinkTypes::GUI, None, |items : Vec<Entity<GUIEntry>>| {
	Ok( items.into_iter()
	    .filter(|entity| {
		entity.content.deprecation.is_none()
	    })
	    .collect() )
    }) );

    Ok(composition( collection, ENTITY_COLLECTION_MD ))
}


// GUI Releases
#[hdk_extern]
fn create_gui_release(input: gui_release::CreateInput) -> ExternResult<EntityResponse<GUIReleaseEntry>> {
    let entity = catch!( gui_release::create_gui_release( input ) );

    Ok(composition( entity, ENTITY_MD ))
}

#[hdk_extern]
fn get_gui_release(input: GetEntityInput) -> ExternResult<EntityResponse<GUIReleaseEntry>> {
    let entity = catch!( gui_release::get_gui_release( input ) );

    Ok(composition( entity, ENTITY_MD ))
}

#[hdk_extern]
fn update_gui_release(input: gui_release::GUIReleaseUpdateInput) -> ExternResult<EntityResponse<GUIReleaseEntry>> {
    let entity = catch!( gui_release::update_gui_release( input ) );

    Ok(composition( entity, ENTITY_MD ))
}

#[hdk_extern]
fn delete_gui_release(input: gui_release::DeleteInput) -> ExternResult<DevHubResponse<ActionHash>> {
    let value = catch!( gui_release::delete_gui_release( input ) );

    Ok(composition( value, VALUE_MD ))
}

#[hdk_extern]
fn get_gui_releases(input: gui_release::GetGUIReleasesInput) -> ExternResult<DevHubResponse<Vec<Entity<GUIReleaseEntry>>>> {
    let collection = catch!( gui_release::get_gui_releases( input ) );

    Ok(composition( collection, ENTITY_COLLECTION_MD ))
}


// Packaging
#[hdk_extern]
fn get_webasset(input: packaging::GetWebAssetInput) -> ExternResult<EntityResponse<FilePackage>> {
    let entity = catch!( packaging::get_webasset( input ) );

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
