use std::sync::Arc;
use parking_lot::RwLock;
use smallvec::alloc::collections::BTreeMap;
use super::{Action, Node};
use raftkv::NodeId;

pub struct TestNetwork{
    pub nodes: Arc<RwLock<BTreeMap<NodeId,Node>>>
}