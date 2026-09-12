use std::sync::Arc;

use tauri::{AppHandle, State};
use tokio::net::{TcpListener, TcpStream};

use crate::engine::receiver::receive_one;
use crate::events::ConnectionStatusPayload;
use crate::net::direct_link::list_link_local_addresses as detect_link_local;
use crate::net::framing::new_framed;
use crate::net::transport::Transport;
use crate::net::DIRECT_PORT;
use crate::state::AppState;

#[tauri::command]
pub fn list_link_local_addresses() -> Vec<String> {
    detect_link_local()
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
            Ok(Some(_)) => continue,
            Ok(None) | Err(_) => break,
        }
    }

    if let Some(sink) = state.event_sink.as_ref() {
        sink.emit_connection_status(ConnectionStatusPayload {
            connected: false,
            peer_address: Some(peer_addr),
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
        });
    }
    Ok(())
}
