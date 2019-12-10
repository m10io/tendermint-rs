use crate::block::Height;
use crate::lite::requester::interface::Requester;
use crate::rpc;
use crate::rpc::{endpoint::commit, Client};
use crate::validator::Set;

pub struct RpcRequester {
    client: Client,
}

impl RpcRequester {
    fn new(rpc_client: Client) -> Self {
        RpcRequester { client: rpc_client }
    }
}

impl Requester for RpcRequester {
    type SignedHeader = commit::SignedHeader;
    type ValidatorSet = Set;
    type Error = rpc::Error;

    fn signed_header<H>(&self, h: H) -> Result<Self::SignedHeader, rpc::Error>
    where
        H: Into<Height>,
    {
        Ok(self.client.commit(h)?.signed_header)
    }

    fn validator_set<H>(&self, h: H) -> Result<Self::ValidatorSet, rpc::Error>
    where
        H: Into<Height>,
    {
        Ok(Set::new(self.client.validators(h)?.validators))
    }
}
