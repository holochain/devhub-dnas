use devhub_types::{ Entity, Collection, EntityResponse, EntityCollectionResponse, EntryModel, DevHubResponse,
		    ENTITY_MD, ENTITY_COLLECTION_MD };
use hdk::prelude::*;
use hc_dna_utils as utils;
use hc_dna_utils::catch;

use crate::constants::{ TAG_HAPP };
use crate::entry_types::{ HappEntry, HappInfo, HappSummary, DeprecationNotice };



#[derive(Debug, Deserialize)]
pub struct CreateInput {
    pub name: String,
    pub description: String,

    // optional
    pub thumbnail_image: Option<SerializedBytes>,
    pub published_at: Option<u64>,
    pub last_updated: Option<u64>,
}


pub fn create_happ(input: CreateInput) -> ExternResult<Entity<HappInfo>> {
    debug!("Creating HAPP: {}", input.name );
    let pubkey = agent_info()?.agent_initial_pubkey;

    let happ = HappEntry {
	name: input.name,
	description: input.description,
	designer: pubkey.clone(),
	published_at: input.published_at
	    .unwrap_or( sys_time()?.as_millis() as u64 ),
	last_updated: input.last_updated
	    .unwrap_or( sys_time()?.as_millis() as u64 ),
	thumbnail_image: input.thumbnail_image,
	deprecation: None,
    };

    let entity = utils::create_entity( &happ, TAG_HAPP )?
	.new_content( happ.to_info() );

    Ok( entity )
}



#[derive(Debug, Deserialize)]
pub struct HappUpdateInput {
    pub id: Option<EntryHash>,
    pub addr: EntryHash,
    pub properties: HappUpdateOptions
}
#[derive(Debug, Deserialize)]
pub struct HappUpdateOptions {
    pub name: Option<String>,
    pub description: Option<String>,
    pub thumbnail_image: Option<SerializedBytes>,
    pub published_at: Option<u64>,
    pub last_updated: Option<u64>,
}

pub fn update_happ(input: HappUpdateInput) -> ExternResult<Entity<HappInfo>> {
    let props = input.properties;

    let entity : Entity<HappEntry> = utils::update_entity(
	input.id, input.addr,
	|element| {
	    let current = HappEntry::try_from( &element )
		.map_err(|e| WasmError::Guest("Nothing here".into()) )?;

	    Ok(HappEntry {
		name: props.name
		    .unwrap_or( current.name ),
		description: props.description
		    .unwrap_or( current.description ),
		designer: current.designer,
		published_at: props.published_at
		    .unwrap_or( current.published_at ),
		last_updated: props.last_updated
		    .unwrap_or( sys_time()?.as_millis() as u64 ),
		thumbnail_image: props.thumbnail_image
		    .or( current.thumbnail_image ),
		deprecation: current.deprecation,
	    })
	}
    )?;

    let info = entity.content.clone().to_info();

    Ok( entity.new_content( info ) )
}
