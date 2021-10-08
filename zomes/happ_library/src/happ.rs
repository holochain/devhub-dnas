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
    let default_now = now()?;

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

    let entity = create_entity( &happ )?
	.change_model( |happ| happ.to_info() );
    let base = crate::root_path_hash( None )?;

    debug!("Linking pubkey ({}) to ENTRY: {}", base, entity.id );
    entity.link_from( &base, TAG_HAPP.into() )?;

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
    pub thumbnail_image: Option<SerializedBytes>,
    pub published_at: Option<u64>,
    pub last_updated: Option<u64>,
    pub gui: Option<HappGUIConfig>,
}
pub type HappUpdateInput = UpdateEntityInput<HappUpdateOptions>;

pub fn update_happ(input: HappUpdateInput) -> AppResult<Entity<HappInfo>> {
    debug!("Updating hApp: {}", input.addr );
    let props = input.properties;

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
		thumbnail_image: props.thumbnail_image
		    .or( current.thumbnail_image ),
		deprecation: current.deprecation,
		gui: props.gui
		    .or( current.gui ),
	    })
	})?;

    Ok( entity.change_model( |happ| happ.to_info() ) )
}


#[derive(Debug, Deserialize)]
pub struct HappDeprecateInput {
    pub id: Option<EntryHash>,
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


pub fn get_happ_collection(maybe_pubkey: Option<AgentPubKey>) -> AppResult<Collection<Entity<HappSummary>>> {
    let base = crate::root_path_hash( maybe_pubkey )?;

    debug!("Getting hApp links for Agent entry: {}", base );
    let all_links: Vec<Link> = get_links(
        base.clone(),
	Some(LinkTag::new(TAG_HAPP))
    )?.into();

    let happs = all_links.into_iter()
	.filter_map(|link| {
	    get_entity::<HappEntry>( &link.target ).ok()
	})
	.map( |entity| {
	    entity.change_model( |happ| happ.to_summary() )
	})
	.collect();

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
