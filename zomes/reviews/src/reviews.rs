use std::collections::BTreeMap;
use devhub_types::{
    AppResult, UpdateEntityInput, GetEntityInput,
    dnarepo_entry_types::{
	ReviewEntry,
    },
    fmt_path,
};
use hc_crud::{
    now, create_entity, get_entity, update_entity,
    Entity, // UtilsError,
};
use hdk::prelude::*;

use crate::constants::{
    LT_NONE,
    TAG_REVIEW,
    ANCHOR_REVIEWS,
};




#[derive(Debug, Deserialize)]
pub struct ReviewInput {
    pub subject_ids: Vec<(EntryHash, HeaderHash)>,
    pub ratings: BTreeMap<String,u8>,
    pub message: String,

    // optional
    pub published_at: Option<u64>,
    pub last_updated: Option<u64>,
    pub metadata: Option<BTreeMap<String, serde_yaml::Value>>,
    pub related_entries: Option<BTreeMap<String, EntryHash>>,
}

pub fn create_review(input: ReviewInput) -> AppResult<Entity<ReviewEntry>> {
    debug!("Creating Review for: subject IDs {:?} ", input.subject_ids );
    let pubkey = agent_info()?.agent_initial_pubkey;
    let default_now = now()?;

    let review = ReviewEntry {
	subject_ids: input.subject_ids.to_owned(),
	author: pubkey.clone(),
	ratings: input.ratings,
	message: input.message,
	published_at: input.published_at
	    .unwrap_or( default_now ),
	last_updated: input.last_updated
	    .unwrap_or( default_now ),
	metadata: input.metadata
	    .unwrap_or( BTreeMap::new() ),
	related_entries: input.related_entries,
	deleted: false,
    };

    let entity = create_entity( &review )?;

    // Author (Agent) anchor
    let (base, base_hash) = devhub_types::create_path( &crate::agent_path_base( None ), vec![ ANCHOR_REVIEWS ] );
    debug!("Linking agent ({}) to ENTRY: {}", fmt_path( &base ), entity.id );
    entity.link_from( &base_hash, LT_NONE, TAG_REVIEW.into() )?;

    for (subject_id, _) in input.subject_ids {
	let (base, base_hash) = devhub_types::create_path( ANCHOR_REVIEWS, vec![ subject_id.to_owned() ] );
	debug!("Linking agent ({}) to ENTRY: {}", fmt_path( &base ), entity.id );
	entity.link_from( &base_hash, LT_NONE, TAG_REVIEW.into() )?;
    }

    Ok( entity )
}




pub fn get_review(input: GetEntityInput) -> AppResult<Entity<ReviewEntry>> {
    debug!("Get Review: {}", input.id );
    let entity = get_entity::<ReviewEntry>( &input.id )?;

    Ok( entity )
}




#[derive(Debug, Deserialize, Clone)]
pub struct ReviewUpdateOptions {
    pub ratings: Option<BTreeMap<String,u8>>,
    pub message: Option<String>,
    pub published_at: Option<u64>,
    pub last_updated: Option<u64>,
    pub metadata: Option<BTreeMap<String, serde_yaml::Value>>,
    pub related_entries: Option<BTreeMap<String, EntryHash>>,
}
pub type ReviewUpdateInput = UpdateEntityInput<ReviewUpdateOptions>;

pub fn update_review(input: ReviewUpdateInput) -> AppResult<Entity<ReviewEntry>> {
    debug!("Updating Review: {}", input.addr );
    let props = input.properties.clone();

    let entity = update_entity(
	&input.addr,
	|mut current : ReviewEntry, _| {
	    current.ratings = props.ratings
		.unwrap_or( current.ratings );
	    current.message = props.message
		.unwrap_or( current.message );
	    current.published_at = props.published_at
		.unwrap_or( current.published_at );
	    current.last_updated = props.last_updated
		.unwrap_or( current.last_updated );
	    current.metadata = props.metadata
		.unwrap_or( current.metadata );
	    current.related_entries = props.related_entries
		.or( current.related_entries );

	    Ok( current )
	})?;

    Ok( entity )
}




pub fn delete_review(addr: EntryHash) -> AppResult<Entity<ReviewEntry>> {
    debug!("Delete Review Version: {}", addr );
    let entity = update_entity(
	&addr,
	|mut current : ReviewEntry, _| {
	    current.deleted = true;

	    Ok( current )
	})?;

    Ok( entity )
}
