use std::collections::BTreeMap;
use hc_crud::{
    EntryModel, EntityType
};
use hdk::prelude::*;

use crate::{ call_local_zome };


//
// File Entry
//
#[hdk_entry(id = "file_details", visibility="public")]
#[derive(Clone)]
pub struct FileEntry {
    pub author: AgentPubKey,
    pub published_at: u64,
    pub last_updated: u64,
    pub file_size: u64,
    pub mere_memory_addr: EntryHash,
    pub mere_memory_hash: String,

    // optional
    pub name: Option<String>,
    pub metadata: BTreeMap<String, serde_yaml::Value>,
}

impl EntryModel for FileEntry {
    fn get_type(&self) -> EntityType {
	EntityType::new( "file", "info" )
    }
}

// Full
#[derive(Debug, Serialize, Deserialize)]
pub struct FilePackage {
    pub author: AgentPubKey,
    pub published_at: u64,
    pub last_updated: u64,
    pub file_size: u64,
    pub mere_memory_addr: EntryHash,
    pub mere_memory_hash: String,

    // optional
    pub bytes: Option<Vec<u8>>,
    pub name: Option<String>,
    pub metadata: BTreeMap<String, serde_yaml::Value>,
}
impl EntryModel for FilePackage {
    fn get_type(&self) -> EntityType {
	EntityType::new( "file", "package" )
    }
}

impl FileEntry {
    pub fn to_package(&self) -> FilePackage {
	let mut file_bytes : Option<Vec<u8>> = None;

	if let Some(bytes) = call_local_zome("mere_memory", "retrieve_bytes", self.mere_memory_addr.to_owned() ).ok() {
	    file_bytes = Some( bytes );
	};

	FilePackage {
	    author: self.author.clone(),
	    published_at: self.published_at.clone(),
	    last_updated: self.last_updated.clone(),
	    file_size: self.file_size.clone(),
	    mere_memory_addr: self.mere_memory_addr.clone(),
	    mere_memory_hash: self.mere_memory_hash.clone(),
	    bytes: file_bytes,
	    name: self.name.clone(),
	    metadata: self.metadata.clone(),
	}
    }
}
