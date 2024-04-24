use crate::{
    hdi,
    hdi_extensions,
    EntryTypes,
};

use hdi::prelude::*;
use hdi_extensions::{
    // Macros
    invalid,
};


pub fn validation(
    app_entry: EntryTypes,
    _update: Update,
    _original_action_hash: ActionHash,
    _original_entry_hash: EntryHash
) -> ExternResult<ValidateCallbackResult> {
    match app_entry {
        EntryTypes::Zome(_) => {
            invalid!(format!("ZomeEntry are not intended to be updated"))
        },
        // _ => invalid!(format!("Update validation not implemented for entry type: {:#?}", update.entry_type )),
    }
}
