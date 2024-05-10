use crate::hdi;

use hdi::prelude::*;
use hdi_extensions::{
    guest_error,
};
use mere_memory_types::MemoryEntry;
use serde::{
    Serialize, Serializer,
    Deserialize, Deserializer,
};


#[derive(Debug, Clone)]
pub enum ZomeType {
    Integrity,
    Coordinator,
}

impl From<ZomeType> for String {
    fn from(zome_type: ZomeType) -> Self {
        match zome_type {
            ZomeType::Integrity => "integrity",
            ZomeType::Coordinator => "coordinator",
        }.to_string()
    }
}

impl TryFrom<String> for ZomeType {
    type Error = WasmError;

    fn try_from(name: String) -> Result<Self, Self::Error> {
        Ok(
            match (&name).to_lowercase().as_str() {
                "integrity" => ZomeType::Integrity,
                "coordinator" => ZomeType::Coordinator,
                _ => return Err(guest_error!(format!("Unknown ZomeType variant: {}", name ))),
            }
        )
    }
}

impl Serialize for ZomeType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str( &String::from( self.clone() ) )
    }
}

impl<'de> Deserialize<'de> for ZomeType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: String = Deserialize::deserialize(deserializer)?;
        Ok(
            ZomeType::try_from( s.clone() )
                .map_err( |err| serde::de::Error::custom(format!("{:?}", err )) )?
        )
    }
}



//
// Zome Entry
//
#[hdk_entry_helper]
#[derive(Clone)]
pub struct ZomeEntry {
    pub zome_type: ZomeType,
    pub mere_memory_addr: EntryHash,
    pub file_size: u64,
    pub hash: String,
}

impl ZomeEntry {
    pub fn new( wtype: ZomeType, addr: EntryHash ) -> ExternResult<Self> {
        let memory : MemoryEntry = must_get_entry( addr.clone() )?.content.try_into()?;
        let entry = ZomeEntry {
            zome_type: wtype,
            mere_memory_addr: addr,
            file_size: memory.uncompressed_size
                .unwrap_or( memory.memory_size ),
            hash: memory.hash,
        };

        Ok( entry )
    }

    pub fn new_integrity( addr: EntryHash ) -> ExternResult<Self> {
        Self::new( ZomeType::Integrity, addr )
    }

    pub fn new_coordinator( addr: EntryHash ) -> ExternResult<Self> {
        Self::new( ZomeType::Coordinator, addr )
    }
}
