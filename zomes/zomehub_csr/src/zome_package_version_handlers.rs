use crate::{
    hdk,
    hdk_extensions,
    ZomePackageBase,
};
use std::collections::BTreeMap;

use hdk::prelude::*;
use hdk_extensions::{
    must_get,
    hdi_extensions::{
        trace_origin_root,
        ScopedTypeConnector,
    },
};
use zomehub::{
    // LinkTypes,
    RmpvValue,
    Authority,
    ApiCompatibility,

    ZomePackageVersionEntry,
    hc_crud::{
        Entity, EntityId,
        EntryModel,
        create_entity, get_entity, update_entity,
        UpdateEntityInput,
    },
};
use zomehub_sdk::{
    // LinkBase,
    EntityPointerMap,
    ZomePackageVersionMap,
    CreateZomePackageVersionInput,
};
use coop_content_sdk::{
    get_group_content_latest,
};



#[hdk_extern]
fn create_zome_package_version_entry(input: ZomePackageVersionEntry) ->
    ExternResult<Entity<ZomePackageVersionEntry>>
{
    let entity = create_entity( &input )?;

    // TODO: Link from package

    Ok( entity )
}


#[hdk_extern]
fn create_zome_package_version(input: CreateZomePackageVersionInput) ->
    ExternResult<Entity<ZomePackageVersionEntry>>
{
    let entry : ZomePackageVersionEntry = input.clone().try_into()?;

    let entity = create_zome_package_version_entry( entry )?;

    create_zome_package_link_to_version(CreateLinkZomePackageVersionInput {
	version: input.version,
	zome_package_id: input.for_package,
	zome_package_version_addr: entity.id.clone(),
    })?;

    Ok( entity )
}


#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CreateLinkZomePackageVersionInput {
    pub version: String,
    pub zome_package_id: EntityId,
    pub zome_package_version_addr: ActionHash,
}

#[hdk_extern]
pub fn create_zome_package_link_to_version(input: CreateLinkZomePackageVersionInput) ->
    ExternResult<ActionHash>
{
    let zome_base = ZomePackageBase::new( &input.zome_package_id );

    zome_base.create_version_link( &input.zome_package_version_addr, input.version.as_str() )
}


#[hdk_extern]
pub fn get_zome_package_version_links(zome_package_id: EntityId) ->
    ExternResult<Vec<Link>>
{
    let base = ZomePackageBase::new( &zome_package_id );

    Ok( base.version_links()? )
}


#[hdk_extern]
pub fn get_zome_package_version_targets(zome_package_id: EntityId) ->
    ExternResult<EntityPointerMap>
{
    let base = ZomePackageBase::new( &zome_package_id );

    Ok( base.version_targets()? )
}


#[hdk_extern]
pub fn get_zome_package_versions(zome_package_id: EntityId) ->
    ExternResult<ZomePackageVersionMap>
{
    let base = ZomePackageBase::new( &zome_package_id );

    Ok( base.versions()? )
}


#[hdk_extern]
fn get_zome_package_version_entry(addr: AnyDhtHash) ->
    ExternResult<Entity<ZomePackageVersionEntry>>
{
    let record = must_get( &addr )?;
    let content = ZomePackageVersionEntry::try_from_record( &record )?;
    let id = record.action_address().to_owned();
    let addr = hash_entry( content.clone() )?;

    Ok(
        Entity {
            id: id.clone(),
            action: id,
	    address: addr,
	    ctype: content.get_type(),
	    content: content,
        }
    )
}

#[hdk_extern]
pub fn get_zome_package_version(addr: EntityId) -> ExternResult<Entity<ZomePackageVersionEntry>> {
    let addr = trace_origin_root( &addr )?.0;
    let record = must_get( &addr )?;
    let zome_package_entry = ZomePackageVersionEntry::try_from_record( &record )?;

    Ok(match zome_package_entry.maintainer {
        Authority::Agent(_) => {
            get_entity( &addr )?
        },
        Authority::Group(group_id, _) => {
            let latest_addr = get_group_content_latest!({
                group_id: group_id,
                content_id: addr.clone().into(),
            })?;
            let record = must_get( &latest_addr )?;
            let content = ZomePackageVersionEntry::try_from_record( &record )?;
            let id = record.action_address().to_owned();
            let hash = hash_entry( content.clone() )?;

            Entity {
                id: addr,
                action: id,
	        address: hash,
	        ctype: content.get_type(),
	        content: content,
            }
        },
    })
}


#[derive(Debug, Deserialize, Clone)]
pub struct UpdateProperties {
    pub maintainer: Option<Authority>,
    pub changelog: Option<String>,
    pub source_code_revision_uri: Option<String>,
    pub api_compatibility: Option<ApiCompatibility>,
    pub metadata: Option<BTreeMap<String, RmpvValue>>,
}
pub type UpdateInput = UpdateEntityInput<UpdateProperties>;

#[hdk_extern]
pub fn update_zome_package_version(input: UpdateInput) -> ExternResult<Entity<ZomePackageVersionEntry>> {
    debug!("Updating zome package: {}", input.base );
    let props = input.properties.clone();

    let entity = update_entity(
	&input.base,
	|mut current : ZomePackageVersionEntry, _| {
	    current.maintainer = props.maintainer
		.unwrap_or( current.maintainer );
	    current.changelog = props.changelog
		.or( current.changelog );
	    current.source_code_revision_uri = props.source_code_revision_uri
		.or( current.source_code_revision_uri );
	    current.api_compatibility = props.api_compatibility
		.unwrap_or( current.api_compatibility );
	    current.metadata = props.metadata
		.unwrap_or( current.metadata );

	    Ok( current )
	})?;

    Ok( entity )
}


#[hdk_extern]
pub fn delete_zome_package_version(id: EntityId) -> ExternResult<bool> {
    let package_version = get_zome_package_version( id.clone() )?.content;
    let zome_base = ZomePackageBase::new( &package_version.for_package );

    zome_base.version_link_base().delete_all_my_links_to_target( &id, None )?;

    Ok(true)
}
