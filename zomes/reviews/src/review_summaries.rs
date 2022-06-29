use std::collections::BTreeMap;
use devhub_types::{
    AppResult, AppError,
    dnarepo_entry_types::{
	ReviewEntry,
	ReviewSummaryEntry,
	trace_header_origin_entry,
    },
    fmt_path,
};
use hc_crud::{
    now, create_entity, get_entity,
    Entity,
    UtilsError,
};
use hdk::prelude::*;

use crate::constants::{
    LT_NONE,
    TAG_REVIEW,
    TAG_SUMMARY,
    ANCHOR_REVIEWS,
    ANCHOR_SUMMARIES,
};



#[derive(Debug, Deserialize)]
pub struct ReviewSummaryInput {
    pub subject_id: EntryHash,
    pub subject_addr: EntryHash,
}

pub fn create_summary(input: ReviewSummaryInput) -> AppResult<Entity<ReviewSummaryEntry>> {
    debug!("Creating Review Summary for: {} (subject {})", input.subject_addr, input.subject_id );

    let mut review_refs : BTreeMap<String,(EntryHash,HeaderHash)> = BTreeMap::new();

    let (_, base_hash) = devhub_types::create_path( ANCHOR_REVIEWS, vec![ &input.subject_addr ] );
    let review_links = get_links(
        base_hash.clone(),
	Some(LinkTag::new( Vec::<u8>::from(TAG_REVIEW) ))
    )?;

    let mut all_ratings = Vec::new();
    let mut deleted_reviews = Vec::new();
    let mut rating_sum : f32 = 0.0;
    let mut factored_count : u64 = 0;

    for link in review_links.iter() {
	let review_id_b64 = format!("{}", link.target );

	if review_refs.contains_key( &review_id_b64 ) {
	    continue;
	}

	let review = match get_entity::<ReviewEntry>( &link.target.to_owned().into() ) {
	    Err(UtilsError::EntryNotFoundError(_)) => {
		deleted_reviews.push( link.target.to_owned().into() );
		continue;
	    },
	    response => response?,
	};

	factored_count = factored_count + 1;

	if review.id != review.address {
	    let (origin_id, depth) = trace_header_origin_entry( &review.header, None )?;

	    if origin_id != review.id {
		Err(AppError::UnexpectedStateError(format!("Traced origin ID for header ({}) does not match review ID: {} != {}", review.header, origin_id, review.id )))?
	    }

	    factored_count = factored_count + depth;
	}

	review_refs.insert( review_id_b64, (review.id, review.header) );

	all_ratings.push( review.content.rating );
	rating_sum = rating_sum + (review.content.rating as f32);
    }

    let rating_count = all_ratings.len() as f32;
    all_ratings.sort();
    let median : u8 = all_ratings[ (all_ratings.len() - 1) / 2 ];

    debug!(
	"Ratings average {} / {} = {} : {:?}",
	rating_sum,
	rating_count,
	rating_sum / rating_count,
	all_ratings
    );
    let summary = ReviewSummaryEntry {
	subject_id: input.subject_id.to_owned(),
	subject_addr: input.subject_addr.to_owned(),
	published_at: now()?,

	average: rating_sum / rating_count,
	median: median,

	review_count: review_refs.len() as u64,
	factored_action_count: factored_count,
	review_refs: review_refs,
	deleted_reviews: deleted_reviews,
    };

    let entity = create_entity( &summary )?;

    // Revision's summarys
    let (base, base_hash) = devhub_types::create_path( ANCHOR_SUMMARIES, vec![ input.subject_addr.to_owned() ] );
    debug!("Linking agent ({}) to ENTRY: {}", fmt_path( &base ), entity.id );
    entity.link_from( &base_hash, LT_NONE, TAG_SUMMARY.into() )?;

    if input.subject_id != input.subject_addr {
	// Subject's summarys
	let (base, base_hash) = devhub_types::create_path( ANCHOR_SUMMARIES, vec![ input.subject_id ] );
	debug!("Linking agent ({}) to ENTRY: {}", fmt_path( &base ), entity.id );
	entity.link_from( &base_hash, LT_NONE, TAG_SUMMARY.into() )?;
    }

    Ok( entity )
}
