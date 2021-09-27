use devhub_types::{
    AppResult,
    happ_entry_types::{
	HappEntry, HappInfo, HappSummary,
	DeprecationNotice, HappGUIConfig,
    },
};
use hc_crud::{
    now, create_entity, get_entity, update_entity,
    Entity, Collection, UpdateEntityInput, GetEntityInput,
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
    let entity = get_entity( &input.id )?;
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

    let entity : Entity<HappEntry> = update_entity(
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
		    .unwrap_or( now()? ),
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
    let entity : Entity<HappEntry> = update_entity(
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


pub fn get_happ_links(maybe_pubkey: Option<AgentPubKey>) -> AppResult<(EntryHash, Vec<Link>)> {
    let base : EntryHash = match maybe_pubkey {
	None => agent_info()?.agent_initial_pubkey,
	Some(agent) => agent,
    }.into();

    debug!("Getting hApp links for Agent entry: {}", base );
    let all_links: Vec<Link> = get_links(
        base.clone(),
	Some(LinkTag::new(TAG_HAPP))
    )?.into();

    Ok( (base, all_links) )
}

#[derive(Debug, Deserialize)]
pub struct GetHappsInput {
    pub agent: Option<AgentPubKey>,
}

pub fn get_happs(input: GetHappsInput) -> AppResult<Collection<Entity<HappSummary>>> {
    let (base, links) = get_happ_links( input.agent.clone() )?;

    let happs = links.into_iter()
	.filter_map(|link| {
	    get_entity( &link.target ).ok()
	})
	.filter_map(|entity| {
	    let mut maybe_entity : Option<Entity<HappSummary>> = None;

	    if let Some(happ) = HappEntry::try_from( &entity.content ).ok() {
		if happ.deprecation.is_none() {
		    let summary = happ.to_summary();
		    let entity = entity.new_content( summary );

		    maybe_entity.replace( entity );
		}
	    }

	    maybe_entity
	})
	.collect();

    Ok(Collection {
	base,
	items: happs
    })
}

pub fn get_my_happs() -> AppResult<Collection<Entity<HappSummary>>> {
    get_happs(GetHappsInput {
	agent: None,
    })
}
