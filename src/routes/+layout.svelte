<script lang="ts">
  import "../lib/styles/global.css";
  import "../lib/styles/fluent.css";

  let { children } = $props();

  function suppressNativeContextMenu(e: MouseEvent) {
    const target = e.target as HTMLElement;
    // Let inputs/textareas keep the native menu (cut/copy/paste) — only
    // suppress Tauri's WebView2 default menu (Back/Forward/Reload/Inspect).
    if (target.closest("input, textarea")) return;
    e.preventDefault();
  }
</script>

<svelte:document oncontextmenu={suppressNativeContextMenu} />

{@render children()}
