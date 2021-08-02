use hc_entities::{ EntryModel, EntityType, Entity };
use hdk::prelude::*;
use hc_dna_utils as utils;


#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DeprecationNotice {
    pub message: String,

    // optional
    pub recommended_alternatives: Option<Vec<EntryHash>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct HoloGUIConfig {
    pub uses_web_sdk: bool,
    pub approved: bool,
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
		approved: false,
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

    // optional
    pub thumbnail_image: Option<SerializedBytes>,
    pub deprecation: Option<DeprecationNotice>,
    pub gui: Option<HappGUIConfig>,
}
utils::try_from_element![ HappEntry ];

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

    // optional
    pub thumbnail_image: Option<SerializedBytes>,
    pub deprecation: bool,
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

    // optional
    pub thumbnail_image: Option<SerializedBytes>,
    pub deprecation: Option<DeprecationNotice>,
    pub gui: Option<HappGUIConfig>,
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
	    thumbnail_image: self.thumbnail_image.clone(),
	    deprecation: self.deprecation.clone(),
	    gui: self.gui.clone(),
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
	    thumbnail_image: self.thumbnail_image.clone(),
	    deprecation: self.deprecation.clone().map_or(false, |_| true),
	}
    }
}



//
// Happ Release Entry
//
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SlotProvisioning {
    pub strategy: String,
    pub deferred: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SlotDnaInfo {
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
pub struct SlotInfo {
    pub id: String,
    pub dna: SlotDnaInfo,

    // Optional fields
    pub provisioning: Option<SlotProvisioning>,
}

// {
//     "manifest_version": "1",
//     "name": "devhub",
//     "description": "Holochain App Store",
//     "slots": [
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
    pub slots: Vec<SlotInfo>,

    // Optional fields
    pub name: Option<String>,
    pub description: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DnaReference {
    pub name: String,
    pub dna : EntryHash, // Dna ID
    pub version : EntryHash, // Version ID
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
    pub dnas: Vec<DnaReference>,
}
utils::try_from_element![ HappReleaseEntry ];

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
    pub dnas: Vec<DnaReference>,
}
impl EntryModel for HappReleaseInfo {
    fn get_type(&self) -> EntityType {
	EntityType::new( "happ_release", "info" )
    }
}

impl HappReleaseEntry {
    pub fn to_info(&self) -> HappReleaseInfo {
	let mut happ_entity : Option<Entity<HappSummary>> = None;

	if let Some(entity) = utils::get_entity( &self.for_happ ).ok() {
	    if let Some(happ_entry) = HappEntry::try_from( &entity.content ).ok() {
		happ_entity = Some( entity.new_content( happ_entry.to_summary() ) );
	    }
	};

	HappReleaseInfo {
	    name: self.name.clone(),
	    description: self.description.clone(),
	    for_happ: happ_entity,
	    published_at: self.published_at.clone(),
	    last_updated: self.last_updated.clone(),
	    manifest: self.manifest.clone(),
	    dnas: self.dnas.clone(),
	}
    }

    pub fn to_summary(&self) -> HappReleaseSummary {
	HappReleaseSummary {
	    name: self.name.clone(),
	    description: self.description.clone(),
	    for_happ: self.for_happ.clone(),
	    published_at: self.published_at.clone(),
	    last_updated: self.last_updated.clone(),
	}
    }
}
