use crate::{
    hdi,
    hdi_extensions,
    EntryTypes,
    LinkTypes,
};

use hdi::prelude::*;
use hdi_extensions::{
    // Macros
    valid, // invalid,
};


#[hdk_extern]
fn validate(op: Op) -> ExternResult<ValidateCallbackResult> {
    let _result = match op.flattened::<EntryTypes, LinkTypes>()? {
        _ => valid!(),
    };
}
