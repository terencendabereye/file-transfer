use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HelloMsg {
    pub app_version: String,
    pub device_name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ManifestMsg {
    pub transfer_id: Uuid,
    pub file_name: String,
    pub size: u64,
    pub hash: [u8; 32],
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ManifestAckMsg {
    pub transfer_id: Uuid,
    pub ready: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChunkDataMsg {
    pub transfer_id: Uuid,
    pub offset: u64,
    pub data: Vec<u8>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ErrorMsg {
    pub message: String,
}

/// Wire message shape shared by Direct and Network (P2P) modes. Written once, kept
/// generic over the underlying `Transport` so `engine::sender`/`receiver` never branch
/// on mode. Noise handshake/pairing variants are added in the Network-mode milestone.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Message {
    Hello(HelloMsg),
    Manifest(ManifestMsg),
    ManifestAck(ManifestAckMsg),
    ChunkData(ChunkDataMsg),
    TransferComplete { transfer_id: Uuid, success: bool },
    Error(ErrorMsg),
}
