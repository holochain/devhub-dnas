use std::collections::HashMap;
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
    pub metadata: HashMap<String, serde_yaml::Value>,

    // optional
    pub tags: Option<Vec<String>>,
    pub icon: Option<SerializedBytes>,
    pub deprecation: Option<DeprecationNotice>,
}

impl EntryModel for DnaEntry {
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
    pub metadata: HashMap<String, serde_yaml::Value>,

    // optional
    pub tags: Option<Vec<String>>,
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
	    tags: self.tags.clone(),
	    published_at: self.published_at.clone(),
	    last_updated: self.last_updated.clone(),
	    developer: self.developer.clone(),
	    deprecation: self.deprecation.clone(),
	    metadata: self.metadata.clone(),
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
    pub hdk_version: String,
    pub zomes: Vec<ZomeReference>,
    pub metadata: HashMap<String, serde_yaml::Value>,
}

impl EntryModel for DnaVersionEntry {
    fn get_type(&self) -> EntityType {
	EntityType::new( "dna_version", "summary" )
    }
}

// Full
#[derive(Debug, Serialize, Deserialize)]
pub struct DnaVersionInfo {
    pub for_dna: Option<Entity<DnaEntry>>,
    pub version: u64,
    pub published_at: u64,
    pub last_updated: u64,
    pub changelog: String,
    pub wasm_hash : String,
    pub hdk_version: String,
    pub zomes: HashMap<String, Entity<ZomeVersionEntry>>,
    pub metadata: HashMap<String, serde_yaml::Value>,
}
impl EntryModel for DnaVersionInfo {
    fn get_type(&self) -> EntityType {
	EntityType::new( "dna_version", "info" )
    }
}

// Package
#[derive(Debug, Serialize, Deserialize)]
pub struct DnaVersionPackage {
    pub for_dna: Option<Entity<DnaEntry>>,
    pub version: u64,
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
	let dna_entity = get_entity::<DnaEntry>( &self.for_dna ).ok();

	DnaVersionPackage {
	    for_dna: dna_entity,
	    version: self.version.clone(),
	    published_at: self.published_at.clone(),
	    last_updated: self.last_updated.clone(),
	    changelog: self.changelog.clone(),
	    hdk_version: self.hdk_version.clone(),
	    bytes: dna_bytes,
	}
    }

    pub fn to_info(&self) -> DnaVersionInfo {
	let dna_entity = get_entity::<DnaEntry>( &self.for_dna ).ok();

	DnaVersionInfo {
	    for_dna: dna_entity,
	    version: self.version.clone(),
	    published_at: self.published_at.clone(),
	    last_updated: self.last_updated.clone(),
	    changelog: self.changelog.clone(),
	    wasm_hash: self.wasm_hash.clone(),
	    hdk_version: self.hdk_version.clone(),
	    zomes: self.zomes.iter()
		.filter_map( |zome_ref| {
		    get_entity::<ZomeVersionEntry>( &zome_ref.version ).ok().map( |entity| {
			( zome_ref.name.clone(), entity )
		    })
		})
		.collect(),
	    metadata: self.metadata.clone(),
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
    pub metadata: HashMap<String, serde_yaml::Value>,

    // optional
    pub tags: Option<Vec<String>>,
    pub deprecation: Option<DeprecationNotice>,
}

impl EntryModel for ZomeEntry {
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
    pub metadata: HashMap<String, serde_yaml::Value>,

    // optional
    pub tags: Option<Vec<String>>,
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
	    metadata: self.metadata.clone(),
	    tags: self.tags.clone(),
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
    pub hdk_version: String,
    pub metadata: HashMap<String, serde_yaml::Value>,
}

impl EntryModel for ZomeVersionEntry {
    fn get_type(&self) -> EntityType {
	EntityType::new( "zome_version", "summary" )
    }
}

// Full
#[derive(Debug, Serialize, Deserialize)]
pub struct ZomeVersionInfo {
    pub for_zome: Option<Entity<ZomeEntry>>,
    pub version: u64,
    pub published_at: u64,
    pub last_updated: u64,
    pub changelog: String,
    pub mere_memory_addr: EntryHash,
    pub mere_memory_hash: String,
    pub hdk_version: String,
    pub metadata: HashMap<String, serde_yaml::Value>,
}
impl EntryModel for ZomeVersionInfo {
    fn get_type(&self) -> EntityType {
	EntityType::new( "zome_version", "info" )
    }
}

impl ZomeVersionEntry {
    pub fn to_info(&self) -> ZomeVersionInfo {
	let zome_entity = get_entity::<ZomeEntry>( &self.for_zome ).ok();

	ZomeVersionInfo {
	    for_zome: zome_entity,
	    version: self.version.clone(),
	    published_at: self.published_at.clone(),
	    last_updated: self.last_updated.clone(),
	    changelog: self.changelog.clone(),
	    mere_memory_addr: self.mere_memory_addr.clone(),
	    mere_memory_hash: self.mere_memory_hash.clone(),
	    hdk_version: self.hdk_version.clone(),
	    metadata: self.metadata.clone(),
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
	    tags: Some(vec![ String::from("Games") ]),
	    published_at: 1618855430,
	    last_updated: 1618855430,

	    // optional
	    developer: DeveloperProfileLocation {
		pubkey: hash.into(),
	    },
	    deprecation: None,
	    metadata: HashMap::new(),
	}
    }

    #[test]
    ///
    fn dna_to_info_test() {
	let dna1 = create_dnaentry();
	let dna2 = create_dnaentry();

	assert_eq!(dna1.name, "Game Turns");

	let dna_info = dna1.to_info();

	assert_eq!(dna_info.name, "Game Turns");

	let dna_info = dna2.to_info();

	assert_eq!(dna_info.name, "Game Turns");
    }
}
