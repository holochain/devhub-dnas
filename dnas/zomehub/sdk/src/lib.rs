pub use zomehub_types;
pub use devhub_sdk;
pub use devhub_sdk::*;

use hdk::prelude::*;
use hdk_extensions::{
    must_get,
};
use zomehub_types::{
    ZomeEntry,
    mere_memory_types,
};
use mere_memory_types::{
    MemoryEntry,
};



#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MemoryWithBytes(
    MemoryEntry,
    Vec<u8>
);


#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ZomePackage {
    pub zome_entry: ZomeEntry,
    pub bytes: Vec<u8>,
}

impl TryInto<ZomePackage> for EntryHash {
    type Error = WasmError;
    fn try_into(self) -> ExternResult<ZomePackage> {
        let zome_entry : ZomeEntry = must_get( &self )?.try_into()?;

        let memory_with_bytes : MemoryWithBytes = call_zome(
            "mere_memory_api",
            "get_memory_with_bytes",
            zome_entry.mere_memory_addr.clone(),
            (),
        )?;

        Ok(
            ZomePackage {
                zome_entry: zome_entry,
                bytes: memory_with_bytes.1.to_vec(),
            }
        )
    }
}
