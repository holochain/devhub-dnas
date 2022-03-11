use std::collections::BTreeMap;

use devhub_types::{
    AppResult, GetEntityInput,
    errors::{ AppError },
    dnarepo_entry_types::{ DnaVersionPackage },
    happ_entry_types::{ HappManifest, WebHappManifest, ResourceRef },
    web_asset_entry_types::{ FileInfo },
    call_local_dna_zome,
    encode_bundle,
};
use hc_crud::{
    Entity,
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
    let cell_id = CellId::new( input.dna_hash, pubkey );

    let file_info = call_local_dna_zome( &cell_id, "web_assets", "get_file", GetEntityInput {
	id: input.id,
    })?;

    Ok( file_info )
}


// {
//     "manifest": {
//         "manifest_version": "1",
//         "name": "devhub",
//         "description": "Holochain App Store",
//         "roles": [
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

	let path = format!("./{}.dna", dna_ref.role_id );

	debug!("Adding resource pack '{}' with {} bytes", path, version_entity.content.bytes.len() );
	resources.insert( path, version_entity.content.bytes );
    }
    debug!("Finished collecting DNAs for package with {} resources: {:?}", resources.len(), resources.clone().into_iter().map( |(k,v)| (k, v.len()) ).collect::<Vec<(String, usize)>>() );

    debug!("Manifest: {:?}", entity.content.manifest );
    let mut package = Bundle {
	manifest: entity.content.manifest,
	resources: resources,
    };

    for role in package.manifest.roles.iter_mut() {
	role.dna.bundled = format!("./{}.dna", role.id );
    }

    let happ_pack_bytes = encode_bundle( package )?;

    Ok( happ_pack_bytes )
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
pub struct WebHappBundle {
    pub manifest: WebHappManifest,
    pub resources: BTreeMap<String, Vec<u8>>,
}
#[derive(Debug, Deserialize)]
pub struct GetWebHappPackageInput {
    pub name: String,
    pub id: EntryHash,
    pub dnarepo_dna_hash: holo_hash::DnaHash,
    pub webassets_dna_hash: holo_hash::DnaHash,
}
pub fn get_webhapp_package(input: GetWebHappPackageInput) -> AppResult<Vec<u8>> {
    let happ_release = crate::happ_release::get_happ_release(GetEntityInput {
	id: input.id.clone(),
    })?;

    debug!("Get release package: {}", input.id );
    let happ_pack_bytes = get_release_package(GetReleasePackageInput {
	id: input.id.clone(),
	dnarepo_dna_hash: input.dnarepo_dna_hash.clone(),
    })?;

    let _ui_bytes = get_gui(GetGUIInput {
	id: happ_release.content.gui.ok_or(AppError::UnexpectedStateError(String::from("Missing GUI asset")))?.asset_group_id,
	dna_hash: input.webassets_dna_hash,
    })?;

    let mut resources : BTreeMap<String, Vec<u8>> = BTreeMap::new();

    // add UI resource
    let ui_ref = String::from("./ui.zip");
    debug!("Adding UI resource with {} bytes", 0 );
    resources.insert( ui_ref.clone(), vec![] );

    // add hApp bundle resource
    let happ_ref = String::from("./bundle.happ");
    debug!("Adding hApp bundle resource with {} bytes", happ_pack_bytes.len() );
    resources.insert( happ_ref.clone(), happ_pack_bytes );

    debug!("Assembling 'webhapp' package: {:?}", happ_release.content.for_happ.ok_or(AppError::UnexpectedStateError(String::from("Missing parent hApp")))?.content.title );
    let package = WebHappBundle {
	manifest: WebHappManifest {
	    manifest_version: String::from("1"),
	    name: input.name,
	    ui: ResourceRef {
		bundled: ui_ref,
	    },
	    happ_manifest: ResourceRef {
		bundled: happ_ref,
	    },
	},
	resources: resources,
    };

    let happ_pack_bytes = encode_bundle( package )?;

    Ok( happ_pack_bytes )
}
