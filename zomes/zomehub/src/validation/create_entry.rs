use crate::{
    hdi,
    hdi_extensions,
    mere_memory_types,
    EntryTypes,
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
    _create: Create
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

            valid!()
        },
        // _ => invalid!(format!("Create validation not implemented for entry type: {:#?}", create.entry_type )),
    }
}
