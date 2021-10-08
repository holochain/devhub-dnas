use devhub_types::{
    AppResult, UpdateEntityInput, GetEntityInput,
    happ_entry_types::{
	HappEntry,
	HappReleaseEntry, HappReleaseInfo, HappReleaseSummary,
	HappManifest, DnaReference,
    },
};
use hc_crud::{
    now, create_entity, get_entity, update_entity, delete_entity, get_entities,
    Entity, Collection,
};
use hdk::prelude::*;

use crate::constants::{ TAG_HAPP_RELEASE };



#[derive(Debug, Deserialize)]
pub struct CreateInput {
    pub name: String,
    pub description: String,
    pub for_happ: EntryHash,
    pub manifest: HappManifest,
    pub dnas: Vec<DnaReference>,

    // optional
    pub published_at: Option<u64>,
    pub last_updated: Option<u64>,
}

pub fn create_happ_release(input: CreateInput) -> AppResult<Entity<HappReleaseInfo>> {
    debug!("Creating HAPPRELEASE: {}", input.name );
    let default_now = now()?;

    let happ_release = HappReleaseEntry {
	name: input.name,
	description: input.description,
	for_happ: input.for_happ.clone(),
	published_at: input.published_at
	    .unwrap_or( default_now ),
	last_updated: input.last_updated
	    .unwrap_or( default_now ),
	manifest: input.manifest,
	dnas: input.dnas,
    };

    let entity = create_entity( &happ_release )?
	.change_model( |release| release.to_info() );

    debug!("Linking happ ({}) to ENTRY: {}", input.for_happ, entity.id );
    entity.link_from( &input.for_happ, TAG_HAPP_RELEASE.into() )?;

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
    pub manifest: Option<HappManifest>,
    pub dnas: Option<Vec<DnaReference>>,
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
		manifest: props.manifest
		    .unwrap_or( current.manifest ),
		dnas: props.dnas
		    .unwrap_or( current.dnas ),
	    })
	})?;

    Ok(	entity.change_model( |release| release.to_info() ) )
}



#[derive(Debug, Deserialize)]
pub struct DeleteInput {
    pub id: EntryHash,
}

pub fn delete_happ_release(input: DeleteInput) -> AppResult<HeaderHash> {
    debug!("Delete HAPPRELEASE Version: {}", input.id );
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
