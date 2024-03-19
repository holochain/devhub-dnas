use crate::hdi;

use hdi::prelude::*;
use mere_memory_types::MemoryEntry;



//
// UI Entry
//
#[hdk_entry_helper]
#[derive(Clone)]
pub struct UiEntry {
    pub mere_memory_addr: EntryHash,
    pub file_size: u64,
}

impl UiEntry {
    pub fn new( addr: EntryHash ) -> ExternResult<Self> {
        let memory : MemoryEntry = must_get_entry( addr.clone() )?.content.try_into()?;
        let entry = UiEntry {
            mere_memory_addr: addr,
            file_size: memory.uncompressed_size
                .unwrap_or( memory.memory_size ),
        };

        Ok( entry )
    }
}
