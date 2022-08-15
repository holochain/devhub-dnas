use hdi::prelude::*;
use devhub_types::{
    dnarepo_entry_types::{
	// ProfileEntry,
	ZomeEntry,
	ZomeVersionEntry,
	DnaEntry,
	DnaVersionEntry,
	ReviewEntry,
	ReviewSummaryEntry,
    },
    trace_action_origin_entry,
};
use crate::{
    EntryTypes,
};



#[hdk_extern]
fn validate(op: Op) -> ExternResult<ValidateCallbackResult> {
    match op {
	Op::StoreRecord( store_record ) => {
	    if let Some(EntryType::App(AppEntryType{ id, zome_id, .. })) = store_record.record.action().entry_type() {
		if *zome_id != zome_info().unwrap().id {
		    // This Record does not belong to our Zome so we don't know how to validate it
		    return Ok(ValidateCallbackResult::Valid);
		}

		debug!("Forwarding validation for StoreRecord->Action::Create->EntryType::App to validation handler");
		if let RecordEntry::Present(entry) = store_record.record.entry() {
		    if let Some(entry_type) = EntryTypes::deserialize_from_type(*zome_id, *id, &entry )? {
			validate_record( entry_type, &store_record.record )
		    }
		    else {
			Ok(ValidateCallbackResult::Invalid(format!("No matching EntryTypes value for: {}/{}", zome_id.0, id.0 )))
		    }
		}
		else {
		    Ok(ValidateCallbackResult::Invalid(format!("Record with AppEntryType was expected to have a Present(entry): {:?}", store_record )))
		}
	    }
	    else if let Action::Delete(delete) = store_record.record.action() {
		let original_record = must_get_valid_record( delete.deletes_address.to_owned() )?;
		let original_action = original_record.signed_action.action();

		if let Some(EntryType::App(AppEntryType{ id, zome_id, .. })) = original_action.entry_type() {
		    if *zome_id != zome_info().unwrap().id {
			// This Record does not belong to our Zome so we don't know how to validate it
			return Ok(ValidateCallbackResult::Valid)
		    }

		    if let RecordEntry::Present(entry) = original_record.entry() {
			if let Some(entry_type) = EntryTypes::deserialize_from_type(*zome_id, *id, &entry )? {
			    validate_record( entry_type, &store_record.record )
			}
			else {
			    Ok(ValidateCallbackResult::Invalid(format!("No matching EntryTypes value for: {}/{}", zome_id.0, id.0 )))
			}
		    }
		    else {
			Ok(ValidateCallbackResult::Invalid(format!("Record with AppEntryType was expected to have a Present(entry): {:?}", store_record )))
		    }
		}
		else {
		    debug!("Ignoring Delete event of Action that doesn't contain EntryType::App: {:?}", original_action );
		    Ok(ValidateCallbackResult::Valid)
		}
	    }
	    else {
		debug!("Ignoring Op::StoreRecord event that doesn't contain EntryType::App: {:?}", store_record );
		Ok(ValidateCallbackResult::Valid)
	    }
	},
	_ => {
	    debug!("Ignoring Op event");
	    Ok(ValidateCallbackResult::Valid)
	},
    }
}

fn validate_record(entry_type: EntryTypes, record: &Record) -> ExternResult<ValidateCallbackResult> {
    // By the time we get here, we know it is for our Zome and the record contains an App Entry

    match record.action() {
	Action::Create(create) => {
	    debug!("Running create validation for: {:?}", entry_type );
	    match entry_type {
		EntryTypes::Zome(entry) => validate_zome_create( create, entry ),
		EntryTypes::ZomeVersion(entry) => validate_zome_version_create( create, entry ),
		EntryTypes::Dna(entry) => validate_dna_create( create, entry ),
		EntryTypes::DnaVersion(entry) => validate_dna_version_create( create, entry ),
		EntryTypes::Review(entry) => validate_review_create( create, entry ),
		EntryTypes::ReviewSummary(entry) => validate_review_summary_create( create, entry ),
		_ => {
		    debug!("Ignoring create Op for: {:?}", entry_type );
		    Ok(ValidateCallbackResult::Valid)
		}
	    }
	}
	Action::Update(update) => {
	    debug!("Running update validation for: {:?}", entry_type );
	    match entry_type {
		EntryTypes::Zome(entry) => validate_zome_update( update, entry ),
		EntryTypes::ZomeVersion(entry) => validate_zome_version_update( update, entry ),
		EntryTypes::Dna(entry) => validate_dna_update( update, entry ),
		EntryTypes::DnaVersion(entry) => validate_dna_version_update( update, entry ),
		EntryTypes::Review(entry) => validate_review_update( update, entry ),
		EntryTypes::ReviewSummary(entry) => validate_review_summary_update( update, entry ),
		_ => {
		    debug!("Ignoring update Op for: {:?}", entry_type );
		    Ok(ValidateCallbackResult::Valid)
		}
	    }
	},
	Action::Delete(delete) => {
	    debug!("Running delete validation for: {:?}", entry_type );
	    match entry_type {
		EntryTypes::ZomeVersion(_) => validate_zome_version_delete( delete ),
		EntryTypes::DnaVersion(_) => validate_dna_version_delete( delete ),
		EntryTypes::Review(_) => validate_review_delete( delete ),
		EntryTypes::ReviewSummary(_) => Ok(ValidateCallbackResult::Invalid("Review summaries cannot be deleted".to_string())),
		_ => {
		    debug!("Ignoring delete Op for: {:?}", entry_type );
		    Ok(ValidateCallbackResult::Valid)
		}
	    }
	},
	_ => {
	    debug!("Nothing implemented for Action type");
	    Ok(ValidateCallbackResult::Invalid(format!("Unknown entry type: {:?}", entry_type )))
	},
    }
}



//
// Zome
//
fn validate_zome_create(action: &action::Create, zome: ZomeEntry) -> ExternResult<ValidateCallbackResult> {
    if zome.developer != action.author {
	Ok(ValidateCallbackResult::Invalid(format!("ZomeEntry author does not match Action author: {} != {}", zome.developer, action.author )))
    }
    else {
	Ok(ValidateCallbackResult::Valid)
    }
}

fn validate_zome_update(action: &action::Update, zome: ZomeEntry) -> ExternResult<ValidateCallbackResult> {
    let prev_entry : ZomeEntry = must_get_entry( action.original_entry_address.to_owned() )?.try_into()?;

    if prev_entry.developer != action.author {
	return Ok(ValidateCallbackResult::Invalid(format!("Previous entry author does not match Action author: {} != {}", prev_entry.developer, action.author )));
    }

    if prev_entry.deprecation.is_some() {
	return Ok(ValidateCallbackResult::Invalid("Cannot update deprecated Zome".to_string()));
    }

    if zome.developer != prev_entry.developer  {
	return Ok(ValidateCallbackResult::Invalid(format!("Cannot change zome developer: {} => {}", prev_entry.developer, zome.developer )));
    }

    if prev_entry.developer != action.author {
	return Ok(ValidateCallbackResult::Invalid(format!("Previous entry author does not match Action author: {} != {}", prev_entry.developer, action.author )));
    }

    Ok(ValidateCallbackResult::Valid)
}



//
// Zome Version
//
fn validate_zome_version_create(action: &action::Create, zome_version: ZomeVersionEntry) -> ExternResult<ValidateCallbackResult> {
    let zome : ZomeEntry = must_get_entry( zome_version.for_zome.into() )?.try_into()?;

    if zome.developer != action.author {
	Ok(ValidateCallbackResult::Invalid(format!("ZomeEntry author does not match Action author: {} != {}", zome.developer, action.author )))
    }
    else {
	Ok(ValidateCallbackResult::Valid)
    }
}

fn validate_zome_version_update(action: &action::Update, zome_version: ZomeVersionEntry) -> ExternResult<ValidateCallbackResult> {
    let zome : ZomeEntry = must_get_entry( zome_version.for_zome.to_owned().into() )?.try_into()?;
    let prev_entry : ZomeVersionEntry = must_get_entry( action.original_entry_address.to_owned() )?.try_into()?;

    if zome_version.review_summary.is_some() {
	let mut prev_zome_copy = prev_entry.clone();
	prev_zome_copy.review_summary = zome_version.review_summary.clone();

	if prev_zome_copy == zome_version {
	    return Ok(ValidateCallbackResult::Valid);
	}
    }

    if zome.developer != action.author {
	return Ok(ValidateCallbackResult::Invalid(format!("ZomeEntry author does not match Action author: {} != {}", zome.developer, action.author )));
    }

    if zome_version.version != prev_entry.version {
	return Ok(ValidateCallbackResult::Invalid(format!("Cannot change ZomeVersionEntry version #: {} => {}", prev_entry.version, zome_version.version )));
    }

    if prev_entry.review_summary.is_some() && prev_entry.review_summary != zome_version.review_summary {
	return Ok(ValidateCallbackResult::Invalid(format!("Cannot change ZomeVersionEntry review summary ID once it is set: {:?} => {:?}", prev_entry.review_summary, zome_version.review_summary )));
    }

    if zome_version.mere_memory_addr != prev_entry.mere_memory_addr || zome_version.mere_memory_hash != prev_entry.mere_memory_hash {
	return Ok(ValidateCallbackResult::Invalid("Cannot change ZomeVersionEntry mere memory values".to_string()));
    }

    Ok(ValidateCallbackResult::Valid)
}

fn validate_zome_version_delete(action: &action::Delete) -> ExternResult<ValidateCallbackResult> {
    let sh_action = must_get_action( action.deletes_address.to_owned() )?;
    let original_action = sh_action.action();

    if *original_action.author() != action.author {
	return Ok(ValidateCallbackResult::Invalid(format!("Delete author does not match Create author: {} != {}", original_action.author(), action.author )));
    }

    Ok(ValidateCallbackResult::Valid)
}



//
// Dna
//
fn validate_dna_create(action: &action::Create, dna: DnaEntry) -> ExternResult<ValidateCallbackResult> {
    if dna.developer != action.author {
	Ok(ValidateCallbackResult::Invalid(format!("DnaEntry author does not match Action author: {} != {}", dna.developer, action.author )))
    }
    else {
	Ok(ValidateCallbackResult::Valid)
    }
}

fn validate_dna_update(action: &action::Update, dna: DnaEntry) -> ExternResult<ValidateCallbackResult> {
    let prev_entry : DnaEntry = must_get_entry( action.original_entry_address.to_owned() )?.try_into()?;

    if prev_entry.developer != action.author {
	return Ok(ValidateCallbackResult::Invalid(format!("Previous entry author does not match Action author: {} != {}", prev_entry.developer, action.author )));
    }

    if prev_entry.deprecation.is_some() {
	return Ok(ValidateCallbackResult::Invalid("Cannot update deprecated DNA".to_string()));
    }

    if dna.developer != prev_entry.developer  {
	return Ok(ValidateCallbackResult::Invalid(format!("Cannot change dna developer: {} => {}", prev_entry.developer, dna.developer )));
    }

    Ok(ValidateCallbackResult::Valid)
}



//
// Dna Version
//
fn validate_dna_version_create(action: &action::Create, dna_version: DnaVersionEntry) -> ExternResult<ValidateCallbackResult> {
    let dna : DnaEntry = must_get_entry( dna_version.for_dna.into() )?.try_into()?;

    if dna.developer != action.author {
	Ok(ValidateCallbackResult::Invalid(format!("DnaEntry author does not match Action author: {} != {}", dna.developer, action.author )))
    }
    else if dna_version.integrity_zomes.len() == 0 {
	return Ok(ValidateCallbackResult::Invalid("DnaVersionEntry Zomes list cannot be empty".to_string()));
    }
    else {
	Ok(ValidateCallbackResult::Valid)
    }
}

fn validate_dna_version_update(action: &action::Update, dna_version: DnaVersionEntry) -> ExternResult<ValidateCallbackResult> {
    let dna : DnaEntry = must_get_entry( dna_version.for_dna.into() )?.try_into()?;

    if dna.developer != action.author {
	return Ok(ValidateCallbackResult::Invalid(format!("DnaEntry author does not match Action author: {} != {}", dna.developer, action.author )));
    }

    let prev_entry : DnaVersionEntry = must_get_entry( action.original_entry_address.to_owned() )?.try_into()?;

    if dna_version.version != prev_entry.version {
	return Ok(ValidateCallbackResult::Invalid(format!("Cannot change DnaVersionEntry version #: {} => {}", prev_entry.version, dna_version.version )));
    }

    if dna_version.zomes.len() != prev_entry.zomes.len() || dna_version.integrity_zomes.len() != prev_entry.integrity_zomes.len() {
	return Ok(ValidateCallbackResult::Invalid("Cannot change DnaVersionEntry zome list".to_string()));
    }
    else {
	for (i, zome_ref) in dna_version.integrity_zomes.iter().enumerate() {
	    if *zome_ref != prev_entry.integrity_zomes[i] {
		return Ok(ValidateCallbackResult::Invalid(format!("Cannot change DnaVersionEntry integrity zome list item {}: {:?} => {:?}", i, zome_ref, prev_entry )));
	    }
	}

	for (i, zome_ref) in dna_version.zomes.iter().enumerate() {
	    if *zome_ref != prev_entry.zomes[i] {
		return Ok(ValidateCallbackResult::Invalid(format!("Cannot change DnaVersionEntry zome list item {}: {:?} => {:?}", i, zome_ref, prev_entry )));
	    }
	}
    }

    if dna_version.properties != prev_entry.properties {
	return Ok(ValidateCallbackResult::Invalid("Cannot change DnaVersionEntry properties".to_string()));
    }

    Ok(ValidateCallbackResult::Valid)
}

fn validate_dna_version_delete(action: &action::Delete) -> ExternResult<ValidateCallbackResult> {
    let sh_action = must_get_action( action.deletes_address.to_owned() )?;
    let original_action = sh_action.action();

    if *original_action.author() != action.author {
	return Ok(ValidateCallbackResult::Invalid(format!("Delete author does not match Create author: {} != {}", original_action.author(), action.author )));
    }

    Ok(ValidateCallbackResult::Valid)
}


//
// Review
//
fn validate_review_create(action: &action::Create, review: ReviewEntry) -> ExternResult<ValidateCallbackResult> {
    if review.author != action.author {
	return Ok(ValidateCallbackResult::Invalid(format!("ReviewEntry author does not match Action author: {} != {}", review.author, action.author )))
    }

    for (key, rating) in review.ratings {
	if rating > 10 {
	    return Ok(ValidateCallbackResult::Invalid(format!("ReviewEntry {} rating ({}) out of range: valid range 0-10", key, rating )))
	}
    }

    Ok(ValidateCallbackResult::Valid)
}

fn validate_review_update(action: &action::Update, review: ReviewEntry) -> ExternResult<ValidateCallbackResult> {
    let prev_entry : ReviewEntry = must_get_entry( action.original_entry_address.to_owned() )?.try_into()?;

    if review.reaction_summary.is_some() {
	let mut prev_review_copy = prev_entry.clone();
	prev_review_copy.reaction_summary = review.reaction_summary.clone();

	if prev_review_copy == review {
	    return Ok(ValidateCallbackResult::Valid);
	}
    }

    if prev_entry.author != action.author {
	return Ok(ValidateCallbackResult::Invalid(format!("Previous entry author does not match Action author: {} != {}", prev_entry.author, action.author )));
    }

    if review.author != prev_entry.author  {
	return Ok(ValidateCallbackResult::Invalid(format!("Cannot change review author: {} => {}", prev_entry.author, review.author )));
    }

    Ok(ValidateCallbackResult::Valid)
}

fn validate_review_delete(action: &action::Delete) -> ExternResult<ValidateCallbackResult> {
    let prev_entry : ReviewEntry = must_get_entry( action.deletes_entry_address.to_owned() )?.try_into()?;

    if prev_entry.author != action.author {
	Ok(ValidateCallbackResult::Invalid(format!("Deleted entry author does not match Action author: {} != {}", prev_entry.author, action.author )))
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
    let mut all_factored_reviews : Vec<(EntryHash,ActionHash)> = Vec::new();

    all_factored_reviews.extend( review_summary.review_refs.values().map( |values| (values.0.to_owned(), values.1.to_owned()) ).collect::<Vec<(EntryHash,ActionHash)>>() );
    // all_factored_reviews.extend( review_summary.deleted_reviews.values().cloned().collect::<Vec<(EntryHash,ActionHash)>>() );
    all_factored_reviews.extend( review_summary.deleted_reviews.values().map( |values| (values.0.to_owned(), values.1.to_owned()) ).collect::<Vec<(EntryHash,ActionHash)>>() );

    // Verfiy review references
    for (review_id, review_action_hash) in all_factored_reviews {
	let review_record = must_get_valid_record( review_action_hash.to_owned().into() )?;

	if let Action::Update(update) = review_record.action() {
	    let (origin_id, depth) = trace_action_origin_entry( &update.original_action_address, Some(1) )?;

	    if origin_id != review_id {
		return Ok(ValidateCallbackResult::Invalid(format!("Traced origin ID for action ({}) does not match review ID: {} != {}", review_action_hash, origin_id, review_id )))
	    }

	    debug!("Counting depth {} for {}", depth, origin_id );
	    factored_count += depth;
	}

	if let Action::Create(create) = review_record.action() {
	    if create.entry_hash != review_id {
		return Ok(ValidateCallbackResult::Invalid(format!("Action is not related to review ID: {} != {}", create.entry_hash, review_id )))
	    }
	}

	match review_record.entry() {
	    RecordEntry::Present(entry) => {
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
		return Ok(ValidateCallbackResult::Invalid(format!("Expected action {} to have an app entry, not {:?}", review_action_hash, entry )))
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

fn validate_review_summary_create(_: &action::Create, review_summary: ReviewSummaryEntry) -> ExternResult<ValidateCallbackResult> {
    Ok( validate_review_summary_content( &review_summary )? )
}

fn validate_review_summary_update(action: &action::Update, review_summary: ReviewSummaryEntry) -> ExternResult<ValidateCallbackResult> {
    let current_summary : ReviewSummaryEntry = must_get_entry( action.original_entry_address.to_owned().into() )?.try_into()?;

    if let ValidateCallbackResult::Invalid(message) = validate_review_summary_content( &review_summary )? {
	return Ok(ValidateCallbackResult::Invalid(message))
    }

    if !( review_summary.factored_action_count > current_summary.factored_action_count ) {
	return Ok(ValidateCallbackResult::Invalid(format!("The updated summary is not better than the current summary: new factored action count ({}) must be greater than {}", review_summary.factored_action_count, current_summary.factored_action_count )))
    }

    Ok(ValidateCallbackResult::Valid)
}
