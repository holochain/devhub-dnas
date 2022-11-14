mod validation;

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
    EntryModel, EntityType,
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



impl EntryModel<EntryTypes> for HappEntry {
    fn name() -> &'static str { "Happ" }
    fn get_type(&self) -> EntityType {
	EntityType::new( "happ", "entry" )
    }
    fn to_input(&self) -> EntryTypes {
	EntryTypes::Happ(self.clone())
    }
}

impl EntryModel<EntryTypes> for HappReleaseEntry {
    fn name() -> &'static str { "HappRelease" }
    fn get_type(&self) -> EntityType {
	EntityType::new( "happ_release", "entry" )
    }
    fn to_input(&self) -> EntryTypes {
	EntryTypes::HappRelease(self.clone())
    }
}

impl EntryModel<EntryTypes> for GUIEntry {
    fn name() -> &'static str { "GUI" }
    fn get_type(&self) -> EntityType {
	EntityType::new( "gui", "entry" )
    }
    fn to_input(&self) -> EntryTypes {
	EntryTypes::GUI(self.clone())
    }
}

impl EntryModel<EntryTypes> for GUIReleaseEntry {
    fn name() -> &'static str { "GUIRelease" }
    fn get_type(&self) -> EntityType {
	EntityType::new( "gui_release", "entry" )
    }
    fn to_input(&self) -> EntryTypes {
	EntryTypes::GUIRelease(self.clone())
    }
}
