<script lang="ts">
  import Sidebar from "$lib/components/Sidebar.svelte";
  import MainPane from "$lib/components/MainPane.svelte";
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
