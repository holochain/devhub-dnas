// mod validation;

use hdi::prelude::*;
use serde::de::{ Deserializer, Error };
use devhub_types::{
    web_asset_entry_types::{
	FileEntry,
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
    File(FileEntry),
}


#[hdk_link_types]
pub enum LinkTypes {
    Agent,
    File,
}

impl<'de> Deserialize<'de> for LinkTypes {
    fn deserialize<D>(deserializer: D) -> Result<LinkTypes, D::Error>
    where
	D: Deserializer<'de>,
    {
	let name : &str = Deserialize::deserialize(deserializer)?;
	match name {
	    "Agent" => Ok(LinkTypes::File),
	    "File" => Ok(LinkTypes::File),
	    value => Err(D::Error::custom(format!("No LinkTypes value matching '{}'", value ))),
	}
    }
}



impl EntryModel<EntryTypes> for FileEntry {
    fn name() -> &'static str { "File" }
    fn get_type(&self) -> EntityType {
	EntityType::new( "file", "entry" )
    }
    fn to_input(&self) -> EntryTypes {
	EntryTypes::File(self.clone())
    }
}
