use crate::block::Height;

pub trait Requester {
    type SignedHeader;
    type ValidatorSet;
    type Error;

    fn signed_header<H>(&self, h: H) -> Result<Self::SignedHeader, Self::Error>
    where
        H: Into<Height>;
    fn validator_set<H>(&self, h: H) -> Result<Self::ValidatorSet, Self::Error>
    where
        H: Into<Height>;
}
