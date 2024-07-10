use crate::{
    hdi,
    hdi_extensions,
    EntryTypes,
};

use hdi::prelude::*;
use hdi_extensions::{
    // Macros
    valid, invalid,
};

pub fn validation(
    app_entry: EntryTypes,
    _create: Create
) -> ExternResult<ValidateCallbackResult> {
    match app_entry {
        EntryTypes::Dna(dna_entry) => {
            let dna_token = dna_entry.calc_dna_token()?;
            let integrities_token = dna_entry.calc_integrities_token()?;
            let coordinators_token = dna_entry.calc_coordinators_token()?;

            if dna_entry.dna_token != dna_token {
                invalid!(format!("Invalid DNA Token; expected {:?}", dna_token ))
            }

            if dna_entry.integrities_token != integrities_token {
                invalid!(format!("Invalid Integrities Token; expected {:?}", integrities_token ))
            }

            if dna_entry.coordinators_token != coordinators_token {
                invalid!(format!("Invalid Coordinators Token; expected {:?}", coordinators_token ))
            }

            valid!()
        },
        // _ => invalid!(format!("Create validation not implemented for entry type: {:#?}", create.entry_type )),
    }
}
