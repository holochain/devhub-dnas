use crate::{
    hdk,
    hdk_extensions,
    WebAppPackageAnchor,
    ALL_APPS_ANCHOR,
};

use std::collections::BTreeMap;
use hdk::prelude::*;
use apphub::{
    LinkTypes,
    WebAppPackageEntry,
    Authority,
    MemoryAddr,
    hc_crud::{
        Entity, EntityId,
        create_entity, get_entity,
    },
};
use apphub_sdk::{
    WebAppPackageVersionMap,
};



#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CreateWebAppPackageEntryInput {
    pub title: String,
    pub subtitle: String,
    pub description: String,
    pub icon: MemoryAddr,
    #[serde(default)]
    pub metadata: BTreeMap<String, rmpv::Value>,

    // Optional
    pub maintainer: Option<Authority>,
    pub source_code_url: Option<String>,
}

#[hdk_extern]
fn create_webapp_package_entry(input: CreateWebAppPackageEntryInput) -> ExternResult<Entity<WebAppPackageEntry>> {
    let agent_id = hdk_extensions::agent_id()?;
    let entry = WebAppPackageEntry {
        title: input.title,
        subtitle: input.subtitle,
        description: input.description,
        maintainer: agent_id.clone().into(),
        icon: input.icon,
        source_code_url: input.source_code_url,
        deprecation: None,
        metadata: input.metadata,
    };

    let entity = create_entity( &entry )?;

    create_link( agent_id, entity.id.clone(), LinkTypes::WebAppPackage, () )?;
    create_link( ALL_APPS_ANCHOR.clone(), entity.id.clone(), LinkTypes::WebAppPackage, () )?;

    Ok( entity )
}

#[hdk_extern]
fn get_webapp_package_entry(addr: ActionHash) -> ExternResult<Entity<WebAppPackageEntry>> {
    Ok( get_entity( &addr )? )
}


#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct LinkWebAppPackageVersionInput {
    pub version: String,
    pub webapp_package_id: EntityId,
    pub webapp_package_version_id: EntityId,
}

#[hdk_extern]
fn link_webapp_package_version(input: LinkWebAppPackageVersionInput) -> ExternResult<ActionHash> {
    create_link(
        input.webapp_package_id,
        input.webapp_package_version_id,
        LinkTypes::WebAppPackageVersion,
        input.version.as_bytes().to_vec()
    )
}


#[hdk_extern]
fn get_webapp_package_versions(webapp_package_id: EntityId) ->
    ExternResult<WebAppPackageVersionMap>
{
    let anchor = WebAppPackageAnchor::new( &webapp_package_id );

    Ok( anchor.versions()? )
}


#[hdk_extern]
fn get_all_webapp_package_entries(_: ()) -> ExternResult<Vec<Entity<WebAppPackageEntry>>> {
    let webapps = get_links( ALL_APPS_ANCHOR.clone(), LinkTypes::WebAppPackage, None )?.into_iter()
        .filter_map(|link| {
            let addr = link.target.into_action_hash()?;
            get_webapp_package_entry( addr ).ok()
        })
        .collect();

    Ok( webapps )
}
