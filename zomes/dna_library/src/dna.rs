use std::collections::HashMap;
use devhub_types::{
    AppResult, UpdateEntityInput,
    dnarepo_entry_types::{
	DnaEntry, DnaInfo,
	DeveloperProfileLocation, DeprecationNotice
    },
    constants::{
	ANCHOR_TAGS,
	ANCHOR_NAMES,
    },
    fmt_path,
};
use hc_crud::{
    now, create_entity, get_entity, update_entity,
    Entity,
};
use hdk::prelude::*;

use crate::constants::{
    LT_NONE,
    TAG_DNA,
    ANCHOR_DNAS,
};



#[derive(Debug, Deserialize)]
pub struct DnaInput {
    pub name: String,
    pub description: String,

    // optional
    pub tags: Option<Vec<String>>,
    pub icon: Option<SerializedBytes>,
    pub published_at: Option<u64>,
    pub last_updated: Option<u64>,
    pub metadata: Option<HashMap<String, serde_yaml::Value>>,
}

pub fn create_dna(input: DnaInput) -> AppResult<Entity<DnaInfo>> {
    debug!("Creating DNA: {}", input.name );
    let pubkey = agent_info()?.agent_initial_pubkey;
    let default_now = now()?;

    let (name_path, name_path_hash) = devhub_types::ensure_path( ANCHOR_NAMES, vec![ &input.name ] )?;
    let (name_path_lc, name_path_lc_hash) = devhub_types::ensure_path( ANCHOR_NAMES, vec![ &input.name.to_lowercase() ] )?;

    let dna = DnaEntry {
	name: input.name,
	description: input.description,
	icon: input.icon,
	tags: input.tags.to_owned(),
	published_at: input.published_at
	    .unwrap_or( default_now ),
	last_updated: input.last_updated
	    .unwrap_or( default_now ),
	developer: DeveloperProfileLocation {
	    pubkey: pubkey.clone(),
	},
	deprecation: None,
	metadata: input.metadata
	    .unwrap_or( HashMap::new() ),
    };

    let entity = create_entity( &dna )?
	.change_model( |dna| dna.to_info() );

    // Developer (Agent) anchor
    let (agent_base, agent_base_hash) = devhub_types::ensure_path( &crate::agent_path_base( None ), vec![ ANCHOR_DNAS ] )?;
    debug!("Linking agent ({}) to ENTRY: {}", fmt_path( &agent_base ), entity.id );
    entity.link_from( &agent_base_hash, LT_NONE, TAG_DNA.into() )?;

    // Name anchors (case sensitive/insensitive)
    debug!("Linking name path ({}) to ENTRY: {}", fmt_path( &name_path ), entity.id );
    entity.link_from( &name_path_hash, LT_NONE, TAG_DNA.into() )?;

    if name_path_lc != name_path {
	debug!("Linking name (lowercase) path ({}) to ENTRY: {}", fmt_path( &name_path_lc ), entity.id );
	entity.link_from( &name_path_lc_hash, LT_NONE, TAG_DNA.into() )?;
    }

    // Global anchor
    let (all_dnas_path, all_dnas_hash) = devhub_types::ensure_path( ANCHOR_DNAS, Vec::<String>::new() )?;
    debug!("Linking all DNAs path ({}) to ENTRY: {}", fmt_path( &all_dnas_path ), entity.id );
    entity.link_from( &all_dnas_hash, LT_NONE, TAG_DNA.into() )?;

    // Tag anchors
    if input.tags.is_some() {
	for tag in input.tags.unwrap() {
	    let (tag_path, tag_hash) = devhub_types::ensure_path( ANCHOR_TAGS, vec![ &tag.to_lowercase() ] )?;
	    debug!("Linking TAG anchor ({}) to entry: {}", fmt_path( &tag_path ), entity.id );
	    entity.link_from( &tag_hash, LT_NONE, TAG_DNA.into() )?;
	}
    }

    Ok( entity )
}




#[derive(Debug, Deserialize)]
pub struct GetDnaInput {
    pub id: EntryHash,
}

pub fn get_dna(input: GetDnaInput) -> AppResult<Entity<DnaInfo>> {
    debug!("Get DNA: {}", input.id );
    let entity = get_entity::<DnaEntry>( &input.id )?;

    Ok( entity.change_model( |dna| dna.to_info() ) )
}




#[derive(Debug, Deserialize, Clone)]
pub struct DnaUpdateOptions {
    pub name: Option<String>,
    pub description: Option<String>,
    pub tags: Option<Vec<String>>,
    pub icon: Option<SerializedBytes>,
    pub published_at: Option<u64>,
    pub last_updated: Option<u64>,
    pub metadata: Option<HashMap<String, serde_yaml::Value>>,
}
pub type DnaUpdateInput = UpdateEntityInput<DnaUpdateOptions>;

pub fn update_dna(input: DnaUpdateInput) -> AppResult<Entity<DnaInfo>> {
    debug!("Updating DNA: {}", input.addr );
    let props = input.properties.clone();
    let previous = get_entity::<DnaEntry>( &input.addr )?.content;

    let entity : Entity<DnaEntry> = update_entity(
	&input.addr,
	|current : DnaEntry, _| {
	    Ok(DnaEntry {
		name: props.name
		    .unwrap_or( current.name ),
		description: props.description
		    .unwrap_or( current.description ),
		icon: props.icon
		    .or( current.icon ),
		tags: props.tags
		    .or( current.tags ),
		published_at: props.published_at
		    .unwrap_or( current.published_at ),
		last_updated: props.last_updated
		    .unwrap_or( now()? ),
		developer: current.developer,
		deprecation: current.deprecation,
		metadata: props.metadata
		    .unwrap_or( current.metadata ),
	    })
	})?;

    if input.properties.name.is_some() {
	let (previous_name_path, previous_path_hash) = devhub_types::create_path( ANCHOR_NAMES, vec![ &previous.name ] );
	let (new_name_path, new_path_hash) = devhub_types::ensure_path( ANCHOR_NAMES, vec![ &entity.content.name ] )?;

	if previous_path_hash != new_path_hash {
	    debug!("Moving name link: {} -> {}", fmt_path( &previous_name_path ), fmt_path( &new_name_path ) );
	    entity.move_link_from( LT_NONE, TAG_DNA.into(), &previous_path_hash, &new_path_hash )?;
	}

	let (previous_name_path, previous_path_hash) = devhub_types::create_path( ANCHOR_NAMES, vec![ &previous.name.to_lowercase() ] );
	let (new_name_path, new_path_hash) = devhub_types::ensure_path( ANCHOR_NAMES, vec![ &entity.content.name.to_lowercase() ] )?;

	if previous_path_hash != new_path_hash {
	    debug!("Moving name (lowercase) link: {} -> {}", fmt_path( &previous_name_path ), fmt_path( &new_name_path ) );
	    entity.move_link_from( LT_NONE, TAG_DNA.into(), &previous_path_hash, &new_path_hash )?;
	}
    }

    devhub_types::update_tag_links( previous.tags, input.properties.tags, &entity, LT_NONE, TAG_DNA.into() )?;

    Ok( entity.change_model( |dna| dna.to_info() ) )
}




#[derive(Debug, Deserialize)]
pub struct DeprecateDnaInput {
    pub addr: EntryHash,
    pub message: String,
}

pub fn deprecate_dna(input: DeprecateDnaInput) -> AppResult<Entity<DnaInfo>> {
    debug!("Deprecating DNA: {}", input.addr );
    let entity : Entity<DnaEntry> = update_entity(
	&input.addr,
	|current : DnaEntry, _| {
	    Ok(DnaEntry {
		name: current.name,
		description: current.description,
		icon: current.icon,
		tags: current.tags,
		published_at: current.published_at,
		last_updated: current.last_updated,
		developer: current.developer,
		deprecation: Some(DeprecationNotice::new( input.message.to_owned() )),
		metadata: current.metadata,
	    })
	})?;

    Ok( entity.change_model( |dna| dna.to_info() ) )
}
