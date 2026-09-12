use uuid::Uuid;

use crate::models::{Direction, FileProgress, ItemStatus, TransferItem};

/// In-memory ordered queue of transfers, shared by all three modes. `priority_order` is
/// assigned once at push time from a monotonic counter and never recomputed — that's
/// what keeps the frontend list from reordering as items complete (see
/// `src/lib/stores/transfers.ts`).
pub struct Queue {
    items: Vec<TransferItem>,
    next_order: i64,
}

impl Queue {
    pub fn new() -> Self {
        Self {
            items: Vec::new(),
            next_order: 0,
        }
    }

    pub fn push_new(
        &mut self,
        id: Uuid,
        direction: Direction,
        file_name: String,
        size: u64,
    ) -> i64 {
        let order = self.next_order;
        self.next_order += 1;
        self.items.push(TransferItem {
            id,
            priority_order: order,
            direction,
            files: vec![FileProgress {
                file_id: id,
                relative_path: file_name,
                size,
                bytes_done: 0,
                status: ItemStatus::Queued,
            }],
            status: ItemStatus::Queued,
            bytes_total: size,
            bytes_done: 0,
        });
        order
    }

    pub fn update<F: FnOnce(&mut TransferItem)>(&mut self, id: Uuid, f: F) {
        if let Some(item) = self.items.iter_mut().find(|i| i.id == id) {
            f(item);
        }
    }

    pub fn snapshot(&self) -> Vec<TransferItem> {
        self.items.clone()
    }
}

impl Default for Queue {
    fn default() -> Self {
        Self::new()
    }
}
