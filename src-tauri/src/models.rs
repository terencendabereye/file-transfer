use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ConnectionMode {
    Direct,
    Network,
    Ssh,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "status", rename_all = "snake_case")]
pub enum ItemStatus {
    Queued,
    Active,
    Paused,
    Reconnecting,
    Error { kind: ErrorKind, message: String },
    Done,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ErrorKind {
    DiskFull,
    PermissionDenied,
    HashMismatch,
    ConnectionLost,
    PeerRejected,
    PairingFailed,
    AuthFailed,
    HostKeyMismatch,
    Other,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Direction {
    Send,
    Receive,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileProgress {
    pub file_id: Uuid,
    pub relative_path: String,
    pub size: u64,
    pub bytes_done: u64,
    pub status: ItemStatus,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransferItem {
    pub id: Uuid,
    pub priority_order: i64,
    pub direction: Direction,
    pub files: Vec<FileProgress>,
    pub status: ItemStatus,
    pub bytes_total: u64,
    pub bytes_done: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PeerInfo {
    pub id: String,
    pub display_name: String,
    pub address: String,
    pub trusted: bool,
}
