use std::collections::HashMap;
use hc_crud::{
    get_entity,
    EntryModel, EntityType, Entity
};
use hdk::prelude::*;


#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DeprecationNotice {
    pub message: String,

    // optional
    pub recommended_alternatives: Option<Vec<EntryHash>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct HoloGUIConfig {
    pub uses_web_sdk: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct HappGUIConfig {
    pub asset_group_id: EntryHash,
    pub holo_hosting_settings: HoloGUIConfig,
}

impl HappGUIConfig {
    pub fn new(asset_group_id: EntryHash, uses_web_sdk: bool) -> Self {
	HappGUIConfig {
	    asset_group_id: asset_group_id,
	    holo_hosting_settings: HoloGUIConfig {
		uses_web_sdk: uses_web_sdk,
	    }
	}
    }
}


//
// Happ Entry
//
#[hdk_entry(id = "happ_details", visibility="public")]
#[derive(Clone)]
pub struct HappEntry {
    pub title: String,
    pub subtitle: String,
    pub description: String,
    pub designer: AgentPubKey,
    pub published_at: u64,
    pub last_updated: u64,
    pub metadata: HashMap<String, serde_yaml::Value>,

    // optional
    pub icon: Option<SerializedBytes>,
    pub deprecation: Option<DeprecationNotice>,
}

impl EntryModel for HappEntry {
    fn get_type(&self) -> EntityType {
	EntityType::new( "happ", "entry" )
    }
}

// Summary
#[derive(Debug, Serialize, Deserialize)]
pub struct HappSummary {
    pub title: String,
    pub subtitle: String,
    pub description: String,
    pub designer: AgentPubKey,
    pub published_at: u64,
    pub last_updated: u64,
    pub deprecation: bool,
    pub metadata: HashMap<String, serde_yaml::Value>,

    // optional
    pub icon: Option<SerializedBytes>,
}
impl EntryModel for HappSummary {
    fn get_type(&self) -> EntityType {
	EntityType::new( "happ", "summary" )
    }
}

// Full
#[derive(Debug, Serialize, Deserialize)]
pub struct HappInfo {
    pub title: String,
    pub subtitle: String,
    pub description: String,
    pub designer: AgentPubKey,
    pub published_at: u64,
    pub last_updated: u64,
    pub metadata: HashMap<String, serde_yaml::Value>,

    // optional
    pub icon: Option<SerializedBytes>,
    pub deprecation: Option<DeprecationNotice>,
}
impl EntryModel for HappInfo {
    fn get_type(&self) -> EntityType {
	EntityType::new( "happ", "info" )
    }
}

impl HappEntry {
    pub fn to_info(&self) -> HappInfo {
	HappInfo {
	    title: self.title.clone(),
	    subtitle: self.subtitle.clone(),
	    description: self.description.clone(),
	    designer: self.designer.clone(),
	    published_at: self.published_at.clone(),
	    last_updated: self.last_updated.clone(),
	    icon: self.icon.clone(),
	    deprecation: self.deprecation.clone(),
	    metadata: self.metadata.clone(),
	}
    }

    pub fn to_summary(&self) -> HappSummary {
	HappSummary {
	    title: self.title.clone(),
	    subtitle: self.subtitle.clone(),
	    description: self.description.clone(),
	    designer: self.designer.clone(),
	    published_at: self.published_at.clone(),
	    last_updated: self.last_updated.clone(),
	    icon: self.icon.clone(),
	    deprecation: self.deprecation.clone().map_or(false, |_| true),
	    metadata: self.metadata.clone(),
	}
    }
}



//
// Happ Release Entry
//
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RoleProvisioning {
    pub strategy: String,
    pub deferred: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RoleDnaInfo {
    #[serde(alias = "path", alias = "url")]
    pub bundled: String,
    #[serde(default)]
    pub clone_limit: u32,

    // Optional fields
    pub uid: Option<String>,
    pub version: Option<String>,
    pub properties: Option<serde_yaml::Value>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RoleInfo {
    pub id: String,
    pub dna: RoleDnaInfo,

    // Optional fields
    pub provisioning: Option<RoleProvisioning>,
}

// {
//     "manifest_version": "1",
//     "name": "devhub",
//     "description": "Holochain App Store",
//     "roles": [
//         {
//             "id": "file_storage",
//             "provisioning": {
//                 "strategy": "create",
//                 "deferred": false
//             },
//             "dna": {
//                 "bundled": "file_storage/file_storage.dna",
//                 "properties": {
//                     "foo": 1111
//                 },
//                 "uuid": null,
//                 "version": null,
//                 "clone_limit": 10
//             }
//         }
//     ]
// }
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct HappManifest {
    pub manifest_version: String,
    pub roles: Vec<RoleInfo>,

    // Optional fields
    pub name: Option<String>,
    pub description: Option<String>,
}



// {
//     "manifest": {
//         "manifest_version": "1",
//         "name": "DevHub",
//         "ui": {
//             "bundled": "../web_assets.zip"
//         },
//         "happ_manifest": {
//             "bundled": "DevHub.happ"
//         }
//     },
//     "resources": {
//         "../web_assets.zip": <Buffer 50 4b 03 04 ... 601482 more bytes>,
//         "DevHub.happ": <Buffer 1f 8b 08 00 ... 4945860 more bytes>
//     }
// }
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ResourceRef {
    pub bundled: String,
}
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct WebHappManifest {
    pub manifest_version: String,
    pub name: String,
    pub ui: ResourceRef,
    pub happ_manifest: ResourceRef,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DnaReference {
    pub role_id: String,
    pub dna : EntryHash, // Dna ID
    pub version : EntryHash, // Version ID
    pub wasm_hash : String,
}

#[hdk_entry(id = "happ_release_details", visibility="public")]
#[derive(Clone)]
pub struct HappReleaseEntry {
    pub name: String,
    pub description: String,
    pub for_happ: EntryHash,
    pub published_at: u64,
    pub last_updated: u64,
    pub manifest: HappManifest,
    pub dna_hash : String,
    pub hdk_version: String,
    pub dnas: Vec<DnaReference>,
    pub gui: Option<HappGUIConfig>,
    pub metadata: HashMap<String, serde_yaml::Value>,
}

impl EntryModel for HappReleaseEntry {
    fn get_type(&self) -> EntityType {
	EntityType::new( "happ_release", "entry" )
    }
}

// Summary
#[derive(Debug, Serialize, Deserialize)]
pub struct HappReleaseSummary {
    pub name: String,
    pub description: String,
    pub for_happ: EntryHash,
    pub published_at: u64,
    pub last_updated: u64,
    pub dna_hash : String,
    pub hdk_version: String,
    pub dnas: Vec<DnaReference>,
    pub gui: Option<HappGUIConfig>,
    pub metadata: HashMap<String, serde_yaml::Value>,
}
impl EntryModel for HappReleaseSummary {
    fn get_type(&self) -> EntityType {
	EntityType::new( "happ_release", "summary" )
    }
}

// Full
#[derive(Debug, Serialize, Deserialize)]
pub struct HappReleaseInfo {
    pub name: String,
    pub description: String,
    pub for_happ: Option<Entity<HappSummary>>,
    pub published_at: u64,
    pub last_updated: u64,
    pub manifest: HappManifest,
    pub dna_hash : String,
    pub hdk_version: String,
    pub dnas: Vec<DnaReference>,
    pub gui: Option<HappGUIConfig>,
    pub metadata: HashMap<String, serde_yaml::Value>,
}
impl EntryModel for HappReleaseInfo {
    fn get_type(&self) -> EntityType {
	EntityType::new( "happ_release", "info" )
    }
}

impl HappReleaseEntry {
    pub fn to_info(&self) -> HappReleaseInfo {
	let mut happ_entity : Option<Entity<HappSummary>> = None;

	if let Some(entity) = get_entity::<HappEntry>( &self.for_happ ).ok() {
	    happ_entity = Some( entity.change_model(
		|happ| happ.to_summary()
	    ));
	};

	HappReleaseInfo {
	    name: self.name.clone(),
	    description: self.description.clone(),
	    for_happ: happ_entity,
	    published_at: self.published_at.clone(),
	    last_updated: self.last_updated.clone(),
	    manifest: self.manifest.clone(),
	    dna_hash: self.dna_hash.clone(),
	    hdk_version: self.hdk_version.clone(),
	    dnas: self.dnas.clone(),
	    gui: self.gui.clone(),
	    metadata: self.metadata.clone(),
	}
    }

    pub fn to_summary(&self) -> HappReleaseSummary {
	HappReleaseSummary {
	    name: self.name.clone(),
	    description: self.description.clone(),
	    for_happ: self.for_happ.clone(),
	    published_at: self.published_at.clone(),
	    last_updated: self.last_updated.clone(),
	    dna_hash: self.dna_hash.clone(),
	    hdk_version: self.hdk_version.clone(),
	    dnas: self.dnas.clone(),
	    gui: self.gui.clone(),
	    metadata: self.metadata.clone(),
	}
    }
}
