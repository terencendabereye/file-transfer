use std::path::Path;

use tokio::io::{AsyncRead, AsyncReadExt, AsyncSeekExt, AsyncWrite};
use tokio_util::codec::{Framed, LengthDelimitedCodec};
use uuid::Uuid;

use crate::error::{AppError, Result};
use crate::net::framing::{recv_message, send_message};
use crate::net::protocol::{ChunkDataMsg, ManifestMsg, Message};

use super::scheduler::plan_chunk_ranges;
use super::{hashing, CHUNK_SIZE};

/// Sends one file over an already-connected transport: hashes it, sends the manifest,
/// waits for the receiver to say it's ready, then streams chunks in order. Returns
/// whether the receiver confirmed the hash matched on its end.
pub async fn send_file<T: AsyncRead + AsyncWrite + Unpin>(
    framed: &mut Framed<T, LengthDelimitedCodec>,
    transfer_id: Uuid,
    path: &Path,
    mut on_progress: impl FnMut(u64, u64),
) -> Result<bool> {
    let metadata = tokio::fs::metadata(path).await?;
    let size = metadata.len();
    let hash = hashing::hash_file(path).await?;
    let file_name = path
        .file_name()
        .map(|n| n.to_string_lossy().into_owned())
        .unwrap_or_else(|| "file".to_string());

    send_message(
        framed,
        &Message::Manifest(ManifestMsg {
            transfer_id,
            file_name,
            size,
            hash,
        }),
    )
    .await?;

    match recv_message(framed).await? {
        Some(Message::ManifestAck(ack)) if ack.transfer_id == transfer_id && ack.ready => {}
        Some(Message::Error(err)) => return Err(AppError::Other(err.message)),
        _ => return Err(AppError::Other("peer did not accept the transfer".to_string())),
    }

    let mut file = tokio::fs::File::open(path).await?;
    let mut buf = vec![0u8; CHUNK_SIZE];
    let mut bytes_sent = 0u64;
    for (start, end) in plan_chunk_ranges(size) {
        if end == start {
            continue;
        }
        file.seek(std::io::SeekFrom::Start(start)).await?;
        let len = (end - start) as usize;
        file.read_exact(&mut buf[..len]).await?;
        send_message(
            framed,
            &Message::ChunkData(ChunkDataMsg {
                transfer_id,
                offset: start,
                data: buf[..len].to_vec(),
            }),
        )
        .await?;
        bytes_sent += len as u64;
        on_progress(bytes_sent, size);
    }

    match recv_message(framed).await? {
        Some(Message::TransferComplete { transfer_id: id, success }) if id == transfer_id => {
            Ok(success)
        }
        Some(Message::Error(err)) => Err(AppError::Other(err.message)),
        _ => Err(AppError::Other("connection closed before transfer completed".to_string())),
    }
}
