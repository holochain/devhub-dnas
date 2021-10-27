use devhub_types::{
    AppResult, UpdateEntityInput,
    dnarepo_entry_types::{
	DnaEntry,
	DnaVersionEntry, DnaVersionInfo, DnaVersionSummary, ZomeReference,
    },
};
use hc_crud::{
    now, create_entity, get_entity, update_entity, delete_entity, get_entities,
    Entity, Collection,
};
use hdk::prelude::*;

use crate::constants::{ TAG_DNAVERSION };



#[derive(Debug, Deserialize)]
pub struct DnaVersionInput {
    pub for_dna: EntryHash,
    pub version: u64,
    pub zomes: Vec<ZomeReference>,

    // optional
    pub changelog: Option<String>,
    pub published_at: Option<u64>,
    pub last_updated: Option<u64>,
}

pub fn create_dna_version(input: DnaVersionInput) -> AppResult<Entity<DnaVersionInfo>> {
    debug!("Creating DNA version ({}) for DNA: {}", input.version, input.for_dna );
    let default_now = now()?;

    let version = DnaVersionEntry {
	for_dna: input.for_dna.clone(),
	version: input.version,
	zomes: input.zomes,
	changelog: input.changelog
	    .unwrap_or( String::from("") ),
	published_at: input.published_at
	    .unwrap_or( default_now ),
	last_updated: input.last_updated
	    .unwrap_or( default_now ),
    };

    let entity = create_entity( &version )?
	.change_model( |version| version.to_info() );

    debug!("Linking DNA ({}) to ENTRY: {}", input.for_dna, entity.id );
    entity.link_from( &input.for_dna, TAG_DNAVERSION.into() )?;

    Ok( entity )
}




#[derive(Debug, Deserialize)]
pub struct GetDnaVersionInput {
    pub id: EntryHash,
}

pub fn get_dna_version(input: GetDnaVersionInput) -> AppResult<Entity<DnaVersionInfo>> {
    debug!("Get DNA Version: {}", input.id );
    let entity = get_entity::<DnaVersionEntry>( &input.id )?;

    Ok(	entity.change_model( |version| version.to_info() ) )
}




#[derive(Debug, Deserialize)]
pub struct GetDnaVersionsInput {
    pub for_dna: EntryHash,
}

pub fn get_dna_versions(input: GetDnaVersionsInput) -> AppResult<Collection<Entity<DnaVersionSummary>>> {
    let collection = get_entities::<DnaEntry, DnaVersionEntry>( &input.for_dna, TAG_DNAVERSION.into() )?;

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
pub struct DnaVersionUpdateOptions {
    pub changelog: Option<String>,
    pub published_at: Option<u64>,
    pub last_updated: Option<u64>,
}
pub type DnaVersionUpdateInput = UpdateEntityInput<DnaVersionUpdateOptions>;

pub fn update_dna_version(input: DnaVersionUpdateInput) -> AppResult<Entity<DnaVersionInfo>> {
    debug!("Updating DNA Version: {}", input.addr );
    let props = input.properties;

    let entity = update_entity(
	&input.addr,
	|current : DnaVersionEntry, _| {
	    Ok(DnaVersionEntry {
		for_dna: current.for_dna,
		version: current.version,
		published_at: props.published_at
		    .unwrap_or( current.published_at ),
		last_updated: props.last_updated
		    .unwrap_or( now()? ),
		zomes: current.zomes,
		changelog: props.changelog
		    .unwrap_or( current.changelog ),
	    })
	})?;

    Ok( entity.change_model( |version| version.to_info() ) )
}




#[derive(Debug, Deserialize)]
pub struct DeleteDnaVersionInput {
    pub id: EntryHash,
}

pub fn delete_dna_version(input: DeleteDnaVersionInput) -> AppResult<HeaderHash> {
    debug!("Delete DNA Version: {}", input.id );
    let delete_header = delete_entity::<DnaVersionEntry>( &input.id )?;
    debug!("Deleted DNA Version via header ({})", delete_header );

    Ok( delete_header )
}
