
mod errors;

use hdk::prelude::*;
use hc_dna_reply_types::{ Entity };

pub use errors::{ RuntimeError };


pub const TAG_UPDATE: &'static str = "update";


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

pub fn fetch_entity(id: EntryHash) -> ExternResult<Entity<Element>> {
    let (header_hash, element) = fetch_entry_latest( id.clone() )?;

    let address = element
	.header()
	.entry_hash()
	.ok_or(WasmError::from(RuntimeError::EntryNotFound))?;

    Ok(Entity {
	id: id,
	header: header_hash,
	address: address.to_owned(),
	ctype: String::from("element"),
	content: element,
    })
}

#[macro_export]
macro_rules! try_from_element {
    ( $( $struct:ident ),* ) => {
	$(
	    impl TryFrom<&Element> for $struct {
		type Error = WasmError;
		fn try_from(element: &Element) -> Result<Self, Self::Error> {
		    let entry = element.entry()
			.to_app_option::<Self>()?
			.ok_or(WasmError::from(hc_dna_utils::RuntimeError::DeserializationError(element.clone())))?;

		    let entry_hash = hash_entry(&entry)?;
		    let expected_hash = element.header().entry_hash().unwrap().clone();

		    if entry_hash == expected_hash {
			Ok( entry )
		    } else {
			Err(WasmError::from(hc_dna_utils::RuntimeError::DeserializationWrongEntryTypeError(entry_hash, expected_hash)))
		    }
		}
	    }
	)*
    };
}
