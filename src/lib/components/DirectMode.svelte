<script lang="ts">
  import { onMount } from "svelte";
  import {
    listLinkLocalAddresses,
    startListener,
    connect,
    disconnect,
    sendFile,
    pickFile,
    getDownloadDir,
    revealDownloadFolder,
  } from "../api/commands";
  import { connectionStatus } from "../stores/connection";
  import { transferProgress, transferLog } from "../stores/transfers";

  let linkLocalAddresses = $state<string[]>([]);
  let listening = $state(false);
  let downloadDir = $state("");
  let targetAddress = $state("");
  let error = $state<string | null>(null);
  let sending = $state(false);

  onMount(async () => {
    linkLocalAddresses = await listLinkLocalAddresses();
    downloadDir = await getDownloadDir();
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
  const isSender = $derived($connectionStatus.connected && $connectionStatus.role === "outbound");
  const isReceivingPeer = $derived($connectionStatus.connected && $connectionStatus.role === "inbound");
</script>

<div class="direct-mode">
  <section class="fluent-card">
    <h3>How this works</h3>
    <p class="hint">
      Pick <strong>one</strong> role per computer. The computer that will <strong>receive</strong> files clicks
      "Start Listening" below. The computer that will <strong>send</strong> a file enters the receiving
      computer's IP address and clicks "Connect", then "Choose File &amp; Send". Both computers must be
      on the same network (or connected directly by cable) and able to reach each other on port 53217 —
      a firewall prompt asking to allow the app on first listen must be accepted.
    </p>
  </section>

  <section class="fluent-card">
    <h3>Receive files on this computer</h3>
    {#if linkLocalAddresses.length > 0}
      <p class="hint">This computer's addresses: {linkLocalAddresses.join(", ")}</p>
    {:else}
      <p class="hint">
        No link-local (direct-cable) address detected — that's fine over a regular network. Find this
        computer's IP with <code>ipconfig</code> and give that to the sending computer.
      </p>
    {/if}
    <button class="fluent-button primary" onclick={handleStartListener} disabled={listening}>
      {listening ? "Listening for incoming files…" : "Start Listening"}
    </button>
    {#if isReceivingPeer}
      <p class="status-ok">Peer connected: {$connectionStatus.peer_address}</p>
    {/if}
    {#if downloadDir}
      <p class="hint">Received files are saved to:<br /><code>{downloadDir}</code></p>
      <button class="fluent-button" onclick={revealDownloadFolder}>Open Received Files Folder</button>
    {/if}
  </section>

  <section class="fluent-card">
    <h3>Send a file from this computer</h3>
    {#if isSender}
      <p class="status-ok">Connected to {$connectionStatus.peer_address}</p>
      <button class="fluent-button" onclick={handleDisconnect}>Disconnect</button>
      <button class="fluent-button primary" onclick={handleSendFile} disabled={sending}>
        {sending ? "Sending…" : "Choose File & Send"}
      </button>
      {#each progressList as p (p.transfer_id)}
        <div class="fluent-progress-track">
          <div class="fluent-progress-fill" style="width: {(p.bytes_done / Math.max(p.bytes_total, 1)) * 100}%"></div>
        </div>
      {/each}
    {:else}
      <input class="fluent-input" bind:value={targetAddress} placeholder="Receiving computer's IP address" />
      <button class="fluent-button primary" onclick={handleConnect} disabled={!targetAddress}>Connect</button>
    {/if}
  </section>

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
    max-width: 520px;
  }

  section {
    padding: var(--spacing-4);
    display: flex;
    flex-direction: column;
    gap: var(--spacing-2);
    align-items: flex-start;
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

  .status-ok {
    color: var(--success);
    font-size: 13px;
    margin: 0;
  }

  .error {
    color: var(--danger);
  }

  code {
    font-size: 11px;
    word-break: break-all;
  }

  .fluent-progress-track {
    width: 100%;
  }

  ul {
    margin: 0;
    padding-left: 18px;
    font-size: 12px;
    color: var(--text-secondary);
  }
</style>
