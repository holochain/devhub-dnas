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

    for (key, rating) in review.ratings {
	if rating > 10 {
	    return Ok(ValidateCallbackResult::Invalid(format!("ReviewEntry {} rating ({}) out of range: valid range 0-10", key, rating )))
	}
    }

    Ok(ValidateCallbackResult::Valid)
}

fn validate_review_update(header: &header::Update, review: ReviewEntry) -> ExternResult<ValidateCallbackResult> {
    let prev_entry : ReviewEntry = must_get_entry( header.original_entry_address.to_owned() )?.try_into()?;

    if review.reaction_summary.is_some() {
	let mut prev_review_copy = prev_entry.clone();
	prev_review_copy.reaction_summary = review.reaction_summary.clone();

	if prev_review_copy == review {
	    return Ok(ValidateCallbackResult::Valid);
	}
    }

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
    let mut factored_count = (review_summary.review_refs.len() + review_summary.deleted_reviews.len()) as u64;
    let mut all_factored_reviews : Vec<(EntryHash,HeaderHash)> = Vec::new();

    all_factored_reviews.extend( review_summary.review_refs.values().map( |values| (values.0.to_owned(), values.1.to_owned()) ).collect::<Vec<(EntryHash,HeaderHash)>>() );
    // all_factored_reviews.extend( review_summary.deleted_reviews.values().cloned().collect::<Vec<(EntryHash,HeaderHash)>>() );
    all_factored_reviews.extend( review_summary.deleted_reviews.values().map( |values| (values.0.to_owned(), values.1.to_owned()) ).collect::<Vec<(EntryHash,HeaderHash)>>() );

    // Verfiy review references
    for (review_id, review_header_hash) in all_factored_reviews {
	let review_element = must_get_valid_element( review_header_hash.to_owned().into() )?;

	if let Header::Update(update) = review_element.header() {
	    let (origin_id, depth) = trace_header_origin_entry( &update.original_header_address, Some(1) )?;

	    if origin_id != review_id {
		return Ok(ValidateCallbackResult::Invalid(format!("Traced origin ID for header ({}) does not match review ID: {} != {}", review_header_hash, origin_id, review_id )))
	    }

	    debug!("Counting depth {} for {}", depth, origin_id );
	    factored_count += depth;
	}

	if let Header::Create(create) = review_element.header() {
	    if create.entry_hash != review_id {
		return Ok(ValidateCallbackResult::Invalid(format!("Header is not related to review ID: {} != {}", create.entry_hash, review_id )))
	    }
	}

	match review_element.entry() {
	    ElementEntry::Present(entry) => {
		let review : ReviewEntry = entry.try_into()?;

		if review.subject_ids.iter().find( |pair| pair.0 == review_summary.subject_id ).is_none() {
		    return Ok(ValidateCallbackResult::Invalid(format!("Contains review that does not belong to subject: {}", review_id )))
		}

		if review.deleted {
		    debug!("Checking for deleted reviews: {:?}", review_summary.review_refs.keys() );
		    if review_summary.review_refs.contains_key(&format!("{}", review_id )) {
			return Ok(ValidateCallbackResult::Invalid(format!("Deleted review {} cannot be used in review refs", review_id )))
		    }
		    continue;
		}
	    },
	    entry => {
		return Ok(ValidateCallbackResult::Invalid(format!("Expected header {} to have an app entry, not {:?}", review_header_hash, entry )))
	    },
	}
    }

    // Add reaction counters
    for (_, _, _, _, _, maybe_reaction_summary) in review_summary.review_refs.values() {
	if let Some((_, reaction_count,_)) = maybe_reaction_summary {
	    factored_count += reaction_count;
	}
    }

    // Add reaction counters from deleted
    for (_, _, _, maybe_reaction_summary) in review_summary.deleted_reviews.values() {
	if let Some((_, reaction_count, _)) = maybe_reaction_summary {
	    factored_count += reaction_count;
	}
    }

    // Check factored review count
    if review_summary.factored_action_count != factored_count {
	return Ok(ValidateCallbackResult::Invalid(format!("ReviewSummaryEntry's factored review count does not equal the number of indirect review references: {} != {}", review_summary.factored_action_count, factored_count )))
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
