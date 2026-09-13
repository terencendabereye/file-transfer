use std::sync::Arc;

use tauri::{AppHandle, State};
use tauri_plugin_opener::OpenerExt;
use tokio::net::{TcpListener, TcpStream};

use crate::engine::receiver::receive_one;
use crate::error::AppError;
use crate::events::{ConnectionStatusPayload, TransferStatusChangedPayload};
use crate::models::{ErrorKind, ItemStatus};
use crate::net::direct_link::list_link_local_addresses as detect_link_local;
use crate::net::framing::new_framed;
use crate::net::transport::Transport;
use crate::net::DIRECT_PORT;
use crate::state::AppState;

#[tauri::command]
pub fn list_link_local_addresses() -> Vec<String> {
    detect_link_local()
}

/// Where received files are written. Surfaced in the UI so "nothing happened" isn't
/// the only feedback a successful-but-invisible receive gives someone.
#[tauri::command]
pub fn get_download_dir(state: State<'_, Arc<AppState>>) -> String {
    state.download_dir.to_string_lossy().into_owned()
}

#[tauri::command]
pub fn reveal_download_folder(app: AppHandle, state: State<'_, Arc<AppState>>) -> crate::error::Result<()> {
    let path = state.download_dir.to_string_lossy().into_owned();
    app.opener()
        .open_path(path, None::<&str>)
        .map_err(|e| AppError::Other(e.to_string()))
}

/// Starts listening for one incoming Direct-mode connection at a time. Each accepted
/// connection is served until it closes, receiving any number of files sequentially
/// (one manifest/chunk-stream/ack cycle after another) before the loop accepts again.
#[tauri::command]
pub async fn start_listener(app: AppHandle, state: State<'_, Arc<AppState>>) -> crate::error::Result<()> {
    let listener = TcpListener::bind(("0.0.0.0", DIRECT_PORT)).await?;
    let state = state.inner().clone();

    tauri::async_runtime::spawn(async move {
        loop {
            let (stream, peer_addr) = match listener.accept().await {
                Ok(pair) => pair,
                Err(_) => break,
            };
            let state = state.clone();
            let app = app.clone();
            tauri::async_runtime::spawn(async move {
                serve_incoming(app, state, stream, peer_addr.to_string()).await;
            });
        }
    });

    Ok(())
}

async fn serve_incoming(_app: AppHandle, state: Arc<AppState>, stream: TcpStream, peer_addr: String) {
    if let Some(sink) = state.event_sink.as_ref() {
        sink.emit_connection_status(ConnectionStatusPayload {
            connected: true,
            peer_address: Some(peer_addr.clone()),
            role: "inbound",
        });
    }

    let mut framed = new_framed(Transport::Plain(stream));
    loop {
        let dest_dir = state.download_dir.clone();
        let state_for_progress = state.clone();
        let result = receive_one(&mut framed, &dest_dir, |id, done, total| {
            if let Some(sink) = state_for_progress.event_sink.as_ref() {
                sink.emit_transfer_progress(crate::events::TransferProgressPayload {
                    transfer_id: id.to_string(),
                    bytes_done: done,
                    bytes_total: total,
                });
            }
        })
        .await;

        match result {
            Ok(Some((id, dest_path, success))) => {
                let file_name = dest_path
                    .file_name()
                    .map(|n| n.to_string_lossy().into_owned())
                    .unwrap_or_default();
                let status = if success {
                    ItemStatus::Done
                } else {
                    ItemStatus::Error {
                        kind: ErrorKind::HashMismatch,
                        message: "received file did not match the sender's hash".to_string(),
                    }
                };
                if let Some(sink) = state.event_sink.as_ref() {
                    sink.emit_transfer_status_changed(TransferStatusChangedPayload {
                        transfer_id: id.to_string(),
                        file_name,
                        status,
                    });
                }
                continue;
            }
            Ok(None) | Err(_) => break,
        }
    }

    if let Some(sink) = state.event_sink.as_ref() {
        sink.emit_connection_status(ConnectionStatusPayload {
            connected: false,
            peer_address: Some(peer_addr),
            role: "inbound",
        });
    }
}

#[tauri::command]
pub async fn connect(app: AppHandle, state: State<'_, Arc<AppState>>, address: String) -> crate::error::Result<()> {
    let target = if address.contains(':') {
        address.clone()
    } else {
        format!("{address}:{DIRECT_PORT}")
    };
    let stream = TcpStream::connect(&target).await?;
    let framed = new_framed(Transport::Plain(stream));
    *state.outbound.lock().await = Some(framed);

    if let Some(sink) = state.event_sink.as_ref() {
        sink.emit_connection_status(ConnectionStatusPayload {
            connected: true,
            peer_address: Some(target),
            role: "outbound",
        });
    }
    let _ = app;
    Ok(())
}

#[tauri::command]
pub async fn disconnect(state: State<'_, Arc<AppState>>) -> crate::error::Result<()> {
    *state.outbound.lock().await = None;
    if let Some(sink) = state.event_sink.as_ref() {
        sink.emit_connection_status(ConnectionStatusPayload {
            connected: false,
            peer_address: None,
            role: "outbound",
        });
    }
    Ok(())
}
