<script lang="ts">
  import { getCurrentWindow } from '@tauri-apps/api/window';
  import "../style.css";
  const appWindow = getCurrentWindow();

  function minimize() {
    appWindow.minimize();
  }

  function toggleMaximize() {
    appWindow.toggleMaximize();
  }

  function closeWindow() {
    appWindow.close();
  }

</script>

<style>
  .titlebar {
    height: 41px;
    background: oklch(52% 0.223 3.958);
    backdrop-filter: blur(8px);
    -webkit-backdrop-filter: blur(8px);
    border-bottom: 1px solid rgba(255, 255, 255, 0.1);
    box-shadow: 0 4px 10px rgba(0, 0, 0, 0.1);
    display: flex;
    align-items: center;
    justify-content: space-between;
    -webkit-app-region: drag;
    padding: 0 5px;
    position: fixed;
    top: -2px;
    left: 0;
    right: 0;
    z-index: 1000;
  }
  .title {
    margin-left: 30px;
    font-size: 1rem;
    color: rgba(255, 255, 255, 0.9);
    text-shadow: 0 1px 2px rgba(0, 0, 0, 0.1);
  }
  .buttons {
    display: flex;
  }
  .titlebar-button {
    width: 30px;
    height: 30px;
    display: flex;
    align-items: center;
    justify-content: center;
    -webkit-app-region: no-drag;
    cursor: pointer;
    margin-left: 5px;
    background: rgba(255, 255, 255, 0.05);
    border: 1px solid rgba(255, 255, 255, 0.1);
    border-radius: 4px;
    padding: 0;
    transition: all 0.2s ease;
  }
  .titlebar-button:hover {
    background: rgba(91, 190, 195, 0.25);
    backdrop-filter: blur(12px);
    -webkit-backdrop-filter: blur(12px);
    border: 1px solid rgba(255, 255, 255, 0.2);
    transform: translateY(-1px);
    box-shadow: 0 2px 5px rgba(0, 0, 0, 0.15);
  }
  .titlebar-button img {
    width: 16px;
    height: 16px;
    opacity: 0.8;
    filter: drop-shadow(0 1px 1px rgba(0, 0, 0, 0.1));
  }
  
  .titlebar-button:hover img {
    opacity: 1;
  }

</style>

<div class="titlebar">
  <div class="title">Smart Inventory</div>
  <div class="buttons">
    <button type="button" class="titlebar-button" on:click={minimize}>
      <img src="https://api.iconify.design/mdi:window-minimize.svg" alt="Minimize" />
    </button>
    <button type="button" class="titlebar-button" on:click={toggleMaximize}>
      <img src="https://api.iconify.design/mdi:window-maximize.svg" alt="Maximize" />
    </button>
    <button type="button" class="titlebar-button" on:click={closeWindow}>
      <img src="https://api.iconify.design/mdi:close.svg" alt="Close" />
    </button>
  </div>
</div>
