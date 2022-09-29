mod validation;

use hdi::prelude::*;
use serde::de::{ Deserializer, Error };
use devhub_types::{
    happ_entry_types::{
	HappEntry,
	HappReleaseEntry,
	WebHappReleaseEntry,
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
    WebHappRelease(WebHappReleaseEntry),
}


#[hdk_link_types]
pub enum LinkTypes {
    Agent,

    Happ,
    HappRelease,
    WebHappRelease,

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
	    "WebHappRelease" => Ok(LinkTypes::WebHappRelease),

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

impl EntryModel<EntryTypes> for WebHappReleaseEntry {
    fn name() -> &'static str { "WebHappRelease" }
    fn get_type(&self) -> EntityType {
	EntityType::new( "webhapp_release", "entry" )
    }
    fn to_input(&self) -> EntryTypes {
	EntryTypes::WebHappRelease(self.clone())
    }
}
