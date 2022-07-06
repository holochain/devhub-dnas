use std::collections::BTreeMap;
use devhub_types::{
    AppResult, AppError, UserError, GetEntityInput,
    dnarepo_entry_types::{
	ReviewEntry,
	ReviewSummaryEntry,
	trace_header_origin_entry,
    },
    fmt_path,
};
use hc_crud::{
    now, create_entity, get_entity, update_entity,
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



fn assemble_summary_entry(subject_id: &EntryHash) -> AppResult<ReviewSummaryEntry> {
    debug!("Creating Review Summary for: {})", subject_id );

    let mut review_refs : BTreeMap<String,(EntryHash,HeaderHash)> = BTreeMap::new();
    let mut deleted_reviews : BTreeMap<String,(EntryHash,HeaderHash)> = BTreeMap::new();

    let (_, base_hash) = devhub_types::create_path( ANCHOR_REVIEWS, vec![ &subject_id ] );
    let review_links = get_links(
        base_hash.clone(),
	Some(LinkTag::new( Vec::<u8>::from(TAG_REVIEW) ))
    )?;

    let mut all_accuracy_ratings = Vec::new();
    let mut all_efficiency_ratings = Vec::new();

    let mut accuracy_rating_sum : f32 = 0.0;
    let mut efficiency_rating_sum : f32 = 0.0;

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

	if !review.content.subject_ids.contains( subject_id ) {
	    debug!("Review doesn't belong to this subject: ID {} not in review subjects {:?}", subject_id, review.content.subject_ids );
	    continue;
	}

	if review.id != review.address {
	    let (origin_id, depth) = trace_header_origin_entry( &review.header, None )?;

	    if origin_id != review.id {
		Err(AppError::UnexpectedStateError(format!("Traced origin ID for header ({}) does not match review ID: {} != {}", review.header, origin_id, review.id )))?
	    }

	    debug!("Adding depth {} for {}", depth, review_id_b64 );
	    factored_count = factored_count + depth;
	}

	if review.content.deleted {
	    debug!("Link target {} is a deleted review", link.target );
	    deleted_reviews.insert( review_id_b64, (review.id, review.header) );
	    continue;
	}

	review_refs.insert( review_id_b64, (review.id, review.header) );

	if let Some(rating) = review.content.accuracy_rating {
	    accuracy_rating_sum = accuracy_rating_sum + (rating as f32);
	    all_accuracy_ratings.push( rating );
	}

	if let Some(rating) = review.content.efficiency_rating {
	    efficiency_rating_sum = efficiency_rating_sum + (rating as f32);
	    all_efficiency_ratings.push( rating );
	}
    }

    let accuracy_rating_count = all_accuracy_ratings.len();
    let (accuracy_average, accuracy_median) = match accuracy_rating_count {
	0 => (0.0, 0),
	count => {
	    all_accuracy_ratings.sort();
	    (
		accuracy_rating_sum / count as f32,
		all_accuracy_ratings[ (count - 1) / 2 ]
	    )
	},
    };

    debug!(
	"Ratings average {} / {} = {} : {:?}",
	accuracy_rating_sum,
	accuracy_rating_count,
	accuracy_average,
	all_accuracy_ratings
    );

    let efficiency_rating_count = all_efficiency_ratings.len();
    let (efficiency_average, efficiency_median) = match efficiency_rating_count {
	0 => (0.0, 0),
	count => {
	    all_efficiency_ratings.sort();
	    (
		efficiency_rating_sum / count as f32,
		all_efficiency_ratings[ (count - 1) / 2 ]
	    )
	},
    };

    debug!(
	"Ratings average {} / {} = {} : {:?}",
	efficiency_rating_sum,
	efficiency_rating_count,
	efficiency_average,
	all_efficiency_ratings
    );

    Ok( ReviewSummaryEntry {
	subject_id: subject_id.to_owned(),
	published_at: now()?,

	accuracy_average: accuracy_average,
	accuracy_median: accuracy_median,

	efficiency_average: efficiency_average,
	efficiency_median: efficiency_median,

	review_count: review_refs.len() as u64,
	factored_action_count: factored_count,
	review_refs: review_refs,
	deleted_reviews: deleted_reviews,
    } )
}



#[derive(Debug, Deserialize)]
pub struct ReviewSummaryInput {
    pub subject_id: EntryHash,
}

pub fn create_review_summary(input: ReviewSummaryInput) -> AppResult<Entity<ReviewSummaryEntry>> {
    debug!("Creating Review Summary for: {}", input.subject_id );
    let summary = assemble_summary_entry( &input.subject_id )?;

    if summary.review_count == 0 {
	Err(UserError::UnmetRequirementsError(format!("Review summary must have at least 1 review: {}", summary.review_count )))?
    }

    let entity = create_entity( &summary )?;

    // Revision's summarys
    let (base, base_hash) = devhub_types::create_path( ANCHOR_SUMMARIES, vec![ input.subject_id.to_owned() ] );
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
    let updated_summary = assemble_summary_entry( &summary.content.subject_id )?;

    debug!("New summary {} + {}: {:?}", updated_summary.review_count, updated_summary.deleted_reviews.len(), updated_summary.deleted_reviews );
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
