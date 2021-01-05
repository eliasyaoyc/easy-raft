use crate::api::{RaftApi, AppendEntriesResponse, VoteResponse, VoteRequest, AppendEntriesRequest, Lifecycle};
use crate::error::IResult;
use std::borrow::Borrow;
use std::sync::atomic::AtomicUsize;
use std::sync::Mutex;
use jsonrpc_core::IoHandler;
use jsonrpc_http_server::{ServerBuilder, Server};
use crate::common::*;

#[derive(Debug)]
#[allow(non_snake_case)]
pub struct RaftKV {
    currentTerm: usize,
    votedFor: String,
    log: Vec<String>,
    commitIndex: AtomicUsize,
    lastApplied: AtomicUsize,
    nextIndex: Mutex<Vec<AtomicUsize>>,
    matchIndex: Mutex<Vec<AtomicUsize>>,
}

#[allow(non_snake_case)]
impl RaftApi for RaftKV {
    fn vote(req: &VoteRequest) -> VoteResponse {}

    fn appendEntries(req: &AppendEntriesRequest) -> AppendEntriesResponse {}
}

impl Lifecycle for RaftKV {
    fn startup() -> IResult<()> {
        let mut io = jsonrpc_core::IoHandler::new();
        io.extend_with(RaftKV.to_delegate());
        let server = ServerBuilder::new(io)
            .threads(3)
            .start_http(&"127.0.0.1:8080".parse().unwrap())
            .unwrap();
        server.wait();
        Ok(())
    }

    fn connect() -> IResult<bool> {

    }
}