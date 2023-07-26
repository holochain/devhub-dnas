mod validation;

pub use hdi_extensions::hdi;

use hdi::prelude::*;
use serde::de::{ Deserializer, Error };
use devhub_types::{
    happ_entry_types::{
	HappEntry,
	HappReleaseEntry,

	GUIEntry,
	GUIReleaseEntry,
    },
};
pub use devhub_types::{
    create_path,
    AppResult,
};
pub use hc_crud::{
    EntryModel,
    entry_model,
};

#[hdk_entry_defs]
#[unit_enum(UnitEntryTypes)]
pub enum EntryTypes {
    #[entry_def]
    Happ(HappEntry),
    #[entry_def]
    HappRelease(HappReleaseEntry),

    #[entry_def]
    GUI(GUIEntry),
    #[entry_def]
    GUIRelease(GUIReleaseEntry),
}

entry_model!( EntryTypes::Happ( HappEntry ) );
entry_model!( EntryTypes::HappRelease( HappReleaseEntry ) );
entry_model!( EntryTypes::GUI( GUIEntry ) );
entry_model!( EntryTypes::GUIRelease( GUIReleaseEntry ) );


#[hdk_link_types]
pub enum LinkTypes {
    Agent,

    Happ,
    HappRelease,

    GUI,
    GUIRelease,

    Tag,
    Anchor,
}

impl<'de> Deserialize<'de> for LinkTypes {
    fn deserialize<D>(deserializer: D) -> Result<LinkTypes, D::Error>
    where
	D: Deserializer<'de>,
    {
	let name : &str = Deserialize::deserialize(deserializer)?;
	match name {
	    "Agent" => Ok(LinkTypes::Agent),

	    "Happ" => Ok(LinkTypes::Happ),
	    "HappRelease" => Ok(LinkTypes::HappRelease),
	    "GUIRelease" => Ok(LinkTypes::GUIRelease),

	    "Tag" => Ok(LinkTypes::Tag),
	    "Anchor" => Ok(LinkTypes::Anchor),

	    value => Err(D::Error::custom(format!("No LinkTypes value matching '{}'", value ))),
	}
    }
}
