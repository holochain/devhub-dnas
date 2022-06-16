use std::collections::BTreeMap;
use devhub_types::{
    AppResult, UpdateEntityInput, GetEntityInput,
    happ_entry_types::{
	HappEntry, HappInfo,
	DeprecationNotice,
    },
    constants::{
	ANCHOR_TAGS,
	ANCHOR_TITLES,
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
    TAG_HAPP,
    ANCHOR_HAPPS,
};



#[derive(Debug, Deserialize)]
pub struct CreateInput {
    pub title: String,
    pub subtitle: String,
    pub description: String,

    // optional
    pub tags: Option<Vec<String>>,
    pub icon: Option<SerializedBytes>,
    pub published_at: Option<u64>,
    pub last_updated: Option<u64>,
    pub metadata: Option<BTreeMap<String, serde_yaml::Value>>,
}


pub fn create_happ(input: CreateInput) -> AppResult<Entity<HappInfo>> {
    debug!("Creating HAPP: {}", input.title );
    let pubkey = agent_info()?.agent_initial_pubkey;
    let default_now = now()?;

    // if true {
    // 	return Err( UserError::DuplicateHappName(input.title).into() );
    // }

    let (title_path, title_path_hash) = devhub_types::ensure_path( ANCHOR_TITLES, vec![ &input.title ] )?;
    let (title_path_lc, title_path_lc_hash) = devhub_types::ensure_path( ANCHOR_TITLES, vec![ &input.title.to_lowercase() ] )?;

    let happ = HappEntry {
	title: input.title,
	subtitle: input.subtitle,
	description: input.description,
	designer: pubkey.clone(),
	published_at: input.published_at
	    .unwrap_or( default_now ),
	last_updated: input.last_updated
	    .unwrap_or( default_now ),
	icon: input.icon,
	tags: input.tags.to_owned(),
	deprecation: None,
	metadata: input.metadata
	    .unwrap_or( BTreeMap::new() ),
    };

    let entity = create_entity( &happ )?
	.change_model( |happ| happ.to_info() );

    // Designer (Agent) anchor
    let (agent_base, agent_base_hash) = devhub_types::ensure_path( &crate::agent_path_base( None ), vec![ ANCHOR_HAPPS ] )?;
    debug!("Linking agent ({}) to entity: {}", fmt_path( &agent_base ), entity.id );
    entity.link_from( &agent_base_hash, LT_NONE, TAG_HAPP.into() )?;

    // Title anchors (case sensitive/insensitive)
    debug!("Linking title path ({}) to entity: {}", fmt_path( &title_path ), entity.id );
    entity.link_from( &title_path_hash, LT_NONE, TAG_HAPP.into() )?;

    if title_path_lc != title_path {
	debug!("Linking title (lowercase) path ({}) to entity: {}", fmt_path( &title_path_lc ), entity.id );
	entity.link_from( &title_path_lc_hash, LT_NONE, TAG_HAPP.into() )?;
    }

    // Global anchor
    let (all_happs_path, all_happs_hash) = devhub_types::ensure_path( ANCHOR_HAPPS, Vec::<String>::new() )?;
    debug!("Linking all hApps path ({}) to ENTRY: {}", fmt_path( &all_happs_path ), entity.id );
    entity.link_from( &all_happs_hash, LT_NONE, TAG_HAPP.into() )?;

    // Tag anchors
    if input.tags.is_some() {
	for tag in input.tags.unwrap() {
	    let (tag_path, tag_hash) = devhub_types::ensure_path( ANCHOR_TAGS, vec![ &tag.to_lowercase() ] )?;
	    debug!("Linking TAG anchor ({}) to entry: {}", fmt_path( &tag_path ), entity.id );
	    entity.link_from( &tag_hash, LT_NONE, TAG_HAPP.into() )?;
	}
    }

    Ok( entity )
}


pub fn get_happ(input: GetEntityInput) -> AppResult<Entity<HappInfo>> {
    debug!("Get hApp: {}", input.id );
    let entity = get_entity::<HappEntry>( &input.id )?;

    Ok(	entity.change_model( |happ| happ.to_info() ) )
}


#[derive(Debug, Deserialize, Clone)]
pub struct HappUpdateOptions {
    pub title: Option<String>,
    pub subtitle: Option<String>,
    pub description: Option<String>,
    pub tags: Option<Vec<String>>,
    pub icon: Option<SerializedBytes>,
    pub published_at: Option<u64>,
    pub last_updated: Option<u64>,
    pub metadata: Option<BTreeMap<String, serde_yaml::Value>>,
}
pub type HappUpdateInput = UpdateEntityInput<HappUpdateOptions>;

pub fn update_happ(input: HappUpdateInput) -> AppResult<Entity<HappInfo>> {
    debug!("Updating hApp: {}", input.addr );
    let props = input.properties.clone();
    let previous = get_entity::<HappEntry>( &input.addr )?.content;

    let entity = update_entity(
	&input.addr,
	|current : HappEntry, _| {
	    Ok(HappEntry {
		title: props.title
		    .unwrap_or( current.title ),
		subtitle: props.subtitle
		    .unwrap_or( current.subtitle ),
		description: props.description
		    .unwrap_or( current.description ),
		designer: current.designer,
		published_at: props.published_at
		    .unwrap_or( current.published_at ),
		last_updated: props.last_updated
		    .unwrap_or( now()? ),
		icon: props.icon
		    .or( current.icon ),
		tags: props.tags
		    .or( current.tags ),
		deprecation: current.deprecation,
		metadata: props.metadata
		    .unwrap_or( current.metadata ),
	    })
	})?;

    if input.properties.title.is_some() {
	let (previous_title_path, previous_path_hash) = devhub_types::create_path( ANCHOR_TITLES, vec![ &previous.title ] );
	let (new_title_path, new_path_hash) = devhub_types::ensure_path( ANCHOR_TITLES, vec![ &entity.content.title ] )?;

	if previous_path_hash != new_path_hash {
	    debug!("Moving title link: {} -> {}", fmt_path( &previous_title_path ), fmt_path( &new_title_path ) );
	    entity.move_link_from( LT_NONE, TAG_HAPP.into(), &previous_path_hash, &new_path_hash )?;
	}

	let (previous_title_path, previous_path_hash) = devhub_types::create_path( ANCHOR_TITLES, vec![ &previous.title.to_lowercase() ] );
	let (new_title_path, new_path_hash) = devhub_types::ensure_path( ANCHOR_TITLES, vec![ &entity.content.title.to_lowercase() ] )?;

	if previous_path_hash != new_path_hash {
	    debug!("Moving title (lowercase) link: {} -> {}", fmt_path( &previous_title_path ), fmt_path( &new_title_path ) );
	    entity.move_link_from( LT_NONE, TAG_HAPP.into(), &previous_path_hash, &new_path_hash )?;
	}
    }

    devhub_types::update_tag_links( previous.tags, input.properties.tags, &entity, LT_NONE, TAG_HAPP.into() )?;

    Ok( entity.change_model( |happ| happ.to_info() ) )
}


#[derive(Debug, Deserialize)]
pub struct HappDeprecateInput {
    pub addr: EntryHash,
    pub message: String,
}

pub fn deprecate_happ(input: HappDeprecateInput) -> AppResult<Entity<HappInfo>> {
    debug!("Deprecating hApp: {}", input.addr );
    let entity = update_entity(
	&input.addr,
	|mut current : HappEntry, _| {
	    current.deprecation = Some(DeprecationNotice {
		message: input.message.to_owned(),
		recommended_alternatives: None,
	    });

	    Ok( current )
	})?;

    Ok( entity.change_model( |happ| happ.to_info() ) )
}
