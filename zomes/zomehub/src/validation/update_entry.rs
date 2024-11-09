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
    // Macros
    valid, invalid,
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
            match ( &previous_entry.maintainer, &entry.maintainer ) {
                (
                    Authority::Group(prev_group_id, _),
                    Authority::Group(group_id, _),
                ) => {
                    if prev_group_id != group_id {
                        invalid!(format!(
                            "The maintainer group cannot be changed: {} => {}",
                            prev_group_id, group_id,
                        ))
                    }
                },
                (
                    Authority::Agent(prev_agent_pubkey),
                    Authority::Agent(agent_pubkey),
                ) => {
                    if prev_agent_pubkey != agent_pubkey {
                        invalid!(format!(
                            "The maintainer agent cannot be changed: {} => {}",
                            prev_agent_pubkey, agent_pubkey,
                        ))
                    }
                },
                (expected_maintainer, maintainer) => {
                    invalid!(format!(
                        "Maintainer type cannot be changed: {:?} => {:?}",
                        expected_maintainer, maintainer,
                    ))
                },
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
            match ( &previous_entry.maintainer, &entry.maintainer ) {
                (
                    Authority::Group(prev_group_id, _),
                    Authority::Group(group_id, _),
                ) => {
                    if prev_group_id != group_id {
                        invalid!(format!(
                            "The maintainer group cannot be changed: {} => {}",
                            prev_group_id, group_id,
                        ))
                    }
                },
                (
                    Authority::Agent(prev_agent_pubkey),
                    Authority::Agent(agent_pubkey),
                ) => {
                    if prev_agent_pubkey != agent_pubkey {
                        invalid!(format!(
                            "The maintainer agent cannot be changed: {} => {}",
                            prev_agent_pubkey, agent_pubkey,
                        ))
                    }
                },
                (expected_maintainer, maintainer) => {
                    invalid!(format!(
                        "Maintainer type cannot be changed: {:?} => {:?}",
                        expected_maintainer, maintainer,
                    ))
                },
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
