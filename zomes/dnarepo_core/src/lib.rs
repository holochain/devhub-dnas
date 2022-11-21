mod validation;

use hdi::prelude::*;
use serde::de::{ Deserializer, Error };
use devhub_types::{
    dnarepo_entry_types::{
	ProfileEntry,
	DnaEntry,
	DnaVersionEntry,
	ZomeEntry,
	ZomeVersionEntry,
	ReviewEntry,
	ReviewSummaryEntry,
	ReactionEntry,
	ReactionSummaryEntry,
    },
};
pub use devhub_types::{
    create_path,
    AppResult,
};
pub use hc_crud::{
    EntryModel, EntityType,
};

#[hdk_entry_defs]
#[unit_enum(UnitEntryTypes)]
pub enum EntryTypes {
    #[entry_def]
    Profile(ProfileEntry),

    #[entry_def]
    Zome(ZomeEntry),
    #[entry_def]
    ZomeVersion(ZomeVersionEntry),

    #[entry_def]
    Dna(DnaEntry),
    #[entry_def]
    DnaVersion(DnaVersionEntry),

    #[entry_def]
    Review(ReviewEntry),
    #[entry_def]
    ReviewSummary(ReviewSummaryEntry),

    #[entry_def]
    Reaction(ReactionEntry),
    #[entry_def]
    ReactionSummary(ReactionSummaryEntry),
}


#[hdk_link_types]
pub enum LinkTypes {
    Agent,
    Profile,

    File,

    Zome,
    ZomeVersion,
    Dna,
    DnaVersion,

    Review,
    ReviewSummary,
    Reaction,
    ReactionSummary,

    Following,
    Tag,
    Anchor,
}

impl<'de> Deserialize<'de> for LinkTypes {
    fn deserialize<D>(deserializer: D) -> Result<LinkTypes, D::Error>
    where
	D: Deserializer<'de>,
    {
	let name : &str = Deserialize::deserialize(deserializer)?;
	match name {
	    "Agent" => Ok(LinkTypes::Agent),
	    "Profile" => Ok(LinkTypes::Profile),

	    "File" => Ok(LinkTypes::File),

	    "Zome" => Ok(LinkTypes::Zome),
	    "ZomeVersion" => Ok(LinkTypes::ZomeVersion),
	    "Dna" => Ok(LinkTypes::Dna),
	    "DnaVersion" => Ok(LinkTypes::DnaVersion),

	    "Review" => Ok(LinkTypes::Review),
	    "ReviewSummary" => Ok(LinkTypes::ReviewSummary),
	    "Reaction" => Ok(LinkTypes::Reaction),
	    "ReactionSummary" => Ok(LinkTypes::ReactionSummary),

	    "Following" => Ok(LinkTypes::Following),
	    "Tag" => Ok(LinkTypes::Tag),
	    "Anchor" => Ok(LinkTypes::Anchor),

	    value => Err(D::Error::custom(format!("No LinkTypes value matching '{}'", value ))),
	}
    }
}



impl EntryModel<EntryTypes> for ProfileEntry {
    fn name() -> &'static str { "Profile" }
    fn get_type(&self) -> EntityType {
	EntityType::new( "profile", "entry" )
    }
    fn to_input(&self) -> EntryTypes {
	EntryTypes::Profile(self.clone())
    }
}

impl EntryModel<EntryTypes> for DnaEntry {
    fn name() -> &'static str { "Dna" }
    fn get_type(&self) -> EntityType {
	EntityType::new( "dna", "entry" )
    }
    fn to_input(&self) -> EntryTypes {
	EntryTypes::Dna(self.clone())
    }
}

impl EntryModel<EntryTypes> for DnaVersionEntry {
    fn name() -> &'static str { "DnaVersion" }
    fn get_type(&self) -> EntityType {
	EntityType::new( "dna_version", "entry" )
    }
    fn to_input(&self) -> EntryTypes {
	EntryTypes::DnaVersion(self.clone())
    }
}

impl EntryModel<EntryTypes> for ZomeEntry {
    fn name() -> &'static str { "Zome" }
    fn get_type(&self) -> EntityType {
	EntityType::new( "zome", "entry" )
    }
    fn to_input(&self) -> EntryTypes {
	EntryTypes::Zome(self.clone())
    }
}

impl EntryModel<EntryTypes> for ZomeVersionEntry {
    fn name() -> &'static str { "ZomeVersion" }
    fn get_type(&self) -> EntityType {
	EntityType::new( "zome_version", "entry" )
    }
    fn to_input(&self) -> EntryTypes {
	EntryTypes::ZomeVersion(self.clone())
    }
}

impl EntryModel<EntryTypes> for ReviewEntry {
    fn name() -> &'static str { "Review" }
    fn get_type(&self) -> EntityType {
	EntityType::new( "review", "entry" )
    }
    fn to_input(&self) -> EntryTypes {
	EntryTypes::Review(self.clone())
    }
}

impl EntryModel<EntryTypes> for ReactionEntry {
    fn name() -> &'static str { "Reaction" }
    fn get_type(&self) -> EntityType {
	EntityType::new( "reaction", "entry" )
    }
    fn to_input(&self) -> EntryTypes {
	EntryTypes::Reaction(self.clone())
    }
}

impl EntryModel<EntryTypes> for ReactionSummaryEntry {
    fn name() -> &'static str { "ReactionSummary" }
    fn get_type(&self) -> EntityType {
	EntityType::new( "reaction_summary", "entry" )
    }
    fn to_input(&self) -> EntryTypes {
	EntryTypes::ReactionSummary(self.clone())
    }
}

impl EntryModel<EntryTypes> for ReviewSummaryEntry {
    fn name() -> &'static str { "ReviewSummary" }
    fn get_type(&self) -> EntityType {
	EntityType::new( "review_summary", "entry" )
    }
    fn to_input(&self) -> EntryTypes {
	EntryTypes::ReviewSummary(self.clone())
    }
}
