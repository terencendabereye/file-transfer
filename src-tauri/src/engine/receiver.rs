use std::path::Path;

use tokio::io::{AsyncRead, AsyncWrite, AsyncWriteExt};
use tokio_util::codec::{Framed, LengthDelimitedCodec};
use uuid::Uuid;

use crate::net::framing::{recv_message, send_message};
use crate::net::protocol::{ErrorMsg, ManifestAckMsg, Message};

/// Receives one incoming file transfer on an already-connected transport: waits for a
/// manifest, accepts it, writes chunks to `dest_dir` as they arrive, then verifies the
/// BLAKE3 hash and reports success/failure back to the sender before returning.
pub async fn receive_one<T: AsyncRead + AsyncWrite + Unpin>(
    framed: &mut Framed<T, LengthDelimitedCodec>,
    dest_dir: &Path,
    mut on_progress: impl FnMut(Uuid, u64, u64),
) -> std::io::Result<Option<(Uuid, std::path::PathBuf, bool)>> {
    let manifest = loop {
        match recv_message(framed).await? {
            Some(Message::Manifest(m)) => break m,
            Some(Message::Hello(_)) => continue,
            Some(_) | None => return Ok(None),
        }
    };

    tokio::fs::create_dir_all(dest_dir).await?;
    let dest_path = dest_dir.join(&manifest.file_name);

    send_message(
        framed,
        &Message::ManifestAck(ManifestAckMsg {
            transfer_id: manifest.transfer_id,
            ready: true,
        }),
    )
    .await?;

    let mut file = tokio::fs::File::create(&dest_path).await?;
    let mut hasher = blake3::Hasher::new();
    let mut bytes_received = 0u64;

    while bytes_received < manifest.size {
        match recv_message(framed).await? {
            Some(Message::ChunkData(chunk)) if chunk.transfer_id == manifest.transfer_id => {
                file.write_all(&chunk.data).await?;
                hasher.update(&chunk.data);
                bytes_received += chunk.data.len() as u64;
                on_progress(manifest.transfer_id, bytes_received, manifest.size);
            }
            Some(_) | None => break,
        }
    }
    file.flush().await?;

    let success = bytes_received == manifest.size && *hasher.finalize().as_bytes() == manifest.hash;

    if success {
        send_message(
            framed,
            &Message::TransferComplete {
                transfer_id: manifest.transfer_id,
                success: true,
            },
        )
        .await?;
    } else {
        send_message(
            framed,
            &Message::Error(ErrorMsg {
                message: "hash mismatch or incomplete transfer".to_string(),
            }),
        )
        .await?;
    }

    Ok(Some((manifest.transfer_id, dest_path, success)))
}
