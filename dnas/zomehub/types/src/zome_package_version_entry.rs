use crate::{
    hdi,
    EntityId,
    Authority,
};

use std::collections::BTreeMap;
use hdi::prelude::*;


#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct ApiCompatibilityBuiltWith {
    pub hdi_version: String,
    pub hdk_version: Option<String>, // Only required for coordinator zomes
}

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct ApiCompatibility {
    pub build_with: ApiCompatibilityBuiltWith,
    pub tested_with: String,
}


//
// Zome Package Version Entry
//
#[hdk_entry_helper]
#[derive(Clone)]
pub struct ZomePackageVersionEntry {
    /// A pointer to the Package ID that this version is related to.
    ///
    /// Create
    ///   - Any [`ActionHash`] that is a Create for ZomePackageEntry
    /// Update
    ///   - Cannot be updated
    pub for_package: EntityId,

    /// A pointer to the WASM for this version.
    ///
    /// Create
    ///   - Any [`EntryHash`] that is a Create for ZomeEntry
    /// Update
    ///   - Cannot be updated
    pub zome_entry: EntryHash,

    // Technically we only need to track the maintainer when it is a group.  But it does save us a
    // lookup to mirror an agent maintainer.
    /// Declares the [Authority] with update permissions for this entry.
    ///
    /// Create / Update
    ///   - Must match the `for_package` ZomePackageEntry's maintainer setting but the group
    ///     revision pointer should be the latest known group revision.
    pub maintainer: Authority,

    // Optional
    pub readme: Option<EntryHash>, // Mere memory addr for README.md
    pub changelog: Option<EntryHash>,
    pub source_code_revision_uri: Option<String>,
    pub api_compatibility: ApiCompatibility,
    /// Used by coordinator zomes to indicate integrity or the expected peer coordinators that are
    /// called.
    pub dependencies: Option<Vec<(String, ActionHash, String, String)>>, // (name, id, version, hash)

    // Common fields
    pub metadata: BTreeMap<String, rmpv::Value>,
}
