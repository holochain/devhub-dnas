use hc_entities::{ EntryModel, EntityType };
use hdk::prelude::*;


//
// File Entry
//
#[hdk_entry(id = "file_details", visibility="public")]
#[derive(Clone)]
pub struct FileEntry {
    pub author: AgentPubKey,
    pub published_at: u64,
    pub file_size: u64,
    pub chunk_addresses: Vec<EntryHash>,

    // optional
    pub name: Option<String>,
}

impl EntryModel for FileEntry {
    fn get_type(&self) -> EntityType {
	EntityType::new( "file", "entry" )
    }
}

// Summary
#[derive(Debug, Serialize, Deserialize)]
pub struct FileSummary {
    pub author: AgentPubKey,
    pub published_at: u64,
    pub file_size: u64,

    // optional
    pub name: Option<String>,
}
impl EntryModel for FileSummary {
    fn get_type(&self) -> EntityType {
	EntityType::new( "file", "summary" )
    }
}

// Full
#[derive(Debug, Serialize, Deserialize)]
pub struct FileInfo {
    pub author: AgentPubKey,
    pub published_at: u64,
    pub file_size: u64,
    pub chunk_addresses: Vec<EntryHash>,

    // optional
    pub name: Option<String>,
}
impl EntryModel for FileInfo {
    fn get_type(&self) -> EntityType {
	EntityType::new( "file", "info" )
    }
}

impl FileEntry {
    pub fn to_info(&self) -> FileInfo {
	FileInfo {
	    author: self.author.clone(),
	    published_at: self.published_at.clone(),
	    file_size: self.file_size.clone(),
	    chunk_addresses: self.chunk_addresses.clone(),
	    name: self.name.clone(),
	}
    }

    pub fn to_summary(&self) -> FileSummary {
	FileSummary {
	    author: self.author.clone(),
	    published_at: self.published_at.clone(),
	    file_size: self.file_size.clone(),
	    name: self.name.clone(),
	}
    }
}


//
// File Chunk Entry
//
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SequencePosition {
    pub position: u64,
    pub length: u64,
}

#[hdk_entry(id = "file_chunk", visibility="public")]
#[derive(Clone)]
pub struct FileChunkEntry {
    pub sequence: SequencePosition,
    pub bytes: SerializedBytes,
}

impl EntryModel for FileChunkEntry {
    fn get_type(&self) -> EntityType {
	EntityType::new( "file_chunk", "info" )
    }
}
