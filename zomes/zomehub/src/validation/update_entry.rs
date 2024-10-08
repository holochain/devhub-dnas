use crate::{
    hdi,
    hdi_extensions,
    EntryTypes,

    Authority,
    ZomePackageEntry,
    ZomePackageVersionEntry,
    validation::{
        check_authority,
    },
};

use hdi::prelude::*;
use hdi_extensions::{
    summon_app_entry,

    // Macros
    valid, invalid,
};

use coop_content_types::{
    GroupEntry,
};


pub fn validation(
    app_entry: EntryTypes,
    update: Update,
    _original_action_hash: ActionHash,
    original_entry_hash: EntryHash
) -> ExternResult<ValidateCallbackResult> {
    match app_entry {
        EntryTypes::Zome(_) => {
            invalid!(format!("ZomeEntry are not intended to be updated"))
        },
        EntryTypes::ZomePackage(entry) => {
            let previous_entry : ZomePackageEntry = must_get_entry( original_entry_hash )?
                .try_into()?;

            //
            // Check if new maintainer is valid
            //
            if let Authority::Group(prev_group_id, prev_group_rev) = previous_entry.maintainer {
                let group : GroupEntry = summon_app_entry( &prev_group_rev.into() )?;

                // Check if group ID changed
                if let Authority::Group(group_id, _) = entry.maintainer.clone() {
                    if prev_group_id != group_id && !group.is_admin( &update.author ) {
                        invalid!(format!(
                            "Admin authority is required to change the maintainer group",
                        ))
                    }
                }
                // Authority changed from Group to Agent
                else {
                    if !group.is_admin( &update.author ) {
                        invalid!(format!(
                            "Admin authority is required to change the maintainer group",
                        ))
                    }
                }
            }

            //
            // Check if update author is authorized
            //
            if let ValidateCallbackResult::Invalid(msg) = check_authority( &entry.maintainer, &update.author )? {
                invalid!(msg)
            }

            valid!()
        },
        EntryTypes::ZomePackageVersion(entry) => {
            let previous_entry : ZomePackageVersionEntry = must_get_entry( original_entry_hash )?
                .try_into()?;

            //
            // Check if new maintainer is valid
            //
            if let Authority::Group(prev_group_id, prev_group_rev) = previous_entry.maintainer {
                let group : GroupEntry = summon_app_entry( &prev_group_rev.into() )?;

                // Check if group ID changed
                if let Authority::Group(group_id, _) = entry.maintainer.clone() {
                    if prev_group_id != group_id && !group.is_admin( &update.author ) {
                        invalid!(format!(
                            "Admin authority is required to change the maintainer group",
                        ))
                    }
                }
                // Authority changed from Group to Agent
                else {
                    if !group.is_admin( &update.author ) {
                        invalid!(format!(
                            "Admin authority is required to change the maintainer group",
                        ))
                    }
                }
            }

            //
            // Check if update author is authorized
            //
            if let ValidateCallbackResult::Invalid(msg) = check_authority( &entry.maintainer, &update.author )? {
                invalid!(msg)
            }

            valid!()
        },
        // _ => invalid!(format!("Update validation not implemented for entry type: {:#?}", update.entry_type )),
    }
}
