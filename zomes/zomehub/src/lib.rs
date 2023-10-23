mod validation;

// pub use hc_crud;
pub use zomehub_types;
pub use zomehub_types::*;

use serde::{
    Deserialize, Deserializer,
};
use hdi::prelude::*;
use hdi_extensions::{
    guest_error,
    scoped_type_connector,
    ScopedTypeConnector,
};
// use hc_crud::{
//     entry_model,
// };



/// The entry types defined for this integrity zome
#[hdk_entry_defs]
#[unit_enum(EntryTypesUnit)]
pub enum EntryTypes {
    #[entry_def]
    Wasm(WasmEntry),
}

scoped_type_connector!(
    EntryTypesUnit::Wasm,
    EntryTypes::Wasm( WasmEntry )
);



/// The link types defined for this integrity zome
#[hdk_link_types]
pub enum LinkTypes {
    Wasm,
}

impl TryFrom<String> for LinkTypes {
    type Error = WasmError;

    fn try_from(name: String) -> Result<Self, Self::Error> {
        Ok(
            match name.as_str() {
                "Wasm" => LinkTypes::Wasm,
                _ => return Err(guest_error!(format!("Unknown LinkTypes variant: {}", name ))),
            }
        )
    }
}

impl<'de> Deserialize<'de> for LinkTypes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: String = Deserialize::deserialize(deserializer)?;
        Ok(
            LinkTypes::try_from( s.clone() )
                .or(Err(serde::de::Error::custom(format!("Unknown LinkTypes variant: {}", s))))?
        )
    }
}
