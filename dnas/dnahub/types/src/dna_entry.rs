pub use crate::{
    hdi,
    hdi_extensions,
    DnaToken,
    IntegritiesToken,
    CoordinatorsToken,
    holochain_types::{
        DnaManifestV1,
    },
};

use hdi::prelude::*;


//
// Dna Entry
//
#[hdk_entry_helper]
#[derive(Clone)]
pub struct DnaEntry {
    pub manifest: DnaManifestV1,
    pub dna_token: DnaToken,
    pub integrities_token: IntegritiesToken,
    pub coordinators_token: CoordinatorsToken,
}

impl DnaEntry {
    pub fn new(manifest: DnaManifestV1) -> ExternResult<Self> {
        let dna_token = manifest.dna_token()?;
        let integrities_token = manifest.integrities_token()?;
        let coordinators_token = manifest.coordinators_token()?;

        Ok(
            Self {
                manifest,
                dna_token,
                integrities_token,
                coordinators_token,
            }
        )
    }
}
