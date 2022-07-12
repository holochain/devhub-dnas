use std::collections::BTreeMap;
use devhub_types::{
    AppResult, AppError, UserError, GetEntityInput,
    dnarepo_entry_types::{
	ReviewEntry,
	ReviewSummaryEntry,
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
    TAG_REVIEW,
    TAG_SUMMARY,
    ANCHOR_REVIEWS,
    ANCHOR_SUMMARIES,
};



fn assemble_summary_entry(subject_header: &HeaderHash) -> AppResult<ReviewSummaryEntry> {
    debug!("Creating Review Summary for: {})", subject_header );
    let subject_history = trace_action_history( subject_header )?;
    let subject_pointer = subject_history.last().unwrap();
    let subject_id = subject_pointer.1.to_owned();

    let mut review_refs = BTreeMap::new();
    let mut deleted_reviews = BTreeMap::new();

    let (_, base_hash) = devhub_types::create_path( ANCHOR_REVIEWS, vec![ &subject_id ] );
    let review_links = get_links(
        base_hash.clone(),
	Some(LinkTag::new( Vec::<u8>::from(TAG_REVIEW) ))
    )?;

    let mut factored_count : u64 = 0;

    debug!("Using {} review links for summary report", review_links.len() );
    for link in review_links.iter() {
	let review_id_b64 = format!("{}", link.target );

	if review_refs.contains_key( &review_id_b64 ) {
	    debug!("Skipping duplicate review {}", review_id_b64 );
	    continue;
	}

	factored_count = factored_count + 1;

	let review = get_entity::<ReviewEntry>( &link.target.to_owned().into() )?;

	if review.content.subject_ids.iter().find( |pair| pair.0 == subject_id ).is_none() {
	    debug!("Review doesn't belong to this subject: ID {} not in review subjects {:?}", subject_id, review.content.subject_ids );
	    continue;
	}

	let mut action_count = 1;

	if review.id != review.address {
	    let (origin_id, depth) = trace_header_origin_entry( &review.header, None )?;

	    if origin_id != review.id {
		Err(AppError::UnexpectedStateError(format!("Traced origin ID for header ({}) does not match review ID: {} != {}", review.header, origin_id, review.id )))?
	    }

	    debug!("Adding depth {} for {}", depth, review_id_b64 );
	    factored_count = factored_count + depth;
	    action_count = action_count + depth;
	}

	if review.content.deleted {
	    debug!("Link target {} is a deleted review", link.target );
	    deleted_reviews.insert( review_id_b64, (review.id, review.header) );
	    continue;
	}

	review_refs.insert( review_id_b64, (review.id, review.header, review.content.author, action_count, review.content.ratings) );
    }

    if review_refs.len() == 0 {
	Err(UserError::UnmetRequirementsError(format!("Review summary must have at least 1 review: {}", review_refs.len() )))?
    }

    Ok( ReviewSummaryEntry {
	subject_id: subject_id.to_owned(),
	subject_history: subject_history.into_iter()
	    .map( |(header,_)| header )
	    .collect(),
	published_at: now()?,

	factored_action_count: factored_count,

	review_refs: review_refs,
	deleted_reviews: deleted_reviews,
    } )
}



#[derive(Debug, Deserialize)]
pub struct ReviewSummaryInput {
    pub subject_header: HeaderHash,
}

pub fn create_review_summary(input: ReviewSummaryInput) -> AppResult<Entity<ReviewSummaryEntry>> {
    debug!("Creating Review Summary for: {}", input.subject_header );
    let summary = assemble_summary_entry( &input.subject_header )?;
    let entity = create_entity( &summary )?;

    // Revision's summarys
    let (base, base_hash) = devhub_types::create_path( ANCHOR_SUMMARIES, vec![ summary.subject_id.to_owned() ] );
    debug!("Linking agent ({}) to ENTRY: {}", fmt_path( &base ), entity.id );
    entity.link_from( &base_hash, LT_NONE, TAG_SUMMARY.into() )?;

    Ok( entity )
}




pub fn get_review_summary(input: GetEntityInput) -> AppResult<Entity<ReviewSummaryEntry>> {
    debug!("Get Review Summary: {}", input.id );
    let entity = get_entity::<ReviewSummaryEntry>( &input.id )?;

    Ok( entity )
}




pub fn update_review_summary(id: EntryHash) -> AppResult<Entity<ReviewSummaryEntry>> {
    let summary = get_entity::<ReviewSummaryEntry>( &id )?;
    let (subject_header, ..) = fetch_element_latest( &summary.content.subject_id )?;
    let updated_summary = assemble_summary_entry( &subject_header )?;

    debug!("New summary {} + {}: {:?}", updated_summary.review_refs.len(), updated_summary.deleted_reviews.len(), updated_summary.deleted_reviews );
    if !( updated_summary.factored_action_count > summary.content.factored_action_count ) {
	Err(UserError::UnmetRequirementsError(format!("The updated summary is not better than the current summary: new factored action count ({}) must be greater than {}", updated_summary.factored_action_count, summary.content.factored_action_count )))?
    }

    let entity = update_entity(
	&summary.address,
	|_ : ReviewSummaryEntry, _| {
	    Ok( updated_summary )
	})?;

    Ok( entity )
}
