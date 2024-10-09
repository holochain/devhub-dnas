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
    create_link_input,
};



#[hdk_extern]
pub fn create_named_group_link(
    (name, group_id): (String, ActionHash)
) -> ExternResult<ActionHash> {
    Ok(
        create_link(
            agent_id()?,
            group_id,
            LinkTypes::NameToGroup,
            name.as_bytes().to_vec()
        )?
    )
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
