use hdk::prelude::*;

use devhub_types::{
    happ_entry_types::{
	HappEntry,
	HappReleaseEntry,
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

    let happ_edi : EntryDefIndex = entry_def_index!(HappEntry).unwrap();
    let happ_release_edi : EntryDefIndex = entry_def_index!(HappReleaseEntry).unwrap();

    match element.header() {
	Header::Create(create) => {
	    debug!("Running create validation for: {:?}", entry_def_index );
	    if entry_def_index == happ_edi {
		validate_happ_create( create, element.try_into()? )
	    }
	    else if entry_def_index == happ_release_edi {
		validate_happ_release_create( create, element.try_into()? )
	    }
	    else {
		debug!("Ignoring create Op for: {:?}", entry_def_index );
		Ok(ValidateCallbackResult::Valid)
	    }
	}
	Header::Update(update) => {
	    debug!("Running update validation for: {:?}", entry_def_index );
	    if entry_def_index == happ_edi {
		validate_happ_update( update, element.try_into()? )
	    }
	    else if entry_def_index == happ_release_edi {
		validate_happ_release_update( update, element.try_into()? )
	    }
	    else {
		debug!("Ignoring update Op for: {:?}", entry_def_index );
		Ok(ValidateCallbackResult::Valid)
	    }
	},
	Header::Delete(delete) => {
	    debug!("Running delete validation for: {:?}", entry_def_index );
	    if entry_def_index == happ_release_edi {
		validate_happ_release_delete( delete )
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
