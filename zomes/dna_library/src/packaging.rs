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
//         "network_seed": null,
//         "properties": null,
//         "zomes": [
//             {
//                 "name": "dna_library",
//                 "hash": null,
//                 "bundled": "../../zomes/dnarepo/target/wasm32-unknown-unknown/release/dna_library.wasm"
//             },
//             {
//                 "name": "mere_memory_api",
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
pub struct DependencyRef {
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct BundleIntegrityZomeInfo {
    pub name: String,
    pub bundled: String,

    // Optional fields
    pub hash: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct BundleZomeInfo {
    pub name: String,
    pub bundled: String,
    pub dependencies: Vec<DependencyRef>,

    // Optional fields
    pub hash: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Manifest {
    pub manifest_version: String,
    pub name: String,
    pub integrity: IntegrityZomes,
    pub coordinator: CoordinatorZomes,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct IntegrityZomes {
    origin_time: HumanTimestamp,
    zomes: Vec<BundleIntegrityZomeInfo>,

    // Optional fields
    pub network_seed: Option<String>,
    pub properties: Option<BTreeMap<String, serde_yaml::Value>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CoordinatorZomes {
    zomes: Vec<BundleZomeInfo>,
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

    let mut integrity_zomes : Vec<BundleIntegrityZomeInfo> = vec![];
    let mut coordinator_zomes : Vec<BundleZomeInfo> = vec![];
    let mut resources : BTreeMap<String, Vec<u8>> = BTreeMap::new();

    for zome_ref in entry.integrity_zomes.iter() {
	let bytes : Vec<u8> = call_local_zome( "mere_memory_api", "retrieve_bytes", zome_ref.resource.clone() )?;
	let path = format!("./{}.wasm", zome_ref.name );

	integrity_zomes.push( BundleIntegrityZomeInfo {
	    name: zome_ref.name.clone(),
	    bundled: path.clone(),
	    hash: None,
	});

	resources.insert(
	    path,
	    bytes
	);
    }

    for zome_ref in entry.zomes.iter() {
	let bytes : Vec<u8> = call_local_zome( "mere_memory_api", "retrieve_bytes", zome_ref.resource.clone() )?;
	let path = format!("./{}.wasm", zome_ref.name );

	coordinator_zomes.push( BundleZomeInfo {
	    name: zome_ref.name.clone(),
	    bundled: path.clone(),
	    hash: None,
	    dependencies: zome_ref.dependencies.iter().map( |name| DependencyRef {
		name: name.to_owned(),
	    }).collect(),
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
	    integrity: IntegrityZomes {
		origin_time: entry.origin_time.clone(),
		network_seed: entry.network_seed.clone(),
		properties: entry.properties.clone(),
		zomes: integrity_zomes,
	    },
	    coordinator: CoordinatorZomes {
		zomes: coordinator_zomes,
	    },
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
