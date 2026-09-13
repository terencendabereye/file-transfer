import { listen } from "@tauri-apps/api/event";
import { connectionStatus } from "../stores/connection";
import { transferProgress, transferLog } from "../stores/transfers";

export interface TransferProgressPayload {
  transfer_id: string;
  bytes_done: number;
  bytes_total: number;
}

export interface TransferStatusChangedPayload {
  transfer_id: string;
  file_name: string;
  status: { status: string; kind?: string; message?: string };
}

export interface ConnectionStatusPayload {
  connected: boolean;
  peer_address: string | null;
  role: "inbound" | "outbound";
}

export async function startEventListeners() {
  await listen<TransferProgressPayload>("transfer:progress", (event) => {
    transferProgress.update((map) => ({ ...map, [event.payload.transfer_id]: event.payload }));
  });

  await listen<TransferStatusChangedPayload>("transfer:status-changed", (event) => {
    const { transfer_id, file_name, status } = event.payload;
    const label =
      status.status === "error"
        ? `Failed (${file_name}): ${status.message ?? status.kind ?? "unknown error"}`
        : status.status === "done"
          ? `Completed: ${file_name}`
          : `${file_name}: ${status.status}`;
    transferLog.update((log) => [{ transferId: transfer_id, label, at: Date.now() }, ...log].slice(0, 20));
  });

  await listen<ConnectionStatusPayload>("connection:status", (event) => {
    connectionStatus.set(event.payload);
  });
}
