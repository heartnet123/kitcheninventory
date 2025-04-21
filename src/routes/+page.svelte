<script lang="ts">
  import { onMount } from 'svelte';
  import Database from "@tauri-apps/plugin-sql";

  // --- State Variables ---
  let isLoading = true;
  let error = '';
  let totalItems = 0;
  let lowStockCount = 0;
  let categoryCounts: { category: string; count: number }[] = [];

  // Define a threshold for low stock
  const LOW_STOCK_THRESHOLD = 5;

  // --- Database Interaction ---
  async function loadDashboardData() {
    isLoading = true;
    error = '';
    try {
      const db = await Database.load("sqlite:inventory.db");

      // Get total items
      const totalResult = await db.select<{ count: number }[]>("SELECT COUNT(*) as count FROM items");
      totalItems = totalResult[0]?.count ?? 0;

      // Get low stock count
      const lowStockResult = await db.select<{ count: number }[]>(`SELECT COUNT(*) as count FROM items WHERE quantity < ${LOW_STOCK_THRESHOLD}`);
      lowStockCount = lowStockResult[0]?.count ?? 0;

      // Get counts per category
      categoryCounts = await db.select<{ category: string; count: number }[]>("SELECT category, COUNT(*) as count FROM items GROUP BY category ORDER BY category");

      isLoading = false;
    } catch (err) {
      console.error("Failed to load dashboard data:", err);
      error = "Failed to load dashboard data - check console";
      isLoading = false;
    }
  }

  onMount(() => {
    loadDashboardData();
  });
</script>


<div class="fade-in container mx-auto px-4 py-8 w-full min-h-screen p-0">
  <div class="max-w-7xl ms-auto mt-10">
    <div class="flex justify-between items-center mb-8">
      <h1 class="text-3xl font-bold">Kitchen Dashboard</h1>
    </div>
    
    <!-- Stats Overview -->
    <div class="grid grid-cols-1 md:grid-cols-2 gap-6 mb-8"> 
      {#if isLoading}
        {#each Array(2) as _} 
          <div class="card bg-base-100 shadow-xl animate-pulse">
            <div class="card-body p-4">
              <div class="flex items-center gap-4">
                <div class="bg-base-300 rounded h-10 w-10"></div>
                <div>
                  <div class="bg-base-300 h-4 w-24 rounded mb-1"></div>
                  <div class="bg-base-300 h-6 w-16 rounded mb-1"></div>
                  <div class="bg-base-300 h-3 w-32 rounded"></div>
                </div>
              </div>
            </div>
          </div>
        {/each}
      {:else if error}
         <div class="card bg-error text-error-content shadow-xl md:col-span-2">
           <div class="card-body p-4">
             <p>{error}</p>
           </div>
         </div>
      {:else}
        <!-- Items in Stock Stat -->
        <div class="card bg-base-100 shadow-xl">
          <div class="card-body p-4">
            <div class="flex items-center gap-4">
              <div class="text-4xl">📦</div>
              <div>
                <div class="stat-title">Items in Stock</div>
                <div class="stat-value">{totalItems}</div>
                <div class="stat-desc">Total distinct items</div>
              </div>
            </div>
          </div>
        </div>
        <!-- Low Stock Stat -->
        <div class="card bg-base-100 shadow-xl">
          <div class="card-body p-4">
            <div class="flex items-center gap-4">
              <div class="text-4xl">⚠️</div>
              <div>
                <div class="stat-title">Low Stock</div>
                <div class="stat-value">{lowStockCount}</div>
                <div class="stat-desc">{`Items below ${LOW_STOCK_THRESHOLD} units`}</div>
              </div>
            </div>
          </div>
        </div>
      {/if}
      
    </div>
    
    <div class="grid grid-cols-1 gap-6"> 
      
      <div class="card bg-base-100 shadow-xl"> 
        <div class="card-body">
          <h2 class="card-title">Inventory Status by Category</h2>
          {#if isLoading}
            <div class="overflow-x-auto animate-pulse">
              <table class="table table-zebra w-full">
                <thead>
                  <tr>
                    <th class="bg-base-300 h-4 w-1/3 rounded"></th>
                    <th class="bg-base-300 h-4 w-1/3 rounded"></th>
                  </tr>
                </thead>
                <tbody>
                  {#each Array(4) as _}
                    <tr>
                      <td class="bg-base-200 h-4 rounded"></td>
                      <td class="bg-base-200 h-4 rounded"></td>
                    </tr>
                  {/each}
                </tbody>
              </table>
            </div>
          {:else if error}
            <p class="text-error">{error}</p>
          {:else if categoryCounts.length === 0}
            <p>No inventory items found to categorize.</p>
          {:else}
            <div class="overflow-x-auto">
              <table class="table table-zebra w-full">
                <thead>
                  <tr>
                    <th>Category</th>
                    <th>Item Count</th>
                  </tr>
                </thead>
                <tbody>
                  {#each categoryCounts as catCount}
                    <tr class="hover">
                      <td>{catCount.category || 'Uncategorized'}</td>
                      <td>{catCount.count}</td>
                    </tr>
                  {/each}
                </tbody>
              </table>
            </div>
          {/if}
        </div>
      </div>
    </div>
  </div>
</div>
