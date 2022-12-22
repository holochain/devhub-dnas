use std::collections::BTreeMap;
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

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct HoloGUIConfig {
    pub uses_web_sdk: bool,
}
impl HoloGUIConfig {
    pub fn default() -> Self {
	HoloGUIConfig {
	    uses_web_sdk: false,
	}
    }
}



//
// Manifests
//

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
pub struct RoleDnaInfo {
    #[serde(alias = "path", alias = "url")]
    pub bundled: String,
    #[serde(default)]
    pub clone_limit: u32,

    // Optional fields
    pub modifiers: Option<DnaModifiersOpt<serde_yaml::Value>>,
    pub version: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RoleProvisioning {
    pub strategy: String,
    pub deferred: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RoleInfo {
    pub name: String,
    pub dna: RoleDnaInfo,

    // Optional fields
    pub provisioning: Option<RoleProvisioning>,
}

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




//
// Happ Entry
//
#[hdk_entry_helper]
#[derive(Clone)]
pub struct HappEntry {
    pub title: String,
    pub subtitle: String,
    pub description: String,
    pub designer: AgentPubKey,
    pub published_at: u64,
    pub last_updated: u64,
    pub metadata: BTreeMap<String, serde_yaml::Value>,

    // optional
    pub tags: Option<Vec<String>>,
    pub icon: Option<SerializedBytes>,
    pub deprecation: Option<DeprecationNotice>,
}



//
// Happ Release Entry
//
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct DnaReference {
    pub role_name: String,
    pub dna : EntryHash, // Dna ID
    pub version : EntryHash, // Version ID
    pub wasm_hash : String,
}

#[hdk_entry_helper]
#[derive(Clone)]
pub struct HappReleaseEntry {
    pub name: String,
    pub description: String,
    pub for_happ: EntryHash,
    pub ordering: u64,
    pub published_at: u64,
    pub last_updated: u64,
    pub manifest: HappManifest,
    pub dna_hash : String,
    pub hdk_version: String,
    pub dnas: Vec<DnaReference>,
    pub metadata: BTreeMap<String, serde_yaml::Value>,

    // Optional fields
    pub official_gui: Option<EntryHash>,
}



//
// GUI Entry
//
#[hdk_entry_helper]
#[derive(Clone)]
pub struct GUIEntry {
    pub name: String,
    pub description: String,
    pub designer: AgentPubKey,
    pub published_at: u64,
    pub last_updated: u64,
    pub holo_hosting_settings: HoloGUIConfig,
    pub metadata: BTreeMap<String, serde_yaml::Value>,

    // optional
    pub tags: Option<Vec<String>>,
    pub screenshots: Option<Vec<EntryHash>>,
    pub deprecation: Option<DeprecationNotice>,
}



//
// GUI Release Entry
//
#[hdk_entry_helper]
#[derive(Clone)]
pub struct GUIReleaseEntry {
    pub version: String,
    pub changelog: String,
    pub for_gui: EntryHash,
    pub for_happ_releases: Vec<EntryHash>,
    pub web_asset_id: EntryHash,
    pub published_at: u64,
    pub last_updated: u64,
    pub metadata: BTreeMap<String, serde_yaml::Value>,

    // Optional fields
    pub screenshots: Option<Vec<EntryHash>>,
    // pub dna_support: Option<Vec<EntryHash>>, // list of DnaEntry IDs of intended support, does not mean they are guaranteed to work for all those DNA's versions
}
