use futures::channel::oneshot;
use crate::{NodeId, IResult, VoteRequest, AppendEntriesRequest, AppendEntriesResponse, Metrics};
use crate::network::{VoteResponse, InstallSnapshotRequest, InstallSnapshotResponse};

pub enum Message<N, D> {
    Initialize {
        members: Vec<(NodeId, N)>,
        reply: oneshot::Sender<IResult<()>>,
    },
    Vote {
        req: VoteRequest,
        reply: oneshot::Sender<IResult<VoteResponse>>,
    },
    AppendEntries {
        req: AppendEntriesRequest<N, D>,
        reply: oneshot::Sender<IResult<AppendEntriesResponse>>,
    },
    InstallSnapshot {
        req: InstallSnapshotRequest<N, D>,
        reply: oneshot::Sender<IResult<InstallSnapshotResponse>>,
    },
    ClientWrite {
        action: D,
        reply: oneshot::Sender<IResult<()>>,
    },
    ClientRead {
        reply: oneshot::Sender<IResult<()>>,
    },
    AddNode {
        id: NodeId,
        info: N,
        reply: oneshot::Sender<IResult<()>>,
    },
    RemoveNode {
        id: NodeId,
        reply: oneshot::Sender<IResult<()>>,
    },
    Metrics {
        reply: oneshot::Sender<IResult<Metrics<N>>>,
    },
    Shutdown,
}