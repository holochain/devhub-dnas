use std::collections::BTreeMap;
use devhub_types::{
    AppResult,
    happ_entry_types::{
	HappReleaseEntry, HappReleaseInfo, HappReleaseSummary
    },
};
use hc_entities::{ Entity, Collection, GetEntityInput, UpdateEntityInput };
use hdk::prelude::*;
use hc_dna_utils as utils;

use crate::constants::{ TAG_HAPP_RELEASE };



#[derive(Debug, Deserialize)]
pub struct CreateInput {
    pub name: String,
    pub description: String,
    pub for_happ: EntryHash,
    pub manifest_yaml: String,
    pub resources: BTreeMap<String, EntryHash>,

    // optional
    pub published_at: Option<u64>,
    pub last_updated: Option<u64>,
}

pub fn create_happ_release(input: CreateInput) -> AppResult<Entity<HappReleaseInfo>> {
    debug!("Creating HAPPRELEASE: {}", input.name );
    let default_now = utils::now()?;

    let happ_release = HappReleaseEntry {
	name: input.name,
	description: input.description,
	for_happ: input.for_happ.clone(),
	published_at: input.published_at
	    .unwrap_or( default_now ),
	last_updated: input.last_updated
	    .unwrap_or( default_now ),
	manifest_yaml: input.manifest_yaml,
	resources: input.resources,
    };

    let entity = utils::create_entity( &happ_release )?
	.new_content( happ_release.to_info() );

    debug!("Linking happ ({}) to ENTRY: {}", input.for_happ, entity.id );
    create_link(
	input.for_happ,
	entity.id.clone(),
	LinkTag::new( TAG_HAPP_RELEASE )
    )?;

    Ok( entity )
}


pub fn get_happ_release(input: GetEntityInput) -> AppResult<Entity<HappReleaseInfo>> {
    debug!("Get happ_release: {}", input.id );
    let entity = utils::get_entity( &input.id )?;
    let info = HappReleaseEntry::try_from( &entity.content )?.to_info();

    Ok(	entity.new_content( info ) )
}


#[derive(Debug, Deserialize)]
pub struct HappReleaseUpdateOptions {
    pub name: Option<String>,
    pub description: Option<String>,
    pub published_at: Option<u64>,
    pub last_updated: Option<u64>,
    pub manifest_yaml: Option<String>,
    pub resources: Option<BTreeMap<String, EntryHash>>,
}
pub type HappReleaseUpdateInput = UpdateEntityInput<HappReleaseUpdateOptions>;

pub fn update_happ_release(input: HappReleaseUpdateInput) -> AppResult<Entity<HappReleaseInfo>> {
    debug!("Updating hApp: {}", input.addr );
    let props = input.properties;

    let entity : Entity<HappReleaseEntry> = utils::update_entity(
	input.id, input.addr,
	|element| {
	    let current = HappReleaseEntry::try_from( &element )?;

	    Ok(HappReleaseEntry {
		name: props.name
		    .unwrap_or( current.name ),
		description: props.description
		    .unwrap_or( current.description ),
		for_happ: current.for_happ,
		published_at: props.published_at
		    .unwrap_or( current.published_at ),
		last_updated: props.last_updated
		    .unwrap_or( utils::now()? ),
		manifest_yaml: props.manifest_yaml
		    .unwrap_or( current.manifest_yaml ),
		resources: props.resources
		    .unwrap_or( current.resources ),
	    })
	})?;

    let info = entity.content.to_info();

    Ok( entity.new_content( info ) )
}


#[derive(Debug, Deserialize)]
pub struct DeleteInput {
    pub id: EntryHash,
}

pub fn delete_happ_release(input: DeleteInput) -> AppResult<HeaderHash> {
    debug!("Delete HAPPRELEASE Version: {}", input.id );
    let (header, _) = utils::fetch_entry( input.id.clone() )?;

    let delete_header = delete_entry( header.clone() )?;
    debug!("Deleted hApp release create {} via header ({})", header, delete_header );

    Ok( header )
}


pub fn get_release_links(happ_id: EntryHash) -> AppResult<Vec<Link>> {
    debug!("Getting release links for HAPP: {}", happ_id );
    let all_links: Vec<Link> = get_links(
        happ_id,
	Some(LinkTag::new( TAG_HAPP_RELEASE ))
    )?.into();

    Ok( all_links )
}


#[derive(Debug, Deserialize)]
pub struct GetHappReleasesInput {
    pub for_happ: EntryHash,
}

pub fn get_happ_releases(input: GetHappReleasesInput) -> AppResult<Collection<Entity<HappReleaseSummary>>> {
    let links = get_release_links( input.for_happ.clone() )?;

    let releases = links.into_iter()
	.filter_map(|link| {
	    utils::get_entity( &link.target ).ok()
	})
	.filter_map(|entity| {
	    let mut maybe_entity : Option<Entity<HappReleaseSummary>> = None;

	    if let Some(release) = HappReleaseEntry::try_from( &entity.content ).ok() {
		let summary = release.to_summary();
		let entity = entity.new_content( summary );

		maybe_entity.replace( entity );
	    }

	    maybe_entity
	})
	.collect();

    Ok(Collection {
	base: input.for_happ,
	items: releases,
    })
}
