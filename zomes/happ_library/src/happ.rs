use std::collections::HashMap;
use devhub_types::{
    AppResult, UpdateEntityInput, GetEntityInput,
    happ_entry_types::{
	HappEntry, HappInfo, HappSummary,
	DeprecationNotice, HappGUIConfig,
    },
};
use hc_crud::{
    now, create_entity, get_entity, update_entity,
    Entity, Collection,
};
use hdk::prelude::*;

use crate::constants::{ TAG_HAPP };



fn happ_title_path(title: &str) -> AppResult<Path> {
    Ok( create_filter_path( "title", title )? )
}

fn filter_path(filter: &str, value: &str) -> AppResult<Path> {
    Ok( hc_crud::path_from_collection( vec![ "happs_by", filter, value ] )? )
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
    pub title: String,
    pub subtitle: String,
    pub description: String,

    // optional
    pub icon: Option<SerializedBytes>,
    pub published_at: Option<u64>,
    pub last_updated: Option<u64>,
    pub gui: Option<GUIConfigInput>,
    pub metadata: Option<HashMap<String, serde_yaml::Value>>,
}


pub fn create_happ(input: CreateInput) -> AppResult<Entity<HappInfo>> {
    debug!("Creating HAPP: {}", input.title );
    let pubkey = agent_info()?.agent_initial_pubkey;
    let default_now = now()?;

    // if true {
    // 	return Err( UserError::DuplicateHappName(input.title).into() );
    // }

    let title_path = happ_title_path( &input.title )?;
    let title_path_hash = title_path.path_entry_hash()?;

    let title_path_lc = happ_title_path( &input.title.to_lowercase() )?;
    let title_path_lc_hash = title_path_lc.path_entry_hash()?;

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
	deprecation: None,
	gui: input.gui.map(|gui| {
	    HappGUIConfig::new( gui.asset_group_id, gui.uses_web_sdk )
	}),
	metadata: input.metadata
	    .unwrap_or( HashMap::new() ),
    };

    let entity = create_entity( &happ )?
	.change_model( |happ| happ.to_info() );
    let base = crate::root_path_hash( None )?;

    debug!("Linking pubkey ({}) to ENTRY: {}", base, entity.id );
    entity.link_from( &base, TAG_HAPP.into() )?;

    debug!("Linking 'title' path (length {}) to entity: {}", title_path.as_ref().len(), entity.id );
    entity.link_from( &title_path_hash, TAG_HAPP.into() )?;

    debug!("Linking lowercase 'title' path ({}) to ENTRY: {}", title_path_lc_hash, entity.id );
    entity.link_from( &title_path_lc_hash, TAG_HAPP.into() )?;

    let all_happs_path = crate::all_happs_path();
    let all_happs_hash = all_happs_path.path_entry_hash()?;
    all_happs_path.ensure()?;
    debug!("Linking all hApp path ({}) to ENTRY: {}", all_happs_hash, entity.id );
    entity.link_from( &all_happs_hash, TAG_HAPP.into() )?;

    Ok( entity )
}


pub fn get_happ(input: GetEntityInput) -> AppResult<Entity<HappInfo>> {
    debug!("Get hApp: {}", input.id );
    let entity = get_entity::<HappEntry>( &input.id )?;

    Ok(	entity.change_model( |happ| happ.to_info() ) )
}


#[derive(Debug, Deserialize)]
pub struct HappUpdateOptions {
    pub title: Option<String>,
    pub subtitle: Option<String>,
    pub description: Option<String>,
    pub icon: Option<SerializedBytes>,
    pub published_at: Option<u64>,
    pub last_updated: Option<u64>,
    pub gui: Option<HappGUIConfig>,
    pub metadata: Option<HashMap<String, serde_yaml::Value>>,
}
pub type HappUpdateInput = UpdateEntityInput<HappUpdateOptions>;

pub fn update_happ(input: HappUpdateInput) -> AppResult<Entity<HappInfo>> {
    debug!("Updating hApp: {}", input.addr );
    let props = input.properties;
    let mut previous_title = String::from("");

    let entity = update_entity(
	&input.addr,
	|current : HappEntry, _| {
	    previous_title = current.title.clone();

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
		deprecation: current.deprecation,
		gui: props.gui
		    .or( current.gui ),
		metadata: props.metadata
		    .unwrap_or( current.metadata ),
	    })
	})?;

    let previous_title_path = happ_title_path( &previous_title )?;
    let previous_path_hash = previous_title_path.path_entry_hash()?;

    let new_title_path = happ_title_path( &entity.content.title )?;
    let new_path_hash = new_title_path.path_entry_hash()?;

    entity.move_link_from( TAG_HAPP.into(), &previous_path_hash, &new_path_hash )?;

    let previous_title_path = happ_title_path( &previous_title.to_lowercase() )?;
    let previous_path_hash = previous_title_path.path_entry_hash()?;

    let new_title_path = happ_title_path( &entity.content.title.to_lowercase() )?;
    let new_path_hash = new_title_path.path_entry_hash()?;

    entity.move_link_from( TAG_HAPP.into(), &previous_path_hash, &new_path_hash )?;

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

fn get_entities_for_links ( links: Vec<Link> ) -> Vec<Entity<HappSummary>> {
    links.into_iter()
	.filter_map(|link| {
	    get_entity::<HappEntry>( &link.target ).ok()
	})
	.map( |entity| {
	    entity.change_model( |happ| happ.to_summary() )
	})
	.collect()
}

pub fn get_happ_collection(maybe_pubkey: Option<AgentPubKey>) -> AppResult<Collection<Entity<HappSummary>>> {
    let base = crate::root_path_hash( maybe_pubkey )?;

    debug!("Getting hApp links for Agent entry: {}", base );
    let all_links = get_links(
        base.clone(),
	Some(LinkTag::new(TAG_HAPP))
    )?;

    let happs = get_entities_for_links( all_links );

    Ok(Collection {
	base,
	items: happs,
    })
}

#[derive(Debug, Deserialize)]
pub struct GetHappsInput {
    pub agent: Option<AgentPubKey>,
}

pub fn get_happs(input: GetHappsInput) -> AppResult<Collection<Entity<HappSummary>>> {
    let happ_collection = get_happ_collection( input.agent.clone() )?;

    let happs = happ_collection.items.into_iter()
	.filter(|entity| {
	    !entity.content.deprecation
	})
	.collect();

    Ok(Collection {
	base: happ_collection.base,
	items: happs,
    })
}

pub fn get_my_happs() -> AppResult<Collection<Entity<HappSummary>>> {
    get_happs(GetHappsInput {
	agent: None,
    })
}


pub fn get_happs_by_filter( filter: String, keyword: String ) -> AppResult<Collection<Entity<HappSummary>>> {
    let base = filter_path( &filter, &keyword )?.path_entry_hash()?;

    debug!("Getting hApp links for base: {:?}", base );
    let all_links = get_links(
        base.clone(),
	Some(LinkTag::new(TAG_HAPP))
    )?;

    let happs = get_entities_for_links( all_links );

    Ok(Collection {
	base,
	items: happs,
    })
}


pub fn get_all_happs() -> AppResult<Collection<Entity<HappSummary>>> {
    let base = crate::all_happs_path().path_entry_hash()?;

    debug!("Getting hApp links for base: {}", base );
    let links = get_links(
        base.clone(),
	Some(LinkTag::new(TAG_HAPP))
    )?;

    let happs = get_entities_for_links( links );

    Ok(Collection {
	base,
	items: happs,
    })
}
