use std::collections::BTreeMap;
use happs_core::{
    EntryTypes, LinkTypes,
};
use devhub_types::{
    AppResult, UpdateEntityInput, GetEntityInput,
    happ_entry_types::{
	GUIReleaseEntry,
    },
};
use hc_crud::{
    now, create_entity, get_entity, update_entity, delete_entity, get_entities,
    Entity,
};
use hdk::prelude::*;



#[derive(Debug, Deserialize)]
pub struct CreateInput {
    pub version: String,
    pub changelog: String,
    pub for_gui: ActionHash,
    pub for_happ_releases: Vec<ActionHash>,
    pub web_asset_id: ActionHash,

    // optional
    pub screenshots: Option<Vec<EntryHash>>,
    pub published_at: Option<u64>,
    pub last_updated: Option<u64>,
    pub metadata: Option<BTreeMap<String, serde_yaml::Value>>,
}

pub fn create_gui_release(input: CreateInput) -> AppResult<Entity<GUIReleaseEntry>> {
    debug!("Creating GUI release: {}", input.version );
    let default_now = now()?;

    let gui_release = GUIReleaseEntry {
	version: input.version,
	changelog: input.changelog,
	for_gui: input.for_gui.clone(),
	for_happ_releases: input.for_happ_releases.clone(),
	web_asset_id: input.web_asset_id.clone(),
	published_at: input.published_at
	    .unwrap_or( default_now ),
	last_updated: input.last_updated
	    .unwrap_or( default_now ),
	metadata: input.metadata
	    .unwrap_or( BTreeMap::new() ),
	screenshots: input.screenshots,
    };

    let entity = create_entity( &gui_release )?;

    // Parent anchor
    debug!("Linking gui ({}) to ENTRY: {}", input.for_gui, entity.id );
    entity.link_from( &input.for_gui, LinkTypes::GUIRelease, None )?;

    // Happ Releases
    for happ_release_id in input.for_happ_releases {
	debug!("Linking happ ({}) to ENTRY: {}", happ_release_id, entity.id );
	entity.link_from( &happ_release_id, LinkTypes::GUIRelease, None )?;
    }

    Ok( entity )
}


pub fn get_gui_release(input: GetEntityInput) -> AppResult<Entity<GUIReleaseEntry>> {
    debug!("Get gui_release: {}", input.id );
    let entity : Entity<GUIReleaseEntry> = get_entity( &input.id )?;

    Ok(	entity )
}


#[derive(Debug, Deserialize)]
pub struct GUIReleaseUpdateOptions {
    pub version: Option<String>,
    pub changelog: Option<String>,
    pub for_happ_releases: Option<Vec<ActionHash>>,
    pub screenshots: Option<Vec<EntryHash>>,
    pub published_at: Option<u64>,
    pub last_updated: Option<u64>,
    pub metadata: Option<BTreeMap<String, serde_yaml::Value>>,
}
pub type GUIReleaseUpdateInput = UpdateEntityInput<GUIReleaseUpdateOptions>;

pub fn update_gui_release(input: GUIReleaseUpdateInput) -> AppResult<Entity<GUIReleaseEntry>> {
    debug!("Updating hApp: {}", input.addr );
    let props = input.properties;

    let entity = update_entity(
	&input.addr,
	|current : GUIReleaseEntry, _| {
	    Ok(GUIReleaseEntry {
		version: props.version
		    .unwrap_or( current.version ),
		changelog: props.changelog
		    .unwrap_or( current.changelog ),
		for_gui: current.for_gui,
		for_happ_releases: props.for_happ_releases
		    .unwrap_or( current.for_happ_releases ),
		web_asset_id: current.web_asset_id,
		screenshots: props.screenshots
		    .or( current.screenshots ),
		published_at: props.published_at
		    .unwrap_or( current.published_at ),
		last_updated: props.last_updated
		    .unwrap_or( now()? ),
		metadata: props.metadata
		    .unwrap_or( current.metadata ),
	    })
	})?;

    Ok(	entity )
}



#[derive(Debug, Deserialize)]
pub struct DeleteInput {
    pub id: ActionHash,
}

pub fn delete_gui_release(input: DeleteInput) -> AppResult<ActionHash> {
    debug!("Delete GUI release Version: {}", input.id );
    let delete_action = delete_entity::<GUIReleaseEntry,EntryTypes>( &input.id )?;
    debug!("Deleted GUI release via action ({})", delete_action );

    Ok( delete_action )
}



#[derive(Debug, Deserialize)]
pub struct GetGUIReleasesInput {
    pub for_gui: ActionHash,
}

pub fn get_gui_releases(input: GetGUIReleasesInput) -> AppResult<Vec<Entity<GUIReleaseEntry>>> {
    Ok( get_entities( &input.for_gui, LinkTypes::GUIRelease, None )? )
}
