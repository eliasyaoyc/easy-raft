use crate::error::{Error, IResult};
use jsonrpc_http_server::hyper::Client;
use crate::common::*;

#[rpc]
#[allow(non_snake_case)]
pub trait RaftApi {
    #[rpc(name = "vote")]
    fn vote(req: &VoteRequest) -> VoteResponse;
    #[rpc(name = "appendEntries")]
    fn appendEntries(req: &AppendEntriesRequest) -> AppendEntriesResponse;
}

pub trait Lifecycle {
    fn startup() -> IResult<()>;
    fn connect() -> IResult<bool>;
}