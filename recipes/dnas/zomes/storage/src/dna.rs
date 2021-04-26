use hdk::prelude::*;

use crate::constants::{ TAG_DNA };
use crate::entry_types::{ DnaEntry, EntityInfo, DeprecationNotice };
use crate::errors::{ RuntimeError };


fn fetch_entry(addr: EntryHash) -> ExternResult<(HeaderHash, Element)> {
    match get(addr, GetOptions::latest())? {
        Some(element) => Ok((element.header_address().to_owned(), element)),
        None => Err(WasmError::from(RuntimeError::EntryNotFound)),
    }
}



#[derive(Debug, Deserialize)]
pub struct DnaInput {
    pub name: String,
    pub description: String,

    // optional
    pub published_at: Option<u64>,
    pub developer: Option<EntityInfo>,
    pub deprecation: Option<DeprecationNotice>,
}

#[hdk_extern]
fn create_dna(input: DnaInput) -> ExternResult<(EntryHash, DnaEntry)> {
    debug!("Creating DNA: {}", input.name );
    let dna = DnaEntry {
	name: input.name,
	description: input.description,
	developer: input.developer,
	deprecation: input.deprecation,
	published_at: match input.published_at {
	    None => {
		sys_time()?.as_millis() as u64
	    },
	    Some(t) => t,
	},
    };

    create_entry(&dna)?;
    let entry_hash = hash_entry(&dna)?;
    let pubkey = agent_info()?.agent_initial_pubkey;

    debug!("Linking pubkey ({}) to DNA: {}", pubkey, entry_hash );
    create_link(
	pubkey.into(),
	entry_hash.clone(),
	LinkTag::new(*TAG_DNA)
    )?;

    Ok( (entry_hash, dna) )
}



#[derive(Debug, Deserialize)]
pub struct GetDnaInput {
    pub addr: EntryHash,
}

#[hdk_extern]
fn get_dna(input: GetDnaInput) -> ExternResult<DnaEntry> {
    debug!("Get DNA: {}", input.addr );
    let (_, element) = fetch_entry(input.addr)?;

    DnaEntry::try_from(element)
}



fn get_my_dna_links() -> ExternResult<Vec<Link>> {
    let pubkey = agent_info()?.agent_initial_pubkey;

    debug!("Getting DNA links for Agent: {}", pubkey );
    let all_links: Vec<Link> = get_links(
        pubkey.into(),
	Some(LinkTag::new(*TAG_DNA))
    )?.into();

    Ok(all_links)
}

#[hdk_extern]
fn get_my_dnas(_:()) -> ExternResult<Vec<Option<DnaEntry>>> {
    let links = get_my_dna_links()?;

    let dnas: Vec<Option<DnaEntry>> = links.into_iter()
	.map(|link| get(link.target, GetOptions::latest()))
	.map(|element_or_err| match element_or_err {
	    Err(e) => Err(e),
	    Ok(None) => Ok(None),
	    Ok(Some(element)) => {
		if let element::ElementEntry::Present( entry ) = element.entry() {
		    if let Ok(dna) = DnaEntry::try_from(entry) {
			return Ok(Some(dna))
		    }
		}
		Err(WasmError::from(RuntimeError::DeserializationError(element)))
	    },
	})
	.collect::<ExternResult<_>>()?;
    Ok(dnas)
}
