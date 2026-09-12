import { writable } from "svelte/store";

export interface ConnectionStatus {
  connected: boolean;
  peer_address: string | null;
}

export const connectionStatus = writable<ConnectionStatus>({ connected: false, peer_address: null });
