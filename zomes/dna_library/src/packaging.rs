use std::collections::BTreeMap;

use devhub_types::{
    AppResult,
    dnarepo_entry_types::{
	DnaEntry,
	DnaVersionEntry, DnaVersionPackage,
    },
    call_local_zome,
    encode_bundle,
};
use hc_crud::{
    get_entity,
    Entity, EntityType,
};
use hdk::prelude::*;


// {
//     "manifest": {
//         "manifest_version": "1",
//         "name": "dnarepo",
//         "uid": null,
//         "properties": null,
//         "zomes": [
//             {
//                 "name": "dna_library",
//                 "hash": null,
//                 "bundled": "../../zomes/dnarepo/target/wasm32-unknown-unknown/release/dna_library.wasm"
//             },
//             {
//                 "name": "mere_memory",
//                 "hash": null,
//                 "bundled": "../../zomes/mere_memory.wasm"
//             }
//         ]
//     },
//     "resources": {
//         "../../zomes/dnarepo/target/wasm32-unknown-unknown/release/dna_library.wasm": <Buffer ... 3490626 more bytes>,
//         "../../zomes/mere_memory.wasm": <Buffer ... 2726804 more bytes>
//     }
// }


#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct BundleZomeInfo {
    pub name: String,
    pub bundled: String,

    // Optional fields
    pub hash: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Manifest {
    pub manifest_version: String,
    pub zomes: Vec<BundleZomeInfo>,

    // Optional fields
    pub name: String,
    pub uid: Option<String>,
    pub properties: Option<BTreeMap<String, serde_yaml::Value>>,
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
    let entity : Entity<DnaVersionEntry> = get_entity( &input.id )?;
    let entry = &entity.content;
    let dna : Entity<DnaEntry> = get_entity( &entry.for_dna )?;

    let mut manifest_zomes : Vec<BundleZomeInfo> = vec![];
    let mut resources : BTreeMap<String, Vec<u8>> = BTreeMap::new();

    for zome_ref in entry.zomes.iter() {
	let bytes : Vec<u8> = call_local_zome( "mere_memory", "retrieve_bytes", zome_ref.resource.clone() )?;
	let path = format!("./{}.wasm", zome_ref.name );

	manifest_zomes.push( BundleZomeInfo {
	    name: zome_ref.name.clone(),
	    bundled: path.clone(),
	    hash: None,
	});

	resources.insert(
	    path,
	    bytes
	);
    }

    let bundle = Bundle {
	manifest: Manifest {
	    manifest_version: "1".into(),
	    name: dna.content.name,
	    uid: None,
	    properties: entry.properties.clone(),
	    zomes: manifest_zomes,
	},
	resources: resources,
    };

    let dna_pack_bytes = encode_bundle( bundle )?;
    let package = entity.content.to_package( dna_pack_bytes );

    Ok(Entity {
	id: entity.id,
	action: entity.action,
	address: entity.address,
	ctype: EntityType::new( "dna_version", "package" ),
	content: package,
    })
}
