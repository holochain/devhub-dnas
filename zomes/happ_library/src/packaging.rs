use std::collections::BTreeMap;

use devhub_types::{
    DevHubResponse, AppResult,
    errors::{ AppError },
    dnarepo_entry_types::{ DnaVersionPackage },
    happ_entry_types::{ HappManifest },
    web_asset_entry_types::{ FileInfo },
    call_local_dna_zome,
    encode_bundle,
};
use hc_crud::{
    Entity, GetEntityInput,
};
use hdk::prelude::*;



#[derive(Debug, Deserialize)]
pub struct GetGUIInput {
    pub id: EntryHash,
    pub dna_hash: holo_hash::DnaHash,
}

pub fn get_gui(input: GetGUIInput) -> AppResult<Entity<FileInfo>> {
    debug!("Get GUI from: {}", input.id );
    let pubkey = agent_info()?.agent_initial_pubkey;

    let zome_call_response = call(
	Some( CellId::new( input.dna_hash, pubkey ) ),
	"web_assets".into(),
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
pub struct Bundle {
    pub manifest: HappManifest,
    pub resources: BTreeMap<String, Vec<u8>>,
}



#[derive(Debug, Deserialize)]
pub struct GetReleasePackageInput {
    pub id: EntryHash,
    pub dnarepo_dna_hash: holo_hash::DnaHash,
}

pub fn get_release_package(input: GetReleasePackageInput) -> AppResult<Vec<u8>> {
    debug!("Get release package: {}", input.id );
    let pubkey = agent_info()?.agent_initial_pubkey;
    let cell_id = CellId::new( input.dnarepo_dna_hash, pubkey );

    let entity = crate::happ_release::get_happ_release(GetEntityInput {
	id: input.id,
    })?;

    let mut resources : BTreeMap<String, Vec<u8>> = BTreeMap::new();

    debug!("Fetching DNA package for {} resources", entity.content.dnas.len() );
    for dna_ref in entity.content.dnas.iter() {
	debug!("Fetching DNA package: {}", dna_ref.version );

	let version_entity : Entity<DnaVersionPackage> = call_local_dna_zome( &cell_id, "dna_library", "get_dna_package", GetEntityInput {
	    id: dna_ref.version.to_owned(),
	})?;

	let path = format!("./{}.dna", dna_ref.name );

	debug!("Adding resource pack '{}' with {} bytes", path, version_entity.content.bytes.len() );
	resources.insert( path, version_entity.content.bytes );
    }
    debug!("Finished collecting DNAs for package with {} resources: {:?}", resources.len(), resources.clone().into_iter().map( |(k,v)| (k, v.len()) ).collect::<Vec<(String, usize)>>() );

    debug!("Manifest: {:?}", entity.content.manifest );
    let mut package = Bundle {
	manifest: entity.content.manifest,
	resources: resources,
    };

    for slot in package.manifest.slots.iter_mut() {
	slot.dna.bundled = format!("./{}.dna", slot.id );
    }

    let happ_pack_bytes = encode_bundle( package )?;

    Ok( happ_pack_bytes )
}
