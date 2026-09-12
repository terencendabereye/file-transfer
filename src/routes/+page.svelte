<script lang="ts">
  import { onMount } from "svelte";
  import Sidebar from "$lib/components/Sidebar.svelte";
  import MainPane from "$lib/components/MainPane.svelte";
  import { startEventListeners } from "$lib/api/events";

  onMount(() => {
    startEventListeners();
  });
</script>

<div class="app-shell">
  <Sidebar />
  <MainPane />
</div>

<style>
  .app-shell {
    display: flex;
    height: 100vh;
    width: 100vw;
  }
</style>
