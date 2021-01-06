use raftkv::{Raft, Config, NodeId};
use smallvec::alloc::sync::Arc;
use crate::test_raft::mem_storage::MemoryStorage;
use std::sync::atomic::AtomicBool;
use parking_lot::RwLock;
use smallvec::alloc::collections::{BTreeMap, BTreeSet};
use crate::test_raft::test_network::TestNetwork;

mod mem_storage;
mod test_network;

#[derive(Debug, Clone)]
pub enum Action {
    Put(String, i32),
    Delete(String),
}

impl Action {
    pub fn put(key: impl Into<String>, value: i32) -> Self {
        Self::Put(key.into(), value)
    }

    pub fn delete(key: impl Into<String>) -> Self {
        Self::Delete(key.into())
    }
}

#[derive(Clone)]
pub struct Node {
    pub raft: Raft<(), Action>,
    pub storage: Arc<MemoryStorage>,
    pub stop: Arc<AtomicBool>,
}

#[derive(Clone)]
pub struct TestHardness {
    config: Arc<Config>,
    nodes: Arc<RwLock<BTreeMap<NodeId, Node>>>,
    isolated_nodes: Arc<RwLock<BTreeSet<NodeId>>>,
    network: Arc<TestNetwork>,
}

impl Default for TestHardness {
    fn default() -> Self {
    }
}


impl TestHardness {

}