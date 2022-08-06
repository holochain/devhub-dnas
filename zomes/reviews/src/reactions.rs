use std::collections::BTreeMap;
use dnarepo_core::{
    LinkTypes,
};
use devhub_types::{
    AppResult, UpdateEntityInput, GetEntityInput,
    dnarepo_entry_types::{
	ReactionEntry,
    },
    fmt_path,
};
use hc_crud::{
    now, create_entity, get_entity, update_entity,
    Entity, // UtilsError,
};
use hdk::prelude::*;

use crate::constants::{
    // LT_NONE,
    // TAG_REACTION,
    ANCHOR_REACTIONS,
};




#[derive(Debug, Deserialize)]
pub struct ReactionInput {
    pub subject_ids: Vec<(EntryHash, ActionHash)>,
    pub reaction_type: u64,

    // optional
    pub published_at: Option<u64>,
    pub last_updated: Option<u64>,
    pub metadata: Option<BTreeMap<String, serde_yaml::Value>>,
    pub related_entries: Option<BTreeMap<String, EntryHash>>,
}

pub fn create_reaction(input: ReactionInput) -> AppResult<Entity<ReactionEntry>> {
    debug!("Creating Reaction for: subject IDs {:?} ", input.subject_ids );
    let pubkey = agent_info()?.agent_initial_pubkey;
    let default_now = now()?;

    let reaction = ReactionEntry {
	subject_ids: input.subject_ids.to_owned(),
	author: pubkey.clone(),
	reaction_type: input.reaction_type,
	published_at: input.published_at
	    .unwrap_or( default_now ),
	last_updated: input.last_updated
	    .unwrap_or( default_now ),
	metadata: input.metadata
	    .unwrap_or( BTreeMap::new() ),
	related_entries: input.related_entries,
	deleted: false,
    };

    let entity = create_entity( &reaction )?;

    // Author (Agent) anchor
    let (base, base_hash) = devhub_types::create_path( &crate::agent_path_base( None ), vec![ ANCHOR_REACTIONS ] );
    debug!("Linking agent ({}) to ENTRY: {}", fmt_path( &base ), entity.id );
    entity.link_from( &base_hash, LinkTypes::Reaction, None )?;

    for (subject_id, _) in input.subject_ids {
	let (base, base_hash) = devhub_types::create_path( ANCHOR_REACTIONS, vec![ subject_id.to_owned() ] );
	debug!("Linking agent ({}) to ENTRY: {}", fmt_path( &base ), entity.id );
	entity.link_from( &base_hash, LinkTypes::Reaction, None )?;
    }

    Ok( entity )
}




pub fn get_reaction(input: GetEntityInput) -> AppResult<Entity<ReactionEntry>> {
    debug!("Get Reaction: {}", input.id );
    let entity = get_entity( &input.id )?;

    Ok( entity )
}




#[derive(Debug, Deserialize, Clone)]
pub struct ReactionUpdateOptions {
    pub reaction_type: Option<u64>,
    pub published_at: Option<u64>,
    pub last_updated: Option<u64>,
    pub metadata: Option<BTreeMap<String, serde_yaml::Value>>,
    pub related_entries: Option<BTreeMap<String, EntryHash>>,
}
pub type ReactionUpdateInput = UpdateEntityInput<ReactionUpdateOptions>;

pub fn update_reaction(input: ReactionUpdateInput) -> AppResult<Entity<ReactionEntry>> {
    debug!("Updating Reaction: {}", input.addr );
    let props = input.properties.clone();
    let default_now = now()?;

    let entity = update_entity(
	&input.addr,
	|mut current : ReactionEntry, _| {
	    current.reaction_type = props.reaction_type
		.unwrap_or( current.reaction_type );
	    current.published_at = props.published_at
		.unwrap_or( current.published_at );
	    current.last_updated = props.last_updated
		    .unwrap_or( default_now );
	    current.metadata = props.metadata
		.unwrap_or( current.metadata );
	    current.related_entries = props.related_entries
		.or( current.related_entries );
	    current.deleted = false;

	    Ok( current )
	})?;

    Ok( entity )
}




pub fn delete_reaction(addr: ActionHash) -> AppResult<Entity<ReactionEntry>> {
    debug!("Delete Reaction Version: {}", addr );
    let entity = update_entity(
	&addr,
	|mut current : ReactionEntry, _| {
	    current.deleted = true;

	    Ok( current )
	})?;

    Ok( entity )
}
