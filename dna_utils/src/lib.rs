
mod errors;

use hdk::prelude::*;
use devhub_types::{ Entity, EntityType };

pub use errors::{ UtilsResult, UtilsError };


pub const TAG_UPDATE: &'static str = "update";



pub fn find_latest_link(links: Vec<Link>) -> UtilsResult<Option<Link>> {
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

pub fn fetch_entry_latest(addr: EntryHash) -> UtilsResult<(HeaderHash, Element)> {
    //
    // - Get event details `ElementDetails` (expect to be a Create or Update)
    // - If it has updates, select the one with the latest Header timestamp
    // - Get the update element
    //
    let result = get(addr.clone(), GetOptions::latest())
	.map_err(UtilsError::HDKError)?;

    let mut element = result.ok_or(UtilsError::EntryNotFoundError(addr.clone()))?;

    let update_links = get_links(addr.clone(), Some(LinkTag::new(TAG_UPDATE)))
	.map_err(UtilsError::HDKError)?.into_inner();

    if update_links.len() > 0 {
	let latest_link = find_latest_link( update_links )?;

	if let Some(link) = latest_link {
	    let result = get(link.target, GetOptions::latest())
		.map_err(UtilsError::HDKError)?;

	    if let Some(v) = result {
		element = v;
	    }
	}
    }

    Ok( (element.header_address().to_owned(), element) )
}

pub fn fetch_entry(addr: EntryHash) -> UtilsResult<(HeaderHash, Element)> {
    match get(addr.clone(), GetOptions::latest())
	.map_err(UtilsError::HDKError)? {
        Some(element) => Ok((element.header_address().to_owned(), element)),
        None => Err(UtilsError::EntryNotFoundError(addr.clone())),
    }
}

pub fn fetch_entity(id: &EntryHash) -> UtilsResult<Entity<Element>> {
    let (header_hash, element) = fetch_entry_latest( id.clone() )?;

    let address = element
	.header()
	.entry_hash()
	.ok_or(UtilsError::EntryNotFoundError(id.clone()))?;

    Ok(Entity {
	id: id.clone(),
	header: header_hash,
	address: address.to_owned(),
	ctype: EntityType {
	    name: "element",
	    model: "entry",
	},
	content: element,
    })
}

#[macro_export]
macro_rules! try_from_element {
    ( $( $struct:ident ),* ) => {
	$(
	    impl TryFrom<&Element> for $struct {
		type Error = hc_dna_utils::UtilsError;

		fn try_from(element: &Element) -> Result<Self, Self::Error> {
		    let entry = element.entry()
			.to_app_option::<Self>().map_err( |e| hc_dna_utils::UtilsError::HDKError( hdk::prelude::WasmError::Serialize(e) ) )?
			.ok_or( hc_dna_utils::UtilsError::DeserializationError(element.clone()) )?;

		    let entry_hash = hash_entry(&entry)
			.map_err( |e| hc_dna_utils::UtilsError::HDKError(e) )?;
		    let expected_hash = element.header().entry_hash().unwrap().clone();

		    if entry_hash == expected_hash {
			Ok( entry )
		    } else {
			Err( hc_dna_utils::UtilsError::DeserializationWrongEntryTypeError(entry_hash, expected_hash) )
		    }
		}
	    }
	)*
    };
}

#[macro_export]
macro_rules! catch { // could change to "trap", "snare", or "capture"
    ( $r:expr ) => {
	match $r {
	    Ok(x) => x,
	    Err(e) => return Ok(DevHubResponse::error( (&e).into(), None )),
	}
    };
    ( $r:expr, $e:expr ) => {
	match $r {
	    Ok(x) => x,
	    Err(e) => return Ok(DevHubResponse::error( (&$e).into(), None )),
	}
    };
}
