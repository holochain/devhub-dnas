use hdk::prelude::*;

pub use essence::{ EssencePackage, ErrorEssencePackage, EssenceResponse, ErrorPayload };


pub trait EntryModel {
    fn get_type(&self) -> String;
}



#[derive(Debug, Serialize)]
pub struct Metadata {
    pub composition: &'static str,
}

pub const ENTITY_MD : Option<Metadata> = Some(Metadata {
    composition: "entity",
});
pub const ENTITY_COLLECTION_MD : Option<Metadata> = Some(Metadata {
    composition: "entity_collection",
});
pub const VALUE_MD : Option<Metadata> = Some(Metadata {
    composition: "value",
});
pub const VALUE_COLLECTION_MD : Option<Metadata> = Some(Metadata {
    composition: "value_collection",
});



#[derive(Debug, Serialize)]
pub struct Entity<T> {
    pub id: EntryHash,
    pub header: HeaderHash,
    pub address: EntryHash,
    #[serde(rename = "type")]
    pub ctype: String,
    pub content: T,
}

impl<T> Entity<T> {
    pub fn replace_content<M>(&self, content: M) -> Entity<M>
    where M: EntryModel {
	Entity {
	    id: self.id.to_owned(),
	    header: self.header.to_owned(),
	    address: self.address.to_owned(),
	    ctype: content.get_type(),
	    content: content,
	}
    }
}



#[derive(Debug, Serialize)]
pub struct Collection<T> {
    pub base: EntryHash,
    pub items: Vec<T>,
}



pub type DevHubResponse<T> = EssenceResponse<T, Metadata, ()>;

pub type CollectionResponse<T> = DevHubResponse<Collection<T>>;

pub type EntityResponse<T> = DevHubResponse<Entity<T>>;
pub type EntityCollectionResponse<T> = DevHubResponse<Collection<Entity<T>>>;



#[cfg(test)]
pub mod tests {
    use super::*;

    use rand::Rng;
    use serde_json::json;
    use thiserror::Error;

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
  "type": "error",
  "payload": {
    "kind": "AppError",
    "error": "BadInput",
    "message": "This is so bad input: This is so bad...",
    "stack": null
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
		ctype: "entity".into(),
		content: true,
	    },
	    Some(Metadata {
		composition: "single".into(),
	    })
	);
    }
}
