use hdk::prelude::*;

use crate::utils;
use crate::constants::{ TAG_DNA, TAG_UPDATE };
use crate::entry_types::{ DnaEntry, DnaInfo, DnaSummary, EntityInfo, DeprecationNotice };



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
fn create_dna(input: DnaInput) -> ExternResult<(EntryHash, DnaInfo)> {
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

    let _header_hash = create_entry(&dna)?;
    let entry_hash = hash_entry(&dna)?;
    let pubkey = agent_info()?.agent_initial_pubkey;

    debug!("Linking pubkey ({}) to DNA: {}", pubkey, entry_hash );
    create_link(
	pubkey.into(),
	entry_hash.clone(),
	LinkTag::new(*TAG_DNA)
    )?;

    Ok( (entry_hash.clone(), dna.to_info( entry_hash )) )
}



#[derive(Debug, Deserialize)]
pub struct GetDnaInput {
    pub addr: EntryHash,
}

#[hdk_extern]
fn get_dna(input: GetDnaInput) -> ExternResult<DnaInfo> {
    debug!("Get DNA: {}", input.addr );
    let (_, element) = utils::fetch_entry_latest(input.addr.clone())?;

    Ok(DnaEntry::try_from(element)?.to_info( input.addr ))
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
fn get_my_dnas(_:()) -> ExternResult<Vec<(EntryHash, DnaSummary)>> {
    let links = get_my_dna_links()?;

    let dnas = links.into_iter()
	.filter_map(|link| {
	    match utils::fetch_entry_latest(link.target.clone()) {
		Ok((_, element)) => Some((link.target, element)),
		Err(_) => None
	    }
	})
	.filter_map(|(hash, element)| {
	    match DnaEntry::try_from( element ) {
		Err(_) => None,
		Ok(dna) => {
		    if let Some(_) = dna.deprecation {
			None
		    }
		    else {
			Some((hash.clone(), dna.to_summary( hash )))
		    }
		}
	    }
	})
	.collect();
    Ok(dnas)
}




#[derive(Debug, Deserialize)]
pub struct UpdateDnaInput {
    pub addr: EntryHash,
    pub properties: DnaUpdateOptions
}
#[derive(Debug, Deserialize)]
pub struct DnaUpdateOptions {
    pub name: Option<String>,
    pub description: Option<String>,
    pub published_at: Option<u64>,
    pub developer: Option<EntityInfo>,
}

#[hdk_extern]
fn update_dna(input: UpdateDnaInput) -> ExternResult<(EntryHash, DnaInfo)> {
    debug!("Updating DNA: {}", input.addr );
    let (header, element) = utils::fetch_entry_latest(input.addr.clone())?;
    let current_dna = DnaEntry::try_from( element )?;

    let dna = DnaEntry {
	name: match input.properties.name {
	    None => current_dna.name,
	    Some(v) => v,
	},
	description: match input.properties.description {
	    None => current_dna.description,
	    Some(v) => v,
	},
	published_at: match input.properties.published_at {
	    None => current_dna.published_at,
	    Some(v) => v,
	},
	developer: match input.properties.developer {
	    None => current_dna.developer,
	    Some(v) => Some(v),
	},
	deprecation: current_dna.deprecation,
    };

    update_entry(header, &dna)?;
    let entry_hash = hash_entry(&dna)?;

    debug!("Linking original ({}) to DNA: {}", input.addr, entry_hash );
    create_link(
	input.addr.clone(),
	entry_hash.clone(),
	LinkTag::new(TAG_UPDATE)
    )?;

    Ok( (entry_hash, dna.to_info( input.addr )) )
}




#[derive(Debug, Deserialize)]
pub struct DeprecateDnaInput {
    pub addr: EntryHash,
    pub message: String,
}

#[hdk_extern]
fn deprecate_dna(input: DeprecateDnaInput) -> ExternResult<(EntryHash, DnaInfo)> {
    debug!("Deprecating DNA: {}", input.addr );
    let (header, element) = utils::fetch_entry_latest(input.addr.clone())?;
    let current_dna = DnaEntry::try_from( element )?;

    let dna = DnaEntry {
	name: current_dna.name,
	description: current_dna.description,
	published_at: current_dna.published_at,
	developer: current_dna.developer,
	deprecation: Some(DeprecationNotice::new( input.message )),
    };

    update_entry(header, &dna)?;
    let entry_hash = hash_entry(&dna)?;

    debug!("Linking original ({}) to DNA: {}", input.addr, entry_hash );
    create_link(
	input.addr.clone(),
	entry_hash.clone(),
	LinkTag::new(TAG_UPDATE)
    )?;

    Ok( (entry_hash, dna.to_info( input.addr )) )
}
