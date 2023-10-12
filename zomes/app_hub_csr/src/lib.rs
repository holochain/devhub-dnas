mod app_handlers;
mod ui_handlers;
mod webapp_handlers;
mod webapp_package_handlers;
mod webapp_package_version_handlers;

pub use app_hub::hdi;
pub use app_hub::hdi_extensions;
pub use app_hub::hc_crud;
pub use app_hub_sdk::hdk;
pub use app_hub_sdk::hdk_extensions;

use hdk::prelude::*;



#[hdk_extern]
fn init(_: ()) -> ExternResult<InitCallbackResult> {
    Ok(InitCallbackResult::Pass)
}


#[hdk_extern]
fn whoami(_: ()) -> ExternResult<AgentInfo> {
    Ok( agent_info()? )
}
