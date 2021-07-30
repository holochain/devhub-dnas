use devhub_types::{
    AppResult,
    dnarepo_entry_types::{ DnaVersionEntry, DnaVersionPackage },
    call_local_zome,
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

    let bytes : Vec<u8> = call_local_zome( "mere_memory", "retrieve_bytes", entry.mere_memory_addr.clone() )?;

    let package = entry.to_package( bytes );

    Ok( entity.new_content( package ) )
}
