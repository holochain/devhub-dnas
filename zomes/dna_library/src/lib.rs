use devhub_types::{
    DevHubResponse, EntityResponse, CollectionResponse, EntityCollectionResponse, FilterInput,
    constants::{ ENTITY_MD, ENTITY_COLLECTION_MD, VALUE_MD, VALUE_COLLECTION_MD },
    dnarepo_entry_types::{
	ProfileEntry, ProfileInfo,
	DnaEntry, DnaInfo, DnaSummary,
	DnaVersionEntry, DnaVersionInfo, DnaVersionSummary, DnaVersionPackage,
	ZomeEntry, ZomeInfo, ZomeSummary,
	ZomeVersionEntry, ZomeVersionInfo, ZomeVersionSummary,
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



entry_defs![
    PathEntry::entry_def(),
    ProfileEntry::entry_def(),
    DnaEntry::entry_def(),
    DnaVersionEntry::entry_def(),
    ZomeEntry::entry_def(),
    ZomeVersionEntry::entry_def()
];

pub fn all_dnas_path() -> Path {
    Path::from( "dnas" )
}
pub fn all_zomes_path() -> Path {
    Path::from( "zomes" )
}
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


// Profile zome functions
#[hdk_extern]
fn create_profile(input: profile::ProfileInput) -> ExternResult<EntityResponse<ProfileInfo>> {
    let entity = catch!( profile::create_profile( input ) );

    Ok(composition( entity, ENTITY_MD ))
}

#[hdk_extern]
pub fn get_profile(input: profile::GetProfileInput) -> ExternResult<EntityResponse<ProfileInfo>> {
    let entity = catch!( profile::get_profile( input ) );

    Ok(composition( entity, ENTITY_MD ))
}

#[hdk_extern]
pub fn update_profile(input: profile::UpdateProfileInput) -> ExternResult<EntityResponse<ProfileInfo>> {
    let entity = catch!( profile::update_profile( input ) );

    Ok(composition( entity, ENTITY_MD ))
}

#[hdk_extern]
fn follow_developer(input: profile::FollowInput) -> ExternResult<DevHubResponse<HeaderHash>> {
    let value = catch!( profile::follow_developer( input ) );

    Ok(composition( value, VALUE_MD ))
}

#[hdk_extern]
fn unfollow_developer(input: profile::UnfollowInput) -> ExternResult<DevHubResponse<Option<HeaderHash>>> {
    let value = catch!( profile::unfollow_developer( input ) );

    Ok(composition( value, VALUE_MD ))
}

#[hdk_extern]
fn get_following(_:()) -> ExternResult<CollectionResponse<Link>> {
    let collection = catch!( profile::get_following() );

    Ok(composition( collection, VALUE_COLLECTION_MD ))
}


// DNA Version zome functions
#[hdk_extern]
fn create_dna(input: dna::DnaInput) -> ExternResult<EntityResponse<DnaInfo>> {
    let entity = catch!( dna::create_dna( input ) );

    Ok(composition( entity, ENTITY_MD ))
}

#[hdk_extern]
fn get_dna(input: dna::GetDnaInput) -> ExternResult<EntityResponse<DnaInfo>> {
    let entity = catch!( dna::get_dna( input ) );

    Ok(composition( entity, ENTITY_MD ))
}

#[hdk_extern]
fn get_dnas(input: dna::GetDnasInput) -> ExternResult<EntityCollectionResponse<DnaSummary>> {
    let collection = catch!( dna::get_dnas( input ) );

    Ok(composition( collection, ENTITY_COLLECTION_MD ))
}

#[hdk_extern]
fn get_deprecated_dnas(input: dna::GetDnasInput) -> ExternResult<EntityCollectionResponse<DnaSummary>> {
    let collection = catch!( dna::get_deprecated_dnas( input ) );

    Ok(composition( collection, ENTITY_COLLECTION_MD ))
}

#[hdk_extern]
fn get_my_dnas(_:()) -> ExternResult<EntityCollectionResponse<DnaSummary>> {
    let collection = catch!( dna::get_my_dnas() );

    Ok(composition( collection, ENTITY_COLLECTION_MD ))
}

#[hdk_extern]
fn get_my_deprecated_dnas(_:()) -> ExternResult<EntityCollectionResponse<DnaSummary>> {
    let collection = catch!( dna::get_my_deprecated_dnas() );

    Ok(composition( collection, ENTITY_COLLECTION_MD ))
}

#[hdk_extern]
fn update_dna(input: dna::DnaUpdateInput) -> ExternResult<EntityResponse<DnaInfo>> {
    let entity = catch!( dna::update_dna( input ) );

    Ok(composition( entity, ENTITY_MD ))
}

#[hdk_extern]
fn deprecate_dna(input: dna::DeprecateDnaInput) -> ExternResult<EntityResponse<DnaInfo>> {
    let entity = catch!( dna::deprecate_dna( input ) );

    Ok(composition( entity, ENTITY_MD ))
}

#[hdk_extern]
fn get_dnas_by_filter( input: FilterInput ) -> ExternResult<EntityCollectionResponse<DnaSummary>> {
    let collection = catch!( dna::get_dnas_by_filter( input.filter, input.keyword ) );

    Ok(composition( collection, ENTITY_COLLECTION_MD ))
}

#[hdk_extern]
fn get_all_dnas(_:()) -> ExternResult<EntityCollectionResponse<DnaSummary>> {
    let collection = catch!( dna::get_all_dnas() );

    Ok(composition( collection, ENTITY_COLLECTION_MD ))
}


// DNA Version zome functions
#[hdk_extern]
fn create_dna_version(input: dnaversions::DnaVersionInput) -> ExternResult<EntityResponse<DnaVersionInfo>> {
    let entity = catch!( dnaversions::create_dna_version( input ) );

    Ok(composition( entity, ENTITY_MD ))
}

#[hdk_extern]
fn get_dna_version(input: dnaversions::GetDnaVersionInput) -> ExternResult<EntityResponse<DnaVersionInfo>> {
    let entity = catch!( dnaversions::get_dna_version( input ) );

    Ok(composition( entity, ENTITY_MD ))
}

#[hdk_extern]
fn get_dna_versions(input: dnaversions::GetDnaVersionsInput) -> ExternResult<EntityCollectionResponse<DnaVersionSummary>> {
    let collection = catch!( dnaversions::get_dna_versions( input ) );

    Ok(composition( collection, ENTITY_COLLECTION_MD ))
}

#[hdk_extern]
fn update_dna_version(input: dnaversions::DnaVersionUpdateInput) -> ExternResult<EntityResponse<DnaVersionInfo>> {
    let entity = catch!( dnaversions::update_dna_version( input ) );

    Ok(composition( entity, ENTITY_MD ))
}

#[hdk_extern]
fn delete_dna_version(input: dnaversions::DeleteDnaVersionInput) -> ExternResult<DevHubResponse<HeaderHash>> {
    let value = catch!( dnaversions::delete_dna_version( input ) );

    Ok(composition( value, VALUE_MD ))
}

#[hdk_extern]
fn get_dna_versions_by_filter( input: FilterInput ) -> ExternResult<EntityCollectionResponse<DnaVersionSummary>> {
    let collection = catch!( dnaversions::get_dna_versions_by_filter( input.filter, input.keyword ) );

    Ok(composition( collection, ENTITY_COLLECTION_MD ))
}


// Packaging
#[hdk_extern]
fn get_dna_package(input: packaging::GetDnaPackageInput) -> ExternResult<EntityResponse<DnaVersionPackage>> {
    let entity = catch!( packaging::get_dna_package( input ) );

    Ok(composition( entity, ENTITY_MD ))
}


// ZOME Version zome functions
#[hdk_extern]
fn create_zome(input: zome::ZomeInput) -> ExternResult<EntityResponse<ZomeInfo>> {
    let entity = catch!( zome::create_zome( input ) );

    Ok(composition( entity, ENTITY_MD ))
}

#[hdk_extern]
fn get_zome(input: zome::GetZomeInput) -> ExternResult<EntityResponse<ZomeInfo>> {
    let entity = catch!( zome::get_zome( input ) );

    Ok(composition( entity, ENTITY_MD ))
}

#[hdk_extern]
fn get_zomes(input: zome::GetZomesInput) -> ExternResult<EntityCollectionResponse<ZomeSummary>> {
    let collection = catch!( zome::get_zomes( input ) );

    Ok(composition( collection, ENTITY_COLLECTION_MD ))
}

#[hdk_extern]
fn get_deprecated_zomes(input: zome::GetZomesInput) -> ExternResult<EntityCollectionResponse<ZomeSummary>> {
    let collection = catch!( zome::get_deprecated_zomes( input ) );

    Ok(composition( collection, ENTITY_COLLECTION_MD ))
}

#[hdk_extern]
fn get_my_zomes(_:()) -> ExternResult<EntityCollectionResponse<ZomeSummary>> {
    let collection = catch!( zome::get_my_zomes() );

    Ok(composition( collection, ENTITY_COLLECTION_MD ))
}

#[hdk_extern]
fn get_my_deprecated_zomes(_:()) -> ExternResult<EntityCollectionResponse<ZomeSummary>> {
    let collection = catch!( zome::get_my_deprecated_zomes() );

    Ok(composition( collection, ENTITY_COLLECTION_MD ))
}

#[hdk_extern]
fn update_zome(input: zome::ZomeUpdateInput) -> ExternResult<EntityResponse<ZomeInfo>> {
    let entity = catch!( zome::update_zome( input ) );

    Ok(composition( entity, ENTITY_MD ))
}

#[hdk_extern]
fn deprecate_zome(input: zome::DeprecateZomeInput) -> ExternResult<EntityResponse<ZomeInfo>> {
    let entity = catch!( zome::deprecate_zome( input ) );

    Ok(composition( entity, ENTITY_MD ))
}

#[hdk_extern]
fn get_zomes_by_filter( input: FilterInput ) -> ExternResult<EntityCollectionResponse<ZomeSummary>> {
    let collection = catch!( zome::get_zomes_by_filter( input.filter, input.keyword ) );

    Ok(composition( collection, ENTITY_COLLECTION_MD ))
}

#[hdk_extern]
fn get_all_zomes(_:()) -> ExternResult<EntityCollectionResponse<ZomeSummary>> {
    let collection = catch!( zome::get_all_zomes() );

    Ok(composition( collection, ENTITY_COLLECTION_MD ))
}


// ZOME Version zome functions
#[hdk_extern]
fn create_zome_version(input: zomeversion::ZomeVersionInput) -> ExternResult<EntityResponse<ZomeVersionInfo>> {
    let entity = catch!( zomeversion::create_zome_version( input ) );

    Ok(composition( entity, ENTITY_MD ))
}

#[hdk_extern]
fn get_zome_version(input: zomeversion::GetZomeVersionInput) -> ExternResult<EntityResponse<ZomeVersionInfo>> {
    let entity = catch!( zomeversion::get_zome_version( input ) );

    Ok(composition( entity, ENTITY_MD ))
}

#[hdk_extern]
fn get_zome_versions(input: zomeversion::GetZomeVersionsInput) -> ExternResult<EntityCollectionResponse<ZomeVersionSummary>> {
    let collection = catch!( zomeversion::get_zome_versions( input ) );

    Ok(composition( collection, ENTITY_COLLECTION_MD ))
}

#[hdk_extern]
fn update_zome_version(input: zomeversion::ZomeVersionUpdateInput) -> ExternResult<EntityResponse<ZomeVersionInfo>> {
    let entity = catch!( zomeversion::update_zome_version( input ) );

    Ok(composition( entity, ENTITY_MD ))
}

#[hdk_extern]
fn delete_zome_version(input: zomeversion::DeleteZomeVersionInput) -> ExternResult<DevHubResponse<HeaderHash>> {
    let value = catch!( zomeversion::delete_zome_version( input ) );

    Ok(composition( value, VALUE_MD ))
}

#[hdk_extern]
fn get_zome_versions_by_filter( input: FilterInput ) -> ExternResult<EntityCollectionResponse<ZomeVersionSummary>> {
    let collection = catch!( zomeversion::get_zome_versions_by_filter( input.filter, input.keyword ) );

    Ok(composition( collection, ENTITY_COLLECTION_MD ))
}
