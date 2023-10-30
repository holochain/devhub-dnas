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
use apphub::{
    LinkTypes,
    WebAppEntry,
    WebAppPackageVersionEntry,
    Authority,
    BundleAddr,
    hc_crud::{
        Entity, EntityId,
        create_entity, get_entity,
    },
};



#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CreateWebAppPackageVersionEntryInput {
    pub for_package: EntityId,
    pub webapp: BundleAddr,

    // Optional
    pub maintainer: Option<Authority>,
    pub source_code_revision_url: Option<String>,
}

#[hdk_extern]
fn create_webapp_package_version_entry(input: CreateWebAppPackageVersionEntryInput) -> ExternResult<Entity<WebAppPackageVersionEntry>> {
    let agent_id = hdk_extensions::agent_id()?;
    let webapp_entry = WebAppEntry::try_from_record( &must_get( &input.webapp )? )?;

    // Get webapp entry info to construct the integrity entry info
    let entry = WebAppPackageVersionEntry {
        for_package: input.for_package,
        webapp: input.webapp,
        webapp_token: webapp_entry.webapp_token,
        maintainer: agent_id.clone().into(),
        source_code_revision_url: input.source_code_revision_url,
    };

    let entity = create_entity( &entry )?;

    create_link( agent_id, entity.id.clone(), LinkTypes::WebAppPackageVersion, () )?;

    Ok( entity )
}

#[hdk_extern]
fn get_webapp_package_version_entry(addr: ActionHash) ->
    ExternResult<Entity<WebAppPackageVersionEntry>>
{
    Ok( get_entity( &addr )? )
}
