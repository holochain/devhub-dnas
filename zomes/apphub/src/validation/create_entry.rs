use crate::{
    hdi,
    hdi_extensions,
    mere_memory_types,
    EntryTypes,
    Authority,
    WebAppEntry,
};

use hdi::prelude::*;
use hdi_extensions::{
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
        EntryTypes::App(app_entry) => {
            app_entry.validate_roles_token()?;

            let app_token = app_entry.calculate_app_token()?;

            if app_entry.app_token != app_token {
                invalid!(format!("Invalid App Token; expected {:?}", app_token ))
            }

            valid!()
        },
        EntryTypes::Ui(ui_entry) => {
            let memory : MemoryEntry = must_get_entry( ui_entry.mere_memory_addr )?.try_into()?;
            let file_size = memory.uncompressed_size
                .unwrap_or( memory.memory_size );

            if ui_entry.file_size != file_size {
                invalid!(format!(
                    "UiEntry file size does not match memory address: {} != {}",
                    ui_entry.file_size, file_size
                ))
            }

            valid!()
        },
        EntryTypes::WebApp(webapp_entry) => {
            let webapp_token = webapp_entry.calculate_webapp_token()?;

            if webapp_entry.webapp_token != webapp_token {
                invalid!(format!("Invalid WebApp Token; expected {:?}", webapp_token ))
            }

            valid!()
        },
        EntryTypes::WebAppPackage(webapp_package_entry) => {
            match webapp_package_entry.maintainer {
                Authority::Agent(agent_id) => {
                    if agent_id != create.author {
                        invalid!(format!(
                            "Invalid maintainer '{}'; must match the creating agent ({})",
                            agent_id, create.author
                        ))
                    }
                },
            }

            valid!()
        },
        EntryTypes::WebAppPackageVersion(webapp_package_version_entry) => {
            match webapp_package_version_entry.maintainer {
                Authority::Agent(agent_id) => {
                    if agent_id != create.author {
                        invalid!(format!(
                            "Invalid maintainer '{}'; must match the creating agent ({})",
                            agent_id, create.author
                        ))
                    }
                },
            }

            let webapp_entry : WebAppEntry = must_get_entry( webapp_package_version_entry.webapp )?
                .try_into()?;
            let webapp_token = webapp_entry.calculate_webapp_token()?;

            if webapp_package_version_entry.webapp_token != webapp_token {
                invalid!(format!("Invalid WebApp Token; expected {:?}", webapp_token ))
            }

            valid!()
        },
        // _ => invalid!(format!("Create validation not implemented for entry type: {:#?}", create.entry_type )),
    }
}
