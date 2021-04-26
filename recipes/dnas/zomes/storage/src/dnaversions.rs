use hdk::prelude::*;

use crate::constants::{ TAG_DNAVERSION };
use crate::entry_types::{ DnaVersionEntry };


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
fn create_dna_version(input: DnaVersionInput) -> ExternResult<(EntryHash, DnaVersionEntry)> {
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

    create_entry(&version)?;
    let entry_hash = hash_entry(&version)?;

    debug!("Linking DNA ({:?}) to manifest: {:?}", input.for_dna, entry_hash );
    create_link(
	input.for_dna,
	entry_hash.clone(),
	LinkTag::new(*TAG_DNAVERSION)
    )?;

    Ok( (entry_hash, version) )
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
fn get_dna_versions(input: GetDnaVersionsInput) -> ExternResult<Vec<Option<DnaVersionEntry>>> {
    let links = get_version_links(input.for_dna)?;

    let versions: Vec<Option<DnaVersionEntry>> = links.into_iter()
	.map(|link| get(link.target, GetOptions::latest()))
	.map(|element_or_err| match element_or_err {
	    Err(e) => Err(e),
	    Ok(None) => Ok(None),
	    Ok(Some(element)) => {
		if let element::ElementEntry::Present( entry ) = element.entry() {
		    if let Ok(version) = DnaVersionEntry::try_from(entry) {
			return Ok(Some(version))
		    }
		}
		Err(WasmError::Guest(format!("Failed to recover DnaVersionEntry from: {:?}", element)))
	    },
	})
	.collect::<ExternResult<_>>()?;
    Ok(versions)
}
