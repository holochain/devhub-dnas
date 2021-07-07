use devhub_types::{
    DevHubResponse, EntityResponse, CollectionResponse, EntityCollectionResponse,
    constants::{ ENTITY_MD, ENTITY_COLLECTION_MD, VALUE_MD, VALUE_COLLECTION_MD },
    errors::{ ErrorKinds },
    dna_entry_types::{
	ProfileEntry, ProfileInfo,
	DnaEntry, DnaInfo, DnaSummary,
	DnaVersionEntry, DnaVersionInfo, DnaVersionSummary,
	DnaChunkEntry,
    },
    catch,
};
use hdk::prelude::*;

mod profile;
mod dna;
mod dnaversions;
mod dnachunks;

mod constants;



entry_defs![
    ProfileEntry::entry_def(),
    DnaEntry::entry_def(),
    DnaVersionEntry::entry_def(),
    DnaChunkEntry::entry_def()
];



#[hdk_extern]
fn init(_: ()) -> ExternResult<InitCallbackResult> {
    Ok(InitCallbackResult::Pass)
}


#[hdk_extern]
fn whoami(_: ()) -> ExternResult<DevHubResponse<AgentInfo>> {
    Ok( DevHubResponse::success( agent_info()?, VALUE_MD ) )
}


// Profile zome functions
#[hdk_extern]
fn create_profile(input: profile::ProfileInput) -> ExternResult<EntityResponse<ProfileInfo>> {
    let entity = catch!( profile::create_profile( input ) );

    Ok(EntityResponse::success( entity, ENTITY_MD ))
}

#[hdk_extern]
pub fn get_profile(input: profile::GetProfileInput) -> ExternResult<EntityResponse<ProfileInfo>> {
    let entity = catch!( profile::get_profile( input ) );

    Ok(EntityResponse::success( entity, ENTITY_MD ))
}

#[hdk_extern]
pub fn update_profile(input: profile::UpdateProfileInput) -> ExternResult<EntityResponse<ProfileInfo>> {
    let entity = catch!( profile::update_profile( input ) );

    Ok(EntityResponse::success( entity, ENTITY_MD ))
}

#[hdk_extern]
fn follow_developer(input: profile::FollowInput) -> ExternResult<DevHubResponse<HeaderHash>> {
    let value = catch!( profile::follow_developer( input ) );

    Ok(DevHubResponse::success( value, VALUE_MD ))
}

#[hdk_extern]
fn unfollow_developer(input: profile::UnfollowInput) -> ExternResult<DevHubResponse<Option<HeaderHash>>> {
    let value = catch!( profile::unfollow_developer( input ) );

    Ok(DevHubResponse::success( value, VALUE_MD ))
}

#[hdk_extern]
fn get_following(_:()) -> ExternResult<CollectionResponse<Link>> {
    let collection = catch!( profile::get_following() );

    Ok(CollectionResponse::success( collection, VALUE_COLLECTION_MD ))
}


// DNA Version zome functions
#[hdk_extern]
fn create_dna(input: dna::DnaInput) -> ExternResult<EntityResponse<DnaInfo>> {
    let entity = catch!( dna::create_dna( input ) );

    Ok(EntityResponse::success( entity, ENTITY_MD ))
}

#[hdk_extern]
fn get_dna(input: dna::GetDnaInput) -> ExternResult<EntityResponse<DnaInfo>> {
    let entity = catch!( dna::get_dna( input ) );

    Ok(EntityResponse::success( entity, ENTITY_MD ))
}

#[hdk_extern]
fn get_dnas(input: dna::GetDnasInput) -> ExternResult<EntityCollectionResponse<DnaSummary>> {
    let collection = catch!( dna::get_dnas( input ) );

    Ok(EntityCollectionResponse::success( collection, ENTITY_COLLECTION_MD ))
}

#[hdk_extern]
fn get_deprecated_dnas(input: dna::GetDnasInput) -> ExternResult<EntityCollectionResponse<DnaSummary>> {
    let collection = catch!( dna::get_deprecated_dnas( input ) );

    Ok(EntityCollectionResponse::success( collection, ENTITY_COLLECTION_MD ))
}

#[hdk_extern]
fn get_my_dnas(_:()) -> ExternResult<EntityCollectionResponse<DnaSummary>> {
    let collection = catch!( dna::get_my_dnas() );

    Ok(EntityCollectionResponse::success( collection, ENTITY_COLLECTION_MD ))
}

#[hdk_extern]
fn get_my_deprecated_dnas(_:()) -> ExternResult<EntityCollectionResponse<DnaSummary>> {
    let collection = catch!( dna::get_my_deprecated_dnas() );

    Ok(EntityCollectionResponse::success( collection, ENTITY_COLLECTION_MD ))
}

#[hdk_extern]
fn update_dna(input: dna::DnaUpdateInput) -> ExternResult<EntityResponse<DnaInfo>> {
    let entity = catch!( dna::update_dna( input ) );

    Ok(EntityResponse::success( entity, ENTITY_MD ))
}

#[hdk_extern]
fn deprecate_dna(input: dna::DeprecateDnaInput) -> ExternResult<EntityResponse<DnaInfo>> {
    let entity = catch!( dna::deprecate_dna( input ) );

    Ok(EntityResponse::success( entity, ENTITY_MD ))
}


// DNA Version zome functions
#[hdk_extern]
fn create_dna_version(input: dnaversions::DnaVersionInput) -> ExternResult<EntityResponse<DnaVersionInfo>> {
    let entity = catch!( dnaversions::create_dna_version( input ) );

    Ok(EntityResponse::success( entity, ENTITY_MD ))
}

#[hdk_extern]
fn get_dna_version(input: dnaversions::GetDnaVersionInput) -> ExternResult<EntityResponse<DnaVersionInfo>> {
    let entity = catch!( dnaversions::get_dna_version( input ) );

    Ok(EntityResponse::success( entity, ENTITY_MD ))
}

#[hdk_extern]
fn get_dna_versions(input: dnaversions::GetDnaVersionsInput) -> ExternResult<EntityCollectionResponse<DnaVersionSummary>> {
    let collection = catch!( dnaversions::get_dna_versions( input ) );

    Ok(EntityCollectionResponse::success( collection, ENTITY_COLLECTION_MD ))
}

#[hdk_extern]
fn update_dna_version(input: dnaversions::DnaVersionUpdateInput) -> ExternResult<EntityResponse<DnaVersionInfo>> {
    let entity = catch!( dnaversions::update_dna_version( input ) );

    Ok(EntityResponse::success( entity, ENTITY_MD ))
}

#[hdk_extern]
fn delete_dna_version(input: dnaversions::DeleteDnaVersionInput) -> ExternResult<DevHubResponse<HeaderHash>> {
    let value = catch!( dnaversions::delete_dna_version( input ) );

    Ok(DevHubResponse::success( value, VALUE_MD ))
}


// DNA Chunk zome functions
#[hdk_extern]
fn create_dna_chunk(input: DnaChunkEntry) -> ExternResult<EntityResponse<DnaChunkEntry>> {
    let entity = catch!( dnachunks::create_dna_chunk( input ) );

    Ok(EntityResponse::success( entity, ENTITY_MD ))
}

#[hdk_extern]
fn get_dna_chunk(input: dnachunks::GetDnaChunksInput) -> ExternResult<EntityResponse<DnaChunkEntry>> {
    let entity = catch!( dnachunks::get_dna_chunk( input ) );

    Ok(EntityResponse::success( entity, ENTITY_MD ))
}
