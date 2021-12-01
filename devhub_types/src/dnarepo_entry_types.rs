use hc_crud::{
    get_entity,
    EntryModel, EntityType, Entity
};
use hdk::prelude::*;



//
// General-use Structs
//
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DeveloperProfileLocation {
    pub pubkey: AgentPubKey,
}

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

// Full
#[derive(Debug, Serialize, Deserialize)]
pub struct ProfileInfo {
    pub name: String,
    pub email: String,
    pub avatar_image: SerializedBytes,
    pub website: String,
}
impl EntryModel for ProfileInfo {
    fn get_type(&self) -> EntityType {
	EntityType::new( "profile", "info" )
    }
}

impl ProfileEntry {
    pub fn to_info(&self) -> ProfileInfo {
	ProfileInfo {
	    name: self.name.clone(),
	    email: self.email.clone(),
	    website: self.website.clone(),
	    avatar_image: self.avatar_image.clone(),
	}
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
    pub developer: DeveloperProfileLocation,

    // optional
    pub icon: Option<SerializedBytes>,
    pub deprecation: Option<DeprecationNotice>,
}

impl EntryModel for DnaEntry {
    fn get_type(&self) -> EntityType {
	EntityType::new( "dna", "entry" )
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
    pub deprecation: bool,

    // optional
    pub icon: Option<SerializedBytes>,
}
impl EntryModel for DnaSummary {
    fn get_type(&self) -> EntityType {
	EntityType::new( "dna", "summary" )
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
    pub deprecation: Option<DeprecationNotice>,
}
impl EntryModel for DnaInfo {
    fn get_type(&self) -> EntityType {
	EntityType::new( "dna", "info" )
    }
}

impl DnaEntry {
    pub fn to_info(&self) -> DnaInfo {
	DnaInfo {
	    name: self.name.clone(),
	    description: self.description.clone(),
	    icon: self.icon.clone(),
	    published_at: self.published_at.clone(),
	    last_updated: self.last_updated.clone(),
	    developer: self.developer.clone(),
	    deprecation: self.deprecation.clone(),
	}
    }

    pub fn to_summary(&self) -> DnaSummary {
	DnaSummary {
	    name: self.name.clone(),
	    description: self.description.clone(),
	    icon: self.icon.clone(),
	    published_at: self.published_at.clone(),
	    last_updated: self.last_updated.clone(),
	    developer: self.developer.pubkey.clone(),
	    deprecation: self.deprecation.clone().map_or(false, |_| true),
	}
    }
}



//
// DNA Version Entry
//
#[derive(Debug, Serialize, Deserialize, Clone)]
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
    pub version: u64,
    pub published_at: u64,
    pub last_updated: u64,
    pub changelog: String,
    pub wasm_hash : String,
    // pub properties: Option<serde_yaml::Value>, // does this make sense?  Intended as a DNA's default properties?
    pub zomes: Vec<ZomeReference>,
}

impl EntryModel for DnaVersionEntry {
    fn get_type(&self) -> EntityType {
	EntityType::new( "dna_version", "entry" )
    }
}

// Summary
#[derive(Debug, Serialize, Deserialize)]
pub struct DnaVersionSummary {
    pub version: u64,
    pub published_at: u64,
    pub last_updated: u64,
    pub wasm_hash : String,
    pub zomes: Vec<EntryHash>,
}
impl EntryModel for DnaVersionSummary {
    fn get_type(&self) -> EntityType {
	EntityType::new( "dna_version", "summary" )
    }
}

// Full
#[derive(Debug, Serialize, Deserialize)]
pub struct DnaVersionInfo {
    pub for_dna: Option<Entity<DnaSummary>>,
    pub version: u64,
    pub published_at: u64,
    pub last_updated: u64,
    pub changelog: String,
    pub wasm_hash : String,
    pub zomes: Vec<ZomeReference>,
}
impl EntryModel for DnaVersionInfo {
    fn get_type(&self) -> EntityType {
	EntityType::new( "dna_version", "info" )
    }
}

// Package
#[derive(Debug, Serialize, Deserialize)]
pub struct DnaVersionPackage {
    pub for_dna: Option<Entity<DnaSummary>>,
    pub version: u64,
    pub published_at: u64,
    pub last_updated: u64,
    pub changelog: String,
    pub bytes: Vec<u8>,
}
impl EntryModel for DnaVersionPackage {
    fn get_type(&self) -> EntityType {
	EntityType::new( "dna_version", "package" )
    }
}

impl DnaVersionEntry {
    pub fn to_package(&self, dna_bytes: Vec<u8>) -> DnaVersionPackage {
	let mut dna_entity : Option<Entity<DnaSummary>> = None;

	if let Some(entity) = get_entity::<DnaEntry>( &self.for_dna ).ok() {
	    dna_entity = Some( entity.change_model(
		|version| version.to_summary()
	    ));
	};

	DnaVersionPackage {
	    for_dna: dna_entity,
	    version: self.version.clone(),
	    published_at: self.published_at.clone(),
	    last_updated: self.last_updated.clone(),
	    changelog: self.changelog.clone(),
	    bytes: dna_bytes,
	}
    }

    pub fn to_info(&self) -> DnaVersionInfo {
	let mut dna_entity : Option<Entity<DnaSummary>> = None;

	if let Some(entity) = get_entity::<DnaEntry>( &self.for_dna ).ok() {
	    dna_entity = Some( entity.change_model(
		|version| version.to_summary()
	    ));
	};

	DnaVersionInfo {
	    for_dna: dna_entity,
	    version: self.version.clone(),
	    published_at: self.published_at.clone(),
	    last_updated: self.last_updated.clone(),
	    changelog: self.changelog.clone(),
	    wasm_hash: self.wasm_hash.clone(),
	    zomes: self.zomes.clone(),
	}
    }

    pub fn to_summary(&self) -> DnaVersionSummary {
	DnaVersionSummary {
	    version: self.version.clone(),
	    published_at: self.published_at.clone(),
	    last_updated: self.last_updated.clone(),
	    wasm_hash: self.wasm_hash.clone(),
	    zomes: self.zomes.clone().into_iter()
		.map( |zome_ref| zome_ref.resource )
		.collect(),
	}
    }
}



//
// ZOME Entry
//
#[hdk_entry(id = "zome", visibility="public")]
#[derive(Clone)]
pub struct ZomeEntry {
    pub name: String,
    pub description: String,
    pub published_at: u64,
    pub last_updated: u64,
    pub developer: DeveloperProfileLocation,

    // optional
    pub deprecation: Option<DeprecationNotice>,
}

impl EntryModel for ZomeEntry {
    fn get_type(&self) -> EntityType {
	EntityType::new( "zome", "entry" )
    }
}

// Summary
#[derive(Debug, Serialize, Deserialize)]
pub struct ZomeSummary {
    pub name: String,
    pub description: String,
    pub published_at: u64,
    pub last_updated: u64,
    pub developer: AgentPubKey,
    pub deprecation: bool,
}
impl EntryModel for ZomeSummary {
    fn get_type(&self) -> EntityType {
	EntityType::new( "zome", "summary" )
    }
}

// Full
#[derive(Debug, Serialize, Deserialize)]
pub struct ZomeInfo {
    pub name: String,
    pub description: String,
    pub published_at: u64,
    pub last_updated: u64,
    pub developer: DeveloperProfileLocation,

    // optional
    pub deprecation: Option<DeprecationNotice>,
}
impl EntryModel for ZomeInfo {
    fn get_type(&self) -> EntityType {
	EntityType::new( "zome", "info" )
    }
}

impl ZomeEntry {
    pub fn to_info(&self) -> ZomeInfo {
	ZomeInfo {
	    name: self.name.clone(),
	    description: self.description.clone(),
	    published_at: self.published_at.clone(),
	    last_updated: self.last_updated.clone(),
	    developer: self.developer.clone(),
	    deprecation: self.deprecation.clone(),
	}
    }

    pub fn to_summary(&self) -> ZomeSummary {
	ZomeSummary {
	    name: self.name.clone(),
	    description: self.description.clone(),
	    published_at: self.published_at.clone(),
	    last_updated: self.last_updated.clone(),
	    developer: self.developer.pubkey.clone(),
	    deprecation: self.deprecation.clone().map_or(false, |_| true),
	}
    }
}



//
// Zome Version Entry
//
#[hdk_entry(id = "zome_version", visibility="public")]
#[derive(Clone)]
pub struct ZomeVersionEntry {
    pub for_zome: EntryHash,
    pub version: u64,
    // pub properties: Option<serde_yaml::Value>,
    pub published_at: u64,
    pub last_updated: u64,
    pub changelog: String,
    pub mere_memory_addr: EntryHash,
    pub mere_memory_hash: String,
}

impl EntryModel for ZomeVersionEntry {
    fn get_type(&self) -> EntityType {
	EntityType::new( "zome_version", "entry" )
    }
}

// Summary
#[derive(Debug, Serialize, Deserialize)]
pub struct ZomeVersionSummary {
    pub version: u64,
    pub published_at: u64,
    pub last_updated: u64,
    pub mere_memory_addr: EntryHash,
    pub mere_memory_hash: String,
}
impl EntryModel for ZomeVersionSummary {
    fn get_type(&self) -> EntityType {
	EntityType::new( "zome_version", "summary" )
    }
}

// Full
#[derive(Debug, Serialize, Deserialize)]
pub struct ZomeVersionInfo {
    pub for_zome: Option<Entity<ZomeSummary>>,
    pub version: u64,
    pub published_at: u64,
    pub last_updated: u64,
    pub changelog: String,
    pub mere_memory_addr: EntryHash,
    pub mere_memory_hash: String,
}
impl EntryModel for ZomeVersionInfo {
    fn get_type(&self) -> EntityType {
	EntityType::new( "zome_version", "info" )
    }
}

impl ZomeVersionEntry {
    pub fn to_info(&self) -> ZomeVersionInfo {
	let mut zome_entity : Option<Entity<ZomeSummary>> = None;

	if let Some(entity) = get_entity::<ZomeEntry>( &self.for_zome ).ok() {
	    zome_entity = Some( entity.change_model(
		|version| version.to_summary()
	    ));
	};

	ZomeVersionInfo {
	    for_zome: zome_entity,
	    version: self.version.clone(),
	    published_at: self.published_at.clone(),
	    last_updated: self.last_updated.clone(),
	    changelog: self.changelog.clone(),
	    mere_memory_addr: self.mere_memory_addr.clone(),
	    mere_memory_hash: self.mere_memory_hash.clone(),
	}
    }

    pub fn to_summary(&self) -> ZomeVersionSummary {
	ZomeVersionSummary {
	    version: self.version.clone(),
	    published_at: self.published_at.clone(),
	    last_updated: self.last_updated.clone(),
	    mere_memory_addr: self.mere_memory_addr.clone(),
	    mere_memory_hash: self.mere_memory_hash.clone(),
	}
    }
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

    fn create_dnaentry() -> DnaEntry {
	let bytes = rand::thread_rng().gen::<[u8; 32]>();
	let hash = EntryHash::from_raw_32( bytes.to_vec() );

	DnaEntry {
	    name: String::from("Game Turns"),
	    description: String::from("A tool for turn-based games to track the order of player actions"),
	    icon: Some(SerializedBytes::try_from(()).unwrap()),
	    published_at: 1618855430,
	    last_updated: 1618855430,

	    // optional
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
