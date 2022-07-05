use hdk::prelude::*;

use devhub_types::{
    dnarepo_entry_types::{
	// ProfileEntry,
	ZomeEntry,
	ZomeVersionEntry,
	DnaEntry,
	DnaVersionEntry,
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

    // let path_edi : EntryDefIndex = entry_def_index!(PathEntry).unwrap();
    // let profile_edi : EntryDefIndex = entry_def_index!(ProfileEntry).unwrap();
    let zome_edi : EntryDefIndex = entry_def_index!(ZomeEntry).unwrap();
    let zome_version_edi : EntryDefIndex = entry_def_index!(ZomeVersionEntry).unwrap();
    let dna_edi : EntryDefIndex = entry_def_index!(DnaEntry).unwrap();
    let dna_version_edi : EntryDefIndex = entry_def_index!(DnaVersionEntry).unwrap();

    match element.header() {
	Header::Create(create) => {
	    debug!("Running create validation for: {:?}", entry_def_index );
	    if entry_def_index == zome_edi {
		validate_zome_create( create, element.try_into()? )
	    }
	    else if entry_def_index == zome_version_edi {
		validate_zome_version_create( create, element.try_into()? )
	    }
	    else if entry_def_index == dna_edi {
		validate_dna_create( create, element.try_into()? )
	    }
	    else if entry_def_index == dna_version_edi {
		validate_dna_version_create( create, element.try_into()? )
	    }
	    else {
		debug!("Ignoring create Op for: {:?}", entry_def_index );
		Ok(ValidateCallbackResult::Valid)
	    }
	}
	Header::Update(update) => {
	    debug!("Running update validation for: {:?}", entry_def_index );
	    if entry_def_index == zome_edi {
		validate_zome_update( update, element.try_into()? )
	    }
	    else if entry_def_index == zome_version_edi {
		validate_zome_version_update( update, element.try_into()? )
	    }
	    else if entry_def_index == dna_edi {
		validate_dna_update( update, element.try_into()? )
	    }
	    else if entry_def_index == dna_version_edi {
		validate_dna_version_update( update, element.try_into()? )
	    }
	    else {
		debug!("Ignoring update Op for: {:?}", entry_def_index );
		Ok(ValidateCallbackResult::Valid)
	    }
	},
	Header::Delete(delete) => {
	    debug!("Running delete validation for: {:?}", entry_def_index );
	    if entry_def_index == zome_version_edi {
		validate_zome_version_delete( delete )
	    }
	    else if entry_def_index == dna_version_edi {
		validate_dna_version_delete( delete )
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
// Zome
//
fn validate_zome_create(header: &header::Create, zome: ZomeEntry) -> ExternResult<ValidateCallbackResult> {
    if zome.developer != header.author {
	Ok(ValidateCallbackResult::Invalid(format!("ZomeEntry author does not match Header author: {} != {}", zome.developer, header.author )))
    }
    else {
	Ok(ValidateCallbackResult::Valid)
    }
}

fn validate_zome_update(header: &header::Update, zome: ZomeEntry) -> ExternResult<ValidateCallbackResult> {
    let prev_entry : ZomeEntry = must_get_entry( header.original_entry_address.to_owned() )?.try_into()?;

    if prev_entry.developer != header.author {
	return Ok(ValidateCallbackResult::Invalid(format!("Previous entry author does not match Header author: {} != {}", prev_entry.developer, header.author )));
    }

    if prev_entry.deprecation.is_some() {
	return Ok(ValidateCallbackResult::Invalid("Cannot update deprecated Zome".to_string()));
    }

    if zome.developer != prev_entry.developer  {
	return Ok(ValidateCallbackResult::Invalid(format!("Cannot change zome developer: {} => {}", prev_entry.developer, zome.developer )));
    }

    if prev_entry.developer != header.author {
	return Ok(ValidateCallbackResult::Invalid(format!("Previous entry author does not match Header author: {} != {}", prev_entry.developer, header.author )));
    }

    Ok(ValidateCallbackResult::Valid)
}



//
// Zome Version
//
fn validate_zome_version_create(header: &header::Create, zome_version: ZomeVersionEntry) -> ExternResult<ValidateCallbackResult> {
    let zome : ZomeEntry = must_get_entry( zome_version.for_zome.into() )?.try_into()?;

    if zome.developer != header.author {
	Ok(ValidateCallbackResult::Invalid(format!("ZomeEntry author does not match Header author: {} != {}", zome.developer, header.author )))
    }
    else {
	Ok(ValidateCallbackResult::Valid)
    }
}

fn validate_zome_version_update(header: &header::Update, zome_version: ZomeVersionEntry) -> ExternResult<ValidateCallbackResult> {
    let zome : ZomeEntry = must_get_entry( zome_version.for_zome.into() )?.try_into()?;

    if zome.developer != header.author {
	return Ok(ValidateCallbackResult::Invalid(format!("ZomeEntry author does not match Header author: {} != {}", zome.developer, header.author )));
    }

    let prev_entry : ZomeVersionEntry = must_get_entry( header.original_entry_address.to_owned() )?.try_into()?;

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

fn validate_zome_version_delete(header: &header::Delete) -> ExternResult<ValidateCallbackResult> {
    let sh_header = must_get_header( header.deletes_address.to_owned() )?;
    let original_header = sh_header.header();

    if *original_header.author() != header.author {
	return Ok(ValidateCallbackResult::Invalid(format!("Delete author does not match Create author: {} != {}", original_header.author(), header.author )));
    }

    Ok(ValidateCallbackResult::Valid)
}



//
// Dna
//
fn validate_dna_create(header: &header::Create, dna: DnaEntry) -> ExternResult<ValidateCallbackResult> {
    if dna.developer != header.author {
	Ok(ValidateCallbackResult::Invalid(format!("DnaEntry author does not match Header author: {} != {}", dna.developer, header.author )))
    }
    else {
	Ok(ValidateCallbackResult::Valid)
    }
}

fn validate_dna_update(header: &header::Update, dna: DnaEntry) -> ExternResult<ValidateCallbackResult> {
    let prev_entry : DnaEntry = must_get_entry( header.original_entry_address.to_owned() )?.try_into()?;

    if prev_entry.developer != header.author {
	return Ok(ValidateCallbackResult::Invalid(format!("Previous entry author does not match Header author: {} != {}", prev_entry.developer, header.author )));
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
fn validate_dna_version_create(header: &header::Create, dna_version: DnaVersionEntry) -> ExternResult<ValidateCallbackResult> {
    let dna : DnaEntry = must_get_entry( dna_version.for_dna.into() )?.try_into()?;

    if dna.developer != header.author {
	Ok(ValidateCallbackResult::Invalid(format!("DnaEntry author does not match Header author: {} != {}", dna.developer, header.author )))
    }
    else if dna_version.zomes.len() == 0 {
	return Ok(ValidateCallbackResult::Invalid("DnaVersionEntry Zomes list cannot be empty".to_string()));
    }
    else {
	Ok(ValidateCallbackResult::Valid)
    }
}

fn validate_dna_version_update(header: &header::Update, dna_version: DnaVersionEntry) -> ExternResult<ValidateCallbackResult> {
    let dna : DnaEntry = must_get_entry( dna_version.for_dna.into() )?.try_into()?;

    if dna.developer != header.author {
	return Ok(ValidateCallbackResult::Invalid(format!("DnaEntry author does not match Header author: {} != {}", dna.developer, header.author )));
    }

    let prev_entry : DnaVersionEntry = must_get_entry( header.original_entry_address.to_owned() )?.try_into()?;

    if dna_version.version != prev_entry.version {
	return Ok(ValidateCallbackResult::Invalid(format!("Cannot change DnaVersionEntry version #: {} => {}", prev_entry.version, dna_version.version )));
    }

    if dna_version.zomes.len() != prev_entry.zomes.len() {
	return Ok(ValidateCallbackResult::Invalid("Cannot change DnaVersionEntry zome list".to_string()));
    }
    else {
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

fn validate_dna_version_delete(header: &header::Delete) -> ExternResult<ValidateCallbackResult> {
    let sh_header = must_get_header( header.deletes_address.to_owned() )?;
    let original_header = sh_header.header();

    if *original_header.author() != header.author {
	return Ok(ValidateCallbackResult::Invalid(format!("Delete author does not match Create author: {} != {}", original_header.author(), header.author )));
    }

    Ok(ValidateCallbackResult::Valid)
}
