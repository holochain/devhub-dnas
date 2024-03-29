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
	DnaEntry,
	DnaVersionEntry,
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
    ANCHOR_DNAS,
};



#[derive(Debug, Deserialize)]
pub struct DnaInput {
    pub name: String,
    pub description: String,

    // optional
    pub display_name: Option<String>,
    pub tags: Option<Vec<String>>,
    pub icon: Option<SerializedBytes>,
    pub published_at: Option<u64>,
    pub last_updated: Option<u64>,
    pub source_code_url: Option<String>,
    pub metadata: Option<BTreeMap<String, serde_yaml::Value>>,
}

pub fn create_dna(input: DnaInput) -> AppResult<Entity<DnaEntry>> {
    debug!("Creating DNA: {}", input.name );
    let pubkey = agent_info()?.agent_initial_pubkey;
    let default_now = now()?;

    let (name_path, name_path_hash) = devhub_types::create_path( ANCHOR_NAMES, vec![ &input.name ] );
    let (name_path_lc, name_path_lc_hash) = devhub_types::create_path( ANCHOR_NAMES, vec![ &input.name.to_lowercase() ] );

    let dna = DnaEntry {
	name: input.name,
	display_name: input.display_name,
	description: input.description,
	icon: input.icon,
	tags: input.tags.to_owned(),
	published_at: input.published_at
	    .unwrap_or( default_now ),
	last_updated: input.last_updated
	    .unwrap_or( default_now ),
	developer: pubkey.clone(),
	source_code_url: input.source_code_url,
	deprecation: None,
	metadata: input.metadata
	    .unwrap_or( BTreeMap::new() ),
    };

    let entity = create_entity( &dna )?;

    // Developer (Agent) anchor
    let (agent_base, agent_base_hash) = devhub_types::create_path( &crate::agent_path_base( None ), vec![ ANCHOR_DNAS ]);
    debug!("Linking agent ({}) to ENTRY: {}", fmt_path( &agent_base ), entity.id );
    entity.link_from( &agent_base_hash, LinkTypes::Dna, None )?;

    // Name anchors (case sensitive/insensitive)
    debug!("Linking name path ({}) to ENTRY: {}", fmt_path( &name_path ), entity.id );
    entity.link_from( &name_path_hash, LinkTypes::Dna, None )?;

    if name_path_lc != name_path {
	debug!("Linking name (lowercase) path ({}) to ENTRY: {}", fmt_path( &name_path_lc ), entity.id );
	entity.link_from( &name_path_lc_hash, LinkTypes::Dna, None )?;
    }

    // Global anchor
    let (all_dnas_path, all_dnas_hash) = devhub_types::create_path( ANCHOR_DNAS, Vec::<String>::new() );
    debug!("Linking all DNAs path ({}) to ENTRY: {}", fmt_path( &all_dnas_path ), entity.id );
    entity.link_from( &all_dnas_hash, LinkTypes::Dna, None )?;

    // Tag anchors
    if input.tags.is_some() {
	for tag in input.tags.unwrap() {
	    let (tag_path, tag_hash) = devhub_types::create_path( ANCHOR_TAGS, vec![ &tag.to_lowercase() ] );
	    debug!("Linking TAG anchor ({}) to entry: {}", fmt_path( &tag_path ), entity.id );
	    entity.link_from( &tag_hash, LinkTypes::Dna, None )?;
	}
    }

    Ok( entity )
}




#[derive(Debug, Deserialize)]
pub struct GetDnaInput {
    pub id: ActionHash,
}

pub fn get_dna(input: GetDnaInput) -> AppResult<Entity<DnaEntry>> {
    debug!("Get DNA: {}", input.id );
    let entity = get_entity( &input.id )?;

    Ok( entity )
}




#[derive(Debug, Deserialize, Clone)]
pub struct DnaUpdateOptions {
    pub name: Option<String>,
    pub display_name: Option<String>,
    pub description: Option<String>,
    pub tags: Option<Vec<String>>,
    pub icon: Option<SerializedBytes>,
    pub published_at: Option<u64>,
    pub last_updated: Option<u64>,
    pub source_code_url: Option<String>,
    pub metadata: Option<BTreeMap<String, serde_yaml::Value>>,
}
pub type DnaUpdateInput = UpdateEntityInput<DnaUpdateOptions>;

pub fn update_dna(input: DnaUpdateInput) -> AppResult<Entity<DnaEntry>> {
    debug!("Updating DNA: {}", input.addr );
    let props = input.properties.clone();
    let mut previous : Option<DnaEntry> = None;

    let entity : Entity<DnaEntry> = update_entity(
	&input.addr,
	|current : DnaEntry, _| {
	    previous = Some(current.clone());

	    Ok(DnaEntry {
		name: props.name
		    .unwrap_or( current.name ),
		display_name: props.display_name
		    .or( current.display_name ),
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
		source_code_url: props.source_code_url
		    .or( current.source_code_url ),
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
	    entity.move_link_from( LinkTypes::Dna, None, &previous_path_hash, &new_path_hash ).ok();
	}

	let (previous_name_path, previous_path_hash) = devhub_types::create_path( ANCHOR_NAMES, vec![ &previous.name.to_lowercase() ] );
	let (new_name_path, new_path_hash) = devhub_types::create_path( ANCHOR_NAMES, vec![ &entity.content.name.to_lowercase() ] );

	if previous_path_hash != new_path_hash {
	    debug!("Moving name (lowercase) link: {} -> {}", fmt_path( &previous_name_path ), fmt_path( &new_name_path ) );
	    entity.move_link_from( LinkTypes::Dna, None, &previous_path_hash, &new_path_hash ).ok();
	}
    }

    devhub_types::update_tag_links( previous.tags, input.properties.tags, &entity, LinkTypes::Dna, LinkTypes::Tag )?;

    Ok( entity )
}




#[derive(Debug, Deserialize)]
pub struct DeprecateDnaInput {
    pub addr: ActionHash,
    pub message: String,
}

pub fn deprecate_dna(input: DeprecateDnaInput) -> AppResult<Entity<DnaEntry>> {
    debug!("Deprecating DNA: {}", input.addr );
    let entity : Entity<DnaEntry> = update_entity(
	&input.addr,
	|current : DnaEntry, _| {
	    Ok(DnaEntry {
		name: current.name,
		display_name: current.display_name,
		description: current.description,
		icon: current.icon,
		tags: current.tags,
		published_at: current.published_at,
		last_updated: current.last_updated,
		developer: current.developer,
		deprecation: Some(DeprecationNotice::new( input.message.to_owned() )),
		source_code_url: current.source_code_url,
		metadata: current.metadata,
	    })
	})?;

    Ok( entity )
}

pub fn get_dnas_with_an_hdk_version( input: String ) -> AppResult<Vec<Entity<DnaEntry>>> {
    let collection : Vec<Entity<DnaVersionEntry>> = devhub_types::get_hdk_version_entities( LinkTypes::DnaVersion, input ).unwrap();

    let mut dnas : BTreeMap<ActionHash, Entity<DnaEntry>> = BTreeMap::new();

    for dna_version in collection.into_iter() {
	let dna = get_entity( &dna_version.content.for_dna )?;

	if !dnas.contains_key( &dna.id ) {
	    dnas.insert( dna.id.to_owned(), dna );
	}
    }

    Ok(dnas.into_values().collect())
}
