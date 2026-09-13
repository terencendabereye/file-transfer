<script lang="ts">
  import { mode } from "../stores/mode";
  import DirectMode from "./DirectMode.svelte";
</script>

<main class="main-pane">
  {#if $mode === "direct"}
    <DirectMode />
  {:else if $mode === "network"}
    <div class="placeholder">
      Network (Wi-Fi/LAN) mode isn't built yet — no discovery is running, so it can't
      find anything. Use Direct mode for now: enter the other computer's IP address
      directly.
    </div>
  {:else if $mode === "ssh"}
    <div class="placeholder">Server (SSH/SFTP) mode isn't built yet.</div>
  {/if}
</main>

<style>
  .main-pane {
    flex: 1;
    height: 100%;
    display: flex;
    align-items: flex-start;
    justify-content: center;
    overflow-y: auto;
  }

  .main-pane:has(.placeholder) {
    align-items: center;
  }

  .placeholder {
    color: var(--text-secondary);
    max-width: 360px;
    text-align: center;
    padding: var(--spacing-4);
  }
</style>
