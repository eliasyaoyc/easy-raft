#[allow(non_snake_case)]
pub struct VoteRequest {
    pub term: usize,
    pub candidateId: String,
    pub lastLogIndex: usize,
    pub lastLogTerm: usize,
}

#[allow(non_snake_case)]
pub struct VoteResponse {
    pub term: usize,
    pub voteGranted: bool,
}

#[allow(non_snake_case)]
pub struct AppendEntriesRequest {
    pub term: usize,
    pub leaderId: String,
    pub prevLogIndex: usize,
    pub entries: Vec<String>,
    pub leaderCommit: usize,
}

#[allow(non_snake_case)]
pub struct AppendEntriesResponse {
    pub term: usize,
    pub success: bool,
}
