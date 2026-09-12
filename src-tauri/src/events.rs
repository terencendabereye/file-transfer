use serde::Serialize;

use crate::models::{ItemStatus, TransferItem};

#[derive(Debug, Clone, Serialize)]
pub struct TransferProgressPayload {
    pub transfer_id: String,
    pub bytes_done: u64,
    pub bytes_total: u64,
}

#[derive(Debug, Clone, Serialize)]
pub struct TransferStatusChangedPayload {
    pub transfer_id: String,
    pub status: ItemStatus,
}

/// Abstracts event emission away from the concrete `tauri::AppHandle` type so the
/// engine (and its tests) never need to reference Tauri/WebView2 types directly.
/// The only implementation used by the real app lives in `tauri_event_sink.rs`;
/// headless contexts (tests) just pass `None`.
pub trait EventSink: Send + Sync {
    fn emit_transfer_progress(&self, payload: TransferProgressPayload);
    fn emit_transfer_status_changed(&self, payload: TransferStatusChangedPayload);
}

#[allow(dead_code)]
pub fn transfer_summary(item: &TransferItem) -> (u64, u64) {
    (item.bytes_done, item.bytes_total)
}
