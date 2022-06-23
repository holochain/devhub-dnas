use devhub_types::{
    DevHubResponse, Entity, EntityResponse, EntityCollectionResponse, GetEntityInput,
    constants::{ ENTITY_MD, ENTITY_COLLECTION_MD, VALUE_MD },
    dnarepo_entry_types::{
	ReviewEntry,
	ReviewSummaryEntry,
    },
    composition,
    catch,
};
use hdk::prelude::*;

// mod misc;
mod reviews;

mod constants;


use constants::{
    TAG_REVIEW,
    ANCHOR_REVIEWS,
};

entry_defs![
    PathEntry::entry_def(),
    ReviewEntry::entry_def(),
    ReviewSummaryEntry::entry_def()
];


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


// Review zome functions
#[hdk_extern]
fn create_review(input: reviews::ReviewInput) -> ExternResult<EntityResponse<ReviewEntry>> {
    let entity = catch!( reviews::create_review( input ) );

    Ok(composition( entity, ENTITY_MD ))
}

#[hdk_extern]
fn get_review(input: GetEntityInput) -> ExternResult<EntityResponse<ReviewEntry>> {
    let entity = catch!( reviews::get_review( input ) );

    Ok(composition( entity, ENTITY_MD ))
}

#[hdk_extern]
fn get_reviews_for_subject(input: GetEntityInput) -> ExternResult<EntityCollectionResponse<ReviewEntry>> {
    let (base_path, _) = devhub_types::create_path( ANCHOR_REVIEWS, vec![ input.id ] );
    let collection = catch!( devhub_types::get_entities_for_path_filtered( TAG_REVIEW.into(), base_path, |items : Vec<Entity<ReviewEntry>>| {
	Ok( items.into_iter().collect() )
    }) );

    Ok(composition( collection, ENTITY_COLLECTION_MD ))
}

#[hdk_extern]
fn get_my_reviews(_:()) -> ExternResult<EntityCollectionResponse<ReviewEntry>> {
    let (base_path, _) = devhub_types::create_path( &crate::agent_path_base( None ), vec![ ANCHOR_REVIEWS ] );
    let collection = catch!( devhub_types::get_entities_for_path( TAG_REVIEW.into(), base_path ) );

    Ok(composition( collection, ENTITY_COLLECTION_MD ))
}

#[hdk_extern]
fn update_review(input: reviews::ReviewUpdateInput) -> ExternResult<EntityResponse<ReviewEntry>> {
    let entity = catch!( reviews::update_review( input ) );

    Ok(composition( entity, ENTITY_MD ))
}

#[hdk_extern]
fn delete_review(input: GetEntityInput) -> ExternResult<DevHubResponse<HeaderHash>> {
    let hash = catch!( reviews::delete_review( input ) );

    Ok(composition( hash, VALUE_MD ))
}


// Review Summary zome functions
#[hdk_extern]
fn create_summary_for_subject(input: reviews::ReviewSummaryInput) -> ExternResult<EntityResponse<ReviewSummaryEntry>> {
    let entity = catch!( reviews::create_summary( input ) );

    Ok(composition( entity, ENTITY_MD ))
}
