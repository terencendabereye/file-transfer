import { writable } from "svelte/store";
import type { TransferProgressPayload } from "../api/events";

export const transferProgress = writable<Record<string, TransferProgressPayload>>({});

export interface LogEntry {
  transferId: string;
  label: string;
  at: number;
}

export const transferLog = writable<LogEntry[]>([]);
