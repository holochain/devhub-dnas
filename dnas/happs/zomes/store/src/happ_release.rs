use std::collections::BTreeMap;
use devhub_types::{
    AppResult,
    happ_entry_types::{ HappReleaseEntry, HappReleaseInfo },
};
use hc_entities::{ Entity, GetEntityInput };
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
