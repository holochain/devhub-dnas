mod validation;

pub use zomehub_types;
pub use zomehub_types::*;
pub use hc_crud;

use serde::{
    Deserialize, Deserializer,
};
use hdi::prelude::*;
use hdi_extensions::{
    guest_error,
    scoped_type_connector,
    ScopedTypeConnector,
};
use hc_crud::{
    entry_model,
};



/// The entry types defined for this integrity zome
#[hdk_entry_types]
#[unit_enum(EntryTypesUnit)]
pub enum EntryTypes {
    #[entry_type]
    Zome(ZomeEntry),
}

scoped_type_connector!(
    EntryTypesUnit::Zome,
    EntryTypes::Zome( ZomeEntry )
);

// Entity implementations
entry_model!( EntryTypes::Zome( ZomeEntry ) );



/// The link types defined for this integrity zome
#[hdk_link_types]
pub enum LinkTypes {
    Zome,
}

impl TryFrom<String> for LinkTypes {
    type Error = WasmError;

    fn try_from(name: String) -> Result<Self, Self::Error> {
        Ok(
            match name.as_str() {
                "Zome" => LinkTypes::Zome,
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
