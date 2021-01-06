use crate::{NodeId, MemberShipConfig, LogIndex, Entry};
use serde::{Serialize, Deserialize};

pub type StorageResult<T> = anyhow::Result<T>;

#[derive(Serialize, Deserialize, Debug, Copy, Clone)]
pub struct HardState {
    pub current_term: u64,
    pub voted_for: Option<NodeId>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct InitialState<N> {
    pub last_log_index: u64,
    pub last_log_term: u64,
    pub hard_state: Option<HardState>,
    pub membership: Option<MemberShipConfig<N>>,
}

pub trait Storage<N, D>: Send + Sync + 'static {
    fn get_initial_state(&self) -> StorageResult<InitialState<N>>;

    fn save_hard_state(&self, hard_state: HardState) -> StorageResult<()>;

    fn last_applied(&self) -> StorageResult<LogIndex>;

    fn get_log_entries(&self, start: LogIndex, end: LogIndex) -> StorageResult<Vec<Entry<N, D>>>;

    fn delete_logs_from(&self, start: LogIndex, end: Option<LogIndex>) -> StorageResult<()>;

    fn append_entries_to_log(&self, entries: &[Entry<N, D>]) -> StorageResult<()>;

    fn apply_entries_to_state_machine(&self, entries: &[Entry<N, D>]) -> StorageResult<()>;
}