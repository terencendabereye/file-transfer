use std::sync::Mutex;

use crate::events::EventSink;
use crate::models::{ConnectionMode, TransferItem};

pub struct AppState {
    pub mode: Mutex<ConnectionMode>,
    pub queue: Mutex<Vec<TransferItem>>,
    pub event_sink: Option<Box<dyn EventSink>>,
}

impl AppState {
    pub fn new(event_sink: Option<Box<dyn EventSink>>) -> Self {
        Self {
            mode: Mutex::new(ConnectionMode::Direct),
            queue: Mutex::new(Vec::new()),
            event_sink,
        }
    }
}
