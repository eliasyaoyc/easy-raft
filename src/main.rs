mod error;
mod api;
mod common;
mod node;
mod storage;
mod config;

use jsonrpc_core::Result;
use jsonrpc_derive::rpc;
use jsonrpc_http_server::ServerBuilder;

#[rpc]
pub trait Rpc {
    /// Adds two numbers and returns a result
    #[rpc(name = "add")]
    fn add(&self, a: u64, b: u64) -> Result<u64>;
}

pub struct RpcImpl;

impl Rpc for RpcImpl {
    fn add(&self, a: u64, b: u64) -> Result<u64> {
        Ok(a + b)
    }
}

fn main() {
    let mut io = jsonrpc_core::IoHandler::new();
    io.extend_with(RpcImpl.to_delegate());
    let server = ServerBuilder::new(io)
        .threads(3)
        .start_http(&"127.0.0.1:3030".parse().unwrap())
        .unwrap();
    server.wait()
}