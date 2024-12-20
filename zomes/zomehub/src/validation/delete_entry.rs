use crate::{
    hdi,
    hdi_extensions,
    EntryTypesUnit,
};

use hdi::prelude::*;
use hdi_extensions::{
    summon_create_action,
    detect_app_entry_unit,
    // Macros
    valid, invalid,
};


pub fn validation(
    original_action_hash: ActionHash,
    _original_entry_hash: EntryHash,
    delete: Delete
) -> ExternResult<ValidateCallbackResult> {
    let create = summon_create_action( &original_action_hash )?;

    match detect_app_entry_unit( &create )? {
        EntryTypesUnit::Zome => {
            if delete.author != create.author {
                invalid!(format!(
                    "Not authorized to delete entry created by author {}",
                    create.author
                ))
            }

            valid!()
        },
        EntryTypesUnit::ZomePackage => {
            valid!()
        },
        EntryTypesUnit::ZomePackageVersion => {
            valid!()
        },
        // entry_type_unit => invalid!(format!("Delete validation not implemented for entry type: {:?}", entry_type_unit )),
    }
}
