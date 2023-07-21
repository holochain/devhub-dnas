use std::collections::BTreeMap;
use hdk::prelude::*;



//
// General-use Structs
//
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DeprecationNotice {
    pub message: String,

    // optional
    pub recommended_alternatives: Option<Vec<ActionHash>>,
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
#[hdk_entry_helper]
#[derive(Clone)]
pub struct ProfileEntry {
    pub name: String,
    pub email: String,
    pub avatar_image: SerializedBytes,
    pub website: String,
}



//
// DNA Entry
//
#[hdk_entry_helper]
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



//
// DNA Version Entry
//
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct IntegrityZomeReference {
    pub name: String,
    pub zome : ActionHash, // Zome ID
    pub version : ActionHash, // Version ID
    pub resource : EntryHash, // Mere Memory address for a short-circuit download
    pub resource_hash : String, // Hash of resource contents
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct ZomeReference {
    pub name: String,
    pub zome : ActionHash, // Zome ID
    pub version : ActionHash, // Version ID
    pub resource : EntryHash, // Mere Memory address for a short-circuit download
    pub resource_hash : String, // Hash of resource contents
    pub dependencies: Vec<String>,
}

#[hdk_entry_helper]
#[derive(Clone)]
pub struct DnaVersionEntry {
    pub for_dna: ActionHash,
    pub version: String,
    pub ordering: u64,
    pub published_at: u64,
    pub last_updated: u64,
    pub changelog: String,
    pub wasm_hash : String,
    pub hdk_version: String,
    pub integrity_zomes: Vec<IntegrityZomeReference>,
    pub zomes: Vec<ZomeReference>,
    pub metadata: BTreeMap<String, serde_yaml::Value>,
    pub origin_time: HumanTimestamp,

    // optional
    pub network_seed: Option<String>,
    pub properties: Option<BTreeMap<String, serde_yaml::Value>>,
    pub source_code_commit_url: Option<String>,
}

// Package
#[derive(Debug, Serialize, Deserialize)]
pub struct DnaVersionPackage {
    pub for_dna: ActionHash,
    pub version: String,
    pub published_at: u64,
    pub last_updated: u64,
    pub changelog: String,
    pub hdk_version: String,
    pub bytes: Vec<u8>,
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
#[hdk_entry_helper]
#[derive(Clone)]
pub struct ZomeEntry {
    pub name: String,
    pub zome_type: u8,
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



//
// Zome Version Entry
//
#[hdk_entry_helper]
#[derive(Clone, PartialEq)]
pub struct ZomeVersionEntry {
    pub for_zome: ActionHash,
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
    pub review_summary: Option<ActionHash>,
    pub source_code_commit_url: Option<String>,
}



//
// Review Entry
//
#[hdk_entry_helper]
#[derive(Clone, PartialEq)]
pub struct ReviewEntry {
    pub subject_ids: Vec<(ActionHash, ActionHash)>,
    pub author: AgentPubKey,
    pub ratings: BTreeMap<String,u8>,
    pub message: String,
    pub published_at: u64,
    pub last_updated: u64,
    pub reaction_summary: Option<ActionHash>,
    pub metadata: BTreeMap<String, serde_yaml::Value>,
    pub deleted: bool,

    // optional
    pub related_entries: Option<BTreeMap<String, ActionHash>>,
}



//
// Reaction Entry
//
#[hdk_entry_helper]
#[derive(Clone)]
pub struct ReactionEntry {
    pub subject_ids: Vec<(ActionHash, ActionHash)>,
    pub author: AgentPubKey,
    pub reaction_type: u64,

    pub published_at: u64,
    pub last_updated: u64,

    pub metadata: BTreeMap<String, serde_yaml::Value>,
    pub deleted: bool,

    // optional
    pub related_entries: Option<BTreeMap<String, ActionHash>>,
}



//
// Reaction Summary Entry
//
#[hdk_entry_helper]
#[derive(Clone)]
pub struct ReactionSummaryEntry {
    pub subject_id: ActionHash,
    pub subject_history: Vec<ActionHash>,

    pub published_at: u64,
    pub last_updated: u64,

    pub factored_action_count: u64,

    pub reaction_refs: BTreeMap<String,(ActionHash, ActionHash, AgentPubKey, u64, u64)>,
    pub deleted_reactions: BTreeMap<String,(ActionHash, ActionHash)>,
}



//
// Review Summary Entry
//
#[hdk_entry_helper]
#[derive(Clone)]
pub struct ReviewSummaryEntry {
    pub subject_id: ActionHash,
    pub subject_history: Vec<ActionHash>,

    pub published_at: u64,
    pub last_updated: u64,

    pub factored_action_count: u64,
    //
    // For each Review, we need to have:
    //
    //   ID - original entry hash
    //   latest action - the action of the review entry that we are using for stats
    //   author - agent ID
    //   action count - the history length
    //   ratings - all rating values from review
    //
    //   reaction_summary - ID + latest action
    //   likes (helpful) - num of likes for this review
    //   dislikes (unhelpful) - num of likes for this review
    //
    //   review_total_activity - the activity count for all review revisions
    //
    pub review_refs: BTreeMap<String,(ActionHash, ActionHash, AgentPubKey, u64, BTreeMap<String,u8>, Option<(ActionHash, u64, BTreeMap<u64,u64>)>)>,
    pub deleted_reviews: BTreeMap<String,(ActionHash, ActionHash, AgentPubKey, Option<(ActionHash, u64, BTreeMap<u64,u64>)>)>,
}



#[cfg(test)]
pub mod tests {
    use super::*;
    use rand::Rng;

    fn create_dnaentry() -> DnaEntry {
	let bytes = rand::thread_rng().gen::<[u8; 32]>();
	let agent = AgentPubKey::from_raw_32( bytes.to_vec() );

	DnaEntry {
	    name: String::from("game_turns"),
	    display_name: Some(String::from("Game Turns")),
	    description: String::from("A tool for turn-based games to track the order of player actions"),
	    icon: Some(SerializedBytes::try_from(()).unwrap()),
	    tags: Some(vec![ String::from("Games") ]),
	    published_at: 1618855430,
	    last_updated: 1618855430,

	    // optional
	    developer: agent.into(),
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
