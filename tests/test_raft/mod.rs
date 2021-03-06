#![allow(dead_code)]
mod mem_storage;
mod test_network;

use raftkv::{Raft, Config, NodeId, IResult, RaftError, Metrics};
use smallvec::alloc::sync::Arc;
use crate::test_raft::mem_storage::MemoryStorage;
use std::sync::atomic::AtomicBool;
use parking_lot::RwLock;
use smallvec::alloc::collections::{BTreeMap, BTreeSet};
use crate::test_raft::test_network::TestNetwork;
use tracing_subscriber::layer::SubscriberExt;
use std::sync::atomic::Ordering::SeqCst;


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
        let subscriber = tracing_subscriber::Registry::default()
            .with(tracing_subscriber::EnvFilter::from_default_env())
            .with(tracing_subscriber::fmt::Layer::default());
        tracing::subscriber::set_global_default(subscriber).unwrap();

        let config = Arc::new(Config::default());
        let network = Arc::new(TestNetwork::default());
        Self {
            config,
            nodes: network.nodes.clone(),
            isolated_nodes: Default::default(),
            network,
        }
    }
}

impl TestHardness {
    pub fn add_node(&self, id: NodeId) {
        let mut nodes = self.nodes.write();
        let storage = Arc::new(MemoryStorage::default());
        let raft = Raft::new(
            format!("node-{}", id),
            id,
            self.config.clone(),
            storage.clone(),
            self.network.clone(),
        ).unwrap();
        nodes.insert(id, Node {
            raft,
            storage,
            stop: Arc::new(AtomicBool::new(false)),
        });
    }

    pub fn change_node_stop(&self, id: NodeId, flag: bool) {
        let nodes = self.nodes.read();
        match nodes.get(&id) {
            Some(node) => node.stop.store(flag, SeqCst),
            None => panic!(format!("not found node in target:{}", id)),
        }
    }

    pub async fn initialize(&self) -> IResult<()> {
        let nodes = self.nodes.read();
        assert!(!nodes.is_empty());
        let raft = &nodes.iter().next().unwrap().1.raft.clone();
        raft.initialize(nodes.keys().map(|id| (*id, ()))).await?;
        Ok(())
    }

    pub async fn add_non_voter(&self, id: NodeId) -> IResult<()> {
        let nodes = self.nodes.read();
        let mut leader = *nodes.keys().next().unwrap();
        loop {
            let node = nodes.get(&leader).unwrap();
            match node.raft.add_non_voter(id, ()).await {
                Ok(()) => break,
                Err(RaftError::ForwardToLeader(Some(forward_to))) => leader = forward_to,
                Err(err) => return Err(err),
            }
        }
        Ok(())
    }

    pub async fn metrics(&self, id: NodeId) -> IResult<Metrics<()>> {
        let nodes = self.nodes.read();
        let node = nodes.get(&id).unwrap();
        node.raft.metrics().await
    }

    pub async fn isolate_node(&self, id: NodeId) {
        self.isolated_nodes.write().insert(id);
    }

    pub async fn restore_node(&self, id: NodeId) {
        self.isolated_nodes.write().remove(&id);
    }

    pub async fn write(&self, action: Action) -> IResult<()> {
        let nodes = self.nodes.read();
        let mut leader = *nodes.keys().next().unwrap();
        loop {
            let node = nodes.get(&leader).unwrap();
            match node.raft.client_write(action.clone()).await {
                Ok(()) => break,
                Err(RaftError::ForwardToLeader(Some(forward_to))) => leader = forward_to,
                Err(err) => return Err(err),
            }
        }
        Ok(())
    }

    pub async fn read(&self, key: impl AsRef<str>) -> IResult<Option<i32>> {
        let nodes = self.nodes.read();
        let mut leader = *nodes.keys().next().unwrap();
        loop {
            let node = nodes.get(&leader).unwrap();
            match node.raft.client_read().await {
                Ok(()) => return Ok(node.storage.get(key)),
                Err(RaftError::ForwardToLeader(Some(forward_to))) => leader = forward_to,
                Err(err) => return Err(err),
            }
        }
    }

    pub async fn read_from(&self, target: NodeId, key: impl AsRef<str>) -> IResult<Option<i32>> {
        let nodes = self.nodes.read();
        let node = nodes.get(&target).unwrap();
        Ok(node.storage.get(key))
    }
}