// mod validation;

pub use hdi_extensions::hdi;

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
    EntryModel,
    entry_model,
};

#[hdk_entry_defs]
#[unit_enum(UnitEntryTypes)]
pub enum EntryTypes {
    #[entry_def]
    File(FileEntry),
}

entry_model!( EntryTypes::File( FileEntry ) );


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
