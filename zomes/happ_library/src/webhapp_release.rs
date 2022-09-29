use std::collections::BTreeMap;
use happs_core::{
    EntryTypes, LinkTypes,
};
use devhub_types::{
    AppResult, UpdateEntityInput, GetEntityInput,
    happ_entry_types::{
	HoloGUIConfig,
	WebHappReleaseEntry,
    },
};
use hc_crud::{
    now, create_entity, get_entity, update_entity, delete_entity,
    Entity,
};
use hdk::prelude::*;



#[derive(Debug, Deserialize)]
pub struct CreateInput {
    pub name: String,
    pub description: String,
    pub for_happ_release: EntryHash,
    pub web_asset_id: EntryHash,

    // optional
    pub holo_hosting_settings: Option<HoloGUIConfig>,
    pub published_at: Option<u64>,
    pub last_updated: Option<u64>,
    pub metadata: Option<BTreeMap<String, serde_yaml::Value>>,
    pub screenshots: Option<Vec<EntryHash>>,
}

pub fn create_webhapp_release(input: CreateInput) -> AppResult<Entity<WebHappReleaseEntry>> {
    debug!("Creating WebHapp release: {}", input.name );
    let default_now = now()?;

    let webhapp_release = WebHappReleaseEntry {
	name: input.name,
	description: input.description,
	for_happ_release: input.for_happ_release.clone(),
	web_asset_id: input.web_asset_id.clone(),
	holo_hosting_settings: input.holo_hosting_settings
	    .unwrap_or( HoloGUIConfig::default() ),
	published_at: input.published_at
	    .unwrap_or( default_now ),
	last_updated: input.last_updated
	    .unwrap_or( default_now ),
	metadata: input.metadata
	    .unwrap_or( BTreeMap::new() ),
	screenshots: input.screenshots.clone(),
    };

    let entity = create_entity( &webhapp_release )?;

    // Parent anchor
    debug!("Linking happ ({}) to ENTRY: {}", input.for_happ_release, entity.id );
    entity.link_from( &input.for_happ_release, LinkTypes::WebHappRelease, None )?;

    Ok( entity )
}


pub fn get_webhapp_release(input: GetEntityInput) -> AppResult<Entity<WebHappReleaseEntry>> {
    debug!("Get webhapp_release: {}", input.id );
    let entity : Entity<WebHappReleaseEntry> = get_entity( &input.id )?;

    Ok(	entity )
}


#[derive(Debug, Deserialize)]
pub struct WebHappReleaseUpdateOptions {
    pub name: Option<String>,
    pub description: Option<String>,
    pub holo_hosting_settings: Option<HoloGUIConfig>,
    pub published_at: Option<u64>,
    pub last_updated: Option<u64>,
    pub metadata: Option<BTreeMap<String, serde_yaml::Value>>,
    pub screenshots: Option<Vec<EntryHash>>,
}
pub type WebHappReleaseUpdateInput = UpdateEntityInput<WebHappReleaseUpdateOptions>;

pub fn update_webhapp_release(input: WebHappReleaseUpdateInput) -> AppResult<Entity<WebHappReleaseEntry>> {
    debug!("Updating hApp: {}", input.addr );
    let props = input.properties;

    let entity = update_entity(
	&input.addr,
	|current : WebHappReleaseEntry, _| {
	    Ok(WebHappReleaseEntry {
		name: props.name
		    .unwrap_or( current.name ),
		description: props.description
		    .unwrap_or( current.description ),
		for_happ_release: current.for_happ_release,
		web_asset_id: current.web_asset_id,
		holo_hosting_settings: props.holo_hosting_settings
		    .unwrap_or( current.holo_hosting_settings ),
		published_at: props.published_at
		    .unwrap_or( current.published_at ),
		last_updated: props.last_updated
		    .unwrap_or( now()? ),
		metadata: props.metadata
		    .unwrap_or( current.metadata ),
		screenshots: props.screenshots
		    .or( current.screenshots ),
	    })
	})?;

    Ok(	entity )
}



#[derive(Debug, Deserialize)]
pub struct DeleteInput {
    pub id: EntryHash,
}

pub fn delete_webhapp_release(input: DeleteInput) -> AppResult<ActionHash> {
    debug!("Delete WebHapp release Version: {}", input.id );
    let delete_action = delete_entity::<WebHappReleaseEntry,EntryTypes>( &input.id )?;
    debug!("Deleted WebHapp release via action ({})", delete_action );

    Ok( delete_action )
}
