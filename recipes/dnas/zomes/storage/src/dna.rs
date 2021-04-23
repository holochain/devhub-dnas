use hdk::prelude::*;

use crate::constants::{ TAG_DNA };
use crate::entry_types::{ DnaEntry, EntityInfo, DeprecationNotice };


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

    debug!("Linking pubkey ({}) to manifest: {}", pubkey, entry_hash );
    create_link(
	pubkey.into(),
	entry_hash.clone(),
	LinkTag::new(*TAG_DNA)
    )?;

    Ok( (entry_hash, dna) )
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
	.map(|link| get(link.target, GetOptions::content()))
	.map(|element_or_err| match element_or_err {
	    Err(e) => Err(e),
	    Ok(None) => Ok(None),
	    Ok(Some(element)) => {
		if let element::ElementEntry::Present( entry ) = element.entry() {
		    if let Ok(dna) = DnaEntry::try_from(entry) {
			return Ok(Some(dna))
		    }
		}
		Err(WasmError::Guest(format!("Failed to recover DnaEntry from: {:?}", element)))
	    },
	})
	.collect::<ExternResult<_>>()?;
    Ok(dnas)
}
