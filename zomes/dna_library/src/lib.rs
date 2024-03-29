use dnarepo_core::{
    LinkTypes,
};
use devhub_types::{
    DevHubResponse, Entity, EntityResponse, FilterInput,
    constants::{
	ENTITY_MD,
	VALUE_MD,
	ENTITY_COLLECTION_MD,
	VALUE_COLLECTION_MD,
    },
    dnarepo_entry_types::{
	ProfileEntry,
	DnaEntry,
	DnaVersionEntry, DnaVersionPackage,
	ZomeEntry,
	ZomeVersionEntry,
    },
    composition,
    catch,
};
use hdk::prelude::*;

mod profile;
mod dna;
mod dnaversions;
mod zome;
mod zomeversion;

mod packaging;
mod constants;


use constants::{
    ANCHOR_DNAS,
    ANCHOR_ZOMES,
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


#[hdk_extern]
fn init(_: ()) -> ExternResult<InitCallbackResult> {
    Ok(InitCallbackResult::Pass)
}


#[hdk_extern]
fn whoami(_: ()) -> ExternResult<DevHubResponse<AgentInfo>> {
    Ok(composition( agent_info()?, VALUE_MD ))
}


// Profile zome functions
#[hdk_extern]
fn create_profile(input: profile::ProfileInput) -> ExternResult<EntityResponse<ProfileEntry>> {
    let entity = catch!( profile::create_profile( input ) );

    Ok(composition( entity, ENTITY_MD ))
}

#[hdk_extern]
pub fn get_profile(input: profile::GetProfileInput) -> ExternResult<EntityResponse<ProfileEntry>> {
    let entity = catch!( profile::get_profile( input ) );

    Ok(composition( entity, ENTITY_MD ))
}

#[hdk_extern]
pub fn update_profile(input: profile::UpdateProfileInput) -> ExternResult<EntityResponse<ProfileEntry>> {
    let entity = catch!( profile::update_profile( input ) );

    Ok(composition( entity, ENTITY_MD ))
}

#[hdk_extern]
fn follow_developer(input: profile::FollowInput) -> ExternResult<DevHubResponse<ActionHash>> {
    let value = catch!( profile::follow_developer( input ) );

    Ok(composition( value, VALUE_MD ))
}

#[hdk_extern]
fn unfollow_developer(input: profile::UnfollowInput) -> ExternResult<DevHubResponse<Option<ActionHash>>> {
    let value = catch!( profile::unfollow_developer( input ) );

    Ok(composition( value, VALUE_MD ))
}

#[hdk_extern]
fn get_following(_:()) -> ExternResult<DevHubResponse<Vec<Link>>> {
    let collection = catch!( profile::get_following() );

    Ok(composition( collection, VALUE_COLLECTION_MD ))
}


// DNA zome functions
#[hdk_extern]
fn create_dna(input: dna::DnaInput) -> ExternResult<EntityResponse<DnaEntry>> {
    let entity = catch!( dna::create_dna( input ) );

    Ok(composition( entity, ENTITY_MD ))
}

#[hdk_extern]
fn get_dna(input: dna::GetDnaInput) -> ExternResult<EntityResponse<DnaEntry>> {
    let entity = catch!( dna::get_dna( input ) );

    Ok(composition( entity, ENTITY_MD ))
}

#[hdk_extern]
fn get_dnas(input: GetAgentItemsInput) -> ExternResult<DevHubResponse<Vec<Entity<DnaEntry>>>> {
    let (base_path, _) = devhub_types::create_path( &agent_path_base( input.agent ), vec![ ANCHOR_DNAS ] );
    let collection = catch!( devhub_types::get_entities_for_path_filtered( base_path, LinkTypes::Dna, None, |items : Vec<Entity<DnaEntry>>| {
	Ok( items.into_iter()
	    .filter(|entity| {
		entity.content.deprecation.is_none()
	    })
	    .collect() )
    }) );

    Ok(composition( collection, ENTITY_COLLECTION_MD ))
}

#[hdk_extern]
fn get_deprecated_dnas(input: GetAgentItemsInput) -> ExternResult<DevHubResponse<Vec<Entity<DnaEntry>>>> {
    let (base_path, _) = devhub_types::create_path( &agent_path_base( input.agent ), vec![ ANCHOR_DNAS ] );
    let collection = catch!( devhub_types::get_entities_for_path_filtered( base_path, LinkTypes::Dna, None, |items : Vec<Entity<DnaEntry>>| {
	Ok( items.into_iter()
	    .filter(|entity| {
		entity.content.deprecation.is_some()
	    })
	    .collect() )
    }) );

    Ok(composition( collection, ENTITY_COLLECTION_MD ))
}

#[hdk_extern]
fn get_my_dnas(_:()) -> ExternResult<DevHubResponse<Vec<Entity<DnaEntry>>>> {
    get_dnas( GetAgentItemsInput {
	agent: None
    })
}

#[hdk_extern]
fn get_my_deprecated_dnas(_:()) -> ExternResult<DevHubResponse<Vec<Entity<DnaEntry>>>> {
    get_deprecated_dnas( GetAgentItemsInput {
	agent: None
    })
}

#[hdk_extern]
fn update_dna(input: dna::DnaUpdateInput) -> ExternResult<EntityResponse<DnaEntry>> {
    let entity = catch!( dna::update_dna( input ) );

    Ok(composition( entity, ENTITY_MD ))
}

#[hdk_extern]
fn deprecate_dna(input: dna::DeprecateDnaInput) -> ExternResult<EntityResponse<DnaEntry>> {
    let entity = catch!( dna::deprecate_dna( input ) );

    Ok(composition( entity, ENTITY_MD ))
}

#[hdk_extern]
fn get_dnas_by_filter( input: FilterInput ) -> ExternResult<DevHubResponse<Vec<Entity<DnaEntry>>>> {
    let collection = catch!( devhub_types::get_by_filter( LinkTypes::Dna, input.filter, input.keyword ) );

    Ok(composition(
	collection.into_iter()
	    .filter(|entity: &Entity<DnaEntry>| {
		entity.content.deprecation.is_none()
	    })
	    .collect(),
	ENTITY_COLLECTION_MD
    ))
}

#[hdk_extern]
fn get_dnas_by_tags( input: Vec<String> ) -> ExternResult<DevHubResponse<Vec<Entity<DnaEntry>>>> {
    let list = catch!( devhub_types::get_by_tags( LinkTypes::Dna, input ) );

    Ok(composition( list.into_iter()
		    .filter(|entity: &Entity<DnaEntry>| {
			entity.content.deprecation.is_none()
		    })
		    .collect(), VALUE_MD ))
}

#[hdk_extern]
fn get_all_dnas(_:()) -> ExternResult<DevHubResponse<Vec<Entity<DnaEntry>>>> {
    let (base_path, _) = devhub_types::create_path( ANCHOR_DNAS, Vec::<String>::new() );
    let collection = catch!( devhub_types::get_entities_for_path_filtered( base_path, LinkTypes::Dna, None, |items : Vec<Entity<DnaEntry>>| {
	Ok( items.into_iter()
	    .filter(|entity| {
		entity.content.deprecation.is_none()
	    })
	    .collect() )
    }) );

    Ok(composition( collection, ENTITY_COLLECTION_MD ))
}

#[hdk_extern]
fn get_dnas_with_an_hdk_version( input: String ) -> ExternResult<DevHubResponse<Vec<Entity<DnaEntry>>>> {
    let collection = catch!( dna::get_dnas_with_an_hdk_version( input ) );

    Ok(composition( collection, ENTITY_COLLECTION_MD ))
}


// DNA Version zome functions
#[hdk_extern]
fn create_dna_version(input: dnaversions::DnaVersionInput) -> ExternResult<EntityResponse<DnaVersionEntry>> {
    let entity = catch!( dnaversions::create_dna_version( input ) );

    Ok(composition( entity, ENTITY_MD ))
}

#[hdk_extern]
fn get_dna_version(input: dnaversions::GetDnaVersionInput) -> ExternResult<EntityResponse<DnaVersionEntry>> {
    let entity = catch!( dnaversions::get_dna_version( input ) );

    Ok(composition( entity, ENTITY_MD ))
}

#[hdk_extern]
fn get_dna_versions(input: dnaversions::GetDnaVersionsInput) -> ExternResult<DevHubResponse<Vec<Entity<DnaVersionEntry>>>> {
    let collection = catch!( dnaversions::get_dna_versions( input ) );

    Ok(composition( collection, ENTITY_COLLECTION_MD ))
}

#[hdk_extern]
fn update_dna_version(input: dnaversions::DnaVersionUpdateInput) -> ExternResult<EntityResponse<DnaVersionEntry>> {
    let entity = catch!( dnaversions::update_dna_version( input ) );

    Ok(composition( entity, ENTITY_MD ))
}

#[hdk_extern]
fn delete_dna_version(input: dnaversions::DeleteDnaVersionInput) -> ExternResult<DevHubResponse<ActionHash>> {
    let value = catch!( dnaversions::delete_dna_version( input ) );

    Ok(composition( value, VALUE_MD ))
}

#[hdk_extern]
fn get_dna_versions_by_filter( input: FilterInput ) -> ExternResult<DevHubResponse<Vec<Entity<DnaVersionEntry>>>> {
    let collection = catch!( devhub_types::get_by_filter( LinkTypes::DnaVersion, input.filter, input.keyword ) );

    Ok(composition( collection, ENTITY_COLLECTION_MD ))
}


// Packaging
#[hdk_extern]
fn get_dna_package(input: packaging::GetDnaPackageInput) -> ExternResult<EntityResponse<DnaVersionPackage>> {
    let entity = catch!( packaging::get_dna_package( input ) );

    Ok(composition( entity, ENTITY_MD ))
}


// ZOME functions
#[hdk_extern]
fn create_zome(input: zome::ZomeInput) -> ExternResult<EntityResponse<ZomeEntry>> {
    let entity = catch!( zome::create_zome( input ) );

    Ok(composition( entity, ENTITY_MD ))
}

#[hdk_extern]
fn get_zome(input: zome::GetZomeInput) -> ExternResult<EntityResponse<ZomeEntry>> {
    let entity = catch!( zome::get_zome( input ) );

    Ok(composition( entity, ENTITY_MD ))
}

#[hdk_extern]
fn get_zomes(input: GetAgentItemsInput) -> ExternResult<DevHubResponse<Vec<Entity<ZomeEntry>>>> {
    let (base_path, _) = devhub_types::create_path( &agent_path_base( input.agent ), vec![ ANCHOR_ZOMES ] );
    let collection = catch!( devhub_types::get_entities_for_path_filtered( base_path, LinkTypes::Zome, None, |items : Vec<Entity<ZomeEntry>>| {
	Ok( items.into_iter()
	    .filter(|entity| {
		entity.content.deprecation.is_none()
	    })
	    .collect() )
    }) );

    Ok(composition( collection, ENTITY_COLLECTION_MD ))
}

#[hdk_extern]
fn get_deprecated_zomes(input: GetAgentItemsInput) -> ExternResult<DevHubResponse<Vec<Entity<ZomeEntry>>>> {
    let (base_path, _) = devhub_types::create_path( &agent_path_base( input.agent ), vec![ ANCHOR_ZOMES ] );
    let collection = catch!( devhub_types::get_entities_for_path_filtered( base_path, LinkTypes::Zome, None, |items : Vec<Entity<ZomeEntry>>| {
	Ok( items.into_iter()
	    .filter(|entity| {
		entity.content.deprecation.is_some()
	    })
	    .collect() )
    }) );

    Ok(composition( collection, ENTITY_COLLECTION_MD ))
}

#[hdk_extern]
fn get_my_zomes(_:()) -> ExternResult<DevHubResponse<Vec<Entity<ZomeEntry>>>> {
    get_zomes( GetAgentItemsInput {
	agent: None
    })
}

#[hdk_extern]
fn get_my_deprecated_zomes(_:()) -> ExternResult<DevHubResponse<Vec<Entity<ZomeEntry>>>> {
    get_deprecated_zomes( GetAgentItemsInput {
	agent: None
    })
}

#[hdk_extern]
fn update_zome(input: zome::ZomeUpdateInput) -> ExternResult<EntityResponse<ZomeEntry>> {
    let entity = catch!( zome::update_zome( input ) );

    Ok(composition( entity, ENTITY_MD ))
}

#[hdk_extern]
fn deprecate_zome(input: zome::DeprecateZomeInput) -> ExternResult<EntityResponse<ZomeEntry>> {
    let entity = catch!( zome::deprecate_zome( input ) );

    Ok(composition( entity, ENTITY_MD ))
}

#[hdk_extern]
fn get_zomes_by_filter( input: FilterInput ) -> ExternResult<DevHubResponse<Vec<Entity<ZomeEntry>>>> {
    let collection = catch!( devhub_types::get_by_filter( LinkTypes::Zome, input.filter, input.keyword ) );

    Ok(composition(
	collection.into_iter()
	    .filter(|entity: &Entity<ZomeEntry>| {
		entity.content.deprecation.is_none()
	    })
	    .collect(),
	ENTITY_COLLECTION_MD
    ))
}

#[hdk_extern]
fn get_zomes_by_tags( input: Vec<String> ) -> ExternResult<DevHubResponse<Vec<Entity<ZomeEntry>>>> {
    let list = catch!( devhub_types::get_by_tags( LinkTypes::Zome, input ) );

    Ok(composition( list.into_iter()
		    .filter(|entity: &Entity<ZomeEntry>| {
			entity.content.deprecation.is_none()
		    })
		    .collect(), VALUE_MD ))
}

#[hdk_extern]
fn get_all_zomes(_:()) -> ExternResult<DevHubResponse<Vec<Entity<ZomeEntry>>>> {
    let (base_path, _) = devhub_types::create_path( ANCHOR_ZOMES, Vec::<String>::new() );
    let collection = catch!( devhub_types::get_entities_for_path_filtered( base_path, LinkTypes::Zome, None, |items : Vec<Entity<ZomeEntry>>| {
	Ok( items.into_iter()
	    .filter(|entity| {
		entity.content.deprecation.is_none()
	    })
	    .collect() )
    }) );

    Ok(composition( collection, ENTITY_COLLECTION_MD ))
}

#[hdk_extern]
fn get_zomes_with_an_hdk_version( input: String ) -> ExternResult<DevHubResponse<Vec<Entity<ZomeEntry>>>> {
    let collection = catch!( zome::get_zomes_with_an_hdk_version( input ) );

    Ok(composition( collection, ENTITY_COLLECTION_MD ))
}


// ZOME Version zome functions
#[hdk_extern]
fn create_zome_version(input: zomeversion::ZomeVersionInput) -> ExternResult<EntityResponse<ZomeVersionEntry>> {
    let entity = catch!( zomeversion::create_zome_version( input ) );

    Ok(composition( entity, ENTITY_MD ))
}

#[hdk_extern]
fn get_zome_version(input: zomeversion::GetZomeVersionInput) -> ExternResult<EntityResponse<ZomeVersionEntry>> {
    let entity = catch!( zomeversion::get_zome_version( input ) );

    Ok(composition( entity, ENTITY_MD ))
}

#[hdk_extern]
fn get_zome_versions(input: zomeversion::GetZomeVersionsInput) -> ExternResult<DevHubResponse<Vec<Entity<ZomeVersionEntry>>>> {
    let collection = catch!( zomeversion::get_zome_versions( input ) );

    Ok(composition( collection, ENTITY_COLLECTION_MD ))
}

#[hdk_extern]
fn update_zome_version(input: zomeversion::ZomeVersionUpdateInput) -> ExternResult<EntityResponse<ZomeVersionEntry>> {
    let entity = catch!( zomeversion::update_zome_version( input ) );

    Ok(composition( entity, ENTITY_MD ))
}

#[hdk_extern]
fn create_zome_version_review_summary(input: zomeversion::EntityAddressInput) -> ExternResult<EntityResponse<ZomeVersionEntry>> {
    let entity = catch!( zomeversion::create_zome_version_review_summary( input ) );

    Ok(composition( entity, ENTITY_MD ))
}

#[hdk_extern]
fn delete_zome_version(input: zomeversion::DeleteZomeVersionInput) -> ExternResult<DevHubResponse<ActionHash>> {
    let value = catch!( zomeversion::delete_zome_version( input ) );

    Ok(composition( value, VALUE_MD ))
}

#[hdk_extern]
fn get_zome_versions_by_filter( input: FilterInput ) -> ExternResult<DevHubResponse<Vec<Entity<ZomeVersionEntry>>>> {
    let collection = catch!( devhub_types::get_by_filter( LinkTypes::ZomeVersion, input.filter, input.keyword ) );

    Ok(composition( collection, ENTITY_COLLECTION_MD ))
}

#[hdk_extern]
fn get_hdk_versions(_:()) -> ExternResult<DevHubResponse<Vec<String>>> {
    let list = catch!( devhub_types::get_hdk_versions( LinkTypes::Anchor ) );

    Ok(composition( list, VALUE_COLLECTION_MD ))
}

#[hdk_extern]
fn get_zome_versions_by_hdk_version( input: String ) -> ExternResult<DevHubResponse<Vec<Entity<ZomeVersionEntry>>>> {
    let collection = catch!( devhub_types::get_hdk_version_entities( LinkTypes::ZomeVersion, input ) );

    Ok(composition( collection, ENTITY_COLLECTION_MD ))
}
