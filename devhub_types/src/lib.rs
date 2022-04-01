pub mod constants;
pub mod errors;
pub mod dnarepo_entry_types;
pub mod happ_entry_types;
pub mod web_asset_entry_types;

use std::collections::{ HashMap, HashSet };
use std::iter::FromIterator;
use std::io::Write;

use hdk::prelude::*;
use hdk::hash_path::path::Component;
use essence::{ EssenceResponse };
use errors::{ ErrorKinds, AppError, UserError };
use sha2::{ Sha256, Digest };
pub use hc_crud::{
    get_entity,
    Collection, Entity, EntryModel,
};

use crate::constants::{
    ANCHOR_TAGS,
    ANCHOR_FILTERS,
    ANCHOR_HDK_VERSIONS,
};

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


pub fn fmt_path( path: &Path ) -> String {
    format!(
	"Path({})[{}]",
	path.as_ref()
	    .iter()
	    .map( |component| {
		let bytes = component.as_ref();
		let fallback = format!("{:?}", bytes );

		format!("\"{}\"", std::str::from_utf8( bytes ).unwrap_or( &fallback ) ).to_string()
	    })
	    .collect::<Vec<String>>()
	    .join("."),
	path.path_entry_hash().unwrap()
    )
}


pub fn fmt_tag( tag: &Vec<u8> ) -> String {
    std::str::from_utf8( tag ).unwrap_or( &format!("{:?}", tag ) ).to_string()
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


pub fn create_path<T>( base: &str, segments: T ) -> (Path, EntryHash)
where
    T: IntoIterator,
    T::Item: std::fmt::Display,
{
    let mut components : Vec<Component> = vec![];

    for seg in base.split(".") {
	let component = Component::from( format!("{}", seg ).as_bytes().to_vec() );
	components.push( component );
    }

    for seg in segments {
	let component = Component::from( format!("{}", seg ).as_bytes().to_vec() );
	components.push( component );
    }

    let path = Path::from( components );
    let hash = path.path_entry_hash().unwrap();

    ( path, hash )
}


pub fn ensure_path<T>( base: &str, segments: T ) -> AppResult<(Path, EntryHash)>
where
    T: IntoIterator,
    T::Item: std::fmt::Display,
{
    let result = create_path( base, segments );
    result.0.ensure()?;

    Ok( result )
}


pub fn get_entities_for_path<T>( tag: Vec<u8>, path : Path ) -> AppResult<Collection<Entity<T>>>
where
    T: Clone + EntryModel + TryFrom<Element, Error = WasmError> + EntryDefRegistration,
    Entry: TryFrom<T, Error = WasmError>,
{
    debug!("Getting all '{}' entities: {}", fmt_tag( &tag ), fmt_path( &path ) );

    let path_hash = path.path_entry_hash().unwrap();
    let links = get_links(
        path_hash.clone(),
	Some(LinkTag::new( tag ))
    )?;

    let list = links.into_iter()
	.filter_map(|link| {
	    get_entity::<T>( &link.target ).ok()
	})
	.collect();

    Ok(Collection {
	base: path_hash,
	items: list,
    })
}


pub fn get_entities_for_path_filtered<T,F>( tag: Vec<u8>, path : Path, filter: F ) -> AppResult<Collection<Entity<T>>>
where
    T: Clone + EntryModel + TryFrom<Element, Error = WasmError> + EntryDefRegistration,
    Entry: TryFrom<T, Error = WasmError>,
    F: FnOnce(Vec<Entity<T>>) -> AppResult<Vec<Entity<T>>>,
{
    let collection = get_entities_for_path( tag, path )?;

    Ok(Collection {
	base: collection.base,
	items: filter( collection.items )?,
    })
}


pub fn get_hdk_version_entities<T>( entity_tag: Vec<u8>, hdk_version: String ) -> AppResult<Collection<Entity<T>>>
where
    T: Clone + EntryModel + TryFrom<Element, Error = WasmError> + EntryDefRegistration,
    Entry: TryFrom<T, Error = WasmError>,
{
    let (base_path, base_hash) = create_path( ANCHOR_HDK_VERSIONS, vec![ &hdk_version ] );

    debug!("Getting entities with tag '{}' and HDK Version '{}': {}", fmt_tag( &entity_tag ), hdk_version, fmt_path( &base_path ) );
    let links = get_links(
        base_hash.clone(),
	Some(LinkTag::new(entity_tag))
    )?;

    let list = links.into_iter()
	.filter_map(|link| {
	    get_entity( &link.target ).ok()
	})
	.collect();

    Ok(Collection {
	base: base_hash,
	items: list,
    })
}


pub fn get_by_filter<T>( entity_tag: Vec<u8>, filter: String, keyword: String ) -> AppResult<Collection<Entity<T>>>
where
    T: Clone + EntryModel + TryFrom<Element, Error = WasmError> + EntryDefRegistration,
    Entry: TryFrom<T, Error = WasmError>,
{
    let (base_path, base_hash) = create_path( ANCHOR_FILTERS, vec![ &filter, &keyword ] );

    debug!("Getting '{}' links for filter: {} => {:?}", fmt_tag( &entity_tag ), fmt_path( &base_path ), base_path );
    let links = get_links(
        base_hash.clone(),
	Some(LinkTag::new( entity_tag ))
    )?;

    let list = links.into_iter()
	.filter_map(|link| {
	    get_entity( &link.target ).ok()
	})
	.collect();

    Ok(Collection {
	base: base_hash,
	items: list,
    })
}


pub fn get_by_tags<T>( entity_tag: Vec<u8>, tags: Vec<String> ) -> AppResult<Vec<Entity<T>>>
where
    T: Clone + EntryModel + TryFrom<Element, Error = WasmError> + EntryDefRegistration,
    Entry: TryFrom<T, Error = WasmError>,
{
    if tags.len() == 0 {
	return Err( UserError::CustomError("Tag list cannot be empty").into() );
    }

    let tag_count = tags.len();
    let mut match_count = HashMap::new();

    debug!("Gathering links for tags: {:?}", tags );
    for tag_name in tags.into_iter() {
	let (base, base_hash) = create_path( ANCHOR_TAGS, vec![ &tag_name.to_lowercase() ] );

	debug!("Getting '{}' links for tag '{}': {} => {:?}", fmt_tag( &entity_tag ), tag_name, fmt_path( &base ), base );
	let links = get_links(
            base_hash.clone(),
	    Some(LinkTag::new( entity_tag.clone() ))
	)?;

	for link in links {
	    if let Some((count, _)) = match_count.get_mut( &link.target ) {
		*count += 1;
	    } else {
		match_count.insert( link.target.to_owned(), (1, link) );
	    }
	}
    }

    let mut full_matches = Vec::new();
    for (count, link) in match_count.values() {
	if *count == tag_count {
	    full_matches.push( link.to_owned() );
	}
    }

    let list = full_matches.into_iter()
	.filter_map(|link| {
	    get_entity( &link.target ).ok()
	})
	.collect();

    Ok( list )
}


pub fn get_hdk_versions() -> AppResult<Collection<String>> {
    let (hdkv_path, hdkv_hash) = create_path( ANCHOR_HDK_VERSIONS, Vec::<String>::new() );

    let hdk_versions : Vec<String> = hdkv_path.children_paths()?.into_iter()
	.filter_map( |path| {
	    debug!("HDK Version PATH: {}", fmt_path( &path ) );
	    match std::str::from_utf8( path.as_ref().clone().last().unwrap().as_ref() ) {
		Err(_) => None,
		Ok(path_str) => Some( path_str.to_string() ),
	    }
	})
	.collect();

    Ok(Collection {
	base: hdkv_hash,
	items: hdk_versions,
    })
}


pub fn update_tag_links<T>(prev_tags: Option<Vec<String>>, new_tags: Option<Vec<String>>, entity: &Entity<T>, link_type: u8, tag: Vec<u8>) -> AppResult<()>
where
    T: Clone + EntryModel + TryFrom<Element, Error = WasmError> + EntryDefRegistration,
    Entry: TryFrom<T, Error = WasmError>,
{
    debug!("Update tag ({}) list for {} from {:?} to {:?}", fmt_tag( &tag ), entity.id, prev_tags, new_tags );
    if new_tags.is_none() {
	return Ok(());
    }
    // current.tags vs given tags
    //
    //   - create a list of removed tags
    //   - create a list of added tags
    //
    let prev_tags : HashSet<String> = HashSet::from_iter( prev_tags.unwrap_or( vec![] ).iter().cloned() );
    let new_tags : HashSet<String> = HashSet::from_iter( new_tags.unwrap_or( vec![] ).iter().cloned() );

    for rm_tag in prev_tags.difference( &new_tags ) {
	let (tag_path, tag_hash) = ensure_path( ANCHOR_TAGS, vec![ &rm_tag.to_lowercase() ] )?;

	let links = get_links(
	    tag_hash.clone(),
	    Some( LinkTag::new( tag.to_owned() ) )
	)?;

	debug!("Removing tag link: {}", fmt_path( &tag_path ) );
	if let Some(link) = links.iter().find(|link| {
	    debug!("Finding tag link match: {:?} == {:?}", link.target, entity.id );
	    link.target == entity.id
	}) {
	    delete_link( link.create_link_hash.clone() )?;
	}
	else {
	    debug!("Expected to remove tag link '{}' but it wasn't found", fmt_path( &tag_path ) );
	}
    }

    for add_tag in new_tags.difference( &prev_tags ) {
	let (tag_path, tag_hash) = ensure_path( ANCHOR_TAGS, vec![ &add_tag.to_lowercase() ] )?;
	debug!("Adding tag link: {}", fmt_path( &tag_path ) );
	entity.link_from( &tag_hash, link_type, tag.to_owned() )?;
    }

    Ok(())
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
