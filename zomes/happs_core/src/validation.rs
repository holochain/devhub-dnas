use hdi::prelude::*;
use devhub_types::{
    happ_entry_types::{
	HappEntry,
	HappReleaseEntry,
    },
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
		    Ok(ValidateCallbackResult::Invalid(format!("Record with AppEntryType was expected to have a Present(entry): {:?}", store_record.record )))
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
			Ok(ValidateCallbackResult::Invalid(format!("Record with AppEntryType was expected to have a Present(entry): {:?}", store_record.record )))
		    }
		}
		else {
		    debug!("Ignoring Delete event of Action that doesn't contain EntryType::App: {:?}", original_action );
		    Ok(ValidateCallbackResult::Valid)
		}
	    }
	    else {
		debug!("Ignoring Op::StoreRecord event that doesn't contain EntryType::App: {:?}", store_record.record );
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
		EntryTypes::Happ(entry) => validate_happ_create( create, entry ),
		EntryTypes::HappRelease(entry) => validate_happ_release_create( create, entry ),
	    }
	}
	Action::Update(update) => {
	    debug!("Running update validation for: {:?}", entry_type );
	    match entry_type {
		EntryTypes::Happ(entry) => validate_happ_update( update, entry ),
		EntryTypes::HappRelease(entry) => validate_happ_release_update( update, entry ),
	    }
	},
	Action::Delete(delete) => {
	    debug!("Running delete validation for: {:?}", entry_type );
	    match entry_type {
		EntryTypes::HappRelease(_) => validate_happ_release_delete( delete ),
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
// Happ
//
fn validate_happ_create(action: &action::Create, happ: HappEntry) -> ExternResult<ValidateCallbackResult> {
    if happ.designer != action.author {
	Ok(ValidateCallbackResult::Invalid(format!("HappEntry author does not match Action author: {} != {}", happ.designer, action.author )))
    }
    else {
	Ok(ValidateCallbackResult::Valid)
    }
}

fn validate_happ_update(action: &action::Update, happ: HappEntry) -> ExternResult<ValidateCallbackResult> {
    let prev_entry : HappEntry = must_get_entry( action.original_entry_address.to_owned() )?.try_into()?;

    if prev_entry.designer != action.author {
	return Ok(ValidateCallbackResult::Invalid(format!("Previous entry author does not match Action author: {} != {}", prev_entry.designer, action.author )));
    }

    if prev_entry.deprecation.is_some() {
	return Ok(ValidateCallbackResult::Invalid("Cannot update deprecated hApp".to_string()));
    }

    if happ.designer != prev_entry.designer  {
	return Ok(ValidateCallbackResult::Invalid(format!("Cannot change hApp designer: {} => {}", prev_entry.designer, happ.designer )));
    }

    Ok(ValidateCallbackResult::Valid)
}



//
// Happ Release
//
fn validate_happ_release_create(action: &action::Create, happ_release: HappReleaseEntry) -> ExternResult<ValidateCallbackResult> {
    let happ : HappEntry = must_get_entry( happ_release.for_happ.into() )?.try_into()?;

    if happ.designer != action.author {
	Ok(ValidateCallbackResult::Invalid(format!("HappEntry author does not match Action author: {} != {}", happ.designer, action.author )))
    }
    else if happ_release.dnas.len() == 0 {
	return Ok(ValidateCallbackResult::Invalid("HappReleaseEntry DNA list cannot be empty".to_string()));
    }
    else {
	Ok(ValidateCallbackResult::Valid)
    }
}

fn validate_happ_release_update(action: &action::Update, happ_release: HappReleaseEntry) -> ExternResult<ValidateCallbackResult> {
    let happ : HappEntry = must_get_entry( happ_release.for_happ.into() )?.try_into()?;

    if happ.designer != action.author {
	return Ok(ValidateCallbackResult::Invalid(format!("HappEntry author does not match Action author: {} != {}", happ.designer, action.author )));
    }

    let prev_entry : HappReleaseEntry = must_get_entry( action.original_entry_address.to_owned() )?.try_into()?;

    if happ_release.dnas.len() != prev_entry.dnas.len() {
	return Ok(ValidateCallbackResult::Invalid("Cannot change HappReleaseEntry DNA list".to_string()));
    }
    else {
	for (i, dna_ref) in happ_release.dnas.iter().enumerate() {
	    if *dna_ref != prev_entry.dnas[i] {
		return Ok(ValidateCallbackResult::Invalid(format!("Cannot change HappReleaseEntry DNA list item {}: {:?} => {:?}", i, dna_ref, prev_entry )));
	    }
	}
    }

    Ok(ValidateCallbackResult::Valid)
}

fn validate_happ_release_delete(action: &action::Delete) -> ExternResult<ValidateCallbackResult> {
    let sh_action = must_get_action( action.deletes_address.to_owned() )?;
    let original_action = sh_action.action();

    if *original_action.author() != action.author {
	return Ok(ValidateCallbackResult::Invalid(format!("Delete author does not match Create author: {} != {}", original_action.author(), action.author )));
    }

    Ok(ValidateCallbackResult::Valid)
}
