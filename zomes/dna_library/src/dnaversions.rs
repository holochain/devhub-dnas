use devhub_types::{
    AppResult,
    dnarepo_entry_types::{ DnaVersionEntry, DnaVersionInfo, DnaVersionSummary, ZomeReference },
};

use hc_entities::{ Entity, Collection, UpdateEntityInput };
use hc_dna_utils as utils;
use hdk::prelude::*;

use crate::constants::{ TAG_DNAVERSION };



#[derive(Debug, Deserialize)]
pub struct DnaVersionInput {
    pub for_dna: EntryHash,
    pub version: u64,
    pub zomes: Vec<ZomeReference>,

    // optional
    pub mere_memory_addr: Option<EntryHash>,
    pub dna_bytes: Option<SerializedBytes>,
    pub changelog: Option<String>,
    pub contributors: Option<Vec<(String, Option<AgentPubKey>)>>,
    pub published_at: Option<u64>,
    pub last_updated: Option<u64>,
}

pub fn create_dna_version(input: DnaVersionInput) -> AppResult<Entity<DnaVersionInfo>> {
    debug!("Creating DNA version ({}) for DNA: {}", input.version, input.for_dna );
    let default_now = utils::now()?;

    let version = DnaVersionEntry {
	for_dna: input.for_dna.clone(),
	version: input.version,
	zomes: input.zomes,
	changelog: input.changelog
	    .unwrap_or( String::from("") ),
	contributors: input.contributors
	    .unwrap_or( vec![] ),
	published_at: input.published_at
	    .unwrap_or( default_now ),
	last_updated: input.last_updated
	    .unwrap_or( default_now ),
    };

    let entity = utils::create_entity( &version )?
	.new_content( version.to_info() );

    debug!("Linking DNA ({}) to ENTRY: {}", input.for_dna, entity.id );
    create_link(
	input.for_dna,
	entity.id.clone(),
	LinkTag::new( TAG_DNAVERSION )
    )?;

    Ok( entity )
}




#[derive(Debug, Deserialize)]
pub struct GetDnaVersionInput {
    pub id: EntryHash,
}

pub fn get_dna_version(input: GetDnaVersionInput) -> AppResult<Entity<DnaVersionInfo>> {
    debug!("Get DNA Version: {}", input.id );
    let entity = utils::get_entity( &input.id )?;
    let info = DnaVersionEntry::try_from( &entity.content )?.to_info();

    Ok(	entity.new_content( info ) )
}




pub fn get_version_links(dna_id: EntryHash) -> AppResult<Vec<Link>> {
    debug!("Getting version links for DNA: {}", dna_id );
    let all_links: Vec<Link> = get_links(
        dna_id,
	Some(LinkTag::new( TAG_DNAVERSION ))
    )?.into();

    Ok( all_links )
}


#[derive(Debug, Deserialize)]
pub struct GetDnaVersionsInput {
    pub for_dna: EntryHash,
}

pub fn get_dna_versions(input: GetDnaVersionsInput) -> AppResult<Collection<Entity<DnaVersionSummary>>> {
    let links = get_version_links( input.for_dna.clone() )?;

    let versions = links.into_iter()
	.filter_map(|link| {
	    utils::get_entity( &link.target ).ok()
	})
	.filter_map(|entity| {
	    let mut maybe_entity : Option<Entity<DnaVersionSummary>> = None;

	    if let Some(version) = DnaVersionEntry::try_from( &entity.content ).ok() {
		let summary = version.to_summary();
		let entity = entity.new_content( summary );

		maybe_entity.replace( entity );
	    }

	    maybe_entity
	})
	.collect();

    Ok(Collection {
	base: input.for_dna,
	items: versions,
    })
}




#[derive(Debug, Deserialize)]
pub struct DnaVersionUpdateOptions {
    pub changelog: Option<String>,
    pub contributors: Option<Vec<(String, Option<AgentPubKey>)>>,
    pub published_at: Option<u64>,
    pub last_updated: Option<u64>,
}
pub type DnaVersionUpdateInput = UpdateEntityInput<DnaVersionUpdateOptions>;

pub fn update_dna_version(input: DnaVersionUpdateInput) -> AppResult<Entity<DnaVersionInfo>> {
    debug!("Updating DNA Version: {}", input.addr );
    let props = input.properties;

    let entity : Entity<DnaVersionEntry> = utils::update_entity(
	input.id, input.addr,
	|element| {
	    let current = DnaVersionEntry::try_from( &element )?;

	    Ok(DnaVersionEntry {
		for_dna: current.for_dna,
		version: current.version,
		published_at: props.published_at
		    .unwrap_or( current.published_at ),
		last_updated: props.last_updated
		    .unwrap_or( utils::now()? ),
		zomes: current.zomes,
		changelog: props.changelog
		    .unwrap_or( current.changelog ),
		contributors: props.contributors
		    .unwrap_or( current.contributors ),
	    })
	})?;

    let info = entity.content.to_info();

    Ok( entity.new_content( info ) )
}




#[derive(Debug, Deserialize)]
pub struct DeleteDnaVersionInput {
    pub id: EntryHash,
}

pub fn delete_dna_version(input: DeleteDnaVersionInput) -> AppResult<HeaderHash> {
    debug!("Delete DNA Version: {}", input.id );
    let (header, _) = utils::fetch_entry( input.id.clone() )?;

    let delete_header = delete_entry( header.clone() )?;
    debug!("Deleted DNA Version create {} via header ({})", header, delete_header );

    Ok( header )
}
