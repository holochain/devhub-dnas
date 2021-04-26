
use hdk::prelude::*;
use crate::errors::{ RuntimeError };


#[derive(Debug, Serialize, Deserialize)]
pub struct EntityInfo {
    pub name: String,

    // optional
    pub website: Option<String>,
}

#[hdk_entry(id = "dna_entry", visibility="public")]
pub struct DnaEntry {
    pub name: String,
    pub description: String,
    pub published_at: u64,

    // optional
    pub developer: Option<EntityInfo>,
    pub deprecation: Option<DeprecationNotice>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeprecationNotice {
    pub message: String,

    // optional
    pub recommended_alternatives: Option<EntryHash>,
}

// Summary
#[derive(Debug, Serialize, Deserialize)]
pub struct DnaSummary {
    pub name: String,
    pub description: String,
    pub published_at: u64,

    // optional
    pub developer: Option<String>,
    pub deprecation: Option<bool>,
}

// Full
#[derive(Debug, Serialize, Deserialize)]
pub struct DnaInfo {
    pub name: String,
    pub description: String,
    pub published_at: u64,

    // optional
    pub developer: Option<EntityInfo>,
    pub deprecation: Option<DeprecationNotice>,
}

impl DnaEntry {
    fn to_info(self) -> DnaInfo {
	self.into()
    }

    fn to_summary(self) -> DnaSummary {
	self.into()
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

impl From<DnaEntry> for DnaInfo {
    fn from(dna: DnaEntry) -> Self {
	DnaInfo {
	    name: dna.name,
	    description: dna.description,
	    published_at: dna.published_at,
	    developer: dna.developer,
	    deprecation: dna.deprecation,
	}
    }
}

impl From<DnaEntry> for DnaSummary {
    fn from(dna: DnaEntry) -> Self {
	DnaSummary {
	    name: dna.name,
	    description: dna.description,
	    published_at: dna.published_at,
	    developer: match dna.developer {
		Some(dev) => Some(dev.name),
		None => None,
	    },
	    deprecation: match dna.deprecation {
		Some(_) => Some(true),
		None => None,
	    },
	}
    }
}





#[hdk_entry(id = "dna_version_entry", visibility="public")]
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
    pub version: u64,
    pub published_at: u64,
    pub file_size: u64,
}

// Full
#[derive(Debug, Serialize, Deserialize)]
pub struct DnaVersionInfo {
    pub for_dna: DnaSummary,
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




#[hdk_entry(id = "dna_chunk_entry", visibility="public")]
pub struct DnaChunkEntry {
    pub sequence: SequencePosition,
    pub bytes: SerializedBytes,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SequencePosition {
    pub position: u64,
    pub length: u64,
}



#[cfg(test)]
pub mod tests {
    use super::*;

    fn create_dnaentry() -> crate::DnaEntry {
	crate::DnaEntry {
	    name: String::from("Game Turns"),
	    description: String::from("A tool for turn-based games to track the order of player actions"),
	    published_at: 1618855430,

	    // optional
	    developer: Some(EntityInfo {
		name: String::from("Open Games Collective"),
		website: Some(String::from("https://github.com/open-games-collective/")),
	    }),
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
