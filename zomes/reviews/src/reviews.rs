use std::collections::BTreeMap;
use devhub_types::{
    AppResult, UpdateEntityInput, GetEntityInput,
    dnarepo_entry_types::{
	ReviewEntry,
    },
    fmt_path,
};
use hc_crud::{
    now, create_entity, get_entity, update_entity, delete_entity,
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
    pub subject_id: EntryHash,
    pub subject_addr: EntryHash,
    pub rating: u8,
    pub message: String,

    // optional
    pub published_at: Option<u64>,
    pub last_updated: Option<u64>,
    pub metadata: Option<BTreeMap<String, serde_yaml::Value>>,
}

pub fn create_review(input: ReviewInput) -> AppResult<Entity<ReviewEntry>> {
    debug!("Creating Review for: {} (subject {})", input.subject_addr, input.subject_id );
    let pubkey = agent_info()?.agent_initial_pubkey;
    let default_now = now()?;

    let review = ReviewEntry {
	subject_id: input.subject_id.to_owned(),
	subject_addr: input.subject_addr.to_owned(),
	author: pubkey.clone(),
	rating: input.rating,
	message: input.message,
	published_at: input.published_at
	    .unwrap_or( default_now ),
	last_updated: input.last_updated
	    .unwrap_or( default_now ),
	metadata: input.metadata
	    .unwrap_or( BTreeMap::new() ),
    };

    let entity = create_entity( &review )?;

    // Author (Agent) anchor
    let (base, base_hash) = devhub_types::create_path( &crate::agent_path_base( None ), vec![ ANCHOR_REVIEWS ] );
    debug!("Linking agent ({}) to ENTRY: {}", fmt_path( &base ), entity.id );
    entity.link_from( &base_hash, LT_NONE, TAG_REVIEW.into() )?;

    // Revision's reviews
    let (base, base_hash) = devhub_types::create_path( ANCHOR_REVIEWS, vec![ input.subject_addr.to_owned() ] );
    debug!("Linking agent ({}) to ENTRY: {}", fmt_path( &base ), entity.id );
    entity.link_from( &base_hash, LT_NONE, TAG_REVIEW.into() )?;

    if input.subject_id != input.subject_addr {
	// Subject's reviews
	let (base, base_hash) = devhub_types::create_path( ANCHOR_REVIEWS, vec![ input.subject_id ] );
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
    pub rating: Option<u8>,
    pub message: Option<String>,
    pub published_at: Option<u64>,
    pub last_updated: Option<u64>,
    pub metadata: Option<BTreeMap<String, serde_yaml::Value>>,
}
pub type ReviewUpdateInput = UpdateEntityInput<ReviewUpdateOptions>;

pub fn update_review(input: ReviewUpdateInput) -> AppResult<Entity<ReviewEntry>> {
    debug!("Updating Review: {}", input.addr );
    let props = input.properties.clone();

    let entity = update_entity(
	&input.addr,
	|mut current : ReviewEntry, _| {
	    current.rating = props.rating
		.unwrap_or( current.rating );
	    current.message = props.message
		.unwrap_or( current.message );
	    current.published_at = props.published_at
		.unwrap_or( current.published_at );
	    current.last_updated = props.last_updated
		.unwrap_or( current.last_updated );
	    current.metadata = props.metadata
		.unwrap_or( current.metadata );

	    Ok( current )
	})?;

    Ok( entity )
}




pub fn delete_review(input: GetEntityInput) -> AppResult<HeaderHash> {
    debug!("Delete Review Version: {}", input.id );
    let delete_header = delete_entity::<ReviewEntry>( &input.id )?;
    debug!("Deleted Review Version header ({})", delete_header );

    Ok( delete_header )
}
