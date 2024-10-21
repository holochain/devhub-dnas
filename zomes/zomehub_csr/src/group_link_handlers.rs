use crate::{
    hdk,
    hdk_extensions,
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
    all_links.sort_by_key( |link| link.timestamp ); // Ascending timestamp order

    Ok( all_links )
}
