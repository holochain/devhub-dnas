mod app_handlers;
mod ui_handlers;
mod webapp_handlers;
mod webapp_package_handlers;
mod webapp_package_version_handlers;
mod webapp_package_anchor;


pub use apphub::hdi;
pub use apphub::hdi_extensions;
pub use apphub::hc_crud;
pub use apphub_sdk::hdk;
pub use apphub_sdk::hdk_extensions;
pub use webapp_package_anchor::*;

use hdk::prelude::*;



#[hdk_extern]
fn init(_: ()) -> ExternResult<InitCallbackResult> {
    Ok(InitCallbackResult::Pass)
}


#[hdk_extern]
fn whoami(_: ()) -> ExternResult<AgentInfo> {
    Ok( agent_info()? )
}
