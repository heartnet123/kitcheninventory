<script lang="ts">
  export let items = [
    { link: "/", label: "Dashboard", icon: "🏠" },
    { link: "/inventory", label: "Inventory", icon: "📦" },
    { link: "/finance", label: "Finance", icon: "📈" },
    { link: "/recipe", label: "Recipe", icon: "📜" },
    { link: "/test", label: "test", icon: "📜" }
  ];
  
  import { onMount } from 'svelte';
  import "../style.css";
  
  let activePath = "/";
  let drawerOpen = false;
  
  onMount(() => {
    activePath = window.location.pathname;
    
    const handleNavigation = () => {
      activePath = window.location.pathname;
    };
    
    window.addEventListener('popstate', handleNavigation);
    
    return () => {
      window.removeEventListener('popstate', handleNavigation);
    };
  });
</script>

<div class="">
  <!-- Mobile Drawer -->
  <div class="drawer lg:hidden">
    <input id="my-drawer" type="checkbox" class="drawer-toggle" bind:checked={drawerOpen} />
    
    <div class="drawer-content">
      <!-- Mobile navbar -->
      <div class="navbar bg-base-200">
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
      <div class="mt-6 menu p-4 w-80 h-full bg-base-200 text-base-content">
        <div class="flex items-center gap-2 mb-6 px-2">
          <span class="text-xl font-bold">Smart Inventory</span>
        </div>
        <ul class="space-y-1">
          {#each items as item}
            <li>
              <a 
                href={item.link} 
                class:active={activePath === item.link}
                on:click={() => {
                  activePath = item.link;
                  drawerOpen = false;
                }}
              >
                <span class="text-xl mr-2">{item.icon}</span>
                {item.label}
              </a>
            </li>
          {/each}
        </ul>
        <div class="mt-auto border-t border-base-300 pt-4">
          <div class="text-sm opacity-80 px-2">v1.0.0</div>
        </div>
      </div>
    </div>
  </div>

  <!-- Desktop Sidebar - Fixed position -->
  <div class="hidden lg:flex lg:w-64 lg:fixed lg:top-0 lg:left-0 lg:h-screen lg:flex-col lg:border-r lg:border-base-300 bg-base-200">
    <div class="mt-6 p-4">
      <ul class="menu space-y-1">
        {#each items as item}
          <li>
            <a 
              href={item.link} 
              class:active={activePath === item.link}
            >
              <span class="text-xl mr-2">{item.icon}</span>
              {item.label}
            </a>
          </li>
        {/each}
      </ul>
    </div>
    <div class="mt-auto p-4 border-t border-base-300">
      <div class="text-sm opacity-80">v1.0.0</div>
    </div>
  </div>

  <!-- Main content area -->
  <div class="">
    <slot />
  </div>
</div>