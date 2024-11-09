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
    #[entry_type]
    ZomePackage(ZomePackageEntry),
    #[entry_type]
    ZomePackageVersion(ZomePackageVersionEntry),
}

scoped_type_connector!(
    EntryTypesUnit::Zome,
    EntryTypes::Zome( ZomeEntry )
);
scoped_type_connector!(
    EntryTypesUnit::ZomePackage,
    EntryTypes::ZomePackage(ZomePackageEntry)
);
scoped_type_connector!(
    EntryTypesUnit::ZomePackageVersion,
    EntryTypes::ZomePackageVersion(ZomePackageVersionEntry)
);

// Entity implementations
entry_model!( EntryTypes::Zome( ZomeEntry ) );
entry_model!( EntryTypes::ZomePackage( ZomePackageEntry ) );
entry_model!( EntryTypes::ZomePackageVersion( ZomePackageVersionEntry ) );



/// The link types defined for this integrity zome
#[hdk_link_types]
pub enum LinkTypes {
    ZomePackage,

    AgentToZome,
    AgentToZomePackage,
    AgentToZomePackageVersion,

    NameToGroup,
    NameToZomePackage,

    AllAgentsToAgent,
    AllOrgsToGroup,

    ZomePackageToZomePackageVersion,
}

impl TryFrom<String> for LinkTypes {
    type Error = WasmError;

    fn try_from(name: String) -> Result<Self, Self::Error> {
        Ok(
            match name.as_str() {
                "ZomePackage" => LinkTypes::ZomePackage,

                "AgentToZome" => LinkTypes::AgentToZome,
                "AgentToZomePackage" => LinkTypes::AgentToZomePackage,
                "AgentToZomePackageVersion" => LinkTypes::AgentToZomePackageVersion,

                "NameToGroup" => LinkTypes::NameToGroup,
                "NameToZomePackage" => LinkTypes::NameToZomePackage,

                "AllAgentsToAgent" => LinkTypes::AllAgentsToAgent,
                "AllOrgsToGroup" => LinkTypes::AllOrgsToGroup,

                "ZomePackageToZomePackageVersion" => LinkTypes::ZomePackageToZomePackageVersion,

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
