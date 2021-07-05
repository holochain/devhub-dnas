mod constants;

use hdk::prelude::*;
use essence::{ EssenceResponse };
use hc_entities::{ Collection, Entity };

pub use constants::{ ENTITY_MD, ENTITY_COLLECTION_MD, VALUE_MD, VALUE_COLLECTION_MD };


#[derive(Debug, Serialize)]
pub struct Metadata {
    pub composition: &'static str,
}

pub type DevHubResponse<T> = EssenceResponse<T, Metadata, ()>;

pub struct Reply<T, E>( DevHubResponse<T>, E )
where
    E: std::error::Error;

impl<T, E> Reply<T, E>
where
    E: std::error::Error {

    pub fn new(payload: Result<T, E>) -> DevHubResponse<T> {
	EssenceResponse::new(payload, ENTITY_MD, None )
    }
}


pub type CollectionResponse<T> = DevHubResponse<Collection<T>>;
pub type EntityResponse<T> = DevHubResponse<Entity<T>>;
pub type EntityCollectionResponse<T> = DevHubResponse<Collection<Entity<T>>>;




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

	    DevHubResponse::error( error.into(), None )
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
