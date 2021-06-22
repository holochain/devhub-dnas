use hdk::prelude::*;
use hc_dna_reply_types::{ EntryModel };
use hc_dna_utils as utils;


//
// Profile Entry
//
#[hdk_entry(id = "profile", visibility="public")]
pub struct ProfileEntry {
    pub name: String,
    pub email: String,
    pub avatar_image: SerializedBytes,
    pub website: String,
}
utils::try_from_element![ ProfileEntry ];

// Full
#[derive(Debug, Serialize, Deserialize)]
pub struct ProfileInfo {
    pub id: EntryHash,
    pub name: String,
    pub email: String,
    pub avatar_image: SerializedBytes,
    pub website: String,
}
impl EntryModel for ProfileInfo {
    fn get_type(&self) -> String {
	"info".into()
    }
}

impl ProfileEntry {
    pub fn to_info(self, id: EntryHash) -> ProfileInfo {
	ProfileInfo {
	    id: id,
	    name: self.name,
	    email: self.email,
	    website: self.website,
	    avatar_image: self.avatar_image,
	}
    }
}




//
// DNA Entry
//
#[hdk_entry(id = "dna", visibility="public")]
pub struct DnaEntry {
    pub name: String,
    pub description: String,
    pub published_at: u64,
    pub last_updated: u64,
    pub developer: DeveloperProfileLocation,

    // optional
    pub icon: Option<SerializedBytes>,
    pub collaborators: Option<Vec<(AgentPubKey, String)>>,
    pub deprecation: Option<DeprecationNotice>,
}
utils::try_from_element![ DnaEntry ];

#[derive(Debug, Serialize, Deserialize)]
pub struct DeveloperProfileLocation {
    pub pubkey: AgentPubKey,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeprecationNotice {
    pub message: String,

    // optional
    pub recommended_alternatives: Option<HeaderHash>,
}

impl DeprecationNotice {
    pub fn new(message: String) -> Self {
	Self {
	    message: message,
	    recommended_alternatives: None,
	}
    }
}

// Summary
#[derive(Debug, Serialize, Deserialize)]
pub struct DnaSummary {
    pub name: String,
    pub description: String,
    pub published_at: u64,
    pub last_updated: u64,
    pub developer: AgentPubKey,

    // optional
    pub icon: Option<SerializedBytes>,
    pub deprecation: Option<bool>,
}
impl EntryModel for DnaSummary {
    fn get_type(&self) -> String {
	"summary".into()
    }
}

// Full
#[derive(Debug, Serialize, Deserialize)]
pub struct DnaInfo {
    pub name: String,
    pub description: String,
    pub published_at: u64,
    pub last_updated: u64,
    pub developer: DeveloperProfileLocation,

    // optional
    pub icon: Option<SerializedBytes>,
    pub collaborators: Option<Vec<(AgentPubKey, String)>>,
    pub deprecation: Option<DeprecationNotice>,
}
impl EntryModel for DnaInfo {
    fn get_type(&self) -> String {
	"info".into()
    }
}

impl DnaEntry {
    pub fn to_info(self) -> DnaInfo {
	DnaInfo {
	    name: self.name,
	    description: self.description,
	    icon: self.icon,
	    published_at: self.published_at,
	    last_updated: self.last_updated,
	    developer: self.developer,
	    collaborators: self.collaborators,
	    deprecation: self.deprecation,
	}
    }

    pub fn to_summary(self) -> DnaSummary {
	DnaSummary {
	    name: self.name,
	    description: self.description,
	    icon: self.icon,
	    published_at: self.published_at,
	    last_updated: self.last_updated,
	    developer: self.developer.pubkey,
	    deprecation: match self.deprecation {
		Some(_) => Some(true),
		None => None,
	    },
	}
    }
}




//
// DNA Version Entry
//
#[hdk_entry(id = "dna_version", visibility="public")]
pub struct DnaVersionEntry {
    pub for_dna: EntryHash,
    pub version: u64,
    pub published_at: u64,
    pub last_updated: u64,
    pub file_size: u64,
    pub contributors: Vec<(String, Option<AgentPubKey>)>,
    pub changelog: String,
    pub chunk_addresses: Vec<EntryHash>,
}
utils::try_from_element![ DnaVersionEntry ];

// Summary
#[derive(Debug, Serialize, Deserialize)]
pub struct DnaVersionSummary {
    pub id: EntryHash,
    pub version: u64,
    pub published_at: u64,
    pub last_updated: u64,
    pub file_size: u64,
}
impl EntryModel for DnaVersionSummary {
    fn get_type(&self) -> String {
	"summary".into()
    }
}

// Full
#[derive(Debug, Serialize, Deserialize)]
pub struct DnaVersionInfo {
    pub id: EntryHash,
    pub for_dna: Option<DnaSummary>,
    pub version: u64,
    pub published_at: u64,
    pub last_updated: u64,
    pub file_size: u64,
    pub contributors: Vec<(String, Option<AgentPubKey>)>,
    pub changelog: String,
    pub chunk_addresses: Vec<EntryHash>,
}
impl EntryModel for DnaVersionInfo {
    fn get_type(&self) -> String {
	"info".into()
    }
}

impl DnaVersionEntry {
    pub fn to_info(self, id: EntryHash) -> DnaVersionInfo {
	let mut dna_summary : Option<DnaSummary> = None;

	if let Some((_,element)) = utils::fetch_entry_latest( self.for_dna.clone() ).ok() {
	    dna_summary = match DnaEntry::try_from( &element ) {
		Ok(dna) => Some(dna.to_summary()),
		Err(_) => None,
	    };
	};

	DnaVersionInfo {
	    id: id,
	    for_dna: dna_summary,
	    version: self.version,
	    published_at: self.published_at,
	    last_updated: self.last_updated,
	    file_size: self.file_size,
	    contributors: self.contributors,
	    changelog: self.changelog,
	    chunk_addresses: self.chunk_addresses,
	}
    }

    pub fn to_summary(self, id: EntryHash) -> DnaVersionSummary {
	DnaVersionSummary {
	    id: id,
	    version: self.version,
	    published_at: self.published_at,
	    last_updated: self.last_updated,
	    file_size: self.file_size,
	}
    }
}





//
// DNA Chunk Entry
//
#[hdk_entry(id = "dna_chunk", visibility="public")]
pub struct DnaChunkEntry {
    pub sequence: SequencePosition,
    pub bytes: SerializedBytes,
}
utils::try_from_element![ DnaChunkEntry ];

#[derive(Debug, Serialize, Deserialize)]
pub struct SequencePosition {
    pub position: u64,
    pub length: u64,
}




#[hdk_extern]
fn validate_create_entry_dna(validate_data: ValidateData) -> ExternResult<ValidateCallbackResult> {
    if let Ok(_dna) = DnaEntry::try_from( &validate_data.element ) {
	return Ok(ValidateCallbackResult::Valid);
    }

    Ok(ValidateCallbackResult::Invalid("DNA entry is not right".to_string()))
}




#[cfg(test)]
pub mod tests {
    use super::*;
    use rand::Rng;

    fn create_dnaentry() -> crate::DnaEntry {
	let bytes = rand::thread_rng().gen::<[u8; 32]>();
	let hash = EntryHash::from_raw_32( bytes.to_vec() );

	crate::DnaEntry {
	    name: String::from("Game Turns"),
	    description: String::from("A tool for turn-based games to track the order of player actions"),
	    icon: Some(SerializedBytes::try_from(()).unwrap()),
	    published_at: 1618855430,
	    last_updated: 1618855430,

	    // optional
	    collaborators: None,
	    developer: DeveloperProfileLocation {
		pubkey: hash.into(),
	    },
	    deprecation: None,
	}
    }

    #[test]
    ///
    fn dna_to_summary_test() {
	let dna1 = create_dnaentry();
	let dna2 = create_dnaentry();

	assert_eq!(dna1.name, "Game Turns");

	let dna_info = dna1.to_info();

	assert_eq!(dna_info.name, "Game Turns");

	let dna_summary = dna2.to_summary();

	assert_eq!(dna_summary.name, "Game Turns");
    }
}
