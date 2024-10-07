use crate::{
    hdi,
    hdi_extensions,
    EntryTypes,

    Authority,
    ZomePackageEntry,
    ZomePackageVersionEntry,
};

use hdi::prelude::*;
use hdi_extensions::{
    trace_origin_root,
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
            // Check if update author is authorized
            //
            match previous_entry.maintainer {
                Authority::Agent(agent_pubkey) => {
                    if update.author != agent_pubkey {
                        invalid!(format!(
                            "{} is not the maintainer",
                            update.author,
                        ))
                    }
                },
                Authority::Group(prev_group_id, prev_group_ref) => {
                    let group : GroupEntry = summon_app_entry( &prev_group_ref.into() )?;

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
                },
            }

            // If authority is group, check author is in the group
            if let Authority::Group(group_id, group_ref) = entry.maintainer.clone() {
                let group : GroupEntry = summon_app_entry( &group_ref.clone().into() )?;

                if !group.is_contributor( &update.author ) {
                    invalid!(format!(
                        "{} is not authorized in group {}",
                        update.author, group_id,
                    ))
                }

                // Check that group_ref belongs to group_id
                if group_id != trace_origin_root( &group_ref )?.0 {
                    invalid!(format!(
                        "Group ref {} is not a descendant of group ID {}",
                        group_ref, group_id,
                    ))
                }
            }

            valid!()
        },
        EntryTypes::ZomePackageVersion(entry) => {
            let previous_entry : ZomePackageVersionEntry = must_get_entry( original_entry_hash )?
                .try_into()?;

            //
            // Check if update author is authorized
            //
            match previous_entry.maintainer {
                Authority::Agent(agent_pubkey) => {
                    if update.author != agent_pubkey {
                        invalid!(format!(
                            "{} is not the maintainer",
                            update.author,
                        ))
                    }
                },
                Authority::Group(prev_group_id, prev_group_ref) => {
                    let group : GroupEntry = summon_app_entry( &prev_group_ref.into() )?;

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
                },
            }

            // If authority is group, check author is in the group
            if let Authority::Group(group_id, group_ref) = entry.maintainer.clone() {
                let group : GroupEntry = summon_app_entry( &group_ref.clone().into() )?;

                if !group.is_contributor( &update.author ) {
                    invalid!(format!(
                        "{} is not authorized in group {}",
                        update.author, group_id,
                    ))
                }

                // Check that group_ref belongs to group_id
                if group_id != trace_origin_root( &group_ref )?.0 {
                    invalid!(format!(
                        "Group ref {} is not a descendant of group ID {}",
                        group_ref, group_id,
                    ))
                }
            }

            valid!()
        },
        // _ => invalid!(format!("Update validation not implemented for entry type: {:#?}", update.entry_type )),
    }
}
