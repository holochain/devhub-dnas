mod validation;

pub use hdi_extensions::hdi;

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
    EntryModel,
    entry_model,
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

entry_model!( EntryTypes::Profile( ProfileEntry ) );
entry_model!( EntryTypes::Zome( ZomeEntry ) );
entry_model!( EntryTypes::ZomeVersion( ZomeVersionEntry ) );
entry_model!( EntryTypes::Dna( DnaEntry ) );
entry_model!( EntryTypes::DnaVersion( DnaVersionEntry ) );
entry_model!( EntryTypes::Review( ReviewEntry ) );
entry_model!( EntryTypes::ReviewSummary( ReviewSummaryEntry ) );
entry_model!( EntryTypes::Reaction( ReactionEntry ) );
entry_model!( EntryTypes::ReactionSummary( ReactionSummaryEntry ) );


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
