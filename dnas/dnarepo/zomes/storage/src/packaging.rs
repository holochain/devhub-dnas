use std::collections::BTreeMap;

use devhub_types::{
    AppResult,
    dnarepo_entry_types::{ DnaVersionEntry, DnaVersionPackage },
    call_local_zome,
    encode_bundle,
};

use hc_entities::{ Entity };
use hc_dna_utils as utils;
use hdk::prelude::*;


// {
//     "manifest": {
//         "manifest_version": "1",
//         "name": "dnarepo",
//         "uid": null,
//         "properties": null,
//         "zomes": [
//             {
//                 "name": "storage",
//                 "hash": null,
//                 "bundled": "../../zomes/dnarepo/target/wasm32-unknown-unknown/release/storage.wasm"
//             },
//             {
//                 "name": "mere_memory",
//                 "hash": null,
//                 "bundled": "../../zomes/mere_memory/target/wasm32-unknown-unknown/release/mere_memory.wasm"
//             }
//         ]
//     },
//     "resources": {
//         "../../zomes/dnarepo/target/wasm32-unknown-unknown/release/storage.wasm": <Buffer ... 3490626 more bytes>,
//         "../../zomes/mere_memory/target/wasm32-unknown-unknown/release/mere_memory.wasm": <Buffer ... 2726804 more bytes>
//     }
// }


#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct BundleZomeInfo {
    pub name: String,
    pub bundle: String,

    // Optional fields
    pub hash: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Manifest {
    pub manifest_version: String,
    pub zomes: Vec<BundleZomeInfo>,

    // Optional fields
    pub name: Option<String>,
    pub uid: Option<String>,
    pub properties: Option<serde_yaml::Value>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Bundle {
    pub manifest: Manifest,
    pub resources: BTreeMap<String, Vec<u8>>,
}



#[derive(Debug, Deserialize)]
pub struct GetDnaPackageInput {
    pub id: EntryHash,
}

pub fn get_dna_package(input: GetDnaPackageInput) -> AppResult<Entity<DnaVersionPackage>> {
    debug!("Get DNA Version: {}", input.id );
    let entity = utils::get_entity( &input.id )?;
    let entry = DnaVersionEntry::try_from( &entity.content )?;

    let mut manifest_zomes : Vec<BundleZomeInfo> = vec![];
    let mut resources : BTreeMap<String, Vec<u8>> = BTreeMap::new();

    for zome_ref in entry.zomes.iter() {
	let bytes : Vec<u8> = call_local_zome( "mere_memory", "retrieve_bytes", zome_ref.resource.clone() )?;
	let path = format!("./{}.wasm", zome_ref.name );

	manifest_zomes.push( BundleZomeInfo {
	    name: zome_ref.name.clone(),
	    bundle: path,
	    hash: None,
	});

	resources.insert(
	    format!("./{}.wasm", zome_ref.name ),
	    bytes
	);
    }

    let bundle = Bundle {
	manifest: Manifest {
	    manifest_version: "1".into(),
	    zomes: manifest_zomes,
	    name: None,
	    uid: None,
	    properties: None,
	},
	resources: resources,
    };

    let dna_pack_bytes = encode_bundle( bundle )?;
    let package = entry.to_package( dna_pack_bytes );

    Ok( entity.new_content( package ) )
}
