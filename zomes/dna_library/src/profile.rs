use devhub_types::{
    AppResult,
    errors::{ UserError },
    dnarepo_entry_types::{ ProfileEntry, ProfileInfo },
};
use hc_crud::{
    create_entity, get_entity, update_entity, find_latest_link,
    Entity, Collection,
};
use hdk::prelude::*;

use crate::constants::{ TAG_PROFILE, TAG_FOLLOW };



#[derive(Debug, Deserialize)]
pub struct ProfileInput {
    pub name: String,
    pub avatar_image: SerializedBytes,

    // optional
    pub email: Option<String>,
    pub website: Option<String>,
}

pub fn create_profile(input: ProfileInput) -> AppResult<Entity<ProfileInfo>> {
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

    let entity = create_entity( &profile )?
	.change_model( |profile| profile.to_info() );

    let root_path = crate::root_path( None )?;
    let base = root_path.hash()?;

    debug!("Linking agent root path ({}) to Profile: {}", base, entity.id );
    entity.link_from( &base, TAG_PROFILE.into() )?;

    Ok( entity )
}



pub fn get_profile_links(maybe_pubkey: Option<AgentPubKey> ) -> ExternResult<Vec<Link>> {
    let root_path = crate::root_path( maybe_pubkey )?;
    let base = root_path.hash()?;

    debug!("Getting Profile links for Agent: {}", base );
    let all_links: Vec<Link> = get_links(
        base.clone(),
	Some(LinkTag::new(TAG_PROFILE))
    )?.into();

    Ok( all_links )
}

#[derive(Debug, Deserialize)]
pub struct GetProfileInput {
    pub agent: Option<AgentPubKey>,
}

pub fn get_profile(input: GetProfileInput) -> AppResult<Entity<ProfileInfo>> {
    let links = get_profile_links( input.agent )?;

    let link = find_latest_link( links )
	.ok_or( UserError::CustomError("Agent Profile has not been created yet") )?;

    debug!("Get Profile: {}", link.target );
    let entity = get_entity::<ProfileEntry>( &link.target )?;

    Ok( entity.change_model( |profile| profile.to_info() ) )
}




#[derive(Debug, Deserialize)]
pub struct UpdateProfileInput {
    pub addr: EntryHash,
    pub properties: ProfileUpdateOptions
}
#[derive(Debug, Deserialize)]
pub struct ProfileUpdateOptions {
    pub name: Option<String>,
    pub email: Option<String>,
    pub website: Option<String>,
    pub avatar_image: Option<SerializedBytes>,
}

pub fn update_profile(input: UpdateProfileInput) -> AppResult<Entity<ProfileInfo>> {
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

    Ok( entity.change_model( |profile| profile.to_info() ) )
}



//
// Following
//
#[derive(Debug, Deserialize)]
pub struct FollowInput {
    pub agent: AgentPubKey,
}

pub fn follow_developer(input: FollowInput) -> AppResult<HeaderHash> {
    let my_agent = crate::root_path( None )?.hash()?;
    let other_agent = crate::root_path( Some(input.agent) )?.hash()?;

    debug!("Creating follow link from this agent ({}) to agent: {}", my_agent, other_agent );

    let header_hash = create_link(
	my_agent,
	other_agent,
	LinkTag::new( TAG_FOLLOW )
    )?;

    Ok( header_hash )
}


#[derive(Debug, Deserialize)]
pub struct UnfollowInput {
    pub agent: AgentPubKey,
}

pub fn unfollow_developer(input: UnfollowInput) -> AppResult<Option<HeaderHash>> {
    let links = get_following()?.items;
    let other_agent = crate::root_path( Some(input.agent.to_owned()) )?.hash()?;

    let maybe_link = links
	.into_iter()
	.find(|link| link.target == other_agent );
    let mut maybe_header : Option<HeaderHash> = None;

    if let Some(link) = maybe_link {
	debug!("Deleting follow link to agent: {}", input.agent );

	let header_hash = delete_link( link.create_link_hash )?;

	maybe_header.replace(header_hash);
    }

    Ok( maybe_header )
}


pub fn get_following() -> AppResult<Collection<Link>> {
    let my_agent = crate::root_path( None )?.hash()?;

    debug!("Getting Profile links for Agent: {}", my_agent );
    let all_links: Vec<Link> = get_links(
        my_agent.to_owned(),
	Some(LinkTag::new(TAG_FOLLOW))
    )?.into();

    Ok(Collection {
	base: my_agent,
	items: all_links
    })
}
