pub use apphub_types;
pub use devhub_sdk;
pub use devhub_sdk::hdk;
pub use devhub_sdk::hdk_extensions;
pub use devhub_sdk::hc_crud;
pub use devhub_sdk::*;

use std::collections::BTreeMap;
// use hdk::prelude::*;
use apphub_types::{
    WebAppEntry,
    WebAppPackageEntry,
    WebAppPackageVersionEntry,
};
use dnahub_sdk::{
    DnaTokenInput,
};
use hc_crud::{
    Entity, EntityId,
};



pub type EntityMap<T> = BTreeMap<String, Entity<T>>;
pub type EntityPointerMap = BTreeMap<String, EntityId>;

pub type WebAppMap = EntityMap<WebAppEntry>;
pub type WebAppPackageMap = EntityMap<WebAppPackageEntry>;
pub type WebAppPackageVersionMap = EntityMap<WebAppPackageVersionEntry>;

pub type RolesDnaTokensInput = BTreeMap<String, DnaTokenInput>;
