<script lang="ts">
  import { mode, type AppMode } from "../stores/mode";

  const items: { key: AppMode; label: string }[] = [
    { key: "direct", label: "Direct (Ethernet)" },
    { key: "network", label: "Network (Wi-Fi/LAN)" },
    { key: "ssh", label: "Server (SSH/SFTP)" },
  ];
</script>

<nav class="sidebar">
  <div class="title">File Transfer</div>

  <ul>
    {#each items as item}
      <li>
        <button class="nav-item" class:selected={$mode === item.key} onclick={() => mode.set(item.key)}>
          <span>{item.label}</span>
        </button>
      </li>
    {/each}
  </ul>

  <div class="spacer"></div>

  <button class="nav-item">Settings</button>
</nav>

<style>
  .sidebar {
    width: 220px;
    min-width: 220px;
    height: 100%;
    background: var(--sidebar-bg);
    border-right: 1px solid var(--card-border);
    display: flex;
    flex-direction: column;
    padding: var(--spacing-3);
    gap: var(--spacing-2);
  }

  .title {
    font-weight: 600;
    padding: 8px 10px 12px;
  }

  ul {
    list-style: none;
    margin: 0;
    padding: 0;
    display: flex;
    flex-direction: column;
    gap: 2px;
  }

  .nav-item {
    width: 100%;
    display: flex;
    align-items: center;
    justify-content: space-between;
    background: transparent;
    border: none;
    border-radius: var(--radius-sm);
    padding: 8px 10px;
    text-align: left;
    color: var(--text-primary);
  }

  .nav-item:hover {
    background: var(--hover-bg);
  }

  .nav-item.selected {
    background: var(--active-bg);
    color: var(--accent);
    font-weight: 600;
  }

  .spacer {
    flex: 1;
  }
</style>
