use std::collections::BTreeMap;
use happs_core::{
    LinkTypes,
};
use devhub_types::{
    AppResult, UpdateEntityInput, GetEntityInput,
    happ_entry_types::{
	HoloGUIConfig,
	GUIEntry,
	DeprecationNotice,
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
    ANCHOR_GUIS,
};



#[derive(Debug, Deserialize)]
pub struct CreateInput {
    pub name: String,
    pub description: String,

    // optional
    pub holo_hosting_settings: Option<HoloGUIConfig>,
    pub tags: Option<Vec<String>>,
    pub screenshots: Option<Vec<EntryHash>>,
    pub published_at: Option<u64>,
    pub last_updated: Option<u64>,
    pub metadata: Option<BTreeMap<String, serde_yaml::Value>>,
}


pub fn create_gui(input: CreateInput) -> AppResult<Entity<GUIEntry>> {
    debug!("Creating GUI: {}", input.name );
    let pubkey = agent_info()?.agent_initial_pubkey;
    let default_now = now()?;

    // if true {
    // 	return Err( UserError::DuplicateGUIName(input.name).into() );
    // }

    let (name_path, name_path_hash) = devhub_types::create_path( ANCHOR_NAMES, vec![ &input.name ] );
    let (name_path_lc, name_path_lc_hash) = devhub_types::create_path( ANCHOR_NAMES, vec![ &input.name.to_lowercase() ] );

    let gui = GUIEntry {
	name: input.name,
	description: input.description,
	designer: pubkey.clone(),
	published_at: input.published_at
	    .unwrap_or( default_now ),
	last_updated: input.last_updated
	    .unwrap_or( default_now ),
	holo_hosting_settings: input.holo_hosting_settings
	    .unwrap_or( HoloGUIConfig::default() ),
	tags: input.tags.to_owned(),
	screenshots: input.screenshots,
	deprecation: None,
	metadata: input.metadata
	    .unwrap_or( BTreeMap::new() ),
    };

    let entity = create_entity( &gui )?;

    // Designer (Agent) anchor
    let (agent_base, agent_base_hash) = devhub_types::create_path( &crate::agent_path_base( None ), vec![ ANCHOR_GUIS ]);
    debug!("Linking agent ({}) to entity: {}", fmt_path( &agent_base ), entity.id );
    entity.link_from( &agent_base_hash, LinkTypes::GUI, None )?;

    // Name anchors (case sensitive/insensitive)
    debug!("Linking name path ({}) to entity: {}", fmt_path( &name_path ), entity.id );
    entity.link_from( &name_path_hash, LinkTypes::GUI, None )?;

    if name_path_lc != name_path {
	debug!("Linking name (lowercase) path ({}) to entity: {}", fmt_path( &name_path_lc ), entity.id );
	entity.link_from( &name_path_lc_hash, LinkTypes::GUI, None )?;
    }

    // Global anchor
    let (all_guis_path, all_guis_hash) = devhub_types::create_path( ANCHOR_GUIS, Vec::<String>::new() );
    debug!("Linking all guis path ({}) to ENTRY: {}", fmt_path( &all_guis_path ), entity.id );
    entity.link_from( &all_guis_hash, LinkTypes::GUI, None )?;

    // Tag anchors
    if input.tags.is_some() {
	for tag in input.tags.unwrap() {
	    let (tag_path, tag_hash) = devhub_types::create_path( ANCHOR_TAGS, vec![ &tag.to_lowercase() ] );
	    debug!("Linking TAG anchor ({}) to entry: {}", fmt_path( &tag_path ), entity.id );
	    entity.link_from( &tag_hash, LinkTypes::GUI, None )?;
	}
    }

    Ok( entity )
}


pub fn get_gui(input: GetEntityInput) -> AppResult<Entity<GUIEntry>> {
    debug!("Get gui: {}", input.id );
    let entity : Entity<GUIEntry> = get_entity( &input.id )?;

    Ok(	entity )
}


#[derive(Debug, Deserialize, Clone)]
pub struct GUIUpdateOptions {
    pub name: Option<String>,
    pub description: Option<String>,
    pub holo_hosting_settings: Option<HoloGUIConfig>,
    pub tags: Option<Vec<String>>,
    pub screenshots: Option<Vec<EntryHash>>,
    pub published_at: Option<u64>,
    pub last_updated: Option<u64>,
    pub metadata: Option<BTreeMap<String, serde_yaml::Value>>,
}
pub type GUIUpdateInput = UpdateEntityInput<GUIUpdateOptions>;

pub fn update_gui(input: GUIUpdateInput) -> AppResult<Entity<GUIEntry>> {
    debug!("Updating gui: {}", input.addr );
    let props = input.properties.clone();
    let mut previous : Option<GUIEntry> = None;

    let entity = update_entity(
	&input.addr,
	|current : GUIEntry, _| {
	    previous = Some(current.clone());

	    Ok(GUIEntry {
		name: props.name
		    .unwrap_or( current.name ),
		description: props.description
		    .unwrap_or( current.description ),
		designer: current.designer,
		holo_hosting_settings: props.holo_hosting_settings
		    .unwrap_or( current.holo_hosting_settings ),
		published_at: props.published_at
		    .unwrap_or( current.published_at ),
		last_updated: props.last_updated
		    .unwrap_or( now()? ),
		tags: props.tags
		    .or( current.tags ),
		screenshots: props.screenshots
		    .or( current.screenshots ),
		deprecation: current.deprecation,
		metadata: props.metadata
		    .unwrap_or( current.metadata ),
	    })
	})?;

    let previous = previous.unwrap();

    if input.properties.name.is_some() {
	let (previous_name_path, previous_path_hash) = devhub_types::create_path( ANCHOR_NAMES, vec![ &previous.name ] );
	let (new_name_path, new_path_hash) = devhub_types::create_path( ANCHOR_NAMES, vec![ &entity.content.name ] );

	if previous_path_hash != new_path_hash {
	    debug!("Moving name link: {} -> {}", fmt_path( &previous_name_path ), fmt_path( &new_name_path ) );
	    entity.move_link_from( LinkTypes::GUI, None, &previous_path_hash, &new_path_hash )?;
	}

	let (previous_name_path, previous_path_hash) = devhub_types::create_path( ANCHOR_NAMES, vec![ &previous.name.to_lowercase() ] );
	let (new_name_path, new_path_hash) = devhub_types::create_path( ANCHOR_NAMES, vec![ &entity.content.name.to_lowercase() ] );

	if previous_path_hash != new_path_hash {
	    debug!("Moving name (lowercase) link: {} -> {}", fmt_path( &previous_name_path ), fmt_path( &new_name_path ) );
	    entity.move_link_from( LinkTypes::GUI, None, &previous_path_hash, &new_path_hash )?;
	}
    }

    devhub_types::update_tag_links( previous.tags, input.properties.tags, &entity, LinkTypes::GUI, LinkTypes::Tag )?;

    Ok( entity )
}


#[derive(Debug, Deserialize)]
pub struct GUIDeprecateInput {
    pub addr: ActionHash,
    pub message: String,
}

pub fn deprecate_gui(input: GUIDeprecateInput) -> AppResult<Entity<GUIEntry>> {
    debug!("Deprecating gui: {}", input.addr );
    let entity = update_entity(
	&input.addr,
	|mut current : GUIEntry, _| {
	    current.deprecation = Some(DeprecationNotice {
		message: input.message.to_owned(),
		recommended_alternatives: None,
	    });

	    Ok( current )
	})?;

    Ok( entity )
}
