use crate::hdk;
use crate::hdk_extensions;
use crate::hdi_extensions;

use hdk::prelude::*;
use hdk_extensions::{
    must_get,
};
use hdi_extensions::{
    ScopedTypeConnector,
};
use dnahub::{
    LinkTypes,
    DnaEntry, DnaManifestV1,
};



#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CreateDnaEntryInput {
    pub manifest: DnaManifestV1,
}

#[hdk_extern]
fn create_dna_entry(input: CreateDnaEntryInput) -> ExternResult<EntryHash> {
    let agent_id = hdk_extensions::agent_id()?;
    let entry = DnaEntry::new( input.manifest )?;

    let entry_hash = hash_entry( entry.clone() )?;
    create_entry( entry.to_input() )?;

    create_link( agent_id, entry_hash.clone(), LinkTypes::Dna, () )?;

    Ok( entry_hash )
}

#[hdk_extern]
fn get_dna_entry(addr: EntryHash) -> ExternResult<DnaEntry> {
    let record = must_get( &addr )?;

    Ok( DnaEntry::try_from_record( &record )? )
}

#[hdk_extern]
fn get_dna_entries_for_agent(maybe_agent_id: Option<AgentPubKey>) -> ExternResult<Vec<DnaEntry>> {
    let agent_id = match maybe_agent_id {
        Some(agent_id) => agent_id,
        None => hdk_extensions::agent_id()?,
    };
    let dnas = get_links( agent_id, LinkTypes::Dna, None )?.into_iter()
        .filter_map(|link| {
            let addr = link.target.into_entry_hash()?;
            get_dna_entry( addr ).ok()
        })
        .collect();

    Ok( dnas )
}
