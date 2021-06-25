use devhub_types::{ Entity, Collection, EntityResponse, EntityCollectionResponse, EntryModel, DevHubResponse,
		    ENTITY_MD, ENTITY_COLLECTION_MD, VALUE_MD };
use hdk::prelude::*;
use hc_dna_utils as utils;
use hc_dna_utils::safe_unwrap;

use crate::errors::{ AppError };
use crate::constants::{ TAG_DNAVERSION };
use crate::entry_types::{ DnaVersionEntry, DnaVersionInfo, DnaVersionSummary };


#[derive(Debug, Deserialize)]
pub struct DnaVersionInput {
    pub for_dna: EntryHash,
    pub version: u64,
    pub file_size: u64,
    pub chunk_addresses: Vec<EntryHash>,

    // optional
    pub changelog: Option<String>,
    pub contributors: Option<Vec<(String, Option<AgentPubKey>)>>,
    pub published_at: Option<u64>,
    pub last_updated: Option<u64>,
}

#[hdk_extern]
fn create_dna_version(input: DnaVersionInput) -> ExternResult<EntityResponse<DnaVersionInfo>> {
    debug!("Creating DNA version ({}) for DNA: {}", input.version, input.for_dna );
    let version = DnaVersionEntry {
	for_dna: input.for_dna.clone(),
	version: input.version,
	file_size: input.file_size,
	chunk_addresses: input.chunk_addresses,
	changelog: input.changelog
	    .unwrap_or( String::from("") ),
	contributors: input.contributors
	    .unwrap_or( vec![] ),
	published_at: input.published_at
	    .unwrap_or( sys_time()?.as_millis() as u64 ),
	last_updated: input.last_updated
	    .unwrap_or( sys_time()?.as_millis() as u64 ),
    };

    let header_hash = create_entry(&version)?;
    let entry_hash = hash_entry(&version)?;

    debug!("Linking DNA ({}) to manifest: {}", input.for_dna, entry_hash );
    create_link(
	input.for_dna,
	entry_hash.clone(),
	LinkTag::new(TAG_DNAVERSION)
    )?;

    let info = version.to_info();

    Ok( EntityResponse::success(Entity {
	id: entry_hash.clone(),
	address: entry_hash,
	header: header_hash,
	ctype: info.get_type(),
	content: info,
    }, ENTITY_MD ))
}




#[derive(Debug, Deserialize)]
pub struct GetDnaVersionInput {
    pub addr: EntryHash,
}

#[hdk_extern]
fn get_dna_version(input: GetDnaVersionInput) -> ExternResult<EntityResponse<DnaVersionInfo>> {
    debug!("Get DNA Version: {}", input.addr );
    let entity = safe_unwrap!( utils::fetch_entity( &input.addr ), AppError::EntryNotFound(&input.addr) );
    let info = DnaVersionEntry::try_from(&entity.content)?.to_info();

    Ok( EntityResponse::success(
	entity.new_content( info ), ENTITY_MD
    ))
}




fn get_version_links(dna: EntryHash) -> ExternResult<Vec<Link>> {
    debug!("Getting version links for DNA: {}", dna );
    let all_links: Vec<Link> = get_links(
        dna,
	Some(LinkTag::new(TAG_DNAVERSION))
    )?.into();

    Ok( all_links )
}


#[derive(Debug, Deserialize)]
pub struct GetDnaVersionsInput {
    pub for_dna: EntryHash,
}

#[hdk_extern]
fn get_dna_versions(input: GetDnaVersionsInput) -> ExternResult<EntityCollectionResponse<DnaVersionSummary>> {
    let links = get_version_links(input.for_dna.clone())?;

    let versions = links.into_iter()
	.filter_map(|link| {
	    utils::fetch_entity( &link.target ).ok()
	})
	.filter_map(|entity| {
	    let mut maybe_entity : Option<Entity<DnaVersionSummary>> = None;

	    if let Some(version) = DnaVersionEntry::try_from(&entity.content).ok() {
		let summary = version.to_summary();
		let entity = entity.new_content( summary );

		maybe_entity.replace( entity );
	    }

	    maybe_entity
	})
	.collect();

    Ok( EntityCollectionResponse::success(Collection {
	base: input.for_dna,
	items: versions,
    }, ENTITY_COLLECTION_MD) )
}




#[derive(Debug, Deserialize)]
pub struct UpdateDnaVersionInput {
    pub addr: EntryHash,
    pub properties: DnaVersionUpdateOptions
}
#[derive(Debug, Deserialize)]
pub struct DnaVersionUpdateOptions {
    pub changelog: Option<String>,
    pub contributors: Option<Vec<(String, Option<AgentPubKey>)>>,
    pub published_at: Option<u64>,
    pub last_updated: Option<u64>,
}

#[hdk_extern]
fn update_dna_version(input: UpdateDnaVersionInput) -> ExternResult<EntityResponse<DnaVersionInfo>> {
    debug!("Updating DNA Version: {}", input.addr );
    let entity = utils::fetch_entity( &input.addr )?;
    let current_version = DnaVersionEntry::try_from( &entity.content )?;

    let version = DnaVersionEntry {
	for_dna: current_version.for_dna,
	version: current_version.version,
	published_at: match input.properties.published_at {
	    None => current_version.published_at,
	    Some(v) => v,
	},
	last_updated: match input.properties.last_updated {
	    None => {
		sys_time()?.as_millis() as u64
	    },
	    Some(t) => t,
	},
	file_size: current_version.file_size,
	chunk_addresses: current_version.chunk_addresses,
	changelog: match input.properties.changelog {
	    None => current_version.changelog,
	    Some(v) => v,
	},
	contributors: match input.properties.contributors {
	    None => current_version.contributors,
	    Some(v) => v,
	},
    };

    let header_hash = update_entry(entity.header.clone(), &version)?;
    let entry_hash = hash_entry(&version)?;

    debug!("Linking original ({}) to DNA Version: {}", input.addr, entry_hash );
    create_link(
	input.addr.clone(),
	entry_hash.clone(),
	LinkTag::new(utils::TAG_UPDATE)
    )?;

    Ok(EntityResponse::success(
	entity.new_content( version.to_info() )
	    .update_header( header_hash )
	    .update_address( entry_hash ),
	ENTITY_MD
    ))
}




#[derive(Debug, Deserialize)]
pub struct DeleteDnaVersionInput {
    pub addr: EntryHash,
}

#[hdk_extern]
fn delete_dna_version(input: DeleteDnaVersionInput) -> ExternResult<DevHubResponse<HeaderHash>> {
    debug!("Delete DNA Version: {}", input.addr );
    let (header, _) = utils::fetch_entry(input.addr.clone())?;

    let delete_header = delete_entry(header.clone())?;
    debug!("Deleted DNA Version create {} via header ({})", header, delete_header );

    Ok( DevHubResponse::success( header, VALUE_MD ) )
}
