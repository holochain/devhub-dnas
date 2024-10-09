use crate::{
    hdk,
    hdk_extensions,
    MY_ZOME_PACKS_ANCHOR,
};

use std::collections::BTreeMap;
use hdk::prelude::*;
use hdk_extensions::{
    must_get,
    hdi_extensions::{
        guest_error,
        trace_origin_root,
        ScopedTypeConnector,
        AnyLinkableHashTransformer,
    },
};
use zomehub::{
    LinkTypes,
    RmpvValue,
    Authority,

    ZomePackageEntry,
    hc_crud::{
        Entity, EntityId,
        EntryModel,
        create_entity, get_entity, update_entity,
        UpdateEntityInput,
    },
};
use zomehub_sdk::{
    LinkBase,
    CreateZomePackageInput,
};
use coop_content_sdk::{
    get_group_content_latest,
};


#[hdk_extern]
fn create_zome_package_entry(input: ZomePackageEntry) -> ExternResult<Entity<ZomePackageEntry>> {
    let entity = create_entity( &input )?;

    MY_ZOME_PACKS_ANCHOR.create_link_if_not_exists( &entity.address, () )?;

    // TODO: Link from package name
    let anchor_path = Path::from( vec![ Component::from(input.name.as_bytes().to_vec()) ] ).path_entry_hash()?;
    let name_anchor = LinkBase::new( anchor_path, LinkTypes::NameToZomePackage );
    name_anchor.create_link_if_not_exists( &entity.id, () )?;

    Ok( entity )
}


#[hdk_extern]
fn create_zome_package(input: CreateZomePackageInput) -> ExternResult<Entity<ZomePackageEntry>> {
    let entry : ZomePackageEntry = input.try_into()?;

    create_zome_package_entry( entry )
}


#[derive(Debug, Deserialize, Clone)]
pub struct UpdateProperties {
    pub name: Option<String>,
    pub title: Option<String>,
    pub description: Option<String>,
    pub maintainer: Option<Authority>,
    pub tags: Option<Vec<String>>,
    pub metadata: Option<BTreeMap<String, RmpvValue>>,
}
pub type UpdateInput = UpdateEntityInput<UpdateProperties>;

#[hdk_extern]
pub fn update_zome_package(input: UpdateInput) -> ExternResult<Entity<ZomePackageEntry>> {
    debug!("Updating zome package: {}", input.base );
    let props = input.properties.clone();

    let entity = update_entity(
	&input.base,
	|mut current : ZomePackageEntry, _| {
	    current.name = props.name
		.unwrap_or( current.name );
	    current.title = props.title
		.unwrap_or( current.title );
	    current.description = props.description
		.unwrap_or( current.description );
	    current.maintainer = props.maintainer
		.unwrap_or( current.maintainer );
	    current.tags = props.tags
		.or( current.tags );
	    current.metadata = props.metadata
		.unwrap_or( current.metadata );

	    Ok( current )
	})?;

    Ok( entity )
}


#[hdk_extern]
fn get_zome_package_entry(addr: AnyDhtHash) -> ExternResult<Entity<ZomePackageEntry>> {
    let record = must_get( &addr )?;
    let content = ZomePackageEntry::try_from_record( &record )?;
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
pub fn get_zome_package(addr: EntityId) -> ExternResult<Entity<ZomePackageEntry>> {
    let addr = trace_origin_root( &addr )?.0;
    let record = must_get( &addr )?;
    let zome_package_entry = ZomePackageEntry::try_from_record( &record )?;

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
            let content = ZomePackageEntry::try_from_record( &record )?;
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


#[hdk_extern]
pub fn get_zome_package_by_name(name: String) -> ExternResult<Entity<ZomePackageEntry>> {
    let anchor_path = Path::from( vec![ Component::from(name.as_bytes().to_vec()) ] ).path_entry_hash()?;
    let name_anchor = LinkBase::new( anchor_path, LinkTypes::NameToZomePackage );
    let package_link = name_anchor.get_links( None )?.first()
        .ok_or(guest_error!(format!(
            "No package found for name '{}'",
            name
        )))?.to_owned();

    Ok( get_entity( &package_link.target.must_be_action_hash()? )? )
}


#[hdk_extern]
fn get_zome_packages_for_agent(maybe_agent_id: Option<AgentPubKey>) ->
    ExternResult<Vec<Entity<ZomePackageEntry>>>
{
    let agent_id = match maybe_agent_id {
        Some(agent_id) => agent_id,
        None => hdk_extensions::agent_id()?,
    };
    let agent_anchor = LinkBase::new( agent_id, LinkTypes::AgentToZomePackage );

    let zome_packages = agent_anchor.get_links( None )?.into_iter()
        .filter_map(|link| {
            let addr = link.target.into_entry_hash()?;
            get_zome_package_entry( addr.into() ).ok()
        })
        .collect();

    Ok( zome_packages )
}
