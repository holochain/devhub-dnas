use hdk::prelude::*;
use crate::{
    composition, create_path,
    DevHubResponse,
    constants::{
	VALUE_MD,
    },
};


#[hdk_extern]
fn get_element(hash: EntryHash) -> ExternResult<DevHubResponse<Element>> {
    let element = get( hash.to_owned(), GetOptions::latest() )?
	.ok_or( WasmError::Guest(format!("Entry not found: {}", hash )) )?;

    Ok(composition( element, VALUE_MD ))
}

#[hdk_extern]
fn get_element_latest(hash: EntryHash) -> ExternResult<DevHubResponse<Element>> {
    let entity = hc_crud::fetch_element_latest( &hash )
	.map_err( |err| WasmError::Guest(format!("fetch_element_latest Error: {:?}", err )) )?;

    Ok(composition( entity.1, VALUE_MD ))
}


#[derive(Debug, Deserialize)]
struct GetLinksInput {
    base: EntryHash,
    tag: Option<String>,
}

#[hdk_extern]
fn get_links(input: GetLinksInput) -> ExternResult<DevHubResponse<Vec<Link>>>
{
    let link_tag = input.tag.map( |tag| LinkTag::new( tag.as_bytes() ) );
    let links = hdk::prelude::get_links( input.base.to_owned(), link_tag )?;

    Ok(composition( links, VALUE_MD ))
}

#[hdk_extern]
fn path( segments: Vec<String> ) -> ExternResult<DevHubResponse<EntryHash>> {
    let path = match segments.len() {
	0 => Err(WasmError::Guest("Path segment input cannot be empty".to_string())),
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
