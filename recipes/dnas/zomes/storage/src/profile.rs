use devhub_types::{ Entity, Collection, DevHubResponse, EntityResponse, CollectionResponse, EntryModel,
		    ENTITY_MD, VALUE_MD, VALUE_COLLECTION_MD };
use hdk::prelude::*;
use hc_dna_utils as utils;
use hc_dna_utils::catch;

use crate::errors::{ AppError };
use crate::constants::{ TAG_PROFILE, TAG_FOLLOW };
use crate::entry_types::{ ProfileEntry, ProfileInfo };



#[derive(Debug, Deserialize)]
pub struct ProfileInput {
    pub name: String,
    pub avatar_image: SerializedBytes,

    // optional
    pub email: Option<String>,
    pub website: Option<String>,
}

#[hdk_extern]
fn create_profile(input: ProfileInput) -> ExternResult<EntityResponse<ProfileInfo>> {
    debug!("Creating Profile: {}", input.name );
    let pubkey = catch!( agent_info() ).agent_initial_pubkey;

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

    let header_hash = catch!( create_entry(&profile) );
    let entry_hash = catch!( hash_entry(&profile) );

    debug!("Linking pubkey ({}) to Profile: {}", pubkey, entry_hash );
    catch!( create_link(
	pubkey.into(),
	entry_hash.clone(),
	LinkTag::new(TAG_PROFILE)
    ) );

    let info = profile.to_info();

    Ok( EntityResponse::success(Entity {
	id: entry_hash.clone(),
	address: entry_hash,
	header: header_hash,
	ctype: info.get_type(),
	content: info,
    }, ENTITY_MD) )
}


#[derive(Debug, Deserialize)]
pub struct GetProfileInput {
    pub agent: Option<AgentPubKey>,
}

#[hdk_extern]
fn get_profile(input: GetProfileInput) -> ExternResult<EntityResponse<ProfileInfo>> {
    let (_, links) = catch!( get_profile_links( input.agent ) );

    if let Some(link) = catch!( utils::find_latest_link( links ) ) {
	debug!("Get Profile: {}", link.target );
	let entity = catch!( utils::fetch_entity( &link.target ) );
	let info = catch!( ProfileEntry::try_from(&entity.content) ).to_info();

	Ok( EntityResponse::success(
	    entity.new_content( info ), ENTITY_MD
	))
    }
    else {
	let error = &AppError::CustomError("Agent Profile has not been created yet");
	Ok( EntityResponse::error( error.into(), None) )
    }
}



fn get_profile_links(maybe_pubkey: Option<AgentPubKey> ) -> ExternResult<(EntryHash, Vec<Link>)> {
    let base : EntryHash = match maybe_pubkey {
	None => agent_info()?.agent_initial_pubkey,
	Some(agent) => agent,
    }.into();

    debug!("Getting Profile links for Agent: {}", base );
    let all_links: Vec<Link> = get_links(
        base.clone(),
	Some(LinkTag::new(TAG_PROFILE))
    )?.into();

    Ok( (base, all_links) )
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

#[hdk_extern]
fn update_profile(input: UpdateProfileInput) -> ExternResult<EntityResponse<ProfileInfo>> {
    debug!("Updating Profile: {}", input.addr );
    let entity = catch!( utils::fetch_entity( &input.addr ) );
    let current_profile = catch!( ProfileEntry::try_from( &entity.content ) );

    let profile = ProfileEntry {
	name: match input.properties.name {
	    None => current_profile.name,
	    Some(v) => v,
	},
	email: match input.properties.email {
	    None => current_profile.email,
	    Some(v) => v,
	},
	website: match input.properties.website {
	    None => current_profile.website,
	    Some(v) => v,
	},
	avatar_image: match input.properties.avatar_image {
	    None => current_profile.avatar_image,
	    Some(v) => v,
	},
    };

    let header_hash = catch!( update_entry(entity.header.clone(), &profile) );
    let entry_hash = catch!( hash_entry(&profile) );

    debug!("Linking original ({}) to Profile: {}", input.addr, entry_hash );
    catch!( create_link(
	input.addr.clone(),
	entry_hash.clone(),
	LinkTag::new(utils::TAG_UPDATE)
    ) );

    Ok(EntityResponse::success(
	entity.new_content( profile.to_info() )
	    .update_header( header_hash )
	    .update_address( entry_hash ), ENTITY_MD
    ))
}



//
// Following
//
#[derive(Debug, Deserialize)]
pub struct FollowInput {
    pub agent: AgentPubKey,
}

#[hdk_extern]
fn follow_developer(input: FollowInput) -> ExternResult<DevHubResponse<HeaderHash>> {
    let pubkey = catch!( agent_info() ).agent_initial_pubkey;
    debug!("Creating follow link from this agent ({}) to agent: {}", pubkey, input.agent );

    let header_hash = catch!( create_link(
	pubkey.into(),
	input.agent.clone().into(),
	LinkTag::new(TAG_FOLLOW)
    ) );

    Ok( DevHubResponse::success( header_hash, VALUE_MD ) )
}


#[derive(Debug, Deserialize)]
pub struct UnfollowInput {
    pub agent: AgentPubKey,
}

#[hdk_extern]
fn unfollow_developer(input: UnfollowInput) -> ExternResult<DevHubResponse<Option<HeaderHash>>> {
    let links = match catch!( get_following(()) ) {
	DevHubResponse::Success(resp) => resp.payload.items,
	DevHubResponse::Failure(resp) => {
	    return Ok( DevHubResponse::Failure(resp) );
	}
    };

    let maybe_link = links
	.into_iter()
	.find(|link| link.target == EntryHash::from(input.agent.clone()));
    let mut maybe_header : Option<HeaderHash> = None;

    if let Some(link) = maybe_link {
	debug!("Deleting follow link to agent: {}", input.agent );

	let header_hash = catch!( delete_link( link.create_link_hash ) );

	maybe_header.replace(header_hash);
    }

    Ok( DevHubResponse::success( maybe_header, VALUE_MD ) )
}


#[hdk_extern]
fn get_following(_:()) -> ExternResult<CollectionResponse<Link>> {
    let base : EntryHash = catch!( agent_info() ).agent_initial_pubkey.into();

    debug!("Getting Profile links for Agent: {}", base );
    let all_links: Vec<Link> = catch!( get_links(
        base.clone(),
	Some(LinkTag::new(TAG_FOLLOW))
    ) ).into();

    Ok( CollectionResponse::success(Collection {
	base: base,
	items: all_links,
    }, VALUE_COLLECTION_MD ) )
}
