use tauri::Emitter;

use crate::events::{EventSink, TransferProgressPayload, TransferStatusChangedPayload};

/// The real app's EventSink: forwards to a live tauri::AppHandle. Only referenced
/// from lib.rs's setup - never from engine code or tests - so headless/test binaries
/// never need to construct a real Tauri window to link successfully.
pub struct TauriEventSink(pub tauri::AppHandle);

impl EventSink for TauriEventSink {
    fn emit_transfer_progress(&self, payload: TransferProgressPayload) {
        let _ = self.0.emit("transfer:progress", payload);
    }

    fn emit_transfer_status_changed(&self, payload: TransferStatusChangedPayload) {
        let _ = self.0.emit("transfer:status-changed", payload);
    }
}
