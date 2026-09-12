<script lang="ts">
  import { mode } from "../stores/mode";
  import DirectMode from "./DirectMode.svelte";
</script>

<main class="main-pane">
  {#if $mode === "direct"}
    <DirectMode />
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
    align-items: flex-start;
    justify-content: center;
    overflow-y: auto;
  }

  .main-pane:has(.placeholder) {
    align-items: center;
  }

  .placeholder {
    color: var(--text-secondary);
  }
</style>
