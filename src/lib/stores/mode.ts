import { writable } from "svelte/store";

export type AppMode = "direct" | "network" | "ssh";

export const mode = writable<AppMode>("direct");
