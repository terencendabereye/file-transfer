<script lang="ts">
  import { onMount } from "svelte";
  import {
    listLinkLocalAddresses,
    startListener,
    connect,
    disconnect,
    sendFile,
    pickFile,
  } from "../api/commands";
  import { connectionStatus } from "../stores/connection";
  import { transferProgress, transferLog } from "../stores/transfers";

  let linkLocalAddresses = $state<string[]>([]);
  let listening = $state(false);
  let targetAddress = $state("127.0.0.1");
  let error = $state<string | null>(null);
  let sending = $state(false);

  onMount(async () => {
    linkLocalAddresses = await listLinkLocalAddresses();
  });

  async function handleStartListener() {
    error = null;
    try {
      await startListener();
      listening = true;
    } catch (e) {
      error = String(e);
    }
  }

  async function handleConnect() {
    error = null;
    try {
      await connect(targetAddress);
    } catch (e) {
      error = String(e);
    }
  }

  async function handleDisconnect() {
    await disconnect();
  }

  async function handleSendFile() {
    error = null;
    const path = await pickFile();
    if (!path) return;
    sending = true;
    try {
      await sendFile(path);
    } catch (e) {
      error = String(e);
    } finally {
      sending = false;
    }
  }

  const progressList = $derived(Object.values($transferProgress));
</script>

<div class="direct-mode">
  <section class="fluent-card">
    <h3>Listen for incoming transfers</h3>
    {#if linkLocalAddresses.length > 0}
      <p class="hint">Link-local addresses detected: {linkLocalAddresses.join(", ")}</p>
    {:else}
      <p class="hint">No link-local (Ethernet-only) address detected yet — you can still test over localhost.</p>
    {/if}
    <button class="fluent-button primary" onclick={handleStartListener} disabled={listening}>
      {listening ? "Listening…" : "Start Listening"}
    </button>
  </section>

  <section class="fluent-card">
    <h3>Connect to a peer</h3>
    {#if $connectionStatus.connected}
      <p>Connected to {$connectionStatus.peer_address}</p>
      <button class="fluent-button" onclick={handleDisconnect}>Disconnect</button>
    {:else}
      <input class="fluent-input" bind:value={targetAddress} placeholder="IP address" />
      <button class="fluent-button primary" onclick={handleConnect}>Connect</button>
    {/if}
  </section>

  {#if $connectionStatus.connected}
    <section class="fluent-card">
      <h3>Send a file</h3>
      <button class="fluent-button primary" onclick={handleSendFile} disabled={sending}>
        {sending ? "Sending…" : "Choose File & Send"}
      </button>
      {#each progressList as p (p.transfer_id)}
        <div class="fluent-progress-track">
          <div class="fluent-progress-fill" style="width: {(p.bytes_done / Math.max(p.bytes_total, 1)) * 100}%"></div>
        </div>
      {/each}
    </section>
  {/if}

  {#if error}
    <p class="error">{error}</p>
  {/if}

  {#if $transferLog.length > 0}
    <section class="fluent-card">
      <h3>Recent activity</h3>
      <ul>
        {#each $transferLog as entry (entry.at)}
          <li>{entry.label}</li>
        {/each}
      </ul>
    </section>
  {/if}
</div>

<style>
  .direct-mode {
    display: flex;
    flex-direction: column;
    gap: var(--spacing-4);
    padding: var(--spacing-5);
    width: 100%;
    max-width: 480px;
  }

  section {
    padding: var(--spacing-4);
    display: flex;
    flex-direction: column;
    gap: var(--spacing-2);
  }

  h3 {
    margin: 0;
    font-size: 14px;
  }

  .hint {
    color: var(--text-secondary);
    font-size: 12px;
    margin: 0;
  }

  .error {
    color: var(--danger);
  }

  ul {
    margin: 0;
    padding-left: 18px;
    font-size: 12px;
    color: var(--text-secondary);
  }
</style>
