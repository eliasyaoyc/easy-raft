use std::sync::Arc;
use parking_lot::RwLock;
use smallvec::alloc::collections::{BTreeMap, BTreeSet};
use super::{Action, Node};
use raftkv::{NodeId, Network, VoteRequest, AppendEntriesRequest, NetworkResult, AppendEntriesResponse, RaftError, VoteResponse, InstallSnapshotRequest, InstallSnapshotResponse};
use std::sync::atomic::Ordering::SeqCst;

#[derive(Default)]
pub struct TestNetwork {
    pub nodes: Arc<RwLock<BTreeMap<NodeId, Node>>>,
    pub isolated_nodes: Arc<RwLock<BTreeSet<NodeId>>>,
}

#[async_trait::async_trait]
impl Network<(), Action> for TestNetwork {
    async fn vote(&self, target: u64, target_info: &(), req: VoteRequest) -> NetworkResult<VoteResponse> {
        let node = self.nodes.read().get(&target).cloned().unwrap();
        if node.stop.load(SeqCst) {
            Err(RaftError::Shutdown)?;
        }
        anyhow::ensure!(
           !self.isolated_nodes.read().contains(&target),
           "Node '{}' is isolated",
           target,
        );
        Ok(node.raft.vote(req).await?)
    }

    async fn append_entries(&self, target: u64, _target_info: &(), req: AppendEntriesRequest<(), Action>) -> NetworkResult<AppendEntriesResponse> {
        let node = self.nodes.read().get(&target).cloned().unwrap();
        if node.stop.load(SeqCst) {
            Err(RaftError::Shutdown)?;
        }
        anyhow::ensure!(
           !self.isolated_nodes.read().contains(&target),
           "Node '{}' is isolated",
           target,
        );
        Ok(node.raft.append_entries(req).await?)
    }

    async fn install_snapshot(&self, target: u64, target_info: &(), req: InstallSnapshotRequest<(), Action>) -> NetworkResult<InstallSnapshotResponse> {
        unimplemented!()
    }
}