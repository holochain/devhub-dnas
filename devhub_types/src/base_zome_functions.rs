use hdk::prelude::*;
use crate::{
    composition, create_path,
    DevHubResponse,
    constants::{
	VALUE_MD,
    },
};


#[hdk_extern]
fn get_record(hash: EntryHash) -> ExternResult<DevHubResponse<Record>> {
    let record = get( hash.to_owned(), GetOptions::latest() )?
	.ok_or( wasm_error!(WasmErrorInner::Guest(format!("Entry not found: {}", hash ))) )?;

    Ok(composition( record, VALUE_MD ))
}

#[hdk_extern]
fn get_record_latest(hash: EntryHash) -> ExternResult<DevHubResponse<Record>> {
    let entity = hc_crud::fetch_record_latest( &hash )
	.map_err( |err| wasm_error!(WasmErrorInner::Guest(format!("fetch_record_latest Error: {:?}", err ))) )?;

    Ok(composition( entity.1, VALUE_MD ))
}


// #[derive(Debug, Deserialize)]
// struct GetLinksInput {
//     base: EntryHash,
//     link_type: LinkTypes,
//     tag: Option<String>,
// }

// #[hdk_extern]
// fn get_links(input: GetLinksInput) -> ExternResult<DevHubResponse<Vec<Link>>>
// {
//     let link_tag = input.tag.map( |tag| LinkTag::new( tag.as_bytes() ) );
//     let links = hdk::prelude::get_links( input.base.to_owned(), input.link_type, link_tag )?;

//     Ok(composition( links, VALUE_MD ))
// }

#[hdk_extern]
fn path( segments: Vec<String> ) -> ExternResult<DevHubResponse<EntryHash>> {
    let path = match segments.len() {
	0 => Err(wasm_error!(WasmErrorInner::Guest("Path segment input cannot be empty".to_string()))),
	1 => Ok( create_path( &segments[0], Vec::<String>::new() ).1 ),
	_ => Ok( create_path( &segments[0], &segments[1..] ).1 ),
    }?;

    Ok(composition( path, VALUE_MD ))
}


#[hdk_extern]
fn dna_info(_: ()) -> ExternResult<DevHubResponse<DnaInfo>> {
    Ok(composition( hdk::prelude::dna_info()?, VALUE_MD ))
}

#[hdk_extern]
fn zome_info(_: ()) -> ExternResult<DevHubResponse<ZomeInfo>> {
    Ok(composition( hdk::prelude::zome_info()?, VALUE_MD ))
}
