use crate::{
    hdk,
    hdk_extensions,
    MY_ZOME_PACKS_ANCHOR,
};
use hdk::prelude::*;
use hdk_extensions::{
    must_get,
    hdi_extensions::{
        guest_error,
        ScopedTypeConnector,
        AnyLinkableHashTransformer,
    },
};
use zomehub::{
    LinkTypes,

    ZomePackageEntry,
    hc_crud::{
        Entity, EntityId,
        EntryModel,
        create_entity, get_entity,
    },
};
use zomehub_sdk::{
    LinkBase,
    CreateZomePackageInput,
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
    Ok( get_entity( &addr )? )
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
