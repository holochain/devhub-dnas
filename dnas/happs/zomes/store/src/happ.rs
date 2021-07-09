use devhub_types::{
    DevHubResponse,
    constants::{ AppResult },
    errors::{ AppError },
    happ_entry_types::{ HappEntry, HappInfo, DeprecationNotice },
    web_asset_entry_types::{ FileInfo },
};
use hc_entities::{ Entity, UpdateEntityInput, GetEntityInput };
use hdk::prelude::*;
use hc_dna_utils as utils;

use crate::constants::{ TAG_HAPP };



#[derive(Debug, Deserialize)]
pub struct CreateInput {
    pub name: String,
    pub description: String,

    // optional
    pub thumbnail_image: Option<SerializedBytes>,
    pub published_at: Option<u64>,
    pub last_updated: Option<u64>,
}


pub fn create_happ(input: CreateInput) -> AppResult<Entity<HappInfo>> {
    debug!("Creating HAPP: {}", input.name );
    let pubkey = agent_info()?.agent_initial_pubkey;
    let default_now = utils::now()?;

    // if true {
    // 	return Err( UserError::DuplicateHappName(input.name).into() );
    // }

    let happ = HappEntry {
	name: input.name,
	description: input.description,
	designer: pubkey.clone(),
	published_at: input.published_at
	    .unwrap_or( default_now ),
	last_updated: input.last_updated
	    .unwrap_or( default_now ),
	thumbnail_image: input.thumbnail_image,
	deprecation: None,
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
    pub name: Option<String>,
    pub description: Option<String>,
    pub thumbnail_image: Option<SerializedBytes>,
    pub published_at: Option<u64>,
    pub last_updated: Option<u64>,
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
		name: props.name
		    .unwrap_or( current.name ),
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


pub fn get_ui(input: GetEntityInput) -> AppResult<Entity<FileInfo>> {
    debug!("Get UI from: {}", input.id );
    let dna_hash : Vec<u8> = vec![
	132,  45,  36,  28,  99,  63,  35, 190,  23,
	229, 249,  48, 122,  72,  85,  92, 120, 230,
	 63,  35, 199, 183,  32,  21,  71, 122,  20,
	129,  99, 253, 231, 237, 181, 171, 200,  19,
	147, 180,   9
    ];
    let pubkey = agent_info()?.agent_initial_pubkey;

    let zome_call_response = call(
	Some( CellId::new( HoloHash::from_raw_39_panicky( dna_hash ).into(), pubkey ) ),
	"files".into(),
	"get_file".into(),
	None,
	input,
    )?;

    if let ZomeCallResponse::Ok(result_io) = zome_call_response {
	let response : DevHubResponse<Entity<FileInfo>> = result_io.decode()
	    .map_err( |e| AppError::UnexpectedStateError(format!("Failed to call another DNA: {:?}", e )) )?;

	if let DevHubResponse::Success(pack) = response {
	    return Ok( pack.payload );
	}
    };

    Err( AppError::UnexpectedStateError("Failed to call another DNA".into()).into() )
}
