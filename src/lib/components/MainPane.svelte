<script lang="ts">
  import { mode } from "../stores/mode";
</script>

<main class="main-pane">
  {#if $mode === "direct"}
    <div class="placeholder">Waiting for a direct Ethernet link…</div>
  {:else if $mode === "network"}
    <div class="placeholder">Searching for peers on the network…</div>
  {:else if $mode === "ssh"}
    <div class="placeholder">Connect to a server to browse files.</div>
  {/if}
</main>

<style>
  .main-pane {
    flex: 1;
    height: 100%;
    display: flex;
    align-items: center;
    justify-content: center;
  }

  .placeholder {
    color: var(--text-secondary);
  }
</style>
