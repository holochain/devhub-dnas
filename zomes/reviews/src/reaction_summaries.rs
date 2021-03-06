use std::collections::BTreeMap;
use devhub_types::{
    AppResult, AppError, UserError, GetEntityInput,
    dnarepo_entry_types::{
	ReactionEntry,
	ReactionSummaryEntry,
	trace_header_origin_entry,
	trace_action_history,
    },
    fmt_path,
};
use hc_crud::{
    now, create_entity, get_entity, update_entity,
    fetch_element_latest,
    Entity,
};
use hdk::prelude::*;

use crate::constants::{
    LT_NONE,
    TAG_REACTION,
    TAG_SUMMARY,
    ANCHOR_REACTIONS,
    ANCHOR_SUMMARIES,
};



fn assemble_summary_entry(subject_header: &HeaderHash) -> AppResult<ReactionSummaryEntry> {
    debug!("Assembling Reaction Summary based on subject starting point: {}", subject_header );
    let subject_history = trace_action_history( subject_header )?;
    let subject_pointer = subject_history.last().unwrap();
    let subject_id = subject_pointer.1.to_owned();
    debug!("Subject's root entry ID: {}", subject_id );

    let mut reaction_refs = BTreeMap::new();
    let mut deleted_reactions = BTreeMap::new();

    let (_, base_hash) = devhub_types::create_path( ANCHOR_REACTIONS, vec![ &subject_id ] );
    let reaction_links = get_links(
        base_hash.clone(),
	Some(LinkTag::new( Vec::<u8>::from(TAG_REACTION) ))
    )?;

    let mut factored_count : u64 = 0;

    debug!("Using {} reaction links for summary report", reaction_links.len() );
    for link in reaction_links.iter() {
	let reaction_id_b64 = format!("{}", link.target );

	if reaction_refs.contains_key( &reaction_id_b64 ) {
	    debug!("Skipping duplicate reaction {}", reaction_id_b64 );
	    continue;
	}

	factored_count = factored_count + 1;

	let reaction = get_entity::<ReactionEntry>( &link.target.to_owned().into() )?;

	if reaction.content.subject_ids.iter().find( |pair| pair.0 == subject_id ).is_none() {
	    debug!("Reaction doesn't belong to this subject: ID {} not in reaction subjects {:?}", subject_id, reaction.content.subject_ids );
	    continue;
	}

	let mut action_count = 1;

	if reaction.id != reaction.address {
	    let (origin_id, depth) = trace_header_origin_entry( &reaction.header, None )?;

	    if origin_id != reaction.id {
		Err(AppError::UnexpectedStateError(format!("Traced origin ID for header ({}) does not match reaction ID: {} != {}", reaction.header, origin_id, reaction.id )))?
	    }

	    debug!("Adding depth {} for {}", depth, reaction_id_b64 );
	    factored_count = factored_count + depth;
	    action_count = action_count + depth;
	}

	if reaction.content.deleted {
	    debug!("Link target {} is a deleted reaction", link.target );
	    deleted_reactions.insert( reaction_id_b64, (reaction.id, reaction.header) );
	    continue;
	}

	reaction_refs.insert( reaction_id_b64, (reaction.id, reaction.header, reaction.content.author, action_count, reaction.content.reaction_type) );
    }

    if reaction_refs.len() == 0 {
	Err(UserError::UnmetRequirementsError(format!("Reaction summary must have at least 1 reaction: {}", reaction_refs.len() )))?
    }

    let default_now = now()?;

    Ok( ReactionSummaryEntry {
	subject_id: subject_id.to_owned(),
	subject_history: subject_history.into_iter()
	    .map( |(header,_)| header )
	    .collect(),
	published_at: default_now,
	last_updated: default_now,

	factored_action_count: factored_count,

	reaction_refs: reaction_refs,
	deleted_reactions: deleted_reactions,
    } )
}



#[derive(Debug, Deserialize)]
pub struct ReactionSummaryInput {
    pub subject_header: HeaderHash,
}

pub fn create_reaction_summary(input: ReactionSummaryInput) -> AppResult<Entity<ReactionSummaryEntry>> {
    debug!("Creating Reaction Summary for: {}", input.subject_header );
    let summary = assemble_summary_entry( &input.subject_header )?;
    let entity = create_entity( &summary )?;

    // Revision's summarys
    let (base, base_hash) = devhub_types::create_path( ANCHOR_SUMMARIES, vec![ summary.subject_id.to_owned() ] );
    debug!("Linking agent ({}) to ENTRY: {}", fmt_path( &base ), entity.id );
    entity.link_from( &base_hash, LT_NONE, TAG_SUMMARY.into() )?;

    Ok( entity )
}




pub fn get_reaction_summary(input: GetEntityInput) -> AppResult<Entity<ReactionSummaryEntry>> {
    debug!("Get Reaction Summary: {}", input.id );
    let entity = get_entity::<ReactionSummaryEntry>( &input.id )?;

    Ok( entity )
}




pub fn update_reaction_summary(id: EntryHash) -> AppResult<Entity<ReactionSummaryEntry>> {
    let summary = get_entity::<ReactionSummaryEntry>( &id )?;
    let (subject_header, ..) = fetch_element_latest( &summary.content.subject_id )?;
    let updated_summary = assemble_summary_entry( &subject_header )?;

    debug!("New summary {} + {}: {:?}", updated_summary.reaction_refs.len(), updated_summary.deleted_reactions.len(), updated_summary.deleted_reactions );
    if !( updated_summary.factored_action_count > summary.content.factored_action_count ) {
	Err(UserError::UnmetRequirementsError(format!("The updated summary is not better than the current summary: new factored action count ({}) must be greater than {}", updated_summary.factored_action_count, summary.content.factored_action_count )))?
    }

    let entity = update_entity(
	&summary.address,
	|_ : ReactionSummaryEntry, _| {
	    Ok( updated_summary )
	})?;

    Ok( entity )
}
