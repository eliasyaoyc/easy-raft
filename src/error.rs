use smallvec::alloc::sync::Arc;
use crate::NodeId;
use thiserror::Error;

#[derive(Debug, Error, Clone)]
pub enum RaftError {
    #[error("Storage error: {0}")]
    Storage(Arc<anyhow::Error>),

    #[error("This Raft has already been initialized.")]
    AlreadyInitialized,

    #[error("Node '{0}' is unregistered.")]
    UnknownNode(NodeId),

    #[error("Node '{0} is already registered.'")]
    NodeAlreadyRegistered(NodeId),

    #[error("Forward to leader: {0:?}")]
    ForwardToLeader(Option<NodeId>),

    #[error("Shutdown")]
    Shutdown,
}

impl RaftError {
    #[inline]
    pub(crate) fn storage(err: anyhow::Error) -> Self {
        Self::Storage(Arc::new(err))
    }

    pub fn is_shutdown(&self) -> bool {
        matches!(self,RaftError::Shutdown)
    }
}

pub type IResult<T, E = RaftError> = std::result::Result<T, E>;
