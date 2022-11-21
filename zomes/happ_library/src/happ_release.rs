use std::collections::BTreeMap;
use happs_core::{
    EntryTypes, LinkTypes,
};
use devhub_types::{
    AppResult, UpdateEntityInput, GetEntityInput,
    happ_entry_types::{
	HappReleaseEntry,
	HappManifest, DnaReference,
    },
    constants::{
	ANCHOR_UNIQUENESS,
	ANCHOR_HDK_VERSIONS,
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
pub struct CreateInput {
    pub name: String,
    pub description: String,
    pub for_happ: EntryHash,
    pub ordering: u64,
    pub manifest: HappManifest,
    pub hdk_version: String,
    pub dnas: Vec<DnaReference>,

    // optional
    pub official_gui: Option<EntryHash>,
    pub published_at: Option<u64>,
    pub last_updated: Option<u64>,
    pub metadata: Option<BTreeMap<String, serde_yaml::Value>>,
}

pub fn create_happ_release(input: CreateInput) -> AppResult<Entity<HappReleaseEntry>> {
    debug!("Creating HAPPRELEASE: {}", input.name );
    let default_now = now()?;

    let hashes = input.dnas.iter()
	.map( |dna| hex::decode( dna.wasm_hash.to_owned() ) )
	.collect::<Result<Vec<Vec<u8>>, hex::FromHexError>>()
	.or(Err(devhub_types::errors::UserError::CustomError("Bad hex value")))?;

    let happ_release = HappReleaseEntry {
	name: input.name,
	description: input.description,
	for_happ: input.for_happ.clone(),
	ordering: input.ordering,
	published_at: input.published_at
	    .unwrap_or( default_now ),
	last_updated: input.last_updated
	    .unwrap_or( default_now ),
	manifest: input.manifest,
	dna_hash: hex::encode( devhub_types::hash_of_hashes( &hashes ) ),
	hdk_version: input.hdk_version.clone(),
	dnas: input.dnas,
	official_gui: input.official_gui,
	metadata: input.metadata
	    .unwrap_or( BTreeMap::new() ),
    };

    let entity = create_entity( &happ_release )?;

    // Parent anchor
    debug!("Linking happ ({}) to ENTRY: {}", input.for_happ, entity.id );
    entity.link_from( &input.for_happ, LinkTypes::HappRelease, None )?;

    // Uniqueness anchor
    let (wasm_path, wasm_path_hash) = devhub_types::create_path( ANCHOR_UNIQUENESS, vec![ &happ_release.dna_hash ] );
    debug!("Linking uniqueness path ({}) to ENTRY: {}", fmt_path( &wasm_path ), entity.id );
    entity.link_from( &wasm_path_hash, LinkTypes::HappRelease, None )?;

    // HDK anchor
    let (hdkv_path, hdkv_hash) = devhub_types::create_path( ANCHOR_HDK_VERSIONS, vec![ &input.hdk_version ] );
    debug!("Linking HDK version global anchor ({}) to entry: {}", fmt_path( &hdkv_path ), entity.id );
    entity.link_from( &hdkv_hash, LinkTypes::HappRelease, None )?;

    Ok( entity )
}


pub fn get_happ_release(input: GetEntityInput) -> AppResult<Entity<HappReleaseEntry>> {
    debug!("Get happ_release: {}", input.id );
    let entity : Entity<HappReleaseEntry> = get_entity( &input.id )?;

    Ok(	entity )
}


#[derive(Debug, Deserialize)]
pub struct HappReleaseUpdateOptions {
    pub name: Option<String>,
    pub description: Option<String>,
    pub ordering: Option<u64>,
    pub official_gui: Option<EntryHash>,
    pub published_at: Option<u64>,
    pub last_updated: Option<u64>,
    pub metadata: Option<BTreeMap<String, serde_yaml::Value>>,
}
pub type HappReleaseUpdateInput = UpdateEntityInput<HappReleaseUpdateOptions>;

pub fn update_happ_release(input: HappReleaseUpdateInput) -> AppResult<Entity<HappReleaseEntry>> {
    debug!("Updating hApp: {}", input.addr );
    let props = input.properties;

    let entity = update_entity(
	&input.addr,
	|current : HappReleaseEntry, _| {
	    Ok(HappReleaseEntry {
		name: props.name
		    .unwrap_or( current.name ),
		description: props.description
		    .unwrap_or( current.description ),
		for_happ: current.for_happ,
		ordering: props.ordering
		    .unwrap_or( current.ordering ),
		published_at: props.published_at
		    .unwrap_or( current.published_at ),
		last_updated: props.last_updated
		    .unwrap_or( now()? ),
		manifest: current.manifest,
		dna_hash: current.dna_hash,
		hdk_version: current.hdk_version,
		dnas: current.dnas,
		official_gui: props.official_gui
		    .or( current.official_gui ),
		metadata: props.metadata
		    .unwrap_or( current.metadata ),
	    })
	})?;

    Ok(	entity )
}



#[derive(Debug, Deserialize)]
pub struct DeleteInput {
    pub id: EntryHash,
}

pub fn delete_happ_release(input: DeleteInput) -> AppResult<ActionHash> {
    debug!("Delete HAPP RELEASE Version: {}", input.id );
    let delete_action = delete_entity::<HappReleaseEntry,EntryTypes>( &input.id )?;
    debug!("Deleted hApp release via action ({})", delete_action );

    Ok( delete_action )
}



#[derive(Debug, Deserialize)]
pub struct GetHappReleasesInput {
    pub for_happ: EntryHash,
}

pub fn get_happ_releases(input: GetHappReleasesInput) -> AppResult<Vec<Entity<HappReleaseEntry>>> {
    Ok( get_entities( &input.for_happ, LinkTypes::HappRelease, None )? )
}
