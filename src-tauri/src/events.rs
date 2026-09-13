use serde::Serialize;

use crate::models::ItemStatus;

#[derive(Debug, Clone, Serialize)]
pub struct TransferProgressPayload {
    pub transfer_id: String,
    pub bytes_done: u64,
    pub bytes_total: u64,
}

#[derive(Debug, Clone, Serialize)]
pub struct TransferStatusChangedPayload {
    pub transfer_id: String,
    pub file_name: String,
    pub status: ItemStatus,
}

#[derive(Debug, Clone, Serialize)]
pub struct ConnectionStatusPayload {
    pub connected: bool,
    pub peer_address: Option<String>,
    /// "inbound" for a peer that connected to this machine's listener (this side can
    /// only receive), "outbound" for a connection this machine initiated with
    /// `connect` (this side can send). Kept distinct so the UI doesn't offer a "Send
    /// a file" button on the listening/receiving side, where it would always fail.
    pub role: &'static str,
}

/// Abstracts event emission away from the concrete `tauri::AppHandle` type so the
/// engine (and its tests) never need to reference Tauri/WebView2 types directly.
/// The only implementation used by the real app lives in `tauri_event_sink.rs`;
/// headless contexts (tests) just pass `None`.
pub trait EventSink: Send + Sync {
    fn emit_transfer_progress(&self, payload: TransferProgressPayload);
    fn emit_transfer_status_changed(&self, payload: TransferStatusChangedPayload);
    fn emit_connection_status(&self, payload: ConnectionStatusPayload);
}
