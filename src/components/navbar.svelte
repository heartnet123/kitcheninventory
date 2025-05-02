<script lang="ts">
  export let items = [
    { link: "/", label: "Dashboard", icon: "🏠" },
    { link: "/inventory", label: "Inventory", icon: "📦" },
    { link: "/recipe", label: "Recipe", icon: "📜" },
    { link: "/finance", label: "Finance", icon: "📈" },
    { link: "/test", label: "test", icon: "📜" }
  ];

  import { page } from '$app/stores';
  import { goto } from '$app/navigation';
  import "../style.css";
  
  let drawerOpen = false;
  
  // Get the current path from the page store
  $: activePath = $page.url.pathname;
  
  // Handle navigation with SvelteKit's goto function
  function handleNavigation(path: string | URL) {
    goto(path);
    drawerOpen = false;
  }
</script>

<style>
  /* Glass effect base styles */
  .glass-nav {
    background: oklch(94% 0.028 342.258);
    backdrop-filter: blur(8px);
    -webkit-backdrop-filter: blur(8px);
    border: 1px solid rgba(255, 255, 255, 0.1);
    box-shadow: 0 4px 10px rgba(0, 0, 0, 0.1);
  }

  /* Desktop sidebar specific styles */
  .sidebar-desktop {
    padding-top: 41px; /* Match titlebar height */
    transition: all 0.3s ease;
  }

  /* Menu item hover effects */
  :global(.menu li > *:hover) {
    background: rgba(255, 255, 255, 0.1) !important;
    border: 1px solid rgba(255, 255, 255, 0.2);
    backdrop-filter: blur(12px);
    -webkit-backdrop-filter: blur(12px);
  }

  /* Active menu item */
  :global(.menu li > *.active) {
    background: rgba(91, 190, 195, 0.25) !important;
    border: 1px solid rgba(255, 255, 255, 0.2);
  }

  /* Version display area */
  .version-area {
    border-top: 1px solid rgba(255, 255, 255, 0.1);
    background: oklch(94% 0.028 342.258);
  }
</style>

<div class="">
  <!-- Mobile Drawer -->
  <div class="drawer lg:hidden">
    <input id="my-drawer" type="checkbox" class="drawer-toggle" bind:checked={drawerOpen} />
    
    <div class="drawer-content">
      <!-- Mobile navbar -->
      <div class="navbar glass-nav min-h-[41px]">
        <div class="flex-none">
          <label for="my-drawer" class="btn btn-square btn-ghost">
            <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" class="inline-block w-5 h-5 stroke-current">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 6h16M4 12h16M4 18h16"></path>
            </svg>
          </label>
        </div>
        <div class="flex-1">
          <span class="text-lg font-bold">Smart Inventory</span>
        </div>
      </div>
    </div>
    
    <!-- Mobile sidebar content -->
    <div class="drawer-side z-50">
      <label for="my-drawer" class="drawer-overlay"></label>
      <div class="mt-[41px] menu p-4 w-80 h-full glass-nav text-base-content">
        <div class="flex items-center gap-2 mb-6 px-2">
          <span class="text-xl font-bold">Smart Inventory</span>
        </div>
        <ul class="space-y-1">
          {#each items as item}
            <li>
              <a 
                href={item.link} 
                class:active={activePath === item.link}
                on:click|preventDefault={() => handleNavigation(item.link)}
              >
                <span class="text-xl mr-2">{item.icon}</span>
                {item.label}
              </a>
            </li>
          {/each}
        </ul>
        <div class="mt-auto version-area pt-4">
          <div class="text-sm opacity-80 px-2">v1.0.0</div>
        </div>
      </div>
    </div>
  </div>

  <!-- Desktop Sidebar - Fixed position -->
  <div class="hidden lg:flex lg:w-64 lg:fixed lg:top-0 lg:left-0 lg:h-screen lg:flex-col glass-nav sidebar-desktop">
    <div class="mt-6 p-4">
      <ul class="menu space-y-1">
        {#each items as item}
          <li>
            <a 
              href={item.link} 
              class:active={activePath === item.link}
              on:click|preventDefault={() => handleNavigation(item.link)}
            >
              <span class="text-xl mr-2">{item.icon}</span>
              {item.label}
            </a>
          </li>
        {/each}
      </ul>
    </div>
    <div class="mt-auto version-area pt-4 p-4">
      <div class="text-sm opacity-80">v1.0.0</div>
    </div>
  </div>

  <!-- Main content area -->
  <div class="lg:ml-64 pt-[41px]">
    <slot />
  </div>
</div>