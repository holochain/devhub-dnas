use crate::{
    hdk,
    hdk_extensions,
    ALL_ORGS_ANCHOR,
};

use hdk::prelude::*;
use hdk_extensions::{
    agent_id,
};
use zomehub::{
    LinkTypes,
};
use zomehub_sdk::{
    LinkBase,
    create_link_input,
};



#[hdk_extern]
pub fn create_named_group_link(
    (name, group_id): (String, ActionHash)
) -> ExternResult<(ActionHash, Option<ActionHash>)> {

    let anchor_path = Path::from(vec![
        Component::from(name.as_bytes().to_vec()),
    ]).path_entry_hash()?;
    let name_anchor = LinkBase::new( anchor_path, LinkTypes::NameToGroup );

    ALL_ORGS_ANCHOR.create_link_if_not_exists( &group_id, name.as_bytes().to_vec() )?;

    Ok((
        create_link(
            agent_id()?,
            group_id.clone(),
            LinkTypes::NameToGroup,
            name.as_bytes().to_vec()
        )?,
        name_anchor.create_link_if_not_exists( &group_id, () )?,
    ))
}


#[hdk_extern]
pub fn remove_named_group_link(
    name: String,
) -> ExternResult<Vec<ActionHash>> {
    let my_anchor = LinkBase::new( agent_id()?, LinkTypes::NameToGroup );
    let named_group_links = my_anchor.get_links( None )?;
    let mut delete_addrs = vec![];

    for link in named_group_links {
        if link.tag == LinkTag::from(name.as_bytes().to_vec()) {
            let mut deleted = my_anchor.delete_all_my_links_to_target( &link.target, None )?;
            delete_addrs.append( &mut deleted );
        }
    }

    Ok( delete_addrs )
}


#[hdk_extern]
pub fn get_my_group_links() -> ExternResult<Vec<Link>> {
    let my_agent = agent_id()?;

    Ok(
        get_links(
            create_link_input(
                &my_agent,
                &LinkTypes::NameToGroup,
                &None::<LinkTag>,
            )?
        )?
    )
}


#[hdk_extern]
pub fn get_org_group_links(name: String) -> ExternResult<Vec<Link>> {
    let anchor_path = Path::from(vec![
        Component::from(name.as_bytes().to_vec()),
    ]).path_entry_hash()?;
    let name_anchor = LinkBase::new( anchor_path, LinkTypes::NameToGroup );

    let mut all_links = name_anchor.get_links( None )?;
    all_links.sort_by_key( |link| link.timestamp ); // Oldest first timestamp order

    Ok( all_links )
}


#[hdk_extern]
pub fn get_all_org_group_links() -> ExternResult<Vec<Link>> {
    let mut all_links = ALL_ORGS_ANCHOR.get_links( None )?;
    all_links.sort_by_key( |link| std::cmp::Reverse(link.timestamp) ); // Newest first timestamp order

    Ok( all_links )
}
