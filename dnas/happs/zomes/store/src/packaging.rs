use std::collections::BTreeMap;
use std::io::Write;
use devhub_types::{
    DevHubResponse, AppResult,
    errors::{ AppError },
    dnarepo_entry_types::{ DnaVersionPackage },
    web_asset_entry_types::{ FileInfo },
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


#[derive(Debug, Deserialize)]
pub struct GetReleasePackageInput {
    pub id: EntryHash,
    pub dnarepo_dna_hash: DnaHash,
}

#[derive(Debug, Serialize)]
pub struct Bundle {
    pub manifest: serde_yaml::Value,
    pub resources: BTreeMap<String, Vec<u8>>,
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

    debug!("Fetching DNA package for {} resources", entity.content.resources.len() );
    for (slot_id, version_entry_hash) in entity.content.resources.iter() {
	debug!("Fetching DNA package: {}", version_entry_hash );
	let zome_call_response = call(
	    Some( cell_id.clone() ),
	    "storage".into(),
	    "get_dna_package".into(),
	    None,
	    GetEntityInput {
		id: version_entry_hash.to_owned(),
	    },
	)?;

	if let ZomeCallResponse::Ok(result_io) = zome_call_response {
	    let response : DevHubResponse<Entity<DnaVersionPackage>> = result_io.decode()
		.map_err( |e| AppError::UnexpectedStateError(format!("Failed to call another DNA: {:?}", e )) )?;

	    if let DevHubResponse::Success(pack) = response {
		let slot = package.manifest.get("slots")
		    .ok_or( AppError::UnexpectedStateError("Manifest is missing 'slots' field".into()) )?
		    .as_sequence().unwrap().into_iter()
		    .find( |slot| {
			debug!("Slot ID: {:?}", slot.get("id") );
			match slot.get("id") {
			    Some(id) => {
				debug!("Compare slot ID to DNA id: {:?} == {:?}", slot_id, id );
				id.as_str().unwrap() == slot_id
			    },
			    None => false,
			}
		    })
		    .ok_or( AppError::UnexpectedStateError(format!("Missing slot for resource {}", slot_id )) )?;
		let key = slot["dna"]["path"].as_str()
		    .ok_or( AppError::UnexpectedStateError(format!("DNA path is not a string: {:?}", slot["dna"]["path"] )) )?;

		debug!("Parsed Manifest YAML: {:?}", package.manifest );
		debug!("Adding resource pack '{}' with {} bytes", key, pack.payload.content.bytes.len() );
		package.resources.insert( key.to_string(), pack.payload.content.bytes );
	    }
	}
	else {
	    return Err( AppError::UnexpectedStateError("Failed to call another DNA".into()).into() );
	}
    }
    debug!("Finished collecting DNAs for package with {} resources: {:?}", package.resources.len(), package.resources.clone().into_iter().map( |(k,v)| (k, v.len()) ).collect::<Vec<(String, usize)>>() );

    let packed_bytes = rmp_serde::to_vec_named( &package )
	.map_err( |e| AppError::UnexpectedStateError(format!("Failed to msgpack bundle: {:?}", e )) )?;
    debug!("Message packed bytes: {}", packed_bytes.len() );

    let mut enc = flate2::write::GzEncoder::new(Vec::new(), flate2::Compression::default());
    enc.write_all( &packed_bytes )
	.map_err( |e| AppError::UnexpectedStateError(format!("Failed to gzip package: {:?}", e )) )?;

    let gzipped_package = enc.finish()
	.map_err( |e| AppError::UnexpectedStateError(format!("Failed to finish gzip encoding: {:?}", e )) )?;
    debug!("Gzipped package bytes: {}", gzipped_package.len() );

    Ok( gzipped_package )
}
