use devhub_types::{
    AppResult,
    errors::{ UserError },
    dnarepo_entry_types::{ ZomeVersionEntry, ZomeVersionInfo, ZomeVersionSummary },
    call_local_zome,
};

use hc_entities::{ Entity, Collection, UpdateEntityInput };
use hc_dna_utils as utils;
use hdk::prelude::*;

use crate::constants::{ TAG_ZOMEVERSION };



#[derive(Debug, Deserialize)]
pub struct ZomeVersionInput {
    pub for_zome: EntryHash,
    pub version: u64,

    // optional
    pub mere_memory_addr: Option<EntryHash>,
    pub zome_bytes: Option<SerializedBytes>,
    pub changelog: Option<String>,
    pub published_at: Option<u64>,
    pub last_updated: Option<u64>,
}

pub fn create_zome_version(input: ZomeVersionInput) -> AppResult<Entity<ZomeVersionInfo>> {
    debug!("Creating ZOME version ({}) for ZOME: {}", input.version, input.for_zome );
    let default_now = utils::now()?;

    let version = ZomeVersionEntry {
	for_zome: input.for_zome.clone(),
	version: input.version,
	mere_memory_addr: match input.mere_memory_addr {
	    Some(addr) => addr,
	    None => {
		let bytes = input.zome_bytes
		    .ok_or( UserError::CustomError("You must supply an address or bytes for the ZOME package") )?;

		call_local_zome("mere_memory", "save_bytes", bytes )?
	    },
	},
	changelog: input.changelog
	    .unwrap_or( String::from("") ),
	published_at: input.published_at
	    .unwrap_or( default_now ),
	last_updated: input.last_updated
	    .unwrap_or( default_now ),
    };

    let entity = utils::create_entity( &version )?
	.new_content( version.to_info() );

    debug!("Linking ZOME ({}) to ENTRY: {}", input.for_zome, entity.id );
    create_link(
	input.for_zome,
	entity.id.clone(),
	LinkTag::new( TAG_ZOMEVERSION )
    )?;

    Ok( entity )
}




#[derive(Debug, Deserialize)]
pub struct GetZomeVersionInput {
    pub id: EntryHash,
}

pub fn get_zome_version(input: GetZomeVersionInput) -> AppResult<Entity<ZomeVersionInfo>> {
    debug!("Get ZOME Version: {}", input.id );
    let entity = utils::get_entity( &input.id )?;
    let info = ZomeVersionEntry::try_from( &entity.content )?.to_info();

    Ok(	entity.new_content( info ) )
}




pub fn get_version_links(zome_id: EntryHash) -> AppResult<Vec<Link>> {
    debug!("Getting version links for ZOME: {}", zome_id );
    let all_links: Vec<Link> = get_links(
        zome_id,
	Some(LinkTag::new( TAG_ZOMEVERSION ))
    )?.into();

    Ok( all_links )
}


#[derive(Debug, Deserialize)]
pub struct GetZomeVersionsInput {
    pub for_zome: EntryHash,
}

pub fn get_zome_versions(input: GetZomeVersionsInput) -> AppResult<Collection<Entity<ZomeVersionSummary>>> {
    let links = get_version_links( input.for_zome.clone() )?;

    let versions = links.into_iter()
	.filter_map(|link| {
	    utils::get_entity( &link.target ).ok()
	})
	.filter_map(|entity| {
	    let mut maybe_entity : Option<Entity<ZomeVersionSummary>> = None;

	    if let Some(version) = ZomeVersionEntry::try_from( &entity.content ).ok() {
		let summary = version.to_summary();
		let entity = entity.new_content( summary );

		maybe_entity.replace( entity );
	    }

	    maybe_entity
	})
	.collect();

    Ok(Collection {
	base: input.for_zome,
	items: versions,
    })
}




#[derive(Debug, Deserialize)]
pub struct ZomeVersionUpdateOptions {
    pub changelog: Option<String>,
    pub published_at: Option<u64>,
    pub last_updated: Option<u64>,
}
pub type ZomeVersionUpdateInput = UpdateEntityInput<ZomeVersionUpdateOptions>;

pub fn update_zome_version(input: ZomeVersionUpdateInput) -> AppResult<Entity<ZomeVersionInfo>> {
    debug!("Updating ZOME Version: {}", input.addr );
    let props = input.properties;

    let entity : Entity<ZomeVersionEntry> = utils::update_entity(
	input.id, input.addr,
	|element| {
	    let current = ZomeVersionEntry::try_from( &element )?;

	    Ok(ZomeVersionEntry {
		for_zome: current.for_zome,
		version: current.version,
		published_at: props.published_at
		    .unwrap_or( current.published_at ),
		last_updated: props.last_updated
		    .unwrap_or( utils::now()? ),
		mere_memory_addr: current.mere_memory_addr,
		changelog: props.changelog
		    .unwrap_or( current.changelog ),
	    })
	})?;

    let info = entity.content.to_info();

    Ok( entity.new_content( info ) )
}




#[derive(Debug, Deserialize)]
pub struct DeleteZomeVersionInput {
    pub id: EntryHash,
}

pub fn delete_zome_version(input: DeleteZomeVersionInput) -> AppResult<HeaderHash> {
    debug!("Delete ZOME Version: {}", input.id );
    let (header, _) = utils::fetch_entry( input.id.clone() )?;

    let delete_header = delete_entry( header.clone() )?;
    debug!("Deleted ZOME Version create {} via header ({})", header, delete_header );

    Ok( header )
}
