pub mod constants;
pub mod errors;
pub mod dnarepo_entry_types;
pub mod happ_entry_types;
pub mod web_asset_entry_types;

use std::io::Write;

use hdk::prelude::*;
use hdk::hash_path::path::Component;
use essence::{ EssenceResponse };
use errors::{ ErrorKinds, AppError };
use hc_crud::{ Collection, Entity };
use sha2::{ Sha256, Digest };


pub type AppResult<T> = Result<T, ErrorKinds>;

#[derive(Debug, Serialize, Deserialize)]
pub struct GetEntityInput {
    pub id: EntryHash,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateEntityInput<T> {
    pub id: Option<EntryHash>,
    pub addr: EntryHash,
    pub properties: T,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Metadata {
    pub composition: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FilterInput {
    pub filter: String,
    pub keyword: String,
}


pub type DevHubResponse<T> = EssenceResponse<T, Metadata, ()>;

pub fn composition<T>(payload: T, composition: &str) -> DevHubResponse<T> {
    DevHubResponse::success( payload, Some(Metadata {
	composition: String::from( composition ),
    }) )
}


pub type CollectionResponse<T> = DevHubResponse<Collection<T>>;
pub type EntityResponse<T> = DevHubResponse<Entity<T>>;
pub type EntityCollectionResponse<T> = DevHubResponse<Collection<Entity<T>>>;



#[macro_export]
macro_rules! catch { // could change to "trap", "snare", or "capture"
    ( $r:expr ) => {
	match $r {
	    Ok(x) => x,
	    Err(e) => {
		let error = match e {
		    devhub_types::errors::ErrorKinds::AppError(e) => (&e).into(),
		    devhub_types::errors::ErrorKinds::UserError(e) => (&e).into(),
		    devhub_types::errors::ErrorKinds::HDKError(e) => (&e).into(),
		    devhub_types::errors::ErrorKinds::DnaUtilsError(e) => (&e).into(),
		    devhub_types::errors::ErrorKinds::FailureResponseError(e) => (&e).into(),
		};
		return Ok(devhub_types::DevHubResponse::failure( error, None ))
	    },
	}
    };
    ( $r:expr, $e:expr ) => {
	match $r {
	    Ok(x) => x,
	    Err(e) => return Ok(devhub_types::DevHubResponse::failure( (&$e).into(), None )),
	}
    };
}


fn zome_call_response_as_result(response: ZomeCallResponse) -> AppResult<zome_io::ExternIO> {
    Ok( match response {
	ZomeCallResponse::Ok(bytes)
	    => Ok(bytes),
	ZomeCallResponse::Unauthorized(cell_id, zome, func, agent)
	    => Err(AppError::UnauthorizedError( cell_id, zome, func, agent )),
	ZomeCallResponse::NetworkError(message)
	    => Err(AppError::NetworkError(message)),
	ZomeCallResponse::CountersigningSession(message)
	    => Err(AppError::CountersigningSessionError(message)),
    }? )
}

fn interpret_zome_response<T>(response: ZomeCallResponse) -> AppResult<T>
where
    T: serde::de::DeserializeOwned + std::fmt::Debug,
{
    let result_io = zome_call_response_as_result( response )?;
    let essence : DevHubResponse<T> = result_io.decode()
	.map_err( |e| AppError::DeserializeError(format!("Could not decode Essence response ({} bytes): {}", result_io.as_bytes().len(), e )) )?;

    Ok( essence.as_result()? )
}

pub fn call_local_zome<T, A>(zome: &str, func: &str, input: A) -> AppResult<T>
where
    T: serde::de::DeserializeOwned + std::fmt::Debug,
    A: serde::Serialize + std::fmt::Debug
{
    let response = call(
	CallTargetCell::Local,
	zome.into(),
	func.into(),
	None,
	input,
    )?;

    Ok( interpret_zome_response( response )? )
}

pub fn call_local_dna_zome<T, A>(cell_id: &CellId, zome: &str, func: &str, input: A) -> AppResult<T>
where
    T: serde::de::DeserializeOwned + std::fmt::Debug,
    A: serde::Serialize + std::fmt::Debug,
{
    let response = call(
	CallTargetCell::Other( cell_id.to_owned() ),
	zome.into(),
	func.into(),
	None,
	input,
    )?;

    Ok( interpret_zome_response( response )? )
}


pub fn encode_bundle<T>(bundle: T) -> AppResult<Vec<u8>>
where
    T: serde::Serialize
{
    let packed_bytes = rmp_serde::to_vec_named( &bundle )
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



pub fn hash_of_hashes(hash_list: &Vec<Vec<u8>>) -> [u8; 32] {
    let mut hasher = Sha256::new();
    let mut hashes = hash_list.to_owned();

    hashes.sort();

    hashes.into_iter()
	.for_each( |bytes| hasher.update( bytes ) );

    hasher.finalize().into()
}

pub fn hdk_version_anchor(version: &str) -> Path {
    let path_components = vec![
	Component::from( "hdk_versions" ),
	Component::from( version.replace(".", "[dot]").as_bytes().to_vec() ),
    ];
    Path::from( path_components )
}

pub fn zome_hdk_anchor(entry: &dnarepo_entry_types::ZomeVersionEntry, version: &str) -> AppResult<Path> {
    let path_components = vec![
	Component::from( format!("{}", hash_entry( entry.to_owned() )? ) ),
	Component::from("hdk_versions"),
	Component::from( version.replace(".", "[dot]").as_bytes().to_vec() ),
    ];
    Ok( Path::from( path_components ) )
}



#[cfg(test)]
pub mod tests {
    use super::*;

    use rand::Rng;
    use serde_json::json;
    use thiserror::Error;
    use hc_crud::{ EntityType };

    #[derive(Debug, Error)]
    enum AppError<'a> {
	#[error("This is so bad input: {0}")]
	BadInput(&'a str),
    }

    fn zome_response(fail: bool) -> DevHubResponse<bool> {
	if fail {
	    let error = &AppError::BadInput("This is so bad...");

	    DevHubResponse::failure( error.into(), None )
	}
	else {
	    DevHubResponse::success( true, None )
	}
    }

    #[test]
    ///
    fn success_package_test() {
	assert_eq!(
	    serde_json::to_string_pretty( &json!(zome_response(false)) ).unwrap(),
	    String::from(r#"{
  "type": "success",
  "payload": true
}"#));

	assert_eq!(
	    serde_json::to_string_pretty( &json!(zome_response(true)) ).unwrap(),
	    String::from(r#"{
  "type": "failure",
  "payload": {
    "kind": "AppError",
    "error": "BadInput",
    "message": "This is so bad input: This is so bad...",
    "stack": []
  }
}"#));
    }

    #[test]
    ///
    fn success_entity_test() {
	let bytes = rand::thread_rng().gen::<[u8; 32]>();
	let ehash = crate::holo_hash::EntryHash::from_raw_32( bytes.to_vec() );
	let hhash = crate::holo_hash::HeaderHash::from_raw_32( bytes.to_vec() );

	let _ : DevHubResponse<Entity<_>> = DevHubResponse::success(
	    Entity {
		id: ehash.clone(),
		header: hhash,
		address: ehash,
		ctype: EntityType::new( "boolean", "primitive" ),
		content: true,
	    },
	    Some(Metadata {
		composition: "single".into(),
	    })
	);
    }
}
