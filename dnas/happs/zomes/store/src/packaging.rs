use std::collections::BTreeMap;

use devhub_types::{
    DevHubResponse, AppResult,
    errors::{ AppError },
    dnarepo_entry_types::{ DnaVersionPackage },
    web_asset_entry_types::{ FileInfo },
    call_local_dna_zome,
    encode_bundle,
};
use holo_hash::{ DnaHash };
use hc_entities::{ Entity, GetEntityInput };
use hdk::prelude::*;



#[derive(Debug, Deserialize)]
pub struct GetGUIInput {
    pub id: EntryHash,
    pub dna_hash: DnaHash,
}

pub fn get_gui(input: GetGUIInput) -> AppResult<Entity<FileInfo>> {
    debug!("Get GUI from: {}", input.id );
    let pubkey = agent_info()?.agent_initial_pubkey;

    let zome_call_response = call(
	Some( CellId::new( input.dna_hash, pubkey ) ),
	"files".into(),
	"get_file".into(),
	None,
	GetEntityInput {
	    id: input.id,
	},
    )?;

    if let ZomeCallResponse::Ok(result_io) = zome_call_response {
	let response : DevHubResponse<Entity<FileInfo>> = result_io.decode()
	    .map_err( |e| AppError::UnexpectedStateError(format!("Failed to call another DNA: {:?}", e )) )?;

	if let DevHubResponse::Success(pack) = response {
	    return Ok( pack.payload );
	}
    };

    Err( AppError::UnexpectedStateError("Failed to call another DNA".into()).into() )
}


// {
//     "manifest": {
//         "manifest_version": "1",
//         "name": "devhub",
//         "description": "Holochain App Store",
//         "slots": [
//             {
//                 "id": "file_storage",
//                 "provisioning": {
//                     "strategy": "create",
//                     "deferred": false
//                 },
//                 "dna": {
//                     "bundled": "file_storage/file_storage.dna",
//                     "properties": {
//                         "foo": 1111
//                     },
//                     "uuid": null,
//                     "version": null,
//                     "clone_limit": 10
//                 }
//             }
//         ]
//     },
//     "resources": {
//         "file_storage/file_storage.dna": <Buffer ... 779361 more bytes>
//     }
// }


#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct BundleSlotDnaProvisioning {
    pub strategy: String,
    pub deferred: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct BundleSlotDnaInfo {
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
pub struct BundleSlotInfo {
    pub id: String,
    pub dna: BundleSlotDnaInfo,

    // Optional fields
    pub provisioning: Option<BundleSlotDnaProvisioning>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Manifest {
    pub manifest_version: String,
    pub slots: Vec<BundleSlotInfo>,

    // Optional fields
    pub name: Option<String>,
    pub description: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Bundle {
    pub manifest: Manifest,
    pub resources: BTreeMap<String, Vec<u8>>,
}



#[derive(Debug, Deserialize)]
pub struct GetReleasePackageInput {
    pub id: EntryHash,
    pub dnarepo_dna_hash: DnaHash,
}

pub fn get_release_package(input: GetReleasePackageInput) -> AppResult<Vec<u8>> {
    debug!("Get release package: {}", input.id );
    let pubkey = agent_info()?.agent_initial_pubkey;
    let cell_id = CellId::new( input.dnarepo_dna_hash, pubkey );

    let entity = crate::happ_release::get_happ_release(GetEntityInput {
	id: input.id,
    })?;

    debug!("Manifest YAML: {}", entity.content.manifest_yaml );
    let mut package = Bundle {
	manifest: serde_yaml::from_str( &entity.content.manifest_yaml )
	    .map_err( |e| AppError::UnexpectedStateError(format!("Failed to parse YAML: {:?}", e )) )?,
	resources: BTreeMap::new(),
    };

    for slot in package.manifest.slots.iter_mut() {
	slot.dna.bundled = format!("./{}.dna", slot.id );
    }

    debug!("Fetching DNA package for {} resources", entity.content.resources.len() );
    for (slot_id, version_entry_hash) in entity.content.resources.iter() {
	debug!("Fetching DNA package: {}", version_entry_hash );

	let version_entity : Entity<DnaVersionPackage> = call_local_dna_zome( &cell_id, "storage", "get_dna_package", GetEntityInput {
	    id: version_entry_hash.to_owned(),
	})?;

	let path = format!("./{}.dna", slot_id );

	debug!("Adding resource pack '{}' with {} bytes", path, version_entity.content.bytes.len() );
	package.resources.insert( path, version_entity.content.bytes );
    }
    debug!("Finished collecting DNAs for package with {} resources: {:?}", package.resources.len(), package.resources.clone().into_iter().map( |(k,v)| (k, v.len()) ).collect::<Vec<(String, usize)>>() );

    let happ_pack_bytes = encode_bundle( package )?;

    Ok( happ_pack_bytes )
}
