use std::path::PathBuf;
use std::sync::Arc;

use tauri::State;
use uuid::Uuid;

use crate::engine::sender::send_file as engine_send_file;
use crate::error::{AppError, Result};
use crate::events::{TransferProgressPayload, TransferStatusChangedPayload};
use crate::models::{Direction, ErrorKind, ItemStatus};
use crate::state::AppState;

/// Sends one file over the currently active Direct/Network-mode connection. Milestone
/// 2 scope: single file, no resume — the queue/multi-file UI lands in the next
/// milestone, but the underlying `TransferItem` shape is already the shared one.
#[tauri::command]
pub async fn send_file(state: State<'_, Arc<AppState>>, path: String) -> Result<()> {
    let path = PathBuf::from(path);
    let metadata = tokio::fs::metadata(&path).await?;
    let size = metadata.len();
    let file_name = path
        .file_name()
        .map(|n| n.to_string_lossy().into_owned())
        .unwrap_or_else(|| "file".to_string());

    let transfer_id = Uuid::new_v4();
    state
        .queue
        .lock()
        .unwrap()
        .push_new(transfer_id, Direction::Send, file_name.clone(), size);

    let mut guard = state.outbound.lock().await;
    let Some(framed) = guard.as_mut() else {
        return Err(AppError::Other("not connected to a peer".to_string()));
    };

    let sink = state.event_sink.as_ref();
    let result = engine_send_file(framed, transfer_id, &path, |done, total| {
        if let Some(sink) = sink {
            sink.emit_transfer_progress(TransferProgressPayload {
                transfer_id: transfer_id.to_string(),
                bytes_done: done,
                bytes_total: total,
            });
        }
    })
    .await;

    let status = match &result {
        Ok(true) => ItemStatus::Done,
        Ok(false) => ItemStatus::Error {
            kind: ErrorKind::HashMismatch,
            message: "receiver reported a hash mismatch".to_string(),
        },
        Err(err) => ItemStatus::Error {
            kind: ErrorKind::Other,
            message: err.user_message(),
        },
    };

    state.queue.lock().unwrap().update(transfer_id, |item| {
        item.status = status.clone();
    });
    if let Some(sink) = sink {
        sink.emit_transfer_status_changed(TransferStatusChangedPayload {
            transfer_id: transfer_id.to_string(),
            file_name,
            status,
        });
    }

    result.map(|_| ())
}
