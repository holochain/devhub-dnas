use devhub_types::{
    AppResult, UpdateEntityInput,
    errors::{ UserError },
    dnarepo_entry_types::{
	ZomeEntry,
	ZomeVersionEntry, ZomeVersionInfo, ZomeVersionSummary,
    },
    call_local_zome,
};
use hc_crud::{
    now, create_entity, get_entity, update_entity, delete_entity, get_entities,
    Entity, Collection,
};
use mere_memory_types::{ MemoryEntry };
use hdk::prelude::*;

use crate::constants::{ TAG_ZOMEVERSION };


fn wasm_hash_path(hash: &str) -> AppResult<Path> {
    Ok( create_filter_path( "wasm_hash", hash )? )
}

fn create_filter_path(filter: &str, value: &str) -> AppResult<Path> {
    let path = hc_crud::path_from_collection( vec![ "zome_version_by", filter, value ] )?;
    path.ensure()?;

    Ok( path )
}



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
    let default_now = now()?;
    let mere_memory_addr = match input.mere_memory_addr {
	Some(addr) => addr,
	None => {
	    let bytes = input.zome_bytes
		.ok_or( UserError::CustomError("You must supply an address or bytes for the ZOME package") )?;

	    call_local_zome("mere_memory", "save_bytes", bytes )?
	},
    };
    let memory : MemoryEntry = call_local_zome("mere_memory", "get_memory", mere_memory_addr.to_owned() )?;

    let version = ZomeVersionEntry {
	for_zome: input.for_zome.clone(),
	version: input.version,
	mere_memory_addr: mere_memory_addr,
	mere_memory_hash: memory.hash,
	changelog: input.changelog
	    .unwrap_or( String::from("") ),
	published_at: input.published_at
	    .unwrap_or( default_now ),
	last_updated: input.last_updated
	    .unwrap_or( default_now ),
    };

    let entity = create_entity( &version )?
	.change_model( |version| version.to_info() );

    let wasm_hash_path = wasm_hash_path( &entity.content.mere_memory_hash )?;
    let wasm_path_hash = wasm_hash_path.hash()?;

    debug!("Linking ZOME ({}) to ENTRY: {}", input.for_zome, entity.id );
    entity.link_from( &input.for_zome, TAG_ZOMEVERSION.into() )?;

    debug!("Linking 'wasm' path ({}) to ENTRY: {}", wasm_path_hash, entity.id );
    entity.link_from( &wasm_path_hash, TAG_ZOMEVERSION.into() )?;

    Ok( entity )
}




#[derive(Debug, Deserialize)]
pub struct GetZomeVersionInput {
    pub id: EntryHash,
}

pub fn get_zome_version(input: GetZomeVersionInput) -> AppResult<Entity<ZomeVersionInfo>> {
    debug!("Get ZOME Version: {}", input.id );
    let entity = get_entity::<ZomeVersionEntry>( &input.id )?;

    Ok(	entity.change_model( |version| version.to_info() ) )
}




#[derive(Debug, Deserialize)]
pub struct GetZomeVersionsInput {
    pub for_zome: EntryHash,
}

pub fn get_zome_versions(input: GetZomeVersionsInput) -> AppResult<Collection<Entity<ZomeVersionSummary>>> {
    let collection = get_entities::<ZomeEntry, ZomeVersionEntry>( &input.for_zome, TAG_ZOMEVERSION.into() )?;

    let versions = collection.items.into_iter()
	.map(|entity| {
	    entity.change_model( |version| version.to_summary() )
	})
	.collect();

    Ok(Collection {
	base: collection.base,
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

    let entity = update_entity(
	&input.addr,
	|current : ZomeVersionEntry, _| {
	    Ok(ZomeVersionEntry {
		for_zome: current.for_zome,
		version: current.version,
		published_at: props.published_at
		    .unwrap_or( current.published_at ),
		last_updated: props.last_updated
		    .unwrap_or( now()? ),
		mere_memory_addr: current.mere_memory_addr,
		mere_memory_hash: current.mere_memory_hash,
		changelog: props.changelog
		    .unwrap_or( current.changelog ),
	    })
	})?;

    Ok( entity.change_model( |version| version.to_info() ) )
}



fn get_entities_for_links ( links: Vec<Link> ) -> Vec<Entity<ZomeVersionSummary>> {
    links.into_iter()
	.filter_map(|link| {
	    get_entity::<ZomeVersionEntry>( &link.target ).ok()
	})
	.map( |entity| {
	    entity.change_model( |version| version.to_summary() )
	})
	.collect()
}



#[derive(Debug, Deserialize)]
pub struct DeleteZomeVersionInput {
    pub id: EntryHash,
}

pub fn delete_zome_version(input: DeleteZomeVersionInput) -> AppResult<HeaderHash> {
    debug!("Delete ZOME Version: {}", input.id );
    let delete_header = delete_entity::<ZomeVersionEntry>( &input.id )?;
    debug!("Deleted ZOME Version header ({})", delete_header );

    Ok( delete_header )
}


pub fn get_zome_versions_by_filter( filter: String, keyword: String ) -> AppResult<Collection<Entity<ZomeVersionSummary>>> {
    let base = create_filter_path( &filter, &keyword )?.hash()?;

    debug!("Getting hApp links for base: {:?}", base );
    let all_links = get_links(
        base.clone(),
	Some(LinkTag::new(TAG_ZOMEVERSION))
    )?;

    let versions = get_entities_for_links( all_links );

    Ok(Collection {
	base,
	items: versions,
    })
}
