use std::collections::HashMap;
use devhub_types::{
    AppResult, UpdateEntityInput, GetEntityInput,
    happ_entry_types::{
	HappEntry,
	HappReleaseEntry, HappReleaseInfo, HappReleaseSummary,
	HappManifest, DnaReference, HappGUIConfig,
    },
};
use hc_crud::{
    now, create_entity, get_entity, update_entity, delete_entity, get_entities,
    Entity, Collection,
};
use hdk::prelude::*;
use hex;

use crate::constants::{ TAG_HAPP_RELEASE };


fn happ_release_path(hash: &str) -> AppResult<Path> {
    Ok( create_filter_path( "uniqueness_hash", hash )? )
}

fn filter_path(filter: &str, value: &str) -> AppResult<Path> {
    Ok( hc_crud::path_from_collection( vec![ "happ_release_by", filter, value ] )? )
}

fn create_filter_path(filter: &str, value: &str) -> AppResult<Path> {
    let path = filter_path( filter, value )?;
    path.ensure()?;

    Ok( path )
}



#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GUIConfigInput {
    pub asset_group_id: EntryHash,
    pub uses_web_sdk: bool,
}

#[derive(Debug, Deserialize)]
pub struct CreateInput {
    pub name: String,
    pub description: String,
    pub for_happ: EntryHash,
    pub manifest: HappManifest,
    pub hdk_version: String,
    pub dnas: Vec<DnaReference>,

    // optional
    pub published_at: Option<u64>,
    pub last_updated: Option<u64>,
    pub gui: Option<GUIConfigInput>,
    pub metadata: Option<HashMap<String, serde_yaml::Value>>,
}

pub fn create_happ_release(input: CreateInput) -> AppResult<Entity<HappReleaseInfo>> {
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
	published_at: input.published_at
	    .unwrap_or( default_now ),
	last_updated: input.last_updated
	    .unwrap_or( default_now ),
	manifest: input.manifest,
	dna_hash: hex::encode( devhub_types::hash_of_hashes( &hashes ) ),
	hdk_version: input.hdk_version,
	dnas: input.dnas,
	gui: input.gui.map(|gui| {
	    HappGUIConfig::new( gui.asset_group_id, gui.uses_web_sdk )
	}),
	metadata: input.metadata
	    .unwrap_or( HashMap::new() ),
    };

    let release_path = happ_release_path( &happ_release.dna_hash )?;
    let release_path_hash = release_path.path_entry_hash()?;

    let entity = create_entity( &happ_release )?
	.change_model( |release| release.to_info() );

    debug!("Linking happ ({}) to ENTRY: {}", input.for_happ, entity.id );
    entity.link_from( &input.for_happ, TAG_HAPP_RELEASE.into() )?;

    debug!("Linking uniqueness 'hash' path ({}) to ENTRY: {}", release_path_hash, entity.id );
    entity.link_from( &release_path_hash, TAG_HAPP_RELEASE.into() )?;

    Ok( entity )
}


pub fn get_happ_release(input: GetEntityInput) -> AppResult<Entity<HappReleaseInfo>> {
    debug!("Get happ_release: {}", input.id );
    let entity = get_entity::<HappReleaseEntry>( &input.id )?;

    Ok(	entity.change_model( |release| release.to_info() ) )
}


#[derive(Debug, Deserialize)]
pub struct HappReleaseUpdateOptions {
    pub name: Option<String>,
    pub description: Option<String>,
    pub published_at: Option<u64>,
    pub last_updated: Option<u64>,
    pub gui: Option<HappGUIConfig>,
    pub metadata: Option<HashMap<String, serde_yaml::Value>>,
}
pub type HappReleaseUpdateInput = UpdateEntityInput<HappReleaseUpdateOptions>;

pub fn update_happ_release(input: HappReleaseUpdateInput) -> AppResult<Entity<HappReleaseInfo>> {
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
		published_at: props.published_at
		    .unwrap_or( current.published_at ),
		last_updated: props.last_updated
		    .unwrap_or( now()? ),
		manifest: current.manifest,
		dna_hash: current.dna_hash,
		hdk_version: current.hdk_version,
		dnas: current.dnas,
		gui: props.gui
		    .or( current.gui ),
		metadata: props.metadata
		    .unwrap_or( current.metadata ),
	    })
	})?;

    Ok(	entity.change_model( |release| release.to_info() ) )
}



fn get_entities_for_links ( links: Vec<Link> ) -> Vec<Entity<HappReleaseSummary>> {
    links.into_iter()
	.filter_map(|link| {
	    get_entity::<HappReleaseEntry>( &link.target ).ok()
	})
	.map( |entity| {
	    entity.change_model( |release| release.to_summary() )
	})
	.collect()
}



#[derive(Debug, Deserialize)]
pub struct DeleteInput {
    pub id: EntryHash,
}

pub fn delete_happ_release(input: DeleteInput) -> AppResult<HeaderHash> {
    debug!("Delete HAPP RELEASE Version: {}", input.id );
    let delete_header = delete_entity::<HappReleaseEntry>( &input.id )?;
    debug!("Deleted hApp release via header ({})", delete_header );

    Ok( delete_header )
}



#[derive(Debug, Deserialize)]
pub struct GetHappReleasesInput {
    pub for_happ: EntryHash,
}

pub fn get_happ_releases(input: GetHappReleasesInput) -> AppResult<Collection<Entity<HappReleaseSummary>>> {
    let collection = get_entities::<HappEntry, HappReleaseEntry>( &input.for_happ, TAG_HAPP_RELEASE.into() )?;

    let releases = collection.items.into_iter()
	.map(|entity| {
	    entity.change_model( |release| release.to_summary() )
	})
	.collect();

    Ok(Collection {
	base: collection.base,
	items: releases,
    })
}


pub fn get_happ_releases_by_filter( filter: String, keyword: String ) -> AppResult<Collection<Entity<HappReleaseSummary>>> {
    let base = filter_path( &filter, &keyword )?.path_entry_hash()?;

    debug!("Getting hApp links for base: {:?}", base );
    let all_links = get_links(
        base.clone(),
	Some(LinkTag::new(TAG_HAPP_RELEASE))
    )?;

    let releases = get_entities_for_links( all_links );

    Ok(Collection {
	base,
	items: releases,
    })
}
