
use std::collections::HashMap;
use hdk::prelude::*;
// use hdk::hash_path::path::Component;


#[hdk_entry(id = "app_entry", visibility="public")]
#[derive(Clone)]
pub struct AppEntry {
    pub title: String,
    pub subtitle: String,
    pub description: String,
    pub thumbnail_image: SerializedBytes,
    pub published_at: u64,
    pub architect: SerializedBytes, //AgentPubKey,
    pub maintained_by: EntityInfo,
    pub categories: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EntityInfo {
    pub name: String,

    // optional
    pub website: Option<String>,
}

// Summary
#[derive(Debug, Serialize, Deserialize)]
pub struct AppSummary {
    pub title: String,
    pub subtitle: String,
    pub thumbnail_image: SerializedBytes,
    pub published_at: u64,
    pub architect: SerializedBytes, //AgentPubKey,
    pub categories: Vec<String>,
}

// Full
#[derive(Debug, Serialize, Deserialize)]
pub struct AppInfo {
    pub title: String,
    pub subtitle: String,
    pub description: String,
    pub thumbnail_image: SerializedBytes,
    pub published_at: u64,
    pub architect: SerializedBytes, //AgentPubKey,
    pub maintained_by: EntityInfo,
    pub categories: Vec<String>,
}




#[hdk_entry(id = "manifest_entry", visibility="public")]
pub struct ManifestEntry {
    pub for_happ: EntryHash,
    pub name: String,
    pub description: String,
    pub manifest_version: u64,
    pub published_at: u64,
    pub cells: Vec<CellSlot>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CellSlot {
    pub nick: String,
    pub dna: CellSlotDnaConfig,

    // optional
    pub provisioning: Option<ProvisioningConfig>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct CellSlotSummary {
    pub nick: String,
    pub dna: CellSlotDnaConfigSummary,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProvisioningConfig {
    pub strategy: String,

    // optional
    pub deferred: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CellSlotDnaConfig {
    pub entry_id: EntryHash,
    pub overrideable: bool,

    // optional
    pub url: Option<String>,
    pub uuid: Option<String>,
    pub version: Option<Vec<EntryHash>>,
    pub clone_limit: Option<u64>,
    pub properties: Option<HashMap<String,String>>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct CellSlotDnaConfigSummary {
    pub entry_id: EntryHash,
}

// Summary
#[derive(Debug, Serialize, Deserialize)]
pub struct ManifestSummary {
    pub name: String,
    pub description: String,
    pub manifest_version: u64,
    pub published_at: u64,
    pub cells: Vec<CellSlotSummary>
}

// Full
#[derive(Debug, Serialize, Deserialize)]
pub struct ManifestInfo {
    pub for_happ: AppSummary,
    pub name: String,
    pub description: String,
    pub manifest_version: u64,
    pub published_at: u64,
    pub cells: Vec<CellSlot>
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
    pub developer: EntityInfo,

    // optional
    pub deprecation: Option<DeprecationNotice>,
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


impl AppEntry {
    fn to_info(self) -> AppInfo {
	self.into()
    }

    fn to_summary(self) -> AppSummary {
	self.into()
    }
}

impl From<AppEntry> for AppInfo {
    fn from(app: AppEntry) -> AppInfo {
	AppInfo {
	    title: app.title,
	    subtitle: app.subtitle,
	    description: app.description,
	    thumbnail_image: app.thumbnail_image,
	    published_at: app.published_at,
	    architect: app.architect,
	    maintained_by: app.maintained_by,
	    categories: app.categories,
	}
    }
}

impl From<AppEntry> for AppSummary {
    fn from(app: AppEntry) -> AppSummary {
	AppSummary {
	    title: app.title,
	    subtitle: app.subtitle,
	    thumbnail_image: app.thumbnail_image,
	    published_at: app.published_at,
	    architect: app.architect,
	    categories: app.categories,
	}
    }
}


#[cfg(test)]
pub mod tests {
    use super::*;

    fn create_appentry() -> crate::AppEntry {
	crate::AppEntry {
	    title: String::from("Spider Solitaire"),
	    subtitle: String::from("The popular classic card game"),
	    description: String::from("Play the #1 classic Spider Solitaire for Free! ..."),
	    thumbnail_image: vec![1,2,3,4],
	    published_at: 1618855430,
	    architect: vec![ // AgentPubKey
		222, 230,  91, 220,  87,
		73, 244, 141, 250,  32, 140, 128, 205,
		112, 181, 107,  91, 249, 202,  54, 137,
		100, 234, 127, 172, 207,  41, 187, 205,
		51, 186,  86
	    ],
	    maintained_by: EntityInfo {
		name: String::from("Open Games Collective"),
		website: Some(String::from("https://open-games.example")),
	    },
	    categories: vec![],
	}
    }

    #[test]
    ///
    fn app_to_summary_test() {
	let app1 = create_appentry();
	let app2 = create_appentry();

	assert_eq!(app1.title, "Spider Solitaire");

	let app_info = app1.to_info();

	assert_eq!(app_info.title, "Spider Solitaire");

	let app_summary = app2.to_summary();

	assert_eq!(app_summary.title, "Spider Solitaire");
    }
}
