
mod errors;

use hdk::prelude::*;
use devhub_types::{ Entity, EntityType, EntryModel };

pub use errors::{ UtilsResult, UtilsError };


pub const TAG_UPDATE: &'static str = "update";
pub const TAG_ORIGIN: &'static str = "origin";



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

pub fn get_id_for_addr(addr: EntryHash) -> UtilsResult<EntryHash> {
    let parent_links = get_links(addr.clone(), Some(LinkTag::new(TAG_ORIGIN)))
	.map_err(UtilsError::HDKError)?.into_inner();

    match parent_links.len() {
	0 => Ok( addr ),
	1 => Ok( parent_links.first().unwrap().target.clone() ),
	_ => Err( UtilsError::MultipleOriginsError(addr) ),
    }
}

pub fn fetch_entry(addr: EntryHash) -> UtilsResult<(HeaderHash, Element)> {
    let element = get(addr.clone(), GetOptions::latest())
	.map_err( UtilsError::HDKError )?
	.ok_or( UtilsError::EntryNotFoundError(addr.clone()) )?;

    Ok( (element.header_address().to_owned(), element) )
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
	ctype: EntityType::new( "element", "entry" ),
	content: element,
    })
}


pub fn create_entity<T>(input: &T, tag: &str) -> ExternResult<Entity<T>>
where
    T: Clone + EntryModel,
    EntryWithDefId: TryFrom<T, Error = WasmError>,
{
    let entry_w_id = EntryWithDefId::try_from( input.to_owned() )?;
    let entry: &Entry = entry_w_id.as_ref();

    let pubkey = agent_info()?.agent_initial_pubkey;
    let entry_hash = hash_entry( entry.to_owned() )?;

    let header_hash = create( entry_w_id )?;

    debug!("Linking pubkey ({}) to ENTRY: {}", pubkey, entry_hash );
    create_link(
	pubkey.into(),
	entry_hash.clone(),
	LinkTag::new( tag )
    )?;

    Ok( Entity {
	id: entry_hash.clone(),
	address: entry_hash,
	header: header_hash,
	ctype: input.get_type(),
	content: input.to_owned(),
    } )
}

pub fn update_entity<T, F>(id: Option<EntryHash>, addr: EntryHash, callback: F) -> ExternResult<Entity<T>>
where
    T: Clone + EntryModel,
    EntryWithDefId: TryFrom<T, Error = WasmError>,
    F: FnOnce(Element) -> ExternResult<T>,
{
    let id = match id {
	Some(id) => id,
	None => {
	    get_id_for_addr( addr.clone() )
		.map_err(|e| WasmError::Guest("Nothing here".into()) )?
	},
    };

    let (header, element) = fetch_entry( addr.clone() )
	.map_err(|e| WasmError::Guest("Nothing here".into()) )?;

    let current = callback( element )?;

    let entry_w_id = EntryWithDefId::try_from( current.clone() )?;
    let entry: &Entry = entry_w_id.as_ref();

    let entry_hash = hash_entry( entry.to_owned() )?;
    let header_hash = update( header, entry_w_id )?;

    debug!("Linking original ({}) to DNA: {}", addr, entry_hash );
    create_link(
	addr.clone(),
	entry_hash.clone(),
	LinkTag::new(TAG_UPDATE)
    )?;

    debug!("Linking DNA ({}) to original: {}", entry_hash, addr );
    create_link(
	entry_hash.clone(),
	addr.clone(),
	LinkTag::new(TAG_ORIGIN)
    )?;

    Ok(	Entity {
	id: id,
	header: header_hash,
	address: entry_hash,
	ctype: current.get_type(),
	content: current,
    } )
}



#[macro_export]
macro_rules! try_from_element {
    ( $( $struct:ident ),* ) => {
	$(
	    impl TryFrom<&Element> for $struct {
		type Error = hc_dna_utils::UtilsError;

		fn try_from(element: &Element) -> Result<Self, Self::Error> {
		    let entry = element.entry()
			.to_app_option::<Self>().map_err( |e| hc_dna_utils::UtilsError::HDKError(e.into()) )?
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
