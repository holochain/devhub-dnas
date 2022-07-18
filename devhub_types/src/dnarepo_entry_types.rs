use std::collections::BTreeMap;
use hc_crud::{
    EntryModel, EntityType,
};
use hdk::prelude::*;



//
// General-use Structs
//
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DeprecationNotice {
    pub message: String,

    // optional
    pub recommended_alternatives: Option<Vec<EntryHash>>,
}

impl DeprecationNotice {
    pub fn new(message: String) -> Self {
	Self {
	    message: message,
	    recommended_alternatives: None,
	}
    }
}



//
// Profile Entry
//
#[hdk_entry(id = "profile", visibility="public")]
#[derive(Clone)]
pub struct ProfileEntry {
    pub name: String,
    pub email: String,
    pub avatar_image: SerializedBytes,
    pub website: String,
}

impl EntryModel for ProfileEntry {
    fn get_type(&self) -> EntityType {
	EntityType::new( "profile", "entry" )
    }
}



//
// DNA Entry
//
#[hdk_entry(id = "dna", visibility="public")]
#[derive(Clone)]
pub struct DnaEntry {
    pub name: String,
    pub description: String,
    pub published_at: u64,
    pub last_updated: u64,
    pub developer: AgentPubKey,
    pub metadata: BTreeMap<String, serde_yaml::Value>,

    // optional
    pub display_name: Option<String>,
    pub tags: Option<Vec<String>>,
    pub icon: Option<SerializedBytes>,
    pub source_code_url: Option<String>,
    pub deprecation: Option<DeprecationNotice>,
}

impl EntryModel for DnaEntry {
    fn get_type(&self) -> EntityType {
	EntityType::new( "dna", "info" )
    }
}



//
// DNA Version Entry
//
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct ZomeReference {
    pub name: String,
    pub zome : EntryHash, // Zome ID
    pub version : EntryHash, // Version ID
    pub resource : EntryHash, // Mere Memory address for a short-circuit download
    pub resource_hash : String, // Hash of resource contents
}

#[hdk_entry(id = "dna_version", visibility="public")]
#[derive(Clone)]
pub struct DnaVersionEntry {
    pub for_dna: EntryHash,
    pub version: String,
    pub ordering: u64,
    pub published_at: u64,
    pub last_updated: u64,
    pub changelog: String,
    pub wasm_hash : String,
    pub hdk_version: String,
    pub zomes: Vec<ZomeReference>,
    pub metadata: BTreeMap<String, serde_yaml::Value>,

    // optional
    pub properties: Option<BTreeMap<String, serde_yaml::Value>>,
    pub source_code_commit_url: Option<String>,
}

impl EntryModel for DnaVersionEntry {
    fn get_type(&self) -> EntityType {
	EntityType::new( "dna_version", "info" )
    }
}

// Package
#[derive(Debug, Serialize, Deserialize)]
pub struct DnaVersionPackage {
    pub for_dna: EntryHash,
    pub version: String,
    pub published_at: u64,
    pub last_updated: u64,
    pub changelog: String,
    pub hdk_version: String,
    pub bytes: Vec<u8>,
}
impl EntryModel for DnaVersionPackage {
    fn get_type(&self) -> EntityType {
	EntityType::new( "dna_version", "package" )
    }
}

impl DnaVersionEntry {
    pub fn to_package(&self, dna_bytes: Vec<u8>) -> DnaVersionPackage {
	DnaVersionPackage {
	    for_dna: self.for_dna.clone(),
	    version: self.version.clone(),
	    published_at: self.published_at.clone(),
	    last_updated: self.last_updated.clone(),
	    changelog: self.changelog.clone(),
	    hdk_version: self.hdk_version.clone(),
	    bytes: dna_bytes,
	}
    }
}



//
// Zome Entry
//
#[hdk_entry(id = "zome", visibility="public")]
#[derive(Clone)]
pub struct ZomeEntry {
    pub name: String,
    pub description: String,
    pub published_at: u64,
    pub last_updated: u64,
    pub developer: AgentPubKey,
    pub metadata: BTreeMap<String, serde_yaml::Value>,

    // optional
    pub display_name: Option<String>,
    pub tags: Option<Vec<String>>,
    pub source_code_url: Option<String>,
    pub deprecation: Option<DeprecationNotice>,
}

impl EntryModel for ZomeEntry {
    fn get_type(&self) -> EntityType {
	EntityType::new( "zome", "info" )
    }
}



//
// Zome Version Entry
//
#[hdk_entry(id = "zome_version", visibility="public")]
#[derive(Clone, PartialEq)]
pub struct ZomeVersionEntry {
    pub for_zome: EntryHash,
    pub version: String,
    pub ordering: u64,
    // pub properties: Option<serde_yaml::Value>,
    pub published_at: u64,
    pub last_updated: u64,
    pub changelog: String,
    pub mere_memory_addr: EntryHash,
    pub mere_memory_hash: String,
    pub hdk_version: String,
    pub metadata: BTreeMap<String, serde_yaml::Value>,

    // optional
    pub review_summary: Option<EntryHash>,
    pub source_code_commit_url: Option<String>,
}

impl EntryModel for ZomeVersionEntry {
    fn get_type(&self) -> EntityType {
	EntityType::new( "zome_version", "info" )
    }
}



//
// Review Entry
//
#[hdk_entry(id = "review", visibility="public")]
#[derive(Clone, PartialEq)]
pub struct ReviewEntry {
    pub subject_ids: Vec<(EntryHash, HeaderHash)>,
    pub author: AgentPubKey,
    pub ratings: BTreeMap<String,u8>,
    pub message: String,
    pub published_at: u64,
    pub last_updated: u64,
    pub reaction_summary: Option<EntryHash>,
    pub metadata: BTreeMap<String, serde_yaml::Value>,
    pub deleted: bool,

    // optional
    pub related_entries: Option<BTreeMap<String, EntryHash>>,
}

impl EntryModel for ReviewEntry {
    fn get_type(&self) -> EntityType {
	EntityType::new( "review", "info" )
    }
}



//
// Reaction Entry
//
#[hdk_entry(id = "reaction", visibility="public")]
#[derive(Clone)]
pub struct ReactionEntry {
    pub subject_ids: Vec<(EntryHash, HeaderHash)>,
    pub author: AgentPubKey,
    pub reaction_type: u64,

    pub published_at: u64,
    pub last_updated: u64,

    pub metadata: BTreeMap<String, serde_yaml::Value>,
    pub deleted: bool,

    // optional
    pub related_entries: Option<BTreeMap<String, EntryHash>>,
}

impl EntryModel for ReactionEntry {
    fn get_type(&self) -> EntityType {
	EntityType::new( "reaction", "info" )
    }
}



//
// Reaction Summary Entry
//
#[hdk_entry(id = "reaction_summary", visibility="public")]
#[derive(Clone)]
pub struct ReactionSummaryEntry {
    pub subject_id: EntryHash,
    pub subject_history: Vec<HeaderHash>,

    pub published_at: u64,
    pub last_updated: u64,

    pub factored_action_count: u64,

    pub reaction_refs: BTreeMap<String,(EntryHash, HeaderHash, AgentPubKey, u64, u64)>,
    pub deleted_reactions: BTreeMap<String,(EntryHash, HeaderHash)>,
}

impl EntryModel for ReactionSummaryEntry {
    fn get_type(&self) -> EntityType {
	EntityType::new( "reaction_summary", "info" )
    }
}



//
// Review Summary Entry
//
#[hdk_entry(id = "review_summary", visibility="public")]
#[derive(Clone)]
pub struct ReviewSummaryEntry {
    pub subject_id: EntryHash,
    pub subject_history: Vec<HeaderHash>,

    pub published_at: u64,
    pub last_updated: u64,

    pub factored_action_count: u64,
    //
    // For each Review, we need to have:
    //
    //   ID - original entry hash
    //   latest header - the header of the review entry that we are using for stats
    //   author - agent ID
    //   action count - the history length
    //   ratings - all rating values from review
    //
    //   reaction_summary - ID + latest header
    //   likes (helpful) - num of likes for this review
    //   dislikes (unhelpful) - num of likes for this review
    //
    //   review_total_activity - the activity count for all review revisions
    //
    pub review_refs: BTreeMap<String,(EntryHash, HeaderHash, AgentPubKey, u64, BTreeMap<String,u8>, Option<(HeaderHash, u64, BTreeMap<u64,u64>)>)>,
    pub deleted_reviews: BTreeMap<String,(EntryHash, HeaderHash, AgentPubKey, Option<(HeaderHash, u64, BTreeMap<u64,u64>)>)>,
}

impl EntryModel for ReviewSummaryEntry {
    fn get_type(&self) -> EntityType {
	EntityType::new( "review_summary", "info" )
    }
}


pub fn trace_header_origin_entry(header_hash: &HeaderHash, depth: Option<u64>) -> ExternResult<(EntryHash,u64)> {
    let sh_header = must_get_header( header_hash.to_owned().into() )?;
    let depth : u64 = depth.unwrap_or(0);

    match sh_header.header() {
	Header::Create(create) => Ok( (create.entry_hash.to_owned(), depth) ),
	Header::Update(update) => trace_header_origin_entry( &update.original_header_address, Some(depth+1) ),
	header => Err(WasmError::Guest(format!("Unexpected header type @ depth {}: {:?}", depth, header ))),
    }
}

pub fn trace_action_history_with_chain(header_hash: &HeaderHash, history: Option<Vec<(HeaderHash,EntryHash)>>) -> ExternResult<Vec<(HeaderHash,EntryHash)>> {
    let sh_header = must_get_header( header_hash.to_owned().into() )?;
    let mut history = history.unwrap_or( Vec::new() );

    match sh_header.header() {
	Header::Create(create) => {
	    history.push( (header_hash.to_owned(), create.entry_hash.to_owned()) );

	    Ok( history )
	},
	Header::Update(update) => {
	    history.push( (header_hash.to_owned(), update.entry_hash.to_owned()) );

	    trace_action_history_with_chain( &update.original_header_address, Some(history) )
	},
	header => Err(WasmError::Guest(format!("Unexpected header type @ trace depth {}: {:?}", history.len(), header ))),
    }
}

pub fn trace_action_history(header_hash: &HeaderHash) -> ExternResult<Vec<(HeaderHash,EntryHash)>> {
    trace_action_history_with_chain(header_hash, None)
}



#[cfg(test)]
pub mod tests {
    use super::*;
    use rand::Rng;

    fn create_dnaentry() -> DnaEntry {
	let bytes = rand::thread_rng().gen::<[u8; 32]>();
	let hash = EntryHash::from_raw_32( bytes.to_vec() );

	DnaEntry {
	    name: String::from("game_turns"),
	    display_name: Some(String::from("Game Turns")),
	    description: String::from("A tool for turn-based games to track the order of player actions"),
	    icon: Some(SerializedBytes::try_from(()).unwrap()),
	    tags: Some(vec![ String::from("Games") ]),
	    published_at: 1618855430,
	    last_updated: 1618855430,

	    // optional
	    developer: hash.into(),
	    deprecation: None,
	    source_code_url: None,
	    metadata: BTreeMap::new(),
	}
    }

    #[test]
    ///
    fn dna_to_info_test() {
	let dna1 = create_dnaentry();
	create_dnaentry();

	assert_eq!(dna1.name, "game_turns");
    }
}
