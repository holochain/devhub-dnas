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
        get_links( self.hash(), self.link_type(), tag )
    }

    pub fn create_link(&self, target: impl Into<AnyLinkableHash>, tag: impl Into<LinkTag>) ->
        ExternResult<ActionHash>
    {
        create_link( self.hash(), target, self.link_type(), tag )
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
            if link.target == target
                && link.tag == tag
            {
                debug!("Target ({}) already exists for anchor {}", target, self.hash() );
                return Ok( None );
            }
        }

        Ok( Some( create_link( self.hash(), target, self.link_type(), tag )? ) )
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
