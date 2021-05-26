use hdk::prelude::*;

use crate::utils;
use crate::constants::{ TAG_DNAVERSION, TAG_UPDATE };
use crate::entry_types::{ DnaVersionEntry, DnaVersionInfo, DnaVersionSummary };


#[derive(Debug, Deserialize)]
pub struct DnaVersionInput {
    pub for_dna: EntryHash,
    pub version: u64,
    pub file_size: u64,
    pub chunk_addresses: Vec<EntryHash>,

    // optional
    pub changelog: Option<String>,
    pub contributors: Option<Vec<String>>,
    pub published_at: Option<u64>,
}

#[hdk_extern]
fn create_dna_version(input: DnaVersionInput) -> ExternResult<(EntryHash, DnaVersionInfo)> {
    debug!("Creating DNA version ({}) for DNA: {}", input.version, input.for_dna );
    let version = DnaVersionEntry {
	for_dna: input.for_dna.clone(),
	version: input.version,
	file_size: input.file_size,
	chunk_addresses: input.chunk_addresses,
	changelog: match input.changelog {
	    None => String::from(""),
	    Some(x) => x,
	},
	contributors: match input.contributors {
	    None => vec![],
	    Some(c) => c,
	},
	published_at: match input.published_at {
	    None => {
		sys_time()?.as_millis() as u64
	    },
	    Some(t) => t,
	},
    };

    let header_hash = create_entry(&version)?;
    debug!("Created new DNA Version via header ({})", header_hash );

    let entry_hash = hash_entry(&version)?;

    debug!("Linking DNA ({}) to manifest: {}", input.for_dna, entry_hash );
    create_link(
	input.for_dna,
	entry_hash.clone(),
	LinkTag::new(*TAG_DNAVERSION)
    )?;

    Ok( (entry_hash.clone(), version.to_info( entry_hash )) )
}




#[derive(Debug, Deserialize)]
pub struct GetDnaVersionInput {
    pub addr: EntryHash,
}

#[hdk_extern]
fn get_dna_version(input: GetDnaVersionInput) -> ExternResult<DnaVersionInfo> {
    debug!("Get DNA Version: {}", input.addr );
    let (_, element) = utils::fetch_entry_latest(input.addr.clone())?;

    Ok(DnaVersionEntry::try_from(element)?.to_info( input.addr ))
}




fn get_version_links(dna: EntryHash) -> ExternResult<Vec<Link>> {
    debug!("Getting version links for DNA: {}", dna );
    let all_links: Vec<Link> = get_links(
        dna,
	Some(LinkTag::new(*TAG_DNAVERSION))
    )?.into();

    Ok(all_links)
}


#[derive(Debug, Deserialize)]
pub struct GetDnaVersionsInput {
    pub for_dna: EntryHash,
}

#[hdk_extern]
fn get_dna_versions(input: GetDnaVersionsInput) -> ExternResult<Vec<(EntryHash, DnaVersionSummary)>> {
    let links = get_version_links(input.for_dna)?;

    let versions = links.into_iter()
	.filter_map(|link| {
	    match utils::fetch_entry_latest(link.target.clone()) {
		Ok((_, element)) => Some((link.target, element)),
		Err(_) => None
	    }
	})
	.filter_map(|(hash, element)| {
	    match DnaVersionEntry::try_from( element ) {
		Err(_) => None,
		Ok(version) => Some(( hash.clone(), version.to_summary(hash) )),
	    }
	})
	.collect();
    Ok( versions )
}




#[derive(Debug, Deserialize)]
pub struct UpdateDnaVersionInput {
    pub addr: EntryHash,
    pub properties: DnaVersionUpdateOptions
}
#[derive(Debug, Deserialize)]
pub struct DnaVersionUpdateOptions {
    pub changelog: Option<String>,
    pub contributors: Option<Vec<String>>,
    pub published_at: Option<u64>,
}

#[hdk_extern]
fn update_dna_version(input: UpdateDnaVersionInput) -> ExternResult<(EntryHash, DnaVersionInfo)> {
    debug!("Updating DNA Version: {}", input.addr );
    let (header, element) = utils::fetch_entry_latest(input.addr.clone())?;
    let current_version = DnaVersionEntry::try_from( element )?;

    let version = DnaVersionEntry {
	for_dna: current_version.for_dna,
	version: current_version.version,
	published_at: match input.properties.published_at {
	    None => current_version.published_at,
	    Some(v) => v,
	},
	file_size: current_version.file_size,
	chunk_addresses: current_version.chunk_addresses,
	changelog: match input.properties.changelog {
	    None => current_version.changelog,
	    Some(v) => v,
	},
	contributors: match input.properties.contributors {
	    None => current_version.contributors,
	    Some(v) => v,
	},
    };

    update_entry(header, &version)?;
    let entry_hash = hash_entry(&version)?;

    debug!("Linking original ({}) to DNA Version: {}", input.addr, entry_hash );
    create_link(
	input.addr.clone(),
	entry_hash.clone(),
	LinkTag::new(TAG_UPDATE)
    )?;

    Ok( (entry_hash, version.to_info(input.addr)) )
}




#[derive(Debug, Deserialize)]
pub struct DeleteDnaVersionInput {
    pub addr: EntryHash,
}

#[hdk_extern]
fn delete_dna_version(input: DeleteDnaVersionInput) -> ExternResult<HeaderHash> {
    debug!("Delete DNA Version: {}", input.addr );
    let (header, _) = utils::fetch_entry(input.addr.clone())?;

    let delete_header = delete_entry(header.clone())?;
    debug!("Deleted DNA Version create {} via header ({})", header, delete_header );

    Ok( header )
}
