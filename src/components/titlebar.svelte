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
    background: oklch(44% 0.011 73.639);
    display: flex;
    align-items: center;
    justify-content: space-between;
    -webkit-app-region: drag;
    padding: 0 5px;
    position: fixed;
    top: 0;
    left: 0;
    right: 0;
    z-index: 1000;
  }
  .title {
    margin-left: 30px;
    font-size: 1rem;
    color: rgb(255, 255, 255);
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
    background: none;
    border: none;
    padding: 0;
  }
  .titlebar-button:hover {
    background: #5bbec3;
  }
  .titlebar-button img {
    width: 16px;
    height: 16px;
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
