use bytes::Bytes;
use futures_util::{SinkExt, StreamExt};
use tokio::io::{AsyncRead, AsyncWrite};
use tokio_util::codec::{Framed, LengthDelimitedCodec};

use super::protocol::Message;

const MAX_FRAME_LEN: usize = 2 * 1024 * 1024;
const BINCODE_CONFIG: bincode::config::Configuration = bincode::config::standard();

pub fn new_framed<T: AsyncRead + AsyncWrite + Unpin>(io: T) -> Framed<T, LengthDelimitedCodec> {
    let codec = LengthDelimitedCodec::builder()
        .max_frame_length(MAX_FRAME_LEN)
        .new_codec();
    Framed::new(io, codec)
}

pub async fn send_message<T: AsyncRead + AsyncWrite + Unpin>(
    framed: &mut Framed<T, LengthDelimitedCodec>,
    message: &Message,
) -> std::io::Result<()> {
    let bytes = bincode::serde::encode_to_vec(message, BINCODE_CONFIG)
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?;
    framed.send(Bytes::from(bytes)).await
}

/// Returns `Ok(None)` when the peer closed the connection cleanly.
pub async fn recv_message<T: AsyncRead + AsyncWrite + Unpin>(
    framed: &mut Framed<T, LengthDelimitedCodec>,
) -> std::io::Result<Option<Message>> {
    let Some(frame) = framed.next().await else {
        return Ok(None);
    };
    let frame = frame?;
    let (message, _) = bincode::serde::decode_from_slice(&frame, BINCODE_CONFIG)
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?;
    Ok(Some(message))
}
