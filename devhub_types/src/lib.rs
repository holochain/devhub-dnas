pub mod constants;
pub mod errors;
pub mod dnarepo_entry_types;
pub mod happ_entry_types;
pub mod web_asset_entry_types;

use std::io::Write;

use hdk::prelude::*;
use essence::{ EssenceResponse };
use errors::{ ErrorKinds, AppError };
use hc_entities::{ Collection, Entity };


pub type AppResult<T> = Result<T, ErrorKinds>;


#[derive(Debug, Serialize, Deserialize)]
pub struct Metadata {
    pub composition: String,
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


pub fn call_local_zome<'a, T, A>(zome: &str, func: &str, input: A) -> AppResult<T>
where
    T: serde::de::DeserializeOwned + std::fmt::Debug,
    A: serde::Serialize + std::fmt::Debug
{
    let zome_call_response = call(
	None,
	zome.into(),
	func.into(),
	None,
	input,
    )?;

    if let ZomeCallResponse::Ok(result_io) = zome_call_response {
	let response : DevHubResponse<T> = result_io.decode()
	    .map_err( |e| AppError::UnexpectedStateError(format!("Failed to call another DNA: {:?}", e )) )?;

	if let DevHubResponse::Success(pack) = response {
	    return Ok( pack.payload );
	} else {
	    return Err( AppError::UnexpectedStateError("Essence package is not success".into()).into() );
	}
    } else {
	return Err( AppError::UnexpectedStateError("Zome call response is not Ok".into()).into() );
    }
}

pub fn call_local_dna_zome<'a, T, A>(cell_id: &CellId, zome: &str, func: &str, input: A) -> AppResult<T>
where
    T: serde::de::DeserializeOwned + std::fmt::Debug,
    A: serde::Serialize + std::fmt::Debug
{
    let zome_call_response = call(
	Some( cell_id.to_owned() ),
	zome.into(),
	func.into(),
	None,
	input,
    )?;

    if let ZomeCallResponse::Ok(result_io) = zome_call_response {
	let response : DevHubResponse<T> = result_io.decode()
	    .map_err( |e| AppError::UnexpectedStateError(format!("Failed to call another DNA: {:?}", e )) )?;

	if let DevHubResponse::Success(pack) = response {
	    return Ok( pack.payload );
	} else {
	    return Err( AppError::UnexpectedStateError("Essence package is not success".into()).into() );
	}
    } else {
	return Err( AppError::UnexpectedStateError("Zome call response is not Ok".into()).into() );
    }
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



#[cfg(test)]
pub mod tests {
    use super::*;

    use rand::Rng;
    use serde_json::json;
    use thiserror::Error;
    use hc_entities::{ EntityType };

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
	let ehash = EntryHash::from_raw_32( bytes.to_vec() );
	let hhash = HeaderHash::from_raw_32( bytes.to_vec() );

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
