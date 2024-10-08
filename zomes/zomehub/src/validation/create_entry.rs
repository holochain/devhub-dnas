use crate::{
    hdi,
    hdi_extensions,
    mere_memory_types, // from zomehub_types
    EntryTypes,

    Authority,
    ZomePackageEntry,
    validation::{
        check_authority,
    },
};

use hdi::prelude::*;
use hdi_extensions::{
    trace_origin_root,

    // Macros
    valid, invalid,
};
use mere_memory_types::{
    MemoryEntry,
};


pub fn validation(
    app_entry: EntryTypes,
    create: Create
) -> ExternResult<ValidateCallbackResult> {
    match app_entry {
        EntryTypes::Zome(zome_entry) => {
            let memory : MemoryEntry = must_get_entry( zome_entry.mere_memory_addr )?.try_into()?;
            let file_size = memory.uncompressed_size
                .unwrap_or( memory.memory_size );

            if zome_entry.file_size != file_size {
                invalid!(format!(
                    "ZomeEntry file size does not match memory address: {} != {}",
                    zome_entry.file_size, file_size
                ))
            }

            if zome_entry.hash != memory.hash {
                invalid!(format!(
                    "ZomeEntry hash does not match memory hash: {} != {}",
                    zome_entry.hash, memory.hash
                ))
            }

            valid!()
        },
        EntryTypes::ZomePackage(entry) => {
            //
            // Check if create author is authorized
            //
            if let ValidateCallbackResult::Invalid(msg) = check_authority( &entry.maintainer, &create.author )? {
                invalid!(msg)
            }

            valid!()
        },
        EntryTypes::ZomePackageVersion(entry) => {
            let zome_package : ZomePackageEntry = must_get_valid_record( entry.for_package )?.try_into()?;

            //
            // Check parent maintainer settings
            //
            match ( &zome_package.maintainer, &entry.maintainer ) {

                // Should have matching agent authority
                (
                    Authority::Agent(expected_agent),
                    Authority::Agent(agent)
                ) => {
                    if expected_agent != agent {
                        invalid!(format!(
                            "Maintainer agent must match parent package: {} != {}",
                            expected_agent, agent,
                        ))
                    }
                },

                // Should have matching group ID
                (
                    Authority::Group(expected_group_id, _),
                    Authority::Group(group_id, group_rev)
                ) => {
                    // Ensure same group ID
                    if expected_group_id != group_id {
                        invalid!(format!(
                            "Maintainer group must match parent package: {} != {}",
                            expected_group_id, group_id,
                        ))
                    }

                    // Ensure group rev is related to group ID
                    if trace_origin_root( &group_rev )?.0 != *group_id {
                        invalid!(format!(
                            "Maintainer group revision ({}) must be an evolution of group ID ({})",
                            group_rev, group_id,
                        ))
                    }
                },

                (expected_maintainer, maintainer) => {
                    invalid!(format!(
                        "Maintainer type must match parent package: {:?} != {:?}",
                        maintainer, expected_maintainer,
                    ))
                },
            }

            //
            // Check if create author is authorized
            //
            if let ValidateCallbackResult::Invalid(msg) = check_authority( &entry.maintainer, &create.author )? {
                invalid!(msg);
            }

            valid!()
        },
        // _ => invalid!(format!("Create validation not implemented for entry type: {:#?}", create.entry_type )),
    }
}
