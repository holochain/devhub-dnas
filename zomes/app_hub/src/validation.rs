use crate::hdi;
use crate::hdi::prelude::*;
use crate::hdi_extensions::{
    // Macros
    valid, // invalid,
};
use crate::{
    EntryTypes,
    LinkTypes,
};


#[hdk_extern]
fn validate(op: Op) -> ExternResult<ValidateCallbackResult> {
    let _result = match op.flattened::<EntryTypes, LinkTypes>()? {
        _ => valid!(),
    };
}
