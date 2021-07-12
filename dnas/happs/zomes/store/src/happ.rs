use devhub_types::{
    AppResult,
    happ_entry_types::{ HappEntry, HappInfo, DeprecationNotice, HappGUIConfig },
};
use hc_entities::{ Entity, UpdateEntityInput, GetEntityInput };
use hdk::prelude::*;
use hc_dna_utils as utils;

use crate::constants::{ TAG_HAPP };



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
    pub thumbnail_image: Option<SerializedBytes>,
    pub published_at: Option<u64>,
    pub last_updated: Option<u64>,
    pub gui: Option<GUIConfigInput>,
}


pub fn create_happ(input: CreateInput) -> AppResult<Entity<HappInfo>> {
    debug!("Creating HAPP: {}", input.title );
    let pubkey = agent_info()?.agent_initial_pubkey;
    let default_now = utils::now()?;

    // if true {
    // 	return Err( UserError::DuplicateHappName(input.title).into() );
    // }

    let happ = HappEntry {
	title: input.title,
	subtitle: input.subtitle,
	description: input.description,
	designer: pubkey.clone(),
	published_at: input.published_at
	    .unwrap_or( default_now ),
	last_updated: input.last_updated
	    .unwrap_or( default_now ),
	thumbnail_image: input.thumbnail_image,
	deprecation: None,
	gui: input.gui.map(|gui| {
	    HappGUIConfig::new( gui.asset_group_id, gui.uses_web_sdk )
	}),
    };

    let entity = utils::create_entity( &happ )?
	.new_content( happ.to_info() );

    debug!("Linking pubkey ({}) to ENTRY: {}", pubkey, entity.id );
    create_link(
	pubkey.into(),
	entity.id.clone(),
	LinkTag::new( TAG_HAPP )
    )?;

    Ok( entity )
}


pub fn get_happ(input: GetEntityInput) -> AppResult<Entity<HappInfo>> {
    debug!("Get hApp: {}", input.id );
    let entity = utils::get_entity( &input.id )?;
    let info = HappEntry::try_from( &entity.content )?.to_info();

    Ok(	entity.new_content( info ) )
}


#[derive(Debug, Deserialize)]
pub struct HappUpdateOptions {
    pub title: Option<String>,
    pub subtitle: Option<String>,
    pub description: Option<String>,
    pub thumbnail_image: Option<SerializedBytes>,
    pub published_at: Option<u64>,
    pub last_updated: Option<u64>,
    pub gui: Option<HappGUIConfig>,
}
pub type HappUpdateInput = UpdateEntityInput<HappUpdateOptions>;

pub fn update_happ(input: HappUpdateInput) -> AppResult<Entity<HappInfo>> {
    debug!("Updating hApp: {}", input.addr );
    let props = input.properties;

    let entity : Entity<HappEntry> = utils::update_entity(
	input.id, input.addr,
	|element| {
	    let current = HappEntry::try_from( &element )?;

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
		    .unwrap_or( utils::now()? ),
		thumbnail_image: props.thumbnail_image
		    .or( current.thumbnail_image ),
		deprecation: current.deprecation,
		gui: props.gui
		    .or( current.gui ),
	    })
	})?;

    let info = entity.content.to_info();

    Ok( entity.new_content( info ) )
}


#[derive(Debug, Deserialize)]
pub struct HappDeprecateInput {
    pub id: Option<EntryHash>,
    pub addr: EntryHash,
    pub message: String,
}

pub fn deprecate_happ(input: HappDeprecateInput) -> AppResult<Entity<HappInfo>> {
    debug!("Deprecating hApp: {}", input.addr );
    let entity : Entity<HappEntry> = utils::update_entity(
	input.id.clone(), input.addr.clone(),
	|element| {
	    let mut current = HappEntry::try_from( &element )?;

	    current.deprecation = Some(DeprecationNotice {
		message: input.message,
		recommended_alternatives: None,
	    });

	    Ok( current )
	})?;

    let info = entity.content.to_info();

    Ok( entity.new_content( info ) )
}
