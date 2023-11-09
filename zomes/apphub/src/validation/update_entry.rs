use crate::{
    hdi,
    hdi_extensions,
    EntryTypes,
    Authority,
    WebAppPackageEntry,
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
        EntryTypes::App(_) => {
            invalid!(format!("AppEntry are not intended to be updated"))
        },
        EntryTypes::Ui(_) => {
            invalid!(format!("UiEntry are not intended to be updated"))
        },
        EntryTypes::WebApp(_) => {
            invalid!(format!("WebAppEntry are not intended to be updated"))
        },
        EntryTypes::WebAppPackage(webapp_package_entry) => {
            // Check that the update is made by a maintainer
            match webapp_package_entry.maintainer {
                Authority::Agent(agent_id) => {
                    if agent_id != update.author {
                        invalid!(format!(
                            "Not authorized to update entry; Only maintainer ({}) can make updates",
                            agent_id,
                        ))
                    }
                },
            }

            if webapp_package_entry.deprecation.is_some() {
                let previous_entry : WebAppPackageEntry = must_get_entry( original_entry_hash )?
                    .try_into()?;

                if previous_entry.deprecation.is_some() {
                    invalid!(format!(
                        "Cannot update deprecated entity unless the deprecation is being reversed",
                    ))
                }
            }

            valid!()
        },
        EntryTypes::WebAppPackageVersion(_) => {
            valid!()
        },
        // _ => invalid!(format!("Update validation not implemented for entry type: {:#?}", update.entry_type )),
    }
}
