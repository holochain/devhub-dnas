mod dna_handlers;

pub use dnahub::hdi;
pub use dnahub::hdi_extensions;
pub use devhub_sdk::hdk;
pub use devhub_sdk::hdk_extensions;

use lazy_static::lazy_static;
use hdk::prelude::*;
use hdk_extensions::{
    agent_id,
};
use dnahub::{
    LinkTypes,
};
use dnahub_sdk::{
    LinkBase,
};


pub type TypedLinkBase = LinkBase<LinkTypes>;

lazy_static! {
    pub static ref AGENT_ID : AgentPubKey = agent_id().expect("Unable to obtain current Agent context");

    pub static ref MY_DNAS_ANCHOR : TypedLinkBase = LinkBase::new( AGENT_ID.clone(), LinkTypes::Dna );
}


#[hdk_extern]
fn init(_: ()) -> ExternResult<InitCallbackResult> {
    let zome_settings = zome_info()?;
    let zome_name = zome_settings.name;
    debug!("'{}' init", zome_name );

    let granted_functions = zome_settings.extern_fns.into_iter()
        .filter_map(|fn_name| match fn_name.as_ref().starts_with("get_") {
            true => Some(( zome_name.0.as_ref(), fn_name.0 )),
            false => None,
        })
        .collect();

    portal_sdk::register_if_exists!({
        dna: dna_info()?.hash,
        granted_functions,
    })?;

    Ok(InitCallbackResult::Pass)
}


#[hdk_extern]
fn whoami(_: ()) -> ExternResult<AgentInfo> {
    Ok( agent_info()? )
}
