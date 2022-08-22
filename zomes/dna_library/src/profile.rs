use dnarepo_core::{
    LinkTypes,
};
use devhub_types::{
    AppResult,
    errors::{ UserError },
    dnarepo_entry_types::{ ProfileEntry },
    fmt_path,
};
use hc_crud::{
    create_entity, get_entity, update_entity,
    Entity,
};
use hdk::prelude::*;

// use crate::constants::{
//     LT_NONE,
//     TAG_PROFILE,
//     TAG_FOLLOW,
// };



#[derive(Debug, Deserialize)]
pub struct ProfileInput {
    pub name: String,
    pub avatar_image: SerializedBytes,

    // optional
    pub email: Option<String>,
    pub website: Option<String>,
}

pub fn create_profile(input: ProfileInput) -> AppResult<Entity<ProfileEntry>> {
    debug!("Creating Profile: {}", input.name );
    let profile = ProfileEntry {
	name: input.name,
	email: match input.email {
	    None => String::from(""),
	    Some(e) => e,
	},
	website: match input.website {
	    None => String::from(""),
	    Some(w) => w,
	},
	avatar_image: input.avatar_image,
    };

    let entity = create_entity( &profile )?;

    let (agent_base, agent_base_hash) = devhub_types::create_path( &crate::agent_path_base( None ), vec![ "profiles" ]);
    debug!("Linking agent root path ({}) to Profile: {}", fmt_path( &agent_base ), entity.id );
    entity.link_from( &agent_base_hash, LinkTypes::Profile, None )?;

    Ok( entity )
}



pub fn get_profile_links(maybe_pubkey: Option<AgentPubKey> ) -> ExternResult<Vec<Link>> {
    let (agent_base, agent_base_hash) = devhub_types::create_path( &crate::agent_path_base( maybe_pubkey ), vec![ "profiles" ]);

    debug!("Getting Profile links for path '{}'", fmt_path( &agent_base ) );
    let all_links: Vec<Link> = get_links(
        agent_base_hash,
	LinkTypes::Profile,
	None
    )?.into();

    Ok( all_links )
}

/// Finds and returns the Link with the earliest timestamp from a list
fn find_earliest_link(links: Vec<Link>) -> Option<Link> {
    if links.len() == 0 {
	None
    }
    else {
	Some( links.iter()
            .fold( None, |acc, link| {
		let ts = link.timestamp;
		match acc {
		    None => Some( (ts, link.to_owned()) ),
		    Some(current) => {
			Some(match current.0 < ts {
			    true => current,
			    false => (ts, link.to_owned()),
			})
		    }
		}
	    }).unwrap().1 )
    }
}

#[derive(Debug, Deserialize)]
pub struct GetProfileInput {
    pub agent: Option<AgentPubKey>,
}

pub fn get_profile(input: GetProfileInput) -> AppResult<Entity<ProfileEntry>> {
    let links = get_profile_links( input.agent )?;

    let link = find_earliest_link( links )
	.ok_or( UserError::CustomError("Agent Profile has not been created yet") )?;

    debug!("Get Profile: {}", link.target );
    let entity = get_entity( &link.target.into() )?;

    Ok( entity )
}




#[derive(Debug, Deserialize)]
pub struct UpdateProfileInput {
    pub addr: ActionHash,
    pub properties: ProfileUpdateOptions
}
#[derive(Debug, Deserialize)]
pub struct ProfileUpdateOptions {
    pub name: Option<String>,
    pub email: Option<String>,
    pub website: Option<String>,
    pub avatar_image: Option<SerializedBytes>,
}

pub fn update_profile(input: UpdateProfileInput) -> AppResult<Entity<ProfileEntry>> {
    let props = input.properties;

    let entity : Entity<ProfileEntry> = update_entity(
	&input.addr,
	|current : ProfileEntry, _| {
	    Ok(ProfileEntry {
		name: props.name
		    .unwrap_or( current.name ),
		email: props.email
		    .unwrap_or( current.email ),
		website: props.website
		    .unwrap_or( current.website ),
		avatar_image: props.avatar_image
		    .unwrap_or( current.avatar_image ),
	    })
	})?;

    Ok( entity )
}



//
// Following
//
#[derive(Debug, Deserialize)]
pub struct FollowInput {
    pub agent: AgentPubKey,
}

pub fn follow_developer(input: FollowInput) -> AppResult<ActionHash> {
    let (my_agent_base, my_agent_base_hash) = devhub_types::create_path( &crate::agent_path_base( None ), Vec::<String>::new() );
    let (other_agent_base, other_agent_base_hash) = devhub_types::create_path( &crate::agent_path_base( Some(input.agent) ), Vec::<String>::new() );

    debug!("Creating follow link from this agent ({}) to agent: {}", fmt_path( &my_agent_base ), fmt_path( &other_agent_base ) );

    let action_hash = create_link(
	my_agent_base_hash,
	other_agent_base_hash,
	LinkTypes::Following,
	()
    )?;

    Ok( action_hash )
}


#[derive(Debug, Deserialize)]
pub struct UnfollowInput {
    pub agent: AgentPubKey,
}

pub fn unfollow_developer(input: UnfollowInput) -> AppResult<Option<ActionHash>> {
    let links = get_following()?;
    let (other_agent_base, other_agent_base_hash) = devhub_types::create_path( &crate::agent_path_base( Some(input.agent.to_owned()) ), Vec::<String>::new() );

    debug!("Unfollow Agent: {}", fmt_path( &other_agent_base ) );
    let maybe_link = links
	.into_iter()
	.find(|link| link.target == other_agent_base_hash.to_owned().into() );
    let mut maybe_action : Option<ActionHash> = None;

    if let Some(link) = maybe_link {
	debug!("Deleting follow link to agent: {}", input.agent );

	let action_hash = delete_link( link.create_link_hash )?;

	maybe_action.replace(action_hash);
    }

    Ok( maybe_action )
}


pub fn get_following() -> AppResult<Vec<Link>> {
    let (my_agent_base, my_agent_base_hash) = devhub_types::create_path( &crate::agent_path_base( None ), Vec::<String>::new() );

    debug!("Getting Profile links for Agent: {}", fmt_path( &my_agent_base ) );
    let all_links: Vec<Link> = get_links(
        my_agent_base_hash.to_owned(),
	LinkTypes::Following,
	None
    )?.into();

    Ok(all_links)
}
