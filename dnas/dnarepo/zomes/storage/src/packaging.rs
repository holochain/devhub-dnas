use devhub_types::{
    AppResult,
    dna_entry_types::{ DnaVersionEntry, DnaVersionPackage, DnaChunkEntry },
};

use hc_entities::{ Entity };
use hc_dna_utils as utils;
use hdk::prelude::*;



#[derive(Debug, Deserialize)]
pub struct GetDnaPackageInput {
    pub id: EntryHash,
}

pub fn get_dna_package(input: GetDnaPackageInput) -> AppResult<Entity<DnaVersionPackage>> {
    debug!("Get DNA Version: {}", input.id );
    let entity = utils::get_entity( &input.id )?;
    let entry = DnaVersionEntry::try_from( &entity.content )?;

    let dna_bytes : Vec<u8> = entry.chunk_addresses.clone().into_iter()
	.enumerate()
	.filter_map(|(i, entry_hash)| {
	    debug!("Fetching chunk #{}: {}", i, entry_hash );
	    utils::fetch_entry( entry_hash ).ok()
	})
	.filter_map(|(_, element)| {
	    DnaChunkEntry::try_from( &element ).ok()
	})
	.map( |chunk_entry| chunk_entry.bytes.bytes().to_owned() )
	.flatten()
	.collect();

    let package = entry.to_package( dna_bytes );

    Ok( entity.new_content( package ) )
}
