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
        EntryTypesUnit::App |
        EntryTypesUnit::Ui |
        EntryTypesUnit::WebApp |
        EntryTypesUnit::WebAppPackage |
        EntryTypesUnit::WebAppPackageVersion => {
            if delete.author != create.author {
                invalid!(format!(
                    "Not authorized to delete entry created by author {}",
                    create.author
                ))
            }

            valid!()
        },
        // entry_type_unit => invalid!(format!("Delete validation not implemented for entry type: {:?}", entry_type_unit )),
    }
}
