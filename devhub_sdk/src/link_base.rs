use crate::{
    hdk,
    hdk_extensions,
};

use hdk::prelude::*;
use hdk_extensions::{
    agent_id,
};


pub struct LinkBase<LT>(pub AnyLinkableHash, pub LT)
where
    LT: LinkTypeFilterExt + Copy,
    ScopedLinkType: TryFrom<LT, Error = WasmError>,
;


impl<LT,T> TryFrom<(T, LT)> for LinkBase<LT>
where
    LT: LinkTypeFilterExt + Copy,
    ScopedLinkType: TryFrom<LT, Error = WasmError>,
    AnyLinkableHash: TryFrom<T, Error = WasmError>,
{
    type Error = WasmError;

    fn try_from(input: (T, LT)) -> ExternResult<Self> {
        Ok(
            Self( AnyLinkableHash::try_from(input.0)?, input.1 )
        )
    }
}


impl<LT> LinkBase<LT>
where
    LT: LinkTypeFilterExt + Copy,
    ScopedLinkType: TryFrom<LT, Error = WasmError>,
{
    pub fn new<T>(input: T, link_type: LT) -> Self
    where
        T: Into<AnyLinkableHash>,
    {
        Self( input.into(), link_type )
    }

    pub fn hash(&self) -> AnyLinkableHash {
        self.0.clone()
    }

    pub fn link_type(&self) -> LT {
        self.1
    }

    pub fn get_links(&self, tag: Option<LinkTag>) ->
        ExternResult<Vec<Link>>
    {
        get_links(
            create_link_input(
                &self.hash(),
                &self.link_type(),
                &tag,
            )?
        )
    }

    pub fn create_link<T>(
        &self,
        target: &T,
        tag: impl Into<LinkTag>
    ) -> ExternResult<ActionHash>
    where
        T: Into<AnyLinkableHash> + Clone,
    {
        create_link( self.hash(), target.to_owned(), self.link_type(), tag )
    }

    pub fn links_exist<T>(&self, target: &T, tag: impl Into<LinkTag>) ->
        ExternResult<Option<Link>>
    where
        T: Into<AnyLinkableHash> + Clone,
    {
        let tag : LinkTag = tag.into();

        Ok(
            self.get_links( Some(tag.clone()) )?.into_iter()
                .find( |link| {
                    link.target == target.to_owned().into()
                        && link.tag == tag
                })
        )
    }

    pub fn create_link_if_not_exists<T>(
        &self,
        target: &T,
        tag: impl Into<LinkTag>
    ) -> ExternResult<Option<ActionHash>>
    where
        T: Into<AnyLinkableHash> + Clone,
    {
        let target : AnyLinkableHash = target.to_owned().into();
        let tag : LinkTag = tag.into();

        for link in self.get_links( Some(tag.clone()) )? {
            // We still have to check the tag because we only consider it to exist when the tags are
            // an exact match
            if link.target == target
                && link.tag == tag
            {
                debug!("Target ({}) already exists for anchor {}", target, self.hash() );
                return Ok( None );
            }
        }

        Ok( Some( self.create_link( &target, tag )? ) )
    }

    pub fn delete_all_my_links_to_target<T>(
        &self,
        target: &T,
        tag: Option<LinkTag>,
    ) -> ExternResult<Vec<ActionHash>>
    where
        T: Into<AnyLinkableHash> + Clone,
    {
        let agent_id = agent_id()?;
        let target : AnyLinkableHash = target.to_owned().into();
        let mut deleted_links = vec![];

        for link in self.get_links( tag.clone() )? {
            if link.target == target
                && ( tag == None
                     || Some(link.tag) == tag )
                && link.author == agent_id
            {
                debug!("Deleting link ({}): {} => {}", link.create_link_hash, self.hash(), target );
                let delete_action = delete_link( link.create_link_hash )?;
                deleted_links.push( delete_action );
            }
        }

        Ok( deleted_links )
    }
}


pub fn create_link_input<B,LT,T>(
    base: &B,
    link_type: &LT,
    tag_input: &Option<T>
) -> ExternResult<GetLinksInput>
where
    B: Into<AnyLinkableHash> + Clone,
    LT: LinkTypeFilterExt + Clone,
    T: Into<LinkTag> + Clone,
{
    let mut link_input_builder = GetLinksInputBuilder::try_new(
        base.to_owned(),
        link_type.to_owned(),
    )?;

    if let Some(tag_prefix) = tag_input.to_owned() {
        link_input_builder = link_input_builder.tag_prefix( tag_prefix.into() );
    }

    Ok( link_input_builder.build() )
}
