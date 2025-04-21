<script lang="ts">
    // Mock finance data
    const financeStats = [
      { title: "Monthly Budget", value: "$450", desc: "For March 2025", icon: "💵" },
      { title: "Spent This Month", value: "$312", desc: "69% of budget", icon: "📊" },
      { title: "Savings", value: "$138", desc: "31% remaining", icon: "💰" },
      { title: "Avg. Cost Per Meal", value: "$3.25", desc: "-12% from last month", icon: "🍽️" },
    ];
  
    // Mock expense data
    const expenses = [
      { id: 1, date: "2025-03-18", category: "Dairy", amount: 24.35, store: "Whole Foods", items: 5 },
      { id: 2, date: "2025-03-15", category: "Meat", amount: 45.67, store: "Costco", items: 3 },
      { id: 3, date: "2025-03-12", category: "Produce", amount: 18.22, store: "Farmer's Market", items: 8 },
      { id: 4, date: "2025-03-10", category: "Bakery", amount: 12.45, store: "Local Bakery", items: 2 },
      { id: 5, date: "2025-03-05", category: "Snacks", amount: 15.89, store: "Trader Joe's", items: 7 },
      { id: 6, date: "2025-03-03", category: "Beverages", amount: 22.50, store: "Whole Foods", items: 4 },
      { id: 7, date: "2025-02-28", category: "Frozen", amount: 35.20, store: "Costco", items: 6 },
      { id: 8, date: "2025-02-25", category: "Pantry", amount: 28.75, store: "Walmart", items: 9 },
    ];
  
    // Category spending data for chart
    const categorySpending = [
      { category: "Dairy", amount: 56.45 },
      { category: "Meat", amount: 112.89 },
      { category: "Produce", amount: 67.31 },
      { category: "Bakery", amount: 32.45 },
      { category: "Snacks", amount: 28.75 },
      { category: "Beverages", amount: 42.50 },
      { category: "Frozen", amount: 48.75 },
      { category: "Pantry", amount: 53.80 },
    ];
  
    // For filtering
    let searchQuery = "";
    let selectedCategory = "All";
    let selectedStore = "All";
    
    // Available categories and stores
    const categories = ["All", "Dairy", "Meat", "Produce", "Bakery", "Snacks", "Beverages", "Frozen", "Pantry"];
    const stores = ["All", "Whole Foods", "Costco", "Farmer's Market", "Local Bakery", "Trader Joe's", "Walmart"];
  
    // Filtered items based on search query and filters
    $: filteredExpenses = expenses.filter(item => {
      const matchesSearch = searchQuery === "" || 
        item.category.toLowerCase().includes(searchQuery.toLowerCase()) ||
        item.store.toLowerCase().includes(searchQuery.toLowerCase());
      
      const matchesCategory = selectedCategory === "All" || item.category === selectedCategory;
      const matchesStore = selectedStore === "All" || item.store === selectedStore;
      
      return matchesSearch && matchesCategory && matchesStore;
    });
  
    // Calculate totals
    $: totalSpent = filteredExpenses.reduce((sum, expense) => sum + expense.amount, 0).toFixed(2);
    $: totalItems = filteredExpenses.reduce((sum, expense) => sum + expense.items, 0);
  </script>
  
  <div class="fade-in container mx-auto px-4 py-8 w-full min-h-screen p-0">
    <div class="max-w-7xl ms-auto mt-10">
    <div class="flex justify-between items-center mb-8">
      <h1 class="text-3xl font-bold">Finance Analytics</h1>
      <button class="btn btn-secondary">Add Expense</button>
    </div>
    
    <!-- Stats Overview -->
    <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-6 mb-8">
      {#each financeStats as stat}
        <div class="card bg-base-100 shadow-xl">
          <div class="card-body p-4">
            <div class="flex items-center gap-4">
              <div class="text-4xl">{stat.icon}</div>
              <div>
                <div class="stat-title">{stat.title}</div>
                <div class="stat-value">{stat.value}</div>
                <div class="stat-desc">{stat.desc}</div>
              </div>
            </div>
          </div>
        </div>
      {/each}
    </div>
    
    <div class="grid grid-cols-1 lg:grid-cols-3 gap-6 mb-8">
      <!-- Spending by Category Chart -->
      <div class="card bg-base-100 shadow-xl col-span-2">
        <div class="card-body">
          <h2 class="card-title">Spending by Category</h2>
          <div class="h-64 mt-4">
            <!-- In a real app, you'd use a chart library like Chart.js here -->
            <div class="space-y-2">
              {#each categorySpending as item}
                <div class="flex items-center">
                  <div class="w-24">{item.category}</div>
                  <div class="flex-grow">
                    <div class="h-6 bg-base-300 rounded-full overflow-hidden">
                      <div 
                        class="h-full rounded-full bg-primary" 
                        style="width: {(item.amount / Math.max(...categorySpending.map(c => c.amount)) * 100)}%">
                      </div>
                    </div>
                  </div>
                  <div class="w-16 text-right">${item.amount}</div>
                </div>
              {/each}
            </div>
          </div>
        </div>
      </div>
      
      <!-- Budget Overview -->
      <div class="card bg-base-100 shadow-xl">
        <div class="card-body">
          <h2 class="card-title">Budget Overview</h2>
          <div class="flex flex-col items-center justify-center h-64">
            <!-- Simple donut chart representation -->
            <div class="relative w-40 h-40">
              <div class="absolute inset-0 flex items-center justify-center">
                <span class="text-2xl font-bold">69%</span>
              </div>
              <svg viewBox="0 0 36 36" class="circular-chart">
                <path class="circle-bg"
                  d="M18 2.0845
                    a 15.9155 15.9155 0 0 1 0 31.831
                    a 15.9155 15.9155 0 0 1 0 -31.831"
                  fill="none"
                  stroke="#eee"
                  stroke-width="3"
                />
                <path class="circle"
                  d="M18 2.0845
                    a 15.9155 15.9155 0 0 1 0 31.831
                    a 15.9155 15.9155 0 0 1 0 -31.831"
                  fill="none"
                  stroke="#4338ca"
                  stroke-width="3"
                  stroke-dasharray="69, 100"
                  stroke-linecap="round"
                />
              </svg>
            </div>
            <div class="mt-4 text-center">
              <p class="text-lg">$312 of $450</p>
              <p class="text-sm opacity-75">Monthly budget usage</p>
            </div>
          </div>
        </div>
      </div>
    </div>
    
    <!-- Filters and Search -->
    <div class="bg-base-100 shadow-xl rounded-lg p-4 mb-8">
      <div class="flex flex-col md:flex-row gap-4">
        <div class="form-control flex-grow">
          <div class="input-group">
            <input 
              type="text" 
              placeholder="Search expenses..." 
              class="input input-bordered w-full" 
              bind:value={searchQuery}
            />
          </div>
        </div>
        
        <div class="form-control">
          <select class="select select-bordered" bind:value={selectedCategory}>
            {#each categories as category}
              <option value={category}>{category}</option>
            {/each}
          </select>
        </div>
        
        <div class="form-control">
          <select class="select select-bordered" bind:value={selectedStore}>
            {#each stores as store}
              <option value={store}>{store}</option>
            {/each}
          </select>
        </div>
      </div>
    </div>
    
    <!-- Expenses Table -->
    <div class="card bg-base-100 shadow-xl">
      <div class="card-body">
        <h2 class="card-title mb-4">Recent Expenses</h2>
        <div class="overflow-x-auto">
          <table class="table table-zebra w-full">
            <thead>
              <tr>
                <th>Date</th>
                <th>Category</th>
                <th>Store</th>
                <th>Items</th>
                <th>Amount</th>
                <th>Actions</th>
              </tr>
            </thead>
            <tbody>
              {#if filteredExpenses.length === 0}
                <tr>
                  <td colspan="6" class="text-center py-4">No expenses match your search</td>
                </tr>
              {:else}
                {#each filteredExpenses as expense}
                  <tr>
                    <td>{expense.date}</td>
                    <td>{expense.category}</td>
                    <td>{expense.store}</td>
                    <td>{expense.items}</td>
                    <td>${expense.amount.toFixed(2)}</td>
                    <td>
                      <div class="flex gap-2">
                        <button class="btn btn-xs btn-ghost">View</button>
                        <button class="btn btn-xs btn-error btn-ghost">Delete</button>
                      </div>
                    </td>
                  </tr>
                {/each}
              {/if}
            </tbody>
            <tfoot>
              <tr>
                <th colspan="3">Totals</th>
                <th>{totalItems}</th>
                <th>${totalSpent}</th>
                <th></th>
              </tr>
            </tfoot>
          </table>
        </div>
      </div>
    </div>
    
    <!-- Summary & Pagination -->
    <div class="flex flex-col md:flex-row justify-between items-center mt-4">
      <div class="text-sm mb-4 md:mb-0">
        Showing {filteredExpenses.length} of {expenses.length} expenses
      </div>
      
      <div class="join">
        <input
          class="join-item btn btn-square"
          type="radio"
          name="options"
          aria-label="1"
          checked/>
        <input class="join-item btn btn-square" type="radio" name="options" aria-label="2" />
        <input class="join-item btn btn-square" type="radio" name="options" aria-label="3" />
        <input class="join-item btn btn-square" type="radio" name="options" aria-label="4" />
      </div>
    </div>
  
    <style>
      .circular-chart {
        width: 100%;
        height: 100%;
        transform: rotate(-90deg);
      }
      .circle-bg {
        stroke: #eee;
      }
      .circle {
        animation: progress 1s ease-out forwards;
      }
      @keyframes progress {
        0% {
          stroke-dasharray: 0 100;
        }
      }
    </style>
  </div>
  </div>