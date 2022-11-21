use std::collections::BTreeMap;

use devhub_types::{
    AppResult, GetEntityInput,
    errors::{ AppError },
    dnarepo_entry_types::{ DnaVersionPackage },
    happ_entry_types::{ HappManifest, WebHappManifest, ResourceRef },
    web_asset_entry_types::{ FilePackage },
    call_local_dna_zome,
    encode_bundle,
};
use hc_crud::{
    Entity,
};
use hdk::prelude::*;



#[derive(Debug, Deserialize)]
pub struct GetWebAssetInput {
    pub id: EntryHash,
}

pub fn get_webasset(input: GetWebAssetInput) -> AppResult<Entity<FilePackage>> {
    debug!("Get GUI from: {}", input.id );
    let file_info = call_local_dna_zome( "web_assets", "web_assets", "get_file", GetEntityInput {
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
}

pub fn get_release_package(input: GetReleasePackageInput) -> AppResult<Vec<u8>> {
    debug!("Get release package: {}", input.id );
    let mut entity = crate::happ_release::get_happ_release(GetEntityInput {
	id: input.id,
    })?;

    let mut resources : BTreeMap<String, Vec<u8>> = BTreeMap::new();

    debug!("Fetching DNA package for {} resources", entity.content.dnas.len() );
    for (i, dna_ref) in entity.content.dnas.iter().enumerate() {
	debug!("Fetching DNA package: {}", dna_ref.version );

	let version_entity : Entity<DnaVersionPackage> = call_local_dna_zome( "dnarepo", "dna_library", "get_dna_package", GetEntityInput {
	    id: dna_ref.version.to_owned(),
	})?;

	let path = format!("./{}.dna", dna_ref.role_id );

	debug!("Adding resource pack '{}' with {} bytes", path, version_entity.content.bytes.len() );
	resources.insert( path.clone(), version_entity.content.bytes );
	entity.content.manifest.roles[i].dna.bundled = path.clone();
    }
    debug!("Finished collecting DNAs for package with {} resources: {:?}", resources.len(), resources.clone().into_iter().map( |(k,v)| (k, v.len()) ).collect::<Vec<(String, usize)>>() );

    debug!("Manifest: {:?}", entity.content.manifest );
    let package = Bundle {
	manifest: entity.content.manifest,
	resources: resources,
    };

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
    pub happ_release_id: EntryHash,
    pub gui_release_id: EntryHash,
}
pub fn get_webhapp_package(input: GetWebHappPackageInput) -> AppResult<Vec<u8>> {
    let gui_release = crate::gui_release::get_gui_release(GetEntityInput {
	id: input.gui_release_id.clone(),
    })?;

    let happ_release = crate::happ_release::get_happ_release(GetEntityInput {
	id: input.happ_release_id.clone(),
    })?;

    let happ_pack_bytes = get_release_package(GetReleasePackageInput {
	id: input.happ_release_id.clone(),
    })?;

    let web_asset_entity = get_webasset(GetWebAssetInput {
	id: gui_release.content.web_asset_id,
    })?;

    let mut resources : BTreeMap<String, Vec<u8>> = BTreeMap::new();

    // add UI resource
    let ui_bytes = web_asset_entity.content.bytes.ok_or(AppError::UnexpectedStateError(String::from("Missing GUI asset bytes")))?;
    let ui_ref = String::from("./ui.zip");
    debug!("Adding UI resource with {} bytes", ui_bytes.len() );
    resources.insert( ui_ref.clone(), ui_bytes );

    // add hApp bundle resource
    let happ_ref = String::from("./bundle.happ");
    debug!("Adding hApp bundle resource with {} bytes", happ_pack_bytes.len() );
    resources.insert( happ_ref.clone(), happ_pack_bytes );

    debug!("Assembling 'webhapp' package for hApp: {}", happ_release.content.for_happ );
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

// WasmError: WasmError {
//     file: "crates/holochain/src/core/ribosome/host_fn/call.rs",
//     line: 141,
//     error: Host("Wasm runtime error while working with Ribosome: RuntimeError: ")
// }
// WasmError {
//     file: "/build/cargo-vendor-dir/holochain_wasmer_host-0.0.80/src/guest.rs",
//     line: 259,
//     error: CallError("
// RuntimeError: unreachable
//     at miniz_oxide::deflate::core::compress_inner::h79f9a7ea99bb2840 (<module>[2241]:0x32b0b4)
//     at miniz_oxide::deflate::stream::deflate::h18357b407d294976 (<module>[2244]:0x32c89a)
//     at <flate2::mem::Compress as flate2::zio::Ops>::run_vec::hafa76368e22a0b14 (<module>[2210]:0x326581)
//     at <flate2::zio::Writer<W,D> as std::io::Write>::write::h3f17566e1f8bb1d7 (<module>[98]:0x7eee)
//     at std::io::Write::write_all::h4f003acf95533db1 (<module>[162]:0x17467)
//     at devhub_types::encode_bundle::h61f82fb84419427e (<module>[335]:0x46a48)
//     at dna_library::packaging::get_dna_package::h9c34a8ced6abdd0f (<module>[236]:0x20d4e)
//     at dna_library::get_dna_package::ha90632cc769615d8 (<module>[958]:0x16ec45)
//     at get_dna_package (<module>[305]:0x3a075)
// ") }
