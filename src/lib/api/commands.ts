import { invoke } from "@tauri-apps/api/core";
import { open } from "@tauri-apps/plugin-dialog";

export function listLinkLocalAddresses(): Promise<string[]> {
  return invoke("list_link_local_addresses");
}

export function startListener(): Promise<void> {
  return invoke("start_listener");
}

export function connect(address: string): Promise<void> {
  return invoke("connect", { address });
}

export function disconnect(): Promise<void> {
  return invoke("disconnect");
}

export function sendFile(path: string): Promise<void> {
  return invoke("send_file", { path });
}

export async function pickFile(): Promise<string | null> {
  const selected = await open({ multiple: false, directory: false });
  return typeof selected === "string" ? selected : null;
}
