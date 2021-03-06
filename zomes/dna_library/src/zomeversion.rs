use std::collections::BTreeMap;
use devhub_types::{
    AppResult, UpdateEntityInput,
    errors::{ UserError, AppError },
    dnarepo_entry_types::{
	ZomeEntry,
	ZomeVersionEntry,
	ReviewSummaryEntry,
    },
    constants::{
	ANCHOR_UNIQUENESS,
	ANCHOR_HDK_VERSIONS,
    },
    call_local_zome,
    fmt_path,
};
use hc_crud::{
    now, create_entity, get_entity, update_entity, delete_entity, get_entities,
    Entity, Collection,
};
use mere_memory_types::{ MemoryEntry };
use hdk::prelude::*;

use crate::constants::{
    LT_NONE,
    TAG_ZOMEVERSION,
};




#[derive(Debug, Deserialize)]
pub struct ZomeVersionInput {
    pub for_zome: EntryHash,
    pub version: String,
    pub ordering: u64,
    pub hdk_version: String,

    // optional
    pub mere_memory_addr: Option<EntryHash>,
    pub zome_bytes: Option<SerializedBytes>,
    pub changelog: Option<String>,
    pub published_at: Option<u64>,
    pub last_updated: Option<u64>,
    pub source_code_commit_url: Option<String>,
    pub metadata: Option<BTreeMap<String, serde_yaml::Value>>,
}

pub fn create_zome_version(input: ZomeVersionInput) -> AppResult<Entity<ZomeVersionEntry>> {
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
	ordering: input.ordering,
	mere_memory_addr: mere_memory_addr,
	mere_memory_hash: memory.hash,
	changelog: input.changelog
	    .unwrap_or( String::from("") ),
	published_at: input.published_at
	    .unwrap_or( default_now ),
	last_updated: input.last_updated
	    .unwrap_or( default_now ),
	hdk_version: input.hdk_version.clone(),
	review_summary: None,
	source_code_commit_url: input.source_code_commit_url,
	metadata: input.metadata
	    .unwrap_or( BTreeMap::new() ),
    };

    let entity = create_entity( &version )?;

    // Parent anchor
    debug!("Linking Zome ({}) to entry: {}", input.for_zome, entity.id );
    entity.link_from( &input.for_zome, LT_NONE, TAG_ZOMEVERSION.into() )?;

    // Uniqueness anchor
    let (wasm_path, wasm_path_hash) = devhub_types::ensure_path( ANCHOR_UNIQUENESS, vec![ &entity.content.mere_memory_hash ] )?;
    debug!("Linking uniqueness path ({} => {:?}) to entry: {}", fmt_path( &wasm_path ), wasm_path, entity.id );
    entity.link_from( &wasm_path_hash, LT_NONE, TAG_ZOMEVERSION.into() )?;

    // HDK anchor
    let (hdkv_path, hdkv_hash) = devhub_types::ensure_path( ANCHOR_HDK_VERSIONS, vec![ &input.hdk_version ] )?;
    debug!("Linking HDK version global anchor ({}) to entry: {}", fmt_path( &hdkv_path ), entity.id );
    entity.link_from( &hdkv_hash, LT_NONE, TAG_ZOMEVERSION.into() )?;

    Ok( entity )
}




#[derive(Debug, Deserialize)]
pub struct GetZomeVersionInput {
    pub id: EntryHash,
}

pub fn get_zome_version(input: GetZomeVersionInput) -> AppResult<Entity<ZomeVersionEntry>> {
    debug!("Get ZOME Version: {}", input.id );
    let entity = get_entity::<ZomeVersionEntry>( &input.id )?;

    Ok(	entity )
}




#[derive(Debug, Deserialize)]
pub struct GetZomeVersionsInput {
    pub for_zome: EntryHash,
}

pub fn get_zome_versions(input: GetZomeVersionsInput) -> AppResult<Collection<Entity<ZomeVersionEntry>>> {
    Ok( get_entities::<ZomeEntry, ZomeVersionEntry>( &input.for_zome, TAG_ZOMEVERSION.into() )? )
}




#[derive(Debug, Deserialize)]
pub struct ZomeVersionUpdateOptions {
    pub ordering: Option<u64>,
    pub changelog: Option<String>,
    pub published_at: Option<u64>,
    pub last_updated: Option<u64>,
    pub source_code_commit_url: Option<String>,
    pub metadata: Option<BTreeMap<String, serde_yaml::Value>>,
}
pub type ZomeVersionUpdateInput = UpdateEntityInput<ZomeVersionUpdateOptions>;

pub fn update_zome_version(input: ZomeVersionUpdateInput) -> AppResult<Entity<ZomeVersionEntry>> {
    debug!("Updating ZOME Version: {}", input.addr );
    let props = input.properties;

    let entity = update_entity(
	&input.addr,
	|current : ZomeVersionEntry, _| {
	    Ok(ZomeVersionEntry {
		for_zome: current.for_zome,
		version: current.version,
		ordering: props.ordering
		    .unwrap_or( current.ordering ),
		published_at: props.published_at
		    .unwrap_or( current.published_at ),
		last_updated: props.last_updated
		    .unwrap_or( now()? ),
		mere_memory_addr: current.mere_memory_addr,
		mere_memory_hash: current.mere_memory_hash,
		changelog: props.changelog
		    .unwrap_or( current.changelog ),
		hdk_version: current.hdk_version,
		review_summary: current.review_summary,
		source_code_commit_url: props.source_code_commit_url
		    .or( current.source_code_commit_url ),
		metadata: props.metadata
		    .unwrap_or( current.metadata ),
	    })
	})?;

    Ok( entity )
}




#[derive(Debug, Deserialize)]
pub struct EntityAddressInput {
    pub subject_header: HeaderHash,
    pub addr: EntryHash,
}

#[derive(Debug, Serialize)]
pub struct ReviewSummaryInput {
    pub subject_header: HeaderHash,
}

pub fn create_zome_version_review_summary(input: EntityAddressInput) -> AppResult<Entity<ZomeVersionEntry>> {
    debug!("Updating ZOME Version: {}", input.subject_header );
    let current_summary : ZomeVersionEntry = get( input.addr.to_owned(), GetOptions::content() )?
	.ok_or( AppError::UnexpectedStateError(format!("Given address could not be found: {}", input.addr )) )?
	.try_into()?;

    if let Some(review_summary_id) = current_summary.review_summary {
	Err(UserError::InvalidActionError(format!("You cannot change the review summary because it is already set to: {}", review_summary_id )))?
    }

    let review_summary : Entity<ReviewSummaryEntry> = call_local_zome( "reviews", "create_review_summary_for_subject", ReviewSummaryInput {
	subject_header: input.subject_header,
    })?;

    let entity = update_entity(
	&input.addr,
	|mut current : ZomeVersionEntry, _| {
	    current.review_summary = Some(review_summary.id);
	    Ok( current )
	})?;

    Ok( entity )
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
