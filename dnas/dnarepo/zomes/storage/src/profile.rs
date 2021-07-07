use devhub_types::{
    constants::{ AppResult },
    errors::{ UserError },
    dna_entry_types::{ ProfileEntry, ProfileInfo },
};
use hc_entities::{ Entity, Collection };
use hc_dna_utils as utils;
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
    let pubkey = agent_info()?.agent_initial_pubkey;

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

    let entity = utils::create_entity( &profile )?
	.new_content( profile.to_info() );

    debug!("Linking pubkey ({}) to Profile: {}", pubkey, entity.id );
    create_link(
	pubkey.into(),
	entity.id.clone(),
	LinkTag::new( TAG_PROFILE )
    )?;

    Ok( entity )
}



pub fn get_profile_links(maybe_pubkey: Option<AgentPubKey> ) -> ExternResult<Vec<Link>> {
    let base : EntryHash = match maybe_pubkey {
	None => agent_info()?.agent_initial_pubkey,
	Some(agent) => agent,
    }.into();

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

    let link = utils::find_latest_link( links )
	.ok_or( UserError::CustomError("Agent Profile has not been created yet") )?;

    debug!("Get Profile: {}", link.target );
    let entity = utils::get_entity( &link.target )?;
    let info = ProfileEntry::try_from( &entity.content )?.to_info();

    Ok( entity.new_content( info ) )
}




#[derive(Debug, Deserialize)]
pub struct UpdateProfileInput {
    pub id: Option<EntryHash>,
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

    let entity : Entity<ProfileEntry> = utils::update_entity(
	input.id, input.addr,
	|element| {
	    let current = ProfileEntry::try_from( &element )?;

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

    let info = entity.content.to_info();

    Ok( entity.new_content( info ) )
}



//
// Following
//
#[derive(Debug, Deserialize)]
pub struct FollowInput {
    pub agent: AgentPubKey,
}

pub fn follow_developer(input: FollowInput) -> AppResult<HeaderHash> {
    let pubkey = agent_info()?.agent_initial_pubkey;
    debug!("Creating follow link from this agent ({}) to agent: {}", pubkey, input.agent );

    let header_hash = create_link(
	pubkey.into(),
	input.agent.clone().into(),
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

    let maybe_link = links
	.into_iter()
	.find(|link| link.target == EntryHash::from(input.agent.clone()));
    let mut maybe_header : Option<HeaderHash> = None;

    if let Some(link) = maybe_link {
	debug!("Deleting follow link to agent: {}", input.agent );

	let header_hash = delete_link( link.create_link_hash )?;

	maybe_header.replace(header_hash);
    }

    Ok( maybe_header )
}


pub fn get_following() -> AppResult<Collection<Link>> {
    let base : EntryHash = agent_info()?.agent_initial_pubkey.into();

    debug!("Getting Profile links for Agent: {}", base );
    let all_links: Vec<Link> = get_links(
        base.clone(),
	Some(LinkTag::new(TAG_FOLLOW))
    )?.into();

    Ok(Collection {
	base: base,
	items: all_links
    })
}
