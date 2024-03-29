use std::collections::BTreeMap;
use dnarepo_core::{
    LinkTypes,
};
use devhub_types::{
    AppResult, UpdateEntityInput,
    constants::{
	ANCHOR_TAGS,
	ANCHOR_NAMES,
    },
    dnarepo_entry_types::{
	ZomeEntry,
	ZomeVersionEntry,
	DeprecationNotice,
    },
    fmt_path,
};
use hc_crud::{
    now, create_entity, get_entity, update_entity,
    Entity,
};
use hdk::prelude::*;

use crate::constants::{
    ANCHOR_ZOMES,
};




#[derive(Debug, Deserialize)]
pub struct ZomeInput {
    pub name: String,
    pub description: String,

    // optional
    pub display_name: Option<String>,
    pub zome_type: Option<u8>,
    pub tags: Option<Vec<String>>,
    pub published_at: Option<u64>,
    pub last_updated: Option<u64>,
    pub source_code_url: Option<String>,
    pub metadata: Option<BTreeMap<String, serde_yaml::Value>>,
}

pub fn create_zome(input: ZomeInput) -> AppResult<Entity<ZomeEntry>> {
    debug!("Creating ZOME: {}", input.name );
    let pubkey = agent_info()?.agent_initial_pubkey;
    let default_now = now()?;

    let (name_path, name_path_hash) = devhub_types::create_path( ANCHOR_NAMES, vec![ &input.name ] );
    let (name_path_lc, name_path_lc_hash) = devhub_types::create_path( ANCHOR_NAMES, vec![ &input.name.to_lowercase() ] );

    let zome = ZomeEntry {
	name: input.name,
	zome_type: input.zome_type.unwrap_or(1),
	display_name: input.display_name,
	description: input.description,
	published_at: input.published_at
	    .unwrap_or( default_now ),
	last_updated: input.last_updated
	    .unwrap_or( default_now ),
	developer: pubkey.clone(),
	deprecation: None,
	source_code_url: input.source_code_url,
	metadata: input.metadata
	    .unwrap_or( BTreeMap::new() ),
	tags: input.tags.to_owned(),
    };

    let entity = create_entity( &zome )?;

    // Developer (Agent) anchor
    let (agent_base, agent_base_hash) = devhub_types::create_path( &crate::agent_path_base( None ), vec![ ANCHOR_ZOMES ]);
    debug!("Linking agent ({}) to ENTRY: {}", fmt_path( &agent_base ), entity.id );
    entity.link_from( &agent_base_hash, LinkTypes::Zome, None )?;

    // Name anchors (case sensitive/insensitive)
    debug!("Linking name path ({}) to ENTRY: {}", fmt_path( &name_path ), entity.id );
    entity.link_from( &name_path_hash, LinkTypes::Zome, None )?;

    if name_path_lc != name_path {
	debug!("Linking name (lowercase) path ({}) to ENTRY: {}", fmt_path( &name_path_lc ), entity.id );
	entity.link_from( &name_path_lc_hash, LinkTypes::Zome, None )?;
    }

    // Global anchor
    let (all_zomes_path, all_zomes_hash) = devhub_types::create_path( ANCHOR_ZOMES, Vec::<String>::new() );
    debug!("Linking all Zome path ({}) to ENTRY: {}", fmt_path( &all_zomes_path ), entity.id );
    entity.link_from( &all_zomes_hash, LinkTypes::Zome, None )?;

    // Tag anchors
    if input.tags.is_some() {
	for tag in input.tags.unwrap() {
	    let (tag_path, tag_hash) = devhub_types::create_path( ANCHOR_TAGS, vec![ &tag.to_lowercase() ] );
	    debug!("Linking TAG anchor ({}) to entry: {}", fmt_path( &tag_path ), entity.id );
	    entity.link_from( &tag_hash, LinkTypes::Zome, None )?;
	}
    }

    Ok( entity )
}




#[derive(Debug, Deserialize)]
pub struct GetZomeInput {
    pub id: ActionHash,
}

pub fn get_zome(input: GetZomeInput) -> AppResult<Entity<ZomeEntry>> {
    debug!("Get ZOME: {}", input.id );
    let entity = get_entity( &input.id )?;

    Ok( entity )
}




#[derive(Debug, Deserialize, Clone)]
pub struct ZomeUpdateOptions {
    pub name: Option<String>,
    pub display_name: Option<String>,
    pub description: Option<String>,
    pub zome_type: Option<u8>,
    pub tags: Option<Vec<String>>,
    pub published_at: Option<u64>,
    pub last_updated: Option<u64>,
    pub source_code_url: Option<String>,
    pub metadata: Option<BTreeMap<String, serde_yaml::Value>>,
}
pub type ZomeUpdateInput = UpdateEntityInput<ZomeUpdateOptions>;

pub fn update_zome(input: ZomeUpdateInput) -> AppResult<Entity<ZomeEntry>> {
    debug!("Updating ZOME: {}", input.addr );
    let props = input.properties.clone();
    let mut previous : Option<ZomeEntry> = None;

    let entity = update_entity(
	&input.addr,
	|current : ZomeEntry, _| {
	    previous = Some(current.clone());

	    Ok(ZomeEntry {
		name: props.name
		    .unwrap_or( current.name ),
		display_name: props.display_name
		    .or( current.display_name ),
		zome_type: props.zome_type
		    .unwrap_or( current.zome_type ),
		description: props.description
		    .unwrap_or( current.description ),
		published_at: props.published_at
		    .unwrap_or( current.published_at ),
		last_updated: props.last_updated
		    .unwrap_or( now()? ),
		developer: current.developer,
		deprecation: current.deprecation,
		source_code_url: props.source_code_url
		    .or( current.source_code_url ),
		metadata: props.metadata
		    .unwrap_or( current.metadata ),
		tags: props.tags
		    .or( current.tags ),
	    })
	})?;

    let previous = previous.unwrap();

    if input.properties.name.is_some() {
	let (previous_name_path, previous_path_hash) = devhub_types::create_path( ANCHOR_NAMES, vec![ &previous.name ] );
	let (new_name_path, new_path_hash) = devhub_types::create_path( ANCHOR_NAMES, vec![ &entity.content.name ] );

	if previous_path_hash != new_path_hash {
	    debug!("Moving name link: {} -> {}", fmt_path( &previous_name_path ), fmt_path( &new_name_path ) );
	    entity.move_link_from( LinkTypes::Zome, None, &previous_path_hash, &new_path_hash ).ok();
	}

	let (previous_name_path, previous_path_hash) = devhub_types::create_path( ANCHOR_NAMES, vec![ &previous.name.to_lowercase() ] );
	let (new_name_path, new_path_hash) = devhub_types::create_path( ANCHOR_NAMES, vec![ &entity.content.name.to_lowercase() ] );

	if previous_path_hash != new_path_hash {
	    debug!("Moving name (lowercase) link: {} -> {}", fmt_path( &previous_name_path ), fmt_path( &new_name_path ) );
	    entity.move_link_from( LinkTypes::Zome, None, &previous_path_hash, &new_path_hash ).ok();
	}
    }

    devhub_types::update_tag_links( previous.tags, input.properties.tags, &entity, LinkTypes::Zome, LinkTypes::Tag )?;

    Ok( entity )
}




#[derive(Debug, Deserialize)]
pub struct DeprecateZomeInput {
    pub addr: ActionHash,
    pub message: String,
}

pub fn deprecate_zome(input: DeprecateZomeInput) -> AppResult<Entity<ZomeEntry>> {
    debug!("Deprecating ZOME: {}", input.addr );
    let entity : Entity<ZomeEntry> = update_entity(
	&input.addr,
	|current : ZomeEntry, _| {
	    Ok(ZomeEntry {
		name: current.name,
		display_name: current.display_name,
		description: current.description,
		zome_type: current.zome_type,
		published_at: current.published_at,
		last_updated: current.last_updated,
		developer: current.developer,
		deprecation: Some(DeprecationNotice::new( input.message.to_owned() )),
		source_code_url: current.source_code_url,
		metadata: current.metadata,
		tags: current.tags,
	    })
	})?;

    Ok( entity )
}

pub fn get_zomes_with_an_hdk_version( input: String ) -> AppResult<Vec<Entity<ZomeEntry>>> {
    let collection : Vec<Entity<ZomeVersionEntry>> = devhub_types::get_hdk_version_entities( LinkTypes::ZomeVersion, input )?;

    let mut zomes : BTreeMap<ActionHash, Entity<ZomeEntry>> = BTreeMap::new();

    for zome_version in collection.into_iter() {
	let zome = get_entity( &zome_version.content.for_zome )?;

	if !zomes.contains_key( &zome.id ) {
	    zomes.insert( zome.id.to_owned(), zome );
	}
    }

    Ok(zomes.into_values().collect())
}
