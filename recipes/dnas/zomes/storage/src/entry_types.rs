
use hdk::prelude::*;

use crate::utils;
use crate::errors::{ RuntimeError };


#[hdk_entry(id = "profile", visibility="public")]
pub struct ProfileEntry {
    pub name: String,
    pub email: String,
    pub avatar_image: SerializedBytes,
    pub website: String,
}

// Full
#[derive(Debug, Serialize, Deserialize)]
pub struct ProfileInfo {
    pub id: EntryHash,
    pub name: String,
    pub email: String,
    pub avatar_image: SerializedBytes,
    pub website: String,
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

impl TryFrom<Element> for ProfileEntry {
    type Error = WasmError;
    fn try_from(element: Element) -> Result<Self, Self::Error> {
	element.entry()
	    .to_app_option::<Self>()?
	    .ok_or(WasmError::from(RuntimeError::DeserializationError(element)))
    }
}




#[hdk_entry(id = "dna", visibility="public")]
pub struct DnaEntry {
    pub name: String,
    pub description: String,
    pub icon: SerializedBytes,
    pub published_at: u64,
    pub last_updated: u64,
    pub developer: DeveloperProfileLocation,

    // optional
    pub collaborators: Option<Vec<(AgentPubKey, String)>>,
    pub deprecation: Option<DeprecationNotice>,
}

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
    pub id: EntryHash,
    pub name: String,
    pub description: String,
    pub published_at: u64,
    pub developer: AgentPubKey,

    // optional
    pub deprecation: Option<bool>,
}

// Full
#[derive(Debug, Serialize, Deserialize)]
pub struct DnaInfo {
    pub id: EntryHash,
    pub name: String,
    pub description: String,
    pub published_at: u64,
    pub developer: DeveloperProfileLocation,

    // optional
    pub deprecation: Option<DeprecationNotice>,
}

impl DnaEntry {
    pub fn to_info(self, id: EntryHash) -> DnaInfo {
	DnaInfo {
	    id: id,
	    name: self.name,
	    description: self.description,
	    published_at: self.published_at,
	    developer: self.developer,
	    deprecation: self.deprecation,
	}
    }

    pub fn to_summary(self, id: EntryHash) -> DnaSummary {
	DnaSummary {
	    id: id,
	    name: self.name,
	    description: self.description,
	    published_at: self.published_at,
	    developer: self.developer.pubkey,
	    deprecation: match self.deprecation {
		Some(_) => Some(true),
		None => None,
	    },
	}
    }
}

impl TryFrom<Element> for DnaEntry {
    type Error = WasmError;
    fn try_from(element: Element) -> Result<Self, Self::Error> {
	element.entry()
	    .to_app_option::<Self>()?
	    .ok_or(WasmError::from(RuntimeError::DeserializationError(element)))
    }
}





#[hdk_entry(id = "dna_version", visibility="public")]
pub struct DnaVersionEntry {
    pub for_dna: EntryHash,
    pub version: u64,
    pub published_at: u64,
    pub file_size: u64,
    pub contributors: Vec<String>,
    pub changelog: String,
    pub chunk_addresses: Vec<EntryHash>,
}

// Summary
#[derive(Debug, Serialize, Deserialize)]
pub struct DnaVersionSummary {
    pub id: EntryHash,
    pub version: u64,
    pub published_at: u64,
    pub file_size: u64,
}

// Full
#[derive(Debug, Serialize, Deserialize)]
pub struct DnaVersionInfo {
    pub id: EntryHash,
    pub for_dna: Option<DnaSummary>,
    pub version: u64,
    pub published_at: u64,
    pub file_size: u64,
    pub contributors: Vec<String>,
    pub changelog: String,
    pub chunk_addresses: Vec<EntryHash>,
}

// Package
#[derive(Debug, Serialize, Deserialize)]
pub struct DnaPackage {
    pub for_dna: DnaSummary,
    pub version: u64,
    pub published_at: u64,
    pub file_size: u64,
    pub bytes: SerializedBytes,
    pub contributors: Vec<String>,
    pub changelog: String,
}

impl DnaVersionEntry {
    pub fn to_info(self, id: EntryHash) -> DnaVersionInfo {
	let mut dna_summary : Option<DnaSummary> = None;

	if let Some((_,element)) = utils::fetch_entry_latest( self.for_dna.clone() ).ok() {
	    dna_summary = match DnaEntry::try_from( element ) {
		Ok(dna) => Some(dna.to_summary( self.for_dna )),
		Err(_) => None,
	    };
	};

	DnaVersionInfo {
	    id: id,
	    for_dna: dna_summary,
	    version: self.version,
	    published_at: self.published_at,
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
	    file_size: self.file_size,
	}
    }
}

impl TryFrom<Element> for DnaVersionEntry {
    type Error = WasmError;
    fn try_from(element: Element) -> Result<Self, Self::Error> {
	element.entry()
	    .to_app_option::<Self>()?
	    .ok_or(WasmError::from(RuntimeError::DeserializationError(element)))
    }
}





#[hdk_entry(id = "dna_chunk", visibility="public")]
pub struct DnaChunkEntry {
    pub sequence: SequencePosition,
    pub bytes: SerializedBytes,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SequencePosition {
    pub position: u64,
    pub length: u64,
}

impl TryFrom<Element> for DnaChunkEntry {
    type Error = WasmError;
    fn try_from(element: Element) -> Result<Self, Self::Error> {
	element.entry()
	    .to_app_option::<Self>()?
	    .ok_or(WasmError::from(RuntimeError::DeserializationError(element)))
    }
}




#[hdk_extern]
fn validate_create_entry_dna(validate_data: ValidateData) -> ExternResult<ValidateCallbackResult> {
    if let Ok(_dna) = DnaEntry::try_from( validate_data.element ) {
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
	    icon: SerializedBytes::try_from(()).unwrap(),
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
	let bytes = rand::thread_rng().gen::<[u8; 32]>();
	let hash = EntryHash::from_raw_32( bytes.to_vec() );
	let dna1 = create_dnaentry();
	let dna2 = create_dnaentry();

	assert_eq!(dna1.name, "Game Turns");

	let dna_info = dna1.to_info( hash.clone() );

	assert_eq!(dna_info.name, "Game Turns");

	let dna_summary = dna2.to_summary( hash.clone() );

	assert_eq!(dna_summary.name, "Game Turns");
    }
}
