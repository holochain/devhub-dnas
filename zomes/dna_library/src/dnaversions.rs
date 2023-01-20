use std::collections::BTreeMap;
use dnarepo_core::{
    EntryTypes, LinkTypes,
};
use devhub_types::{
    AppResult, UpdateEntityInput,
    constants::{
	ANCHOR_UNIQUENESS,
	ANCHOR_HDK_VERSIONS,
    },
    dnarepo_entry_types::{
	DnaVersionEntry, IntegrityZomeReference, ZomeReference,
    },
    fmt_path,
};
use hc_crud::{
    now, create_entity, get_entity, update_entity, delete_entity, get_entities,
    Entity,
};
use hdk::prelude::*;
use hex;




#[derive(Debug, Deserialize)]
pub struct DnaVersionInput {
    pub for_dna: EntryHash,
    pub version: String,
    pub ordering: u64,
    pub hdk_version: String,
    pub integrity_zomes: Vec<IntegrityZomeReference>,
    pub zomes: Vec<ZomeReference>,
    pub origin_time: HumanTimestamp,

    // optional
    pub changelog: Option<String>,
    pub published_at: Option<u64>,
    pub last_updated: Option<u64>,
    pub network_seed: Option<String>,
    pub properties: Option<BTreeMap<String, serde_yaml::Value>>,
    pub source_code_commit_url: Option<String>,
    pub metadata: Option<BTreeMap<String, serde_yaml::Value>>,
}

pub fn create_dna_version(input: DnaVersionInput) -> AppResult<Entity<DnaVersionEntry>> {
    debug!("Creating DNA version ({}) for DNA: {}", input.version, input.for_dna );
    let default_now = now()?;

    if input.integrity_zomes.len() == 0 {
	Err(devhub_types::errors::UserError::CustomError("Must have at least 1 integrity zome"))?;
    }

    let hashes = input.integrity_zomes.iter()
	.map( |zome| hex::decode( zome.resource_hash.to_owned() ) )
	.collect::<Result<Vec<Vec<u8>>, hex::FromHexError>>()
	.or(Err(devhub_types::errors::UserError::CustomError("Bad hex value")))?;

    let version = DnaVersionEntry {
	for_dna: input.for_dna.clone(),
	version: input.version,
	ordering: input.ordering,
	hdk_version: input.hdk_version.clone(),
	origin_time: input.origin_time,
	network_seed: input.network_seed,
	properties: input.properties,
	integrity_zomes: input.integrity_zomes,
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
    entity.link_from( &input.for_dna, LinkTypes::DnaVersion, None )?;

    // Uniqueness anchor
    let (wasm_path, wasm_path_hash) = devhub_types::create_path( ANCHOR_UNIQUENESS, vec![ &version.wasm_hash ] );
    debug!("Linking '{:?}' uniqueness path ({}) to ENTRY: {}", LinkTypes::DnaVersion, fmt_path( &wasm_path ), entity.id );
    entity.link_from( &wasm_path_hash, LinkTypes::DnaVersion, None )?;

    // HDK anchor
    let (hdkv_path, hdkv_hash) = devhub_types::create_path( ANCHOR_HDK_VERSIONS, vec![ &input.hdk_version ] );
    debug!("Linking HDK version global anchor ({}) to entry: {}", fmt_path( &hdkv_path ), entity.id );
    entity.link_from( &hdkv_hash, LinkTypes::DnaVersion, None )?;

    Ok( entity )
}




#[derive(Debug, Deserialize)]
pub struct GetDnaVersionInput {
    pub id: EntryHash,
}

pub fn get_dna_version(input: GetDnaVersionInput) -> AppResult<Entity<DnaVersionEntry>> {
    debug!("Get DNA Version: {}", input.id );
    let entity = get_entity( &input.id )?;

    Ok(	entity )
}




#[derive(Debug, Deserialize)]
pub struct GetDnaVersionsInput {
    pub for_dna: EntryHash,
}

pub fn get_dna_versions(input: GetDnaVersionsInput) -> AppResult<Vec<Entity<DnaVersionEntry>>> {
    Ok( get_entities( &input.for_dna, LinkTypes::DnaVersion, None )? )
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
		origin_time: current.origin_time,
		network_seed: current.network_seed,
		properties: current.properties,
		integrity_zomes: current.integrity_zomes,
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

pub fn delete_dna_version(input: DeleteDnaVersionInput) -> AppResult<ActionHash> {
    debug!("Delete DNA Version: {}", input.id );
    let delete_action = delete_entity::<DnaVersionEntry,EntryTypes>( &input.id )?;
    debug!("Deleted DNA Version via action ({})", delete_action );

    Ok( delete_action )
}
