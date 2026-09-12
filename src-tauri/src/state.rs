use std::path::PathBuf;
use std::sync::Mutex as StdMutex;

use tokio::sync::Mutex as AsyncMutex;
use tokio_util::codec::{Framed, LengthDelimitedCodec};

use crate::engine::queue::Queue;
use crate::events::EventSink;
use crate::models::ConnectionMode;
use crate::net::transport::Transport;

pub struct AppState {
    pub mode: StdMutex<ConnectionMode>,
    pub queue: StdMutex<Queue>,
    pub event_sink: Option<Box<dyn EventSink>>,
    pub download_dir: PathBuf,
    /// The single active Direct/Network-mode connection. One at a time, matching the
    /// "sequential, one connection" philosophy carried through to SSH mode later.
    pub outbound: AsyncMutex<Option<Framed<Transport, LengthDelimitedCodec>>>,
}

impl AppState {
    pub fn new(event_sink: Option<Box<dyn EventSink>>, download_dir: PathBuf) -> Self {
        Self {
            mode: StdMutex::new(ConnectionMode::Direct),
            queue: StdMutex::new(Queue::new()),
            event_sink,
            download_dir,
            outbound: AsyncMutex::new(None),
        }
    }
}
