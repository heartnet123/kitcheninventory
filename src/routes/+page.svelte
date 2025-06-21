<script lang="ts">
  import { onMount } from 'svelte';
  import Database from "@tauri-apps/plugin-sql";
  import { formatDate } from '../lib/utils';

  // --- Enhanced State Variables ---
  let isLoading = true;
  let error = '';
  
  // Inventory Stats
  let totalItems = 0;
  let lowStockCount = 0;
  let expiringCount = 0;
  let totalInventoryValue = 0;
  let categoryCounts: { category: string; count: number; value: number }[] = [];
  
  // Financial Stats
  let todayRevenue = 0;
  let monthlyRevenue = 0;
  let monthlyExpenses = 0;
  let monthlyProfit = 0;
  let profitMargin = 0;
  
  // Recipe Stats
  let totalRecipes = 0;
  let topRecipes: { name: string; sold: number; revenue: number }[] = [];
  
  // Recent Activity
  let recentTransactions: { date: string; type: string; description: string; amount: number }[] = [];
  let lowStockItems: { name: string; quantity: number; category: string }[] = [];
  let expiringItems: { name: string; expiration_date: string; days_until_expiry: number }[] = [];

  // Thresholds
  const LOW_STOCK_THRESHOLD = 5;
  const EXPIRY_WARNING_DAYS = 7;

  // --- Enhanced Database Interaction ---
  async function loadDashboardData() {
    isLoading = true;
    error = '';
    try {
      const db = await Database.load("sqlite:inventory.db");
      
      // Inventory Statistics
      await loadInventoryStats(db);
      await loadFinancialStats(db);
      await loadRecipeStats(db);
      await loadRecentActivity(db);
      await loadAlerts(db);

      isLoading = false;
    } catch (err) {
      console.error("Failed to load dashboard data:", err);
      error = "Failed to load dashboard data - check console";
      isLoading = false;
    }
  }

  async function loadInventoryStats(db: any) {
    // Total items
    const totalResult = await db.select("SELECT COUNT(*) as count FROM items");
    totalItems = totalResult[0]?.count ?? 0;

    // Low stock count
    const lowStockResult = await db.select(
      "SELECT COUNT(*) as count FROM items WHERE quantity < $1",
      [LOW_STOCK_THRESHOLD]
    );
    lowStockCount = lowStockResult[0]?.count ?? 0;

    // Expiring items count (items expiring within EXPIRY_WARNING_DAYS)
    const expiringResult = await db.select(
      "SELECT COUNT(*) as count FROM items WHERE expiration_date IS NOT NULL AND julianday(expiration_date) - julianday('now') <= $1 AND julianday(expiration_date) - julianday('now') >= 0",
      [EXPIRY_WARNING_DAYS]
    );
    expiringCount = expiringResult[0]?.count ?? 0;

    // Total inventory value
    const valueResult = await db.select(
      "SELECT COALESCE(SUM(quantity * cost_per_unit), 0) as total_value FROM items"
    );
    totalInventoryValue = valueResult[0]?.total_value ?? 0;

    // Category counts with values
    categoryCounts = await db.select(
      "SELECT category, COUNT(*) as count, COALESCE(SUM(quantity * cost_per_unit), 0) as value FROM items GROUP BY category ORDER BY value DESC"
    );
  }

  async function loadFinancialStats(db: any) {
    const today = new Date().toISOString().split('T')[0];
    const currentMonth = new Date().toISOString().slice(0, 7); // YYYY-MM format

    // Today's revenue
    const todayResult = await db.select(
      "SELECT COALESCE(SUM(amount), 0) as revenue FROM financial_records WHERE record_type = 'Income' AND DATE(record_date) = $1",
      [today]
    );
    todayRevenue = todayResult[0]?.revenue ?? 0;

    // Monthly revenue
    const monthlyRevenueResult = await db.select(
      "SELECT COALESCE(SUM(amount), 0) as revenue FROM financial_records WHERE record_type = 'Income' AND strftime('%Y-%m', record_date) = $1",
      [currentMonth]
    );
    monthlyRevenue = monthlyRevenueResult[0]?.revenue ?? 0;

    // Monthly expenses
    const monthlyExpensesResult = await db.select(
      "SELECT COALESCE(SUM(amount), 0) as expenses FROM financial_records WHERE record_type = 'Expense' AND strftime('%Y-%m', record_date) = $1",
      [currentMonth]
    );
    monthlyExpenses = monthlyExpensesResult[0]?.expenses ?? 0;

    // Calculate profit and margin
    monthlyProfit = monthlyRevenue - monthlyExpenses;
    profitMargin = monthlyRevenue > 0 ? (monthlyProfit / monthlyRevenue) * 100 : 0;
  }

  async function loadRecipeStats(db: any) {
    // Total recipes
    const recipesResult = await db.select("SELECT COUNT(*) as count FROM recipes");
    totalRecipes = recipesResult[0]?.count ?? 0;

    // Top performing recipes (by revenue this month)
    const currentMonth = new Date().toISOString().slice(0, 7);
    topRecipes = await db.select(
      `SELECT r.name,
              COALESCE(SUM(fr.quantity), 0) as sold,
              COALESCE(SUM(fr.amount), 0) as revenue
       FROM recipes r
       LEFT JOIN financial_records fr ON r.id = fr.recipe_id
       WHERE fr.record_type = 'Income' AND strftime('%Y-%m', fr.record_date) = $1
       GROUP BY r.id, r.name
       HAVING sold > 0
       ORDER BY revenue DESC
       LIMIT 5`,
      [currentMonth]
    );
  }

  async function loadRecentActivity(db: any) {
    // Recent financial transactions
    recentTransactions = await db.select(
      "SELECT record_date as date, record_type as type, description, amount FROM financial_records ORDER BY record_date DESC LIMIT 5"
    );
  }

  async function loadAlerts(db: any) {
    // Low stock items
    lowStockItems = await db.select(
      "SELECT name, quantity, category FROM items WHERE quantity < $1 ORDER BY quantity ASC LIMIT 10",
      [LOW_STOCK_THRESHOLD]
    );

    // Expiring items
    expiringItems = await db.select(
      `SELECT name, expiration_date,
              CAST(julianday(expiration_date) - julianday('now') as INTEGER) as days_until_expiry
       FROM items
       WHERE expiration_date IS NOT NULL
       AND julianday(expiration_date) - julianday('now') <= $1
       AND julianday(expiration_date) - julianday('now') >= 0
       ORDER BY days_until_expiry ASC
       LIMIT 10`,
      [EXPIRY_WARNING_DAYS]
    );
  }

  onMount(() => {
    loadDashboardData();
    
    // Auto-refresh every 5 minutes
    const interval = setInterval(loadDashboardData, 5 * 60 * 1000);
    return () => clearInterval(interval);
  });

  // Helper functions
  function formatCurrency(amount: number): string {
    return amount.toLocaleString('th-TH', { style: 'currency', currency: 'THB' });
  }


  function getStatusColor(value: number, threshold: number): string {
    if (value === 0) return 'text-gray-500';
    return value <= threshold ? 'text-error' : 'text-success';
  }
</script>


<div class="fade-in container mx-auto px-4 py-8 w-full min-h-screen p-0">
  <div class="max-w-7xl ms-auto mt-10">
    
    <!-- Header with Refresh -->
    <div class="flex flex-col sm:flex-row justify-between items-start sm:items-center mb-8 gap-4">
      <div>
        <h1 class="text-3xl font-bold">Home Kitchen Dashboard</h1>
      </div>
      <div class="flex gap-2">
        <button class="btn btn-outline btn-sm" on:click={loadDashboardData} disabled={isLoading}>
          <span class="loading loading-spinner loading-sm {isLoading ? '' : 'hidden'}"></span>
          {isLoading ? 'Loading...' : '🔄 Refresh'}
        </button>
        <div class="text-xs text-base-content/50 self-center">
          Auto-refresh every 5 min
        </div>
      </div>
    </div>

    {#if error}
      <div class="alert alert-error shadow-lg mb-8">
        <div>
          <svg xmlns="http://www.w3.org/2000/svg" class="stroke-current flex-shrink-0 h-6 w-6" fill="none" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M10 14l2-2m0 0l2-2m-2 2l-2-2m2 2l2 2m7-2a9 9 0 11-18 0 9 9 0 0118 0z" /></svg>
          <span>{error}</span>
        </div>
      </div>
    {/if}

    <!-- Financial Overview Cards -->
    <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-6 mb-8">
      {#if isLoading}
        {#each Array(4) as _}
          <div class="card bg-base-100 shadow-xl animate-pulse">
            <div class="card-body p-4">
              <div class="flex items-center gap-4">
                <div class="bg-base-300 rounded h-12 w-12"></div>
                <div class="flex-1">
                  <div class="bg-base-300 h-4 w-20 rounded mb-2"></div>
                  <div class="bg-base-300 h-6 w-16 rounded mb-1"></div>
                  <div class="bg-base-300 h-3 w-24 rounded"></div>
                </div>
              </div>
            </div>
          </div>
        {/each}
      {:else}
        <!-- Today's Revenue -->
        <div class="card bg-gradient-to-br from-green-50 to-green-100 shadow-xl border border-green-200">
          <div class="card-body p-4">
            <div class="flex items-center gap-4">
              <div class="text-4xl">💰</div>
              <div>
                <div class="text-sm text-green-700 font-medium">รายรับวันนี้</div>
                <div class="text-2xl font-bold text-green-800">{formatCurrency(todayRevenue)}</div>
                <div class="text-xs text-green-600">Today's Sales</div>
              </div>
            </div>
          </div>
        </div>

        <!-- Monthly Profit -->
        <div class="card bg-gradient-to-br from-blue-50 to-blue-100 shadow-xl border border-blue-200">
          <div class="card-body p-4">
            <div class="flex items-center gap-4">
              <div class="text-4xl">{monthlyProfit >= 0 ? '📈' : '📉'}</div>
              <div>
                <div class="text-sm text-blue-700 font-medium">กำไรเดือนนี้</div>
                <div class="text-2xl font-bold {monthlyProfit >= 0 ? 'text-blue-800' : 'text-red-600'}">
                  {formatCurrency(monthlyProfit)}
                </div>
                <div class="text-xs text-blue-600">Margin: {profitMargin.toFixed(1)}%</div>
              </div>
            </div>
          </div>
        </div>

        <!-- Inventory Value -->
        <div class="card bg-gradient-to-br from-purple-50 to-purple-100 shadow-xl border border-purple-200">
          <div class="card-body p-4">
            <div class="flex items-center gap-4">
              <div class="text-4xl">📦</div>
              <div>
                <div class="text-sm text-purple-700 font-medium">มูลค่าสินค้าคงเหลือ</div>
                <div class="text-2xl font-bold text-purple-800">{formatCurrency(totalInventoryValue)}</div>
                <div class="text-xs text-purple-600">{totalItems} รายการ</div>
              </div>
            </div>
          </div>
        </div>

        <!-- Alerts Summary -->
        <div class="card bg-gradient-to-br from-orange-50 to-orange-100 shadow-xl border border-orange-200">
          <div class="card-body p-4">
            <div class="flex items-center gap-4">
              <div class="text-4xl">⚠️</div>
              <div>
                <div class="text-sm text-orange-700 font-medium">แจ้งเตือน</div>
                <div class="text-2xl font-bold text-orange-800">{lowStockCount + expiringCount}</div>
                <div class="text-xs text-orange-600">
                  Stock: {lowStockCount} | Expiring: {expiringCount}
                </div>
              </div>
            </div>
          </div>
        </div>
      {/if}
    </div>

    <!-- Main Content Grid -->
    <div class="grid grid-cols-1 lg:grid-cols-3 gap-6 mb-8">
      
      <!-- Left Column - Charts & Analytics -->
      <div class="lg:col-span-2 space-y-6">
        
        <!-- Top Recipes Performance -->
        <div class="card bg-base-100 shadow-xl">
          <div class="card-body">
            <h2 class="card-title">🏆 เมนูขายดีประจำเดือน</h2>
            {#if isLoading}
              <div class="space-y-3">
                {#each Array(3) as _}
                  <div class="flex justify-between items-center animate-pulse">
                    <div class="bg-base-300 h-4 w-32 rounded"></div>
                    <div class="bg-base-300 h-4 w-20 rounded"></div>
                  </div>
                {/each}
              </div>
            {:else if topRecipes.length === 0}
              <div class="text-center py-8 text-base-content/50">
                <div class="text-4xl mb-2">📊</div>
                <p>ยังไม่มีข้อมูลการขายในเดือนนี้</p>
              </div>
            {:else}
              <div class="space-y-3">
                {#each topRecipes as recipe, index}
                  <div class="flex items-center justify-between p-3 bg-base-50 rounded-lg">
                    <div class="flex items-center gap-3">
                      <div class="badge badge-primary">{index + 1}</div>
                      <div>
                        <div class="font-medium">{recipe.name}</div>
                        <div class="text-sm text-base-content/70">ขายได้ {recipe.sold} ออเดอร์</div>
                      </div>
                    </div>
                    <div class="text-right">
                      <div class="font-bold text-success">{formatCurrency(recipe.revenue)}</div>
                    </div>
                  </div>
                {/each}
              </div>
            {/if}
          </div>
        </div>

        <!-- Category Overview -->
        <div class="card bg-base-100 shadow-xl">
          <div class="card-body">
            <h2 class="card-title">📊 สินค้าคงเหลือตามหมวดหมู่</h2>
            {#if isLoading}
              <div class="overflow-x-auto animate-pulse">
                <table class="table">
                  <thead>
                    <tr>
                      <th class="bg-base-300 h-4 w-1/3 rounded"></th>
                      <th class="bg-base-300 h-4 w-1/4 rounded"></th>
                      <th class="bg-base-300 h-4 w-1/3 rounded"></th>
                    </tr>
                  </thead>
                  <tbody>
                    {#each Array(4) as _}
                      <tr>
                        <td class="bg-base-200 h-4 rounded"></td>
                        <td class="bg-base-200 h-4 rounded"></td>
                        <td class="bg-base-200 h-4 rounded"></td>
                      </tr>
                    {/each}
                  </tbody>
                </table>
              </div>
            {:else if categoryCounts.length === 0}
              <div class="text-center py-8 text-base-content/50">
                <div class="text-4xl mb-2">📦</div>
                <p>ยังไม่มีสินค้าในระบบ</p>
              </div>
            {:else}
              <div class="overflow-x-auto">
                <table class="table table-zebra">
                  <thead>
                    <tr>
                      <th>หมวดหมู่</th>
                      <th>จำนวนรายการ</th>
                      <th>มูลค่า</th>
                    </tr>
                  </thead>
                  <tbody>
                    {#each categoryCounts as category}
                      <tr class="hover">
                        <td>
                          <div class="font-medium">{category.category || 'ไม่ระบุหมวดหมู่'}</div>
                        </td>
                        <td>
                          <div class="badge badge-outline">{category.count}</div>
                        </td>
                        <td>
                          <div class="font-medium">{formatCurrency(category.value)}</div>
                        </td>
                      </tr>
                    {/each}
                  </tbody>
                </table>
              </div>
            {/if}
          </div>
        </div>
      </div>

      <!-- Right Column - Alerts & Activity -->
      <div class="space-y-6">
        
        <!-- Critical Alerts -->
        <div class="card bg-base-100 shadow-xl">
          <div class="card-body">
            <h2 class="card-title text-warning">🚨 แจ้งเตือนสำคัญ</h2>
            
            <!-- Low Stock Alerts -->
            {#if lowStockItems.length > 0}
              <div class="mb-4">
                <h3 class="font-semibold text-error mb-2">📉 สินค้าใกล้หมด</h3>
                <div class="space-y-2 max-h-40 overflow-y-auto">
                  {#each lowStockItems.slice(0, 5) as item}
                    <div class="alert alert-error alert-sm py-2">
                      <div class="text-sm">
                        <div class="font-medium">{item.name}</div>
                        <div class="text-xs opacity-80">เหลือ {item.quantity} {item.category}</div>
                      </div>
                    </div>
                  {/each}
                </div>
                {#if lowStockItems.length > 5}
                  <div class="text-xs text-center mt-2 opacity-60">
                    และอีก {lowStockItems.length - 5} รายการ
                  </div>
                {/if}
              </div>
            {/if}

            <!-- Expiring Items -->
            {#if expiringItems.length > 0}
              <div class="mb-4">
                <h3 class="font-semibold text-warning mb-2">⏰ สินค้าใกล้หมดอายุ</h3>
                <div class="space-y-2 max-h-40 overflow-y-auto">
                  {#each expiringItems.slice(0, 5) as item}
                    <div class="alert alert-warning alert-sm py-2">
                      <div class="text-sm">
                        <div class="font-medium">{item.name}</div>
                        <div class="text-xs opacity-80">
                          หมดอายุใน {item.days_until_expiry} วัน ({formatDate(item.expiration_date)})
                        </div>
                      </div>
                    </div>
                  {/each}
                </div>
                {#if expiringItems.length > 5}
                  <div class="text-xs text-center mt-2 opacity-60">
                    และอีก {expiringItems.length - 5} รายการ
                  </div>
                {/if}
              </div>
            {/if}

            {#if lowStockItems.length === 0 && expiringItems.length === 0}
              <div class="text-center py-6 text-success">
                <div class="text-4xl mb-2">✅</div>
                <p>ไม่มีแจ้งเตือนสำคัญ</p>
                <p class="text-sm opacity-70">ทุกอย่างเรียบร้อยดี</p>
              </div>
            {/if}
          </div>
        </div>

        <!-- Recent Activity -->
        <div class="card bg-base-100 shadow-xl">
          <div class="card-body">
            <h2 class="card-title">📋 กิจกรรมล่าสุด</h2>
            {#if isLoading}
              <div class="space-y-3">
                {#each Array(3) as _}
                  <div class="flex justify-between items-center animate-pulse">
                    <div class="bg-base-300 h-4 w-32 rounded"></div>
                    <div class="bg-base-300 h-4 w-16 rounded"></div>
                  </div>
                {/each}
              </div>
            {:else if recentTransactions.length === 0}
              <div class="text-center py-6 text-base-content/50">
                <div class="text-4xl mb-2">📝</div>
                <p>ยังไม่มีรายการทางการเงิน</p>
              </div>
            {:else}
              <div class="space-y-3">
                {#each recentTransactions as transaction}
                  <div class="flex items-center justify-between p-3 bg-base-50 rounded">
                    <div class="flex-1">
                      <div class="flex items-center gap-2">
                        <div class="badge {transaction.type === 'Income' ? 'badge-success' : 'badge-error'} badge-sm">
                          {transaction.type === 'Income' ? '💰' : '💸'}
                        </div>
                        <div class="text-sm font-medium truncate">
                          {transaction.description}
                        </div>
                      </div>
                      <div class="text-xs text-base-content/60 mt-1">
                        {formatDate(transaction.date)}
                      </div>
                    </div>
                    <div class="text-right">
                      <div class="font-medium {transaction.type === 'Income' ? 'text-success' : 'text-error'}">
                        {transaction.type === 'Income' ? '+' : '-'}{formatCurrency(Math.abs(transaction.amount))}
                      </div>
                    </div>
                  </div>
                {/each}
              </div>
            {/if}
          </div>
        </div>

        <!-- Quick Actions -->
        <!-- <div class="card bg-base-100 shadow-xl">
          <div class="card-body">
            <h2 class="card-title">⚡ การดำเนินการด่วน</h2>
            <div class="grid grid-cols-1 gap-2">
              <a href="/inventory" class="btn btn-outline btn-sm">
                📦 จัดการสินค้า
              </a>
              <a href="/recipe" class="btn btn-outline btn-sm">
                📜 จัดการสูตรอาหาร
              </a>
              <a href="/finance" class="btn btn-outline btn-sm">
                📊 บันทึกรายรับ-จ่าย
              </a>
              <button class="btn btn-primary btn-sm" on:click={loadDashboardData}>
                🔄 รีเฟรชข้อมูล
              </button>
            </div>
          </div>
        </div> -->
        </div>
    </div>

    <!-- Monthly Financial Summary -->
    <div class="card bg-base-100 shadow-xl">
      <div class="card-body">
        <h2 class="card-title">💹 สรุปทางการเงินรายเดือน</h2>
        <div class="grid grid-cols-1 md:grid-cols-3 gap-6">
          <div class="stat">
            <div class="stat-figure text-success">
              <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" class="inline-block w-8 h-8 stroke-current"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M13 16h-1v-4h-1m1-4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z"></path></svg>
            </div>
            <div class="stat-title">รายรับ</div>
            <div class="stat-value text-success">{formatCurrency(monthlyRevenue)}</div>
            <div class="stat-desc">จากการขายในเดือนนี้</div>
          </div>
          
          <div class="stat">
            <div class="stat-figure text-error">
              <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" class="inline-block w-8 h-8 stroke-current"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 6V4m0 2a2 2 0 100 4m0-4a2 2 0 110 4m-6 8a2 2 0 100-4m0 4a2 2 0 100 4m0-4v2m0-6V4m6 6v10m6-2a2 2 0 100-4m0 4a2 2 0 100 4m0-4v2m0-6V4"></path></svg>
            </div>
            <div class="stat-title">รายจ่าย</div>
            <div class="stat-value text-error">{formatCurrency(monthlyExpenses)}</div>
            <div class="stat-desc">ค่าใช้จ่ายทั้งหมด</div>
          </div>
          
          <div class="stat">
            <div class="stat-figure {monthlyProfit >= 0 ? 'text-success' : 'text-error'}">
              <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" class="inline-block w-8 h-8 stroke-current"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 19v-6a2 2 0 00-2-2H5a2 2 0 00-2 2v6a2 2 0 002 2h2a2 2 0 002-2zm0 0V9a2 2 0 012-2h2a2 2 0 012 2v10m-6 0a2 2 0 002 2h2a2 2 0 002-2m0 0V5a2 2 0 012-2h2a2 2 0 012 2v14a2 2 0 01-2 2h-2a2 2 0 01-2-2z"></path></svg>
            </div>
            <div class="stat-title">กำไรสุทธิ</div>
            <div class="stat-value {monthlyProfit >= 0 ? 'text-success' : 'text-error'}">{formatCurrency(monthlyProfit)}</div>
            <div class="stat-desc {monthlyProfit >= 0 ? 'text-success' : 'text-error'}">
              อัตรากำไร {profitMargin.toFixed(1)}%
            </div>
          </div>
        </div>
      </div>
    </div>
  </div>
</div>
