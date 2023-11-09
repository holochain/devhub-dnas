use crate::{
    hdi,
    hdi_extensions,
    LinkTypes,
    AppEntry,
    UiEntry,
    WebAppEntry,
    WebAppPackageEntry,
    WebAppPackageVersionEntry,
};

use hdi::prelude::*;
use hdi_extensions::{
    AnyLinkableHashTransformer,
    verify_app_entry_struct,
    // Macros
    valid, invalid,
};


pub fn validation(
    base_address: AnyLinkableHash,
    target_address: AnyLinkableHash,
    link_type: LinkTypes,
    _tag: LinkTag,
    create: CreateLink,
) -> ExternResult<ValidateCallbackResult> {
    match link_type {
        LinkTypes::WebAppPackage => {
            verify_app_entry_struct::<WebAppPackageEntry>( &target_address )?;

            valid!()
        },
        LinkTypes::WebAppPackageToWebAppPackageVersion => {
            verify_app_entry_struct::<WebAppPackageEntry>( &base_address )?;
            verify_app_entry_struct::<WebAppPackageVersionEntry>( &target_address )?;

            let webapp_package_id = base_address.must_be_action_hash()?;
            let webapp_package_version_addr = target_address.must_be_action_hash()?;

            let webapp_package_version_entry = WebAppPackageVersionEntry::try_from(
                must_get_valid_record( webapp_package_version_addr )?
            )?;

            if webapp_package_version_entry.for_package != webapp_package_id {
                invalid!(format!(
                    "WebApp Package Version is not for the base target ({}); expected base target '{}'",
                    base_address, webapp_package_version_entry.for_package
                ))
            }

            valid!()
        },
        LinkTypes::AgentToApp => {
            let agent_pubkey = match base_address.clone().into_agent_pub_key() {
                Some(hash) => hash,
                None => invalid!(format!(
                    "AgentApp link base address must be an agent pubkey; not '{}'", base_address
                )),
            };

            if agent_pubkey != create.author {
                invalid!(format!("Not authorized to create link based on agent '{}'", agent_pubkey ))
            }

            verify_app_entry_struct::<AppEntry>( &target_address )?;

            valid!()
        },
        LinkTypes::AgentToUi => {
            let agent_pubkey = match base_address.clone().into_agent_pub_key() {
                Some(hash) => hash,
                None => invalid!(format!("AgentUi link base address must be an agent pubkey; not '{}'", base_address )),
            };

            if agent_pubkey != create.author {
                invalid!(format!("Not authorized to create link based on agent '{}'", agent_pubkey ))
            }

            verify_app_entry_struct::<UiEntry>( &target_address )?;

            valid!()
        },
        LinkTypes::AgentToWebApp => {
            let agent_pubkey = match base_address.clone().into_agent_pub_key() {
                Some(hash) => hash,
                None => invalid!(format!("AgentWebApp link base address must be an agent pubkey; not '{}'", base_address )),
            };

            if agent_pubkey != create.author {
                invalid!(format!("Not authorized to create link based on agent '{}'", agent_pubkey ))
            }

            verify_app_entry_struct::<WebAppEntry>( &target_address )?;

            valid!()
        },
        LinkTypes::AgentToWebAppPackage => {
            let agent_pubkey = match base_address.clone().into_agent_pub_key() {
                Some(hash) => hash,
                None => invalid!(format!("AgentWebAppPackage link base address must be an agent pubkey; not '{}'", base_address )),
            };

            if agent_pubkey != create.author {
                invalid!(format!("Not authorized to create link based on agent '{}'", agent_pubkey ))
            }

            verify_app_entry_struct::<WebAppPackageEntry>( &target_address )?;

            valid!()
        },
        LinkTypes::AgentToWebAppPackageVersion => {
            let agent_pubkey = match base_address.clone().into_agent_pub_key() {
                Some(hash) => hash,
                None => invalid!(format!("AgentWebAppPackageVersion link base address must be an agent pubkey; not '{}'", base_address )),
            };

            if agent_pubkey != create.author {
                invalid!(format!("Not authorized to create link based on agent '{}'", agent_pubkey ))
            }

            verify_app_entry_struct::<WebAppPackageVersionEntry>( &target_address )?;

            valid!()
        },
        _ => invalid!(format!("Create link validation not implemented for link type: {:#?}", create.link_type )),
    }
}
