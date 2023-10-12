mod webapp_package_anchor;

pub use devhub_sdk::hdk;
pub use devhub_sdk::hdk_extensions;

pub use webapp_package_anchor::*;

use std::collections::BTreeMap;
// use hdk::prelude::*;


pub type SimpleMap<T> = BTreeMap<String, T>;
