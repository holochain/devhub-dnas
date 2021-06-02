use hdk::prelude::*;

use crate::utils;
use crate::constants::{ TAG_PROFILE, TAG_UPDATE };
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
fn create_profile(input: ProfileInput) -> ExternResult<(EntryHash, ProfileInfo)> {
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

    let _header_hash = create_entry(&profile)?;
    let entry_hash = hash_entry(&profile)?;

    debug!("Linking pubkey ({}) to Profile: {}", pubkey, entry_hash );
    create_link(
	pubkey.into(),
	entry_hash.clone(),
	LinkTag::new(*TAG_PROFILE)
    )?;

    Ok( (entry_hash.clone(), profile.to_info( entry_hash )) )
}



#[hdk_extern]
fn get_profile(_:()) -> ExternResult<ProfileInfo> {
    if let Some(link) = utils::find_latest_link( get_profile_links()? )? {
	debug!("Get Profile: {}", link.target );
	let (_, element) = utils::fetch_entry_latest(link.target.clone())?;

	Ok(ProfileEntry::try_from(element)?.to_info( link.target ))
    }
    else {
	Err(WasmError::Guest("Agent Profile has not been created yet".into()))
    }
}



fn get_profile_links() -> ExternResult<Vec<Link>> {
    let pubkey = agent_info()?.agent_initial_pubkey;

    debug!("Getting Profile links for Agent: {}", pubkey );
    let all_links: Vec<Link> = get_links(
        pubkey.into(),
	Some(LinkTag::new(*TAG_PROFILE))
    )?.into();

    Ok(all_links)
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
fn update_profile(input: UpdateProfileInput) -> ExternResult<(EntryHash, ProfileInfo)> {
    debug!("Updating Profile: {}", input.addr );
    let (header, element) = utils::fetch_entry_latest(input.addr.clone())?;
    let current_profile = ProfileEntry::try_from( element )?;

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

    update_entry(header, &profile)?;
    let entry_hash = hash_entry(&profile)?;

    debug!("Linking original ({}) to Profile: {}", input.addr, entry_hash );
    create_link(
	input.addr.clone(),
	entry_hash.clone(),
	LinkTag::new(TAG_UPDATE)
    )?;

    Ok( (entry_hash, profile.to_info( input.addr )) )
}
