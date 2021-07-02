use devhub_types::{ EntryModel, EntityType, Entity };
use hdk::prelude::*;
use hc_dna_utils as utils;


#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DeprecationNotice {
    pub message: String,

    // optional
    pub recommended_alternatives: Option<Vec<EntryHash>>,
}


//
// Happ Entry
//
#[hdk_entry(id = "happ_details", visibility="public")]
#[derive(Clone)]
pub struct HappEntry {
    pub name: String,
    pub description: String,
    pub designer: AgentPubKey,
    pub published_at: u64,
    pub last_updated: u64,

    // optional
    pub thumbnail_image: Option<SerializedBytes>,
    pub deprecation: Option<DeprecationNotice>,
}
utils::try_from_element![ HappEntry ];

impl EntryModel for HappEntry {
    fn get_type(&self) -> EntityType {
	EntityType::new( "happ", "entry" )
    }
}

// Summary
#[derive(Debug, Serialize, Deserialize)]
pub struct HappSummary {
    pub name: String,
    pub description: String,
    pub designer: AgentPubKey,
    pub published_at: u64,
    pub last_updated: u64,

    // optional
    pub thumbnail_image: Option<SerializedBytes>,
    pub deprecation: bool,
}
impl EntryModel for HappSummary {
    fn get_type(&self) -> EntityType {
	EntityType::new( "happ", "summary" )
    }
}

// Full
#[derive(Debug, Serialize, Deserialize)]
pub struct HappInfo {
    pub name: String,
    pub description: String,
    pub designer: AgentPubKey,
    pub published_at: u64,
    pub last_updated: u64,

    // optional
    pub thumbnail_image: Option<SerializedBytes>,
    pub deprecation: Option<DeprecationNotice>,
}
impl EntryModel for HappInfo {
    fn get_type(&self) -> EntityType {
	EntityType::new( "happ", "info" )
    }
}

impl HappEntry {
    pub fn to_info(self) -> HappInfo {
	HappInfo {
	    name: self.name,
	    description: self.description,
	    designer: self.designer,
	    published_at: self.published_at,
	    last_updated: self.last_updated,
	    thumbnail_image: self.thumbnail_image,
	    deprecation: self.deprecation,
	}
    }

    pub fn to_summary(self) -> HappSummary {
	HappSummary {
	    name: self.name,
	    description: self.description,
	    designer: self.designer,
	    published_at: self.published_at,
	    last_updated: self.last_updated,
	    thumbnail_image: self.thumbnail_image,
	    deprecation: self.deprecation.map_or(false, |_| true),
	}
    }
}




#[cfg(test)]
pub mod tests {
    use super::*;

    // #[test]
    // ///
    // fn dna_to_summary_test() {
    // }
}
