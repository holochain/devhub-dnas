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
pub enum WasmType {
    Integrity,
    Coordinator,
}

impl From<WasmType> for String {
    fn from(wasm_type: WasmType) -> Self {
        match wasm_type {
            WasmType::Integrity => "integrity",
            WasmType::Coordinator => "coordinator",
        }.to_string()
    }
}

impl TryFrom<String> for WasmType {
    type Error = WasmError;

    fn try_from(name: String) -> Result<Self, Self::Error> {
        Ok(
            match (&name).to_lowercase().as_str() {
                "integrity" => WasmType::Integrity,
                "coordinator" => WasmType::Coordinator,
                _ => return Err(guest_error!(format!("Unknown WasmType variant: {}", name ))),
            }
        )
    }
}

impl Serialize for WasmType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str( &String::from( self.clone() ) )
    }
}

impl<'de> Deserialize<'de> for WasmType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: String = Deserialize::deserialize(deserializer)?;
        Ok(
            WasmType::try_from( s.clone() )
                .or(Err(serde::de::Error::custom(format!("Unknown WasmType variant: {}", s))))?
        )
    }
}



//
// Wasm Entry
//
#[hdk_entry_helper]
#[derive(Clone)]
pub struct WasmEntry {
    pub wasm_type: WasmType,
    pub mere_memory_addr: EntryHash,
    pub file_size: u64,
}

impl WasmEntry {
    pub fn new( wtype: WasmType, addr: EntryHash ) -> ExternResult<Self> {
        let memory : MemoryEntry = must_get_entry( addr.clone() )?.content.try_into()?;
        let entry = WasmEntry {
            wasm_type: wtype,
            mere_memory_addr: addr,
            file_size: memory.memory_size,
        };

        Ok( entry )
    }

    pub fn new_integrity( addr: EntryHash ) -> ExternResult<Self> {
        Self::new( WasmType::Integrity, addr )
    }

    pub fn new_coordinator( addr: EntryHash ) -> ExternResult<Self> {
        Self::new( WasmType::Coordinator, addr )
    }
}
