use hdk::prelude::*;

use crate::constants::{ TAG_UPDATE };
use crate::errors::{ RuntimeError };

pub fn find_latest_link(links: Vec<Link>) -> ExternResult<Option<Link>> {
    Ok(links
       .into_iter()
       .fold(None, |latest: Option<Link>, link: Link| match latest {
	   Some(latest) => {
	       if link.timestamp > latest.timestamp {
		   Some(link)
	       } else {
		   Some(latest)
	       }
	   },
	   None => Some(link),
       }))
}

pub fn fetch_entry_latest(addr: EntryHash) -> ExternResult<(HeaderHash, Element)> {
    //
    // - Get event details `ElementDetails` (expect to be a Create or Update)
    // - If it has updates, select the one with the latest Header timestamp
    // - Get the update element
    //
    let result = get(addr.clone(), GetOptions::latest())?;
    let mut element : Element;

    if let Some(v) = result {
	element = v;
    }
    else {
	return Err(WasmError::from(RuntimeError::EntryNotFound));
    }

    let update_links = get_links(addr.clone(), Some(LinkTag::new(TAG_UPDATE)))?.into_inner();

    if update_links.len() > 0 {
	let latest_link = find_latest_link( update_links )?;

	if let Some(link) = latest_link {
	    let result = get(link.target, GetOptions::latest())?;

	    if let Some(v) = result {
		element = v;
	    }
	}
    }

    Ok( (element.header_address().to_owned(), element) )
}

pub fn fetch_entry(addr: EntryHash) -> ExternResult<(HeaderHash, Element)> {
    match get(addr, GetOptions::latest())? {
        Some(element) => Ok((element.header_address().to_owned(), element)),
        None => Err(WasmError::from(RuntimeError::EntryNotFound)),
    }
}
