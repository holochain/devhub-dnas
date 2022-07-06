use hdk::prelude::*;

use devhub_types::{
    dnarepo_entry_types::{
	ReviewEntry,
	ReviewSummaryEntry,
	trace_header_origin_entry,
    },
};



#[hdk_extern]
fn validate(op: Op) -> ExternResult<ValidateCallbackResult> {
    match op {
	Op::StoreElement { element } => {
	    if let Some(EntryType::App(app_entry_type)) = element.header().entry_type() {
		if app_entry_type.zome_id != zome_info().unwrap().id {
		    // This Element does not belong to our Zome so we don't know how to validate it
		    return Ok(ValidateCallbackResult::Valid);
		}

		debug!("Forwarding validation for StoreElement->Header::Create->EntryType::App to validation handler");
		validate_element( app_entry_type.id, &element )
	    }
	    else if let Header::Delete(delete) = element.header() {
		let sh_header = must_get_header( delete.deletes_address.to_owned() )?;
		let original_header = sh_header.header();

		if let Some(EntryType::App(app_entry_type)) = original_header.entry_type() {
		    if app_entry_type.zome_id != zome_info().unwrap().id {
			// This Element does not belong to our Zome so we don't know how to validate it
			Ok(ValidateCallbackResult::Valid)
		    }
		    else {
			validate_element( app_entry_type.id, &element )
		    }
		}
		else {
		    debug!("Ignoring Delete event of Header that doesn't contain EntryType::App: {:?}", original_header );
		    Ok(ValidateCallbackResult::Valid)
		}
	    }
	    else {
		debug!("Ignoring Op::StoreElement event that doesn't contain EntryType::App: {:?}", element );
		Ok(ValidateCallbackResult::Valid)
	    }
	},
	_ => {
	    debug!("Ignoring Op event");
	    Ok(ValidateCallbackResult::Valid)
	},
    }
}

fn validate_element(entry_def_index: EntryDefIndex, element: &Element) -> ExternResult<ValidateCallbackResult> {
    // By the time we get here, we know it is for our Zome and the element contains an App Entry

    let review_edi : EntryDefIndex = entry_def_index!(ReviewEntry).unwrap();
    let review_summary_edi : EntryDefIndex = entry_def_index!(ReviewSummaryEntry).unwrap();

    match element.header() {
	Header::Create(create) => {
	    debug!("Running create validation for: {:?}", entry_def_index );
	    if entry_def_index == review_edi {
		validate_review_create( create, element.try_into()? )
	    }
	    else if entry_def_index == review_summary_edi {
		validate_review_summary_create( create, element.try_into()? )
	    }
	    else {
		debug!("Ignoring create Op for: {:?}", entry_def_index );
		Ok(ValidateCallbackResult::Valid)
	    }
	}
	Header::Update(update) => {
	    debug!("Running update validation for: {:?}", entry_def_index );
	    if entry_def_index == review_edi {
		validate_review_update( update, element.try_into()? )
	    }
	    else if entry_def_index == review_summary_edi {
		validate_review_summary_update( update, element.try_into()? )
	    }
	    else {
		debug!("Ignoring update Op for: {:?}", entry_def_index );
		Ok(ValidateCallbackResult::Valid)
	    }
	},
	Header::Delete(delete) => {
	    debug!("Running delete validation for: {:?}", entry_def_index );
	    if entry_def_index == review_edi {
		validate_review_delete( delete )
	    }
	    else if entry_def_index == review_summary_edi {
		Ok(ValidateCallbackResult::Invalid("Review summaries cannot be deleted".to_string()))
	    }
	    else {
		debug!("Ignoring delete Op for: {:?}", entry_def_index );
		Ok(ValidateCallbackResult::Valid)
	    }
	},
	_ => {
	    debug!("Nothing implemented for Header type");
	    Ok(ValidateCallbackResult::Invalid(format!("Unknown entry type: {:?}", entry_def_index )))
	},
    }
}



//
// Review
//
fn validate_review_create(header: &header::Create, review: ReviewEntry) -> ExternResult<ValidateCallbackResult> {
    if review.author != header.author {
	return Ok(ValidateCallbackResult::Invalid(format!("ReviewEntry author does not match Header author: {} != {}", review.author, header.author )))
    }
    else if let Some(rating) = review.accuracy_rating {
	if rating > 10 {
	    return Ok(ValidateCallbackResult::Invalid(format!("ReviewEntry accuracy rating ({}) out of range: valid range 0-10", rating )))
	}
    }
    else if let Some(rating) = review.efficiency_rating {
	if rating > 10 {
	    return Ok(ValidateCallbackResult::Invalid(format!("ReviewEntry efficiency rating ({}) out of range: valid range 0-10", rating )))
	}
    }

    Ok(ValidateCallbackResult::Valid)
}

fn validate_review_update(header: &header::Update, review: ReviewEntry) -> ExternResult<ValidateCallbackResult> {
    let prev_entry : ReviewEntry = must_get_entry( header.original_entry_address.to_owned() )?.try_into()?;

    if prev_entry.author != header.author {
	return Ok(ValidateCallbackResult::Invalid(format!("Previous entry author does not match Header author: {} != {}", prev_entry.author, header.author )));
    }

    if review.author != prev_entry.author  {
	return Ok(ValidateCallbackResult::Invalid(format!("Cannot change review author: {} => {}", prev_entry.author, review.author )));
    }

    Ok(ValidateCallbackResult::Valid)
}

fn validate_review_delete(header: &header::Delete) -> ExternResult<ValidateCallbackResult> {
    let prev_entry : ReviewEntry = must_get_entry( header.deletes_entry_address.to_owned() )?.try_into()?;

    if prev_entry.author != header.author {
	Ok(ValidateCallbackResult::Invalid(format!("Deleted entry author does not match Header author: {} != {}", prev_entry.author, header.author )))
    }
    else {
	Ok(ValidateCallbackResult::Valid)
    }
}


//
// Review Summary
//
fn validate_review_summary_content(review_summary: &ReviewSummaryEntry) -> ExternResult<ValidateCallbackResult> {
    let mut all_accuracy_ratings = Vec::new();
    let mut all_efficiency_ratings = Vec::new();

    let mut accuracy_rating_sum : f32 = 0.0;
    let mut efficiency_rating_sum : f32 = 0.0;

    let mut factored_count = (review_summary.review_refs.len() + review_summary.deleted_reviews.len()) as u64;
    let mut all_factored_reviews : Vec<&(EntryHash,HeaderHash)> = Vec::new();

    all_factored_reviews.extend( review_summary.review_refs.values() );
    all_factored_reviews.extend( review_summary.deleted_reviews.values() );

    // Verfiy review references
    for (review_id, review_header_hash) in all_factored_reviews {
	let review_element = must_get_valid_element( review_header_hash.to_owned().into() )?;

	if let Header::Update(update) = review_element.header() {
	    let (origin_id, depth) = trace_header_origin_entry( &update.original_header_address, Some(1) )?;

	    if origin_id != *review_id {
		return Ok(ValidateCallbackResult::Invalid(format!("Traced origin ID for header ({}) does not match review ID: {} != {}", review_header_hash, origin_id, review_id )))
	    }

	    debug!("Counting depth {} for {}", depth, origin_id );
	    factored_count = factored_count + depth;
	}

	if let Header::Create(create) = review_element.header() {
	    if create.entry_hash != *review_id {
		return Ok(ValidateCallbackResult::Invalid(format!("Header is not related to review ID: {} != {}", create.entry_hash, review_id )))
	    }
	}

	match review_element.entry() {
	    ElementEntry::Present(entry) => {
		let review : ReviewEntry = entry.try_into()?;

		if !review.subject_ids.contains( &review_summary.subject_id ) {
		    return Ok(ValidateCallbackResult::Invalid(format!("Contains review that does not belong to subject: {}", review_id )))
		}

		if review.deleted {
		    debug!("Checking for deleted reviews: {:?}", review_summary.review_refs.keys() );
		    if review_summary.review_refs.contains_key(&format!("{}", review_id )) {
			return Ok(ValidateCallbackResult::Invalid(format!("Deleted review {} cannot be used in review refs", review_id )))
		    }
		    continue;
		}

		if let Some(rating) = review.accuracy_rating {
		    accuracy_rating_sum = accuracy_rating_sum + (rating as f32);
		    all_accuracy_ratings.push( rating );
		}

		if let Some(rating) = review.efficiency_rating {
		    efficiency_rating_sum = efficiency_rating_sum + (rating as f32);
		    all_efficiency_ratings.push( rating );
		}
	    },
	    entry => {
		return Ok(ValidateCallbackResult::Invalid(format!("Expected header {} to have an app entry, not {:?}", review_header_hash, entry )))
	    },
	}
    }

    // Check review count
    if review_summary.review_count != review_summary.review_refs.len() as u64 {
	return Ok(ValidateCallbackResult::Invalid(format!("ReviewSummaryEntry's review count does not equal reference count: {} != {}", review_summary.review_count, review_summary.review_refs.len() )))
    }
    // Check factored review count
    if review_summary.factored_action_count != factored_count {
	return Ok(ValidateCallbackResult::Invalid(format!("ReviewSummaryEntry's factored review count does not equal the number of indirect review references: {} != {}", review_summary.factored_action_count, factored_count )))
    }

    // Check accuracy average / median
    if all_accuracy_ratings.len() > 0 {
	let accuracy_rating_count = all_accuracy_ratings.len() as f32;

	if review_summary.accuracy_average != (accuracy_rating_sum / accuracy_rating_count) {
	    return Ok(ValidateCallbackResult::Invalid(format!("ReviewSummaryEntry's accuracy average ({}) is not accurate, expected {}: {:?}", review_summary.accuracy_average, (accuracy_rating_sum / accuracy_rating_count), all_accuracy_ratings )))
	}

	all_accuracy_ratings.sort();
	let accuracy_median : u8 = all_accuracy_ratings[ (all_accuracy_ratings.len() - 1) / 2 ];

	if review_summary.accuracy_median != accuracy_median {
	    return Ok(ValidateCallbackResult::Invalid(format!("ReviewSummaryEntry's accuracy median ({}) is not accurate, expected {}: {:?}", review_summary.accuracy_median, accuracy_median, all_accuracy_ratings )))
	}
    }
    else {
	if review_summary.accuracy_average != 0.0 {
	    return Ok(ValidateCallbackResult::Invalid(format!("ReviewSummaryEntry's accuracy average ({}) should be 0.0 because there are no ratings: {:?}", review_summary.accuracy_average, all_accuracy_ratings )))
	}

	if review_summary.accuracy_median != 0 {
	    return Ok(ValidateCallbackResult::Invalid(format!("ReviewSummaryEntry's accuracy median ({}) should be 0 because there are no ratings: {:?}", review_summary.accuracy_average, all_accuracy_ratings )))
	}
    }

    // Check efficiency average / median
    if all_efficiency_ratings.len() > 0 {
	let efficiency_rating_count = all_efficiency_ratings.len() as f32;

	if review_summary.efficiency_average != (efficiency_rating_sum / efficiency_rating_count) {
	    return Ok(ValidateCallbackResult::Invalid(format!("ReviewSummaryEntry's efficiency average ({}) is not accurate, expected {}: {:?}", review_summary.efficiency_average, (efficiency_rating_sum / efficiency_rating_count), all_efficiency_ratings )))
	}

	all_efficiency_ratings.sort();
	let efficiency_median : u8 = all_efficiency_ratings[ (all_efficiency_ratings.len() - 1) / 2 ];

	if review_summary.efficiency_median != efficiency_median {
	    return Ok(ValidateCallbackResult::Invalid(format!("ReviewSummaryEntry's efficiency median ({}) is not accurate, expected {}: {:?}", review_summary.efficiency_median, efficiency_median, all_efficiency_ratings )))
	}
    }
    else {
	if review_summary.efficiency_average != 0.0 {
	    return Ok(ValidateCallbackResult::Invalid(format!("ReviewSummaryEntry's efficiency average ({}) should be 0.0 because there are no ratings: {:?}", review_summary.efficiency_average, all_efficiency_ratings )))
	}

	if review_summary.efficiency_median != 0 {
	    return Ok(ValidateCallbackResult::Invalid(format!("ReviewSummaryEntry's efficiency median ({}) should be 0 because there are no ratings: {:?}", review_summary.efficiency_average, all_efficiency_ratings )))
	}
    }

    Ok(ValidateCallbackResult::Valid)
}

fn validate_review_summary_create(_: &header::Create, review_summary: ReviewSummaryEntry) -> ExternResult<ValidateCallbackResult> {
    Ok( validate_review_summary_content( &review_summary )? )
}

fn validate_review_summary_update(header: &header::Update, review_summary: ReviewSummaryEntry) -> ExternResult<ValidateCallbackResult> {
    let current_summary : ReviewSummaryEntry = must_get_entry( header.original_entry_address.to_owned().into() )?.try_into()?;

    if let ValidateCallbackResult::Invalid(message) = validate_review_summary_content( &review_summary )? {
	return Ok(ValidateCallbackResult::Invalid(message))
    }

    if !( review_summary.factored_action_count > current_summary.factored_action_count ) {
	return Ok(ValidateCallbackResult::Invalid(format!("The updated summary is not better than the current summary: new factored action count ({}) must be greater than {}", review_summary.factored_action_count, current_summary.factored_action_count )))
    }

    Ok(ValidateCallbackResult::Valid)
}
