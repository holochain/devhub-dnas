use dnarepo_core::{
    LinkTypes,
};
use devhub_types::{
    DevHubResponse, Entity, EntityResponse, GetEntityInput,
    constants::{ ENTITY_MD, ENTITY_COLLECTION_MD, VALUE_MD },
    dnarepo_entry_types::{
	ReviewEntry,
	ReactionEntry,
	ReviewSummaryEntry,
	ReactionSummaryEntry,
    },
    composition,
    catch,
};
use hdk::prelude::*;

mod reviews;
mod reactions;
mod review_summaries;
mod reaction_summaries;

mod constants;


use constants::{
    ANCHOR_REVIEWS,
    ANCHOR_REACTIONS,
    ANCHOR_SUMMARIES,
};



#[derive(Debug, Deserialize)]
pub struct AddrInput {
    pub addr: ActionHash,
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
fn get_reviews_for_subject(input: GetEntityInput) -> ExternResult<DevHubResponse<Vec<Entity<ReviewEntry>>>> {
    let (base_path, _) = devhub_types::create_path( ANCHOR_REVIEWS, vec![ input.id ] );
    let collection = catch!( devhub_types::get_entities_for_path_filtered( base_path, LinkTypes::Review, None, |items : Vec<Entity<ReviewEntry>>| {
	Ok( items.into_iter()
	    .filter(|entity| {
		!entity.content.deleted
	    })
	    .collect() )
    }) );

    Ok(composition( collection, ENTITY_COLLECTION_MD ))
}

#[hdk_extern]
fn get_my_reviews(_:()) -> ExternResult<DevHubResponse<Vec<Entity<ReviewEntry>>>> {
    let (base_path, _) = devhub_types::create_path( &crate::agent_path_base( None ), vec![ ANCHOR_REVIEWS ] );
    let collection = catch!( devhub_types::get_entities_for_path_filtered( base_path, LinkTypes::Review, None, |items : Vec<Entity<ReviewEntry>>| {
	Ok( items.into_iter()
	    .filter(|entity| {
		!entity.content.deleted
	    })
	    .collect() )
    }) );

    Ok(composition( collection, ENTITY_COLLECTION_MD ))
}

#[hdk_extern]
fn create_review_reaction_summary(input: reviews::EntityAddressInput) -> ExternResult<EntityResponse<ReviewEntry>> {
    let entity = catch!( reviews::create_review_reaction_summary( input ) );

    Ok(composition( entity, ENTITY_MD ))
}

#[hdk_extern]
fn update_review(input: reviews::ReviewUpdateInput) -> ExternResult<EntityResponse<ReviewEntry>> {
    let entity = catch!( reviews::update_review( input ) );

    Ok(composition( entity, ENTITY_MD ))
}

#[hdk_extern]
fn delete_review(input: AddrInput) -> ExternResult<EntityResponse<ReviewEntry>> {
    let entity = catch!( reviews::delete_review( input.addr ) );

    Ok(composition( entity, ENTITY_MD ))
}


// Reaction zome functions
#[hdk_extern]
fn create_reaction(input: reactions::ReactionInput) -> ExternResult<EntityResponse<ReactionEntry>> {
    let entity = catch!( reactions::create_reaction( input ) );

    Ok(composition( entity, ENTITY_MD ))
}

#[hdk_extern]
fn get_reaction(input: GetEntityInput) -> ExternResult<EntityResponse<ReactionEntry>> {
    let entity = catch!( reactions::get_reaction( input ) );

    Ok(composition( entity, ENTITY_MD ))
}

#[hdk_extern]
fn get_reactions_for_subject(input: GetEntityInput) -> ExternResult<DevHubResponse<Vec<Entity<ReactionEntry>>>> {
    let (base_path, _) = devhub_types::create_path( ANCHOR_REACTIONS, vec![ input.id ] );
    let collection = catch!( devhub_types::get_entities_for_path_filtered( base_path, LinkTypes::Reaction, None, |items : Vec<Entity<ReactionEntry>>| {
	Ok( items.into_iter()
	    .filter(|entity| {
		!entity.content.deleted
	    })
	    .collect() )
    }) );

    Ok(composition( collection, ENTITY_COLLECTION_MD ))
}

#[hdk_extern]
fn get_my_reactions(_:()) -> ExternResult<DevHubResponse<Vec<Entity<ReactionEntry>>>> {
    let (base_path, _) = devhub_types::create_path( &crate::agent_path_base( None ), vec![ ANCHOR_REACTIONS ] );
    let collection = catch!( devhub_types::get_entities_for_path_filtered( base_path, LinkTypes::Reaction, None, |items : Vec<Entity<ReactionEntry>>| {
	Ok( items.into_iter()
	    .filter(|entity| {
		!entity.content.deleted
	    })
	    .collect() )
    }) );

    Ok(composition( collection, ENTITY_COLLECTION_MD ))
}

#[hdk_extern]
fn update_reaction(input: reactions::ReactionUpdateInput) -> ExternResult<EntityResponse<ReactionEntry>> {
    let entity = catch!( reactions::update_reaction( input ) );

    Ok(composition( entity, ENTITY_MD ))
}

#[hdk_extern]
fn delete_reaction(input: AddrInput) -> ExternResult<EntityResponse<ReactionEntry>> {
    let entity = catch!( reactions::delete_reaction( input.addr ) );

    Ok(composition( entity, ENTITY_MD ))
}


// Review Summary zome functions
#[hdk_extern]
fn create_review_summary_for_subject(input: review_summaries::ReviewSummaryInput) -> ExternResult<EntityResponse<ReviewSummaryEntry>> {
    let entity = catch!( review_summaries::create_review_summary( input ) );

    Ok(composition( entity, ENTITY_MD ))
}

#[hdk_extern]
fn get_review_summary(input: GetEntityInput) -> ExternResult<EntityResponse<ReviewSummaryEntry>> {
    let entity = catch!( review_summaries::get_review_summary( input ) );

    Ok(composition( entity, ENTITY_MD ))
}

#[hdk_extern]
fn get_review_summaries_for_subject(input: GetEntityInput) -> ExternResult<DevHubResponse<Vec<Entity<ReviewSummaryEntry>>>> {
    let (base_path, _) = devhub_types::create_path( ANCHOR_SUMMARIES, vec![ input.id ] );
    let collection = catch!( devhub_types::get_entities_for_path( base_path, LinkTypes::ReviewSummary, None ) );

    Ok(composition( collection, ENTITY_COLLECTION_MD ))
}

#[hdk_extern]
fn update_review_summary(input: GetEntityInput) -> ExternResult<EntityResponse<ReviewSummaryEntry>> {
    let entity = catch!( review_summaries::update_review_summary( input.id ) );

    Ok(composition( entity, ENTITY_MD ))
}


// Reaction Summary zome functions
#[hdk_extern]
fn create_reaction_summary_for_subject(input: reaction_summaries::ReactionSummaryInput) -> ExternResult<EntityResponse<ReactionSummaryEntry>> {
    let entity = catch!( reaction_summaries::create_reaction_summary( input ) );

    Ok(composition( entity, ENTITY_MD ))
}

#[hdk_extern]
fn get_reaction_summary(input: GetEntityInput) -> ExternResult<EntityResponse<ReactionSummaryEntry>> {
    let entity = catch!( reaction_summaries::get_reaction_summary( input ) );

    Ok(composition( entity, ENTITY_MD ))
}

#[hdk_extern]
fn get_reaction_summaries_for_subject(input: GetEntityInput) -> ExternResult<DevHubResponse<Vec<Entity<ReactionSummaryEntry>>>> {
    let (base_path, _) = devhub_types::create_path( ANCHOR_SUMMARIES, vec![ input.id ] );
    let collection = catch!( devhub_types::get_entities_for_path( base_path, LinkTypes::ReactionSummary, None ) );

    Ok(composition( collection, ENTITY_COLLECTION_MD ))
}

#[hdk_extern]
fn update_reaction_summary(input: GetEntityInput) -> ExternResult<EntityResponse<ReactionSummaryEntry>> {
    let entity = catch!( reaction_summaries::update_reaction_summary( input.id ) );

    Ok(composition( entity, ENTITY_MD ))
}
