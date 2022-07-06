use std::collections::BTreeMap;
use devhub_types::{
    AppResult, UpdateEntityInput,
    dnarepo_entry_types::{
	DnaEntry,
	DnaVersionEntry, ZomeReference,
    },
    constants::{
	ANCHOR_UNIQUENESS,
	ANCHOR_HDK_VERSIONS,
    },
    fmt_path,
};
use hc_crud::{
    now, create_entity, get_entity, update_entity, delete_entity, get_entities,
    Entity, Collection,
};
use hdk::prelude::*;
use hex;

use crate::constants::{
    LT_NONE,
    TAG_DNAVERSION,
};



#[derive(Debug, Deserialize)]
pub struct DnaVersionInput {
    pub for_dna: EntryHash,
    pub version: String,
    pub ordering: u64,
    pub hdk_version: String,
    pub zomes: Vec<ZomeReference>,

    // optional
    pub changelog: Option<String>,
    pub published_at: Option<u64>,
    pub last_updated: Option<u64>,
    pub properties: Option<BTreeMap<String, serde_yaml::Value>>,
    pub source_code_commit_url: Option<String>,
    pub metadata: Option<BTreeMap<String, serde_yaml::Value>>,
}

pub fn create_dna_version(input: DnaVersionInput) -> AppResult<Entity<DnaVersionEntry>> {
    debug!("Creating DNA version ({}) for DNA: {}", input.version, input.for_dna );
    let default_now = now()?;

    let hashes = input.zomes.iter()
	.map( |zome| hex::decode( zome.resource_hash.to_owned() ) )
	.collect::<Result<Vec<Vec<u8>>, hex::FromHexError>>()
	.or(Err(devhub_types::errors::UserError::CustomError("Bad hex value")))?;

    let version = DnaVersionEntry {
	for_dna: input.for_dna.clone(),
	version: input.version,
	ordering: input.ordering,
	hdk_version: input.hdk_version.clone(),
	properties: input.properties,
	zomes: input.zomes,
	wasm_hash: hex::encode( devhub_types::hash_of_hashes( &hashes ) ),
	changelog: input.changelog
	    .unwrap_or( String::from("") ),
	published_at: input.published_at
	    .unwrap_or( default_now ),
	last_updated: input.last_updated
	    .unwrap_or( default_now ),
	source_code_commit_url: input.source_code_commit_url,
	metadata: input.metadata
	    .unwrap_or( BTreeMap::new() ),
    };

    let entity = create_entity( &version )?;

    // Parent anchor
    debug!("Linking DNA ({}) to ENTRY: {}", input.for_dna, entity.id );
    entity.link_from( &input.for_dna, LT_NONE, TAG_DNAVERSION.into() )?;

    // Uniqueness anchor
    let (wasm_path, wasm_path_hash) = devhub_types::ensure_path( ANCHOR_UNIQUENESS, vec![ &version.wasm_hash ] )?;
    debug!("Linking uniqueness path ({}) to ENTRY: {}", fmt_path( &wasm_path ), entity.id );
    entity.link_from( &wasm_path_hash, LT_NONE, TAG_DNAVERSION.into() )?;

    // HDK anchor
    let (hdkv_path, hdkv_hash) = devhub_types::ensure_path( ANCHOR_HDK_VERSIONS, vec![ &input.hdk_version ] )?;
    debug!("Linking HDK version global anchor ({}) to entry: {}", fmt_path( &hdkv_path ), entity.id );
    entity.link_from( &hdkv_hash, LT_NONE, TAG_DNAVERSION.into() )?;

    Ok( entity )
}




#[derive(Debug, Deserialize)]
pub struct GetDnaVersionInput {
    pub id: EntryHash,
}

pub fn get_dna_version(input: GetDnaVersionInput) -> AppResult<Entity<DnaVersionEntry>> {
    debug!("Get DNA Version: {}", input.id );
    let entity = get_entity::<DnaVersionEntry>( &input.id )?;

    Ok(	entity )
}




#[derive(Debug, Deserialize)]
pub struct GetDnaVersionsInput {
    pub for_dna: EntryHash,
}

pub fn get_dna_versions(input: GetDnaVersionsInput) -> AppResult<Collection<Entity<DnaVersionEntry>>> {
    Ok( get_entities::<DnaEntry, DnaVersionEntry>( &input.for_dna, TAG_DNAVERSION.into() )? )
}




#[derive(Debug, Deserialize)]
pub struct DnaVersionUpdateOptions {
    pub ordering: Option<u64>,
    pub changelog: Option<String>,
    pub published_at: Option<u64>,
    pub last_updated: Option<u64>,
    pub source_code_commit_url: Option<String>,
    pub metadata: Option<BTreeMap<String, serde_yaml::Value>>,
}
pub type DnaVersionUpdateInput = UpdateEntityInput<DnaVersionUpdateOptions>;

pub fn update_dna_version(input: DnaVersionUpdateInput) -> AppResult<Entity<DnaVersionEntry>> {
    debug!("Updating DNA Version: {}", input.addr );
    let props = input.properties;

    let entity = update_entity(
	&input.addr,
	|current : DnaVersionEntry, _| {
	    Ok(DnaVersionEntry {
		for_dna: current.for_dna,
		version: current.version,
		ordering: props.ordering
		    .unwrap_or( current.ordering ),
		published_at: props.published_at
		    .unwrap_or( current.published_at ),
		last_updated: props.last_updated
		    .unwrap_or( now()? ),
		wasm_hash: current.wasm_hash,
		hdk_version: current.hdk_version,
		properties: current.properties,
		zomes: current.zomes,
		changelog: props.changelog
		    .unwrap_or( current.changelog ),
		source_code_commit_url: props.source_code_commit_url
		    .or( current.source_code_commit_url ),
		metadata: props.metadata
		    .unwrap_or( current.metadata ),
	    })
	})?;

    Ok( entity )
}




#[derive(Debug, Deserialize)]
pub struct DeleteDnaVersionInput {
    pub id: EntryHash,
}

pub fn delete_dna_version(input: DeleteDnaVersionInput) -> AppResult<HeaderHash> {
    debug!("Delete DNA Version: {}", input.id );
    let delete_header = delete_entity::<DnaVersionEntry>( &input.id )?;
    debug!("Deleted DNA Version via header ({})", delete_header );

    Ok( delete_header )
}
