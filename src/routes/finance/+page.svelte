<script lang="ts">
    import { onMount } from 'svelte';
    import Database from '@tauri-apps/plugin-sql'; // Use SQL plugin

    // Define interfaces for data structures
    interface Expense {
      id: number;
      date: string; // Consider using Date type if appropriate
      category: string;
      amount: number;
      store: string;
      items: number; // Assuming this field exists in your DB table
      // Add other relevant fields from your expenses table
    }

    // Reactive variables for real data
    let financeStats: { title: string; value: string; desc: string; icon: string }[] = [];
    let expenses: Expense[] = [];
    let categorySpending: { category: string; amount: number }[] = [];
    let db: Database | null = null;
    let error: string | null = null;
    let isLoading = true;

    // Function to fetch data using SQL plugin
    async function loadData() {
      isLoading = true;
      error = null;
      try {
        db = await Database.load("sqlite:inventory.db"); // Assuming finance data is in the same DB

        // --- Fetch Expenses ---
        // Replace 'expenses_table' and column names with your actual table structure
        // Example: SELECT id, date, category, amount, store, items FROM expenses_table ORDER BY date DESC
        const fetchedExpenses = await db.select<Expense[]>("SELECT id, date, category, amount, store, items FROM expenses ORDER BY date DESC"); // Adjust query as needed
        expenses = fetchedExpenses;

        // --- Calculate Stats & Category Spending ---
        // Use the fetched expenses data for calculations
        financeStats = calculateFinanceStats(expenses);
        categorySpending = calculateCategorySpending(expenses);

      } catch (err) {
        console.error("Error loading finance data:", err);
        error = `Failed to load finance data: ${err}`;
        // Initialize with empty/default states on error
        expenses = [];
        financeStats = [
          { title: "Monthly Budget", value: "Error", desc: "N/A", icon: "💵" },
          { title: "Spent This Month", value: "Error", desc: "N/A", icon: "📊" },
          { title: "Savings", value: "Error", desc: "N/A", icon: "💰" },
          { title: "Avg. Cost Per Meal", value: "Error", desc: "N/A", icon: "🍽️" },
        ];
        categorySpending = [];
      } finally {
        isLoading = false;
      }
    }

    // Placeholder functions for calculations (implement these based on your needs)
    function calculateFinanceStats(data: Expense[]) {
        if (!data || data.length === 0) {
             return [
                { title: "Monthly Budget", value: "$0", desc: "No data", icon: "💵" },
                { title: "Spent This Month", value: "$0", desc: "No data", icon: "📊" },
                { title: "Savings", value: "$0", desc: "No data", icon: "💰" },
                { title: "Avg. Cost Per Meal", value: "$0", desc: "No data", icon: "🍽️" },
            ];
        }
      // --- Example Calculation Logic ---
      const monthlyBudget = 450; // Example budget, replace with dynamic value if needed
      const totalSpent = data.reduce((sum, expense) => sum + expense.amount, 0);
      const savings = monthlyBudget - totalSpent;
      const avgCostPerMeal = data.length > 0 ? (totalSpent / data.reduce((sum, exp) => sum + (exp.items || 1), 0)) : 0; // Avoid division by zero, assumes 'items' represents meals/items contributing to cost

      return [
          { title: "Monthly Budget", value: `$${monthlyBudget.toFixed(2)}`, desc: "For Current Month", icon: "💵" },
          { title: "Spent This Month", value: `$${totalSpent.toFixed(2)}`, desc: `${((totalSpent / monthlyBudget) * 100).toFixed(0)}% of budget`, icon: "📊" },
          { title: "Savings", value: `$${savings.toFixed(2)}`, desc: `${((savings / monthlyBudget) * 100).toFixed(0)}% remaining`, icon: "💰" },
          { title: "Avg. Cost Per Meal", value: `$${avgCostPerMeal.toFixed(2)}`, desc: "Based on expenses", icon: "🍽️" },
        ];
    }

     function calculateCategorySpending(data: Expense[]) {
       const spendingMap = new Map<string, number>();
       data.forEach(expense => {
         spendingMap.set(expense.category, (spendingMap.get(expense.category) || 0) + expense.amount);
       });
       // Sort categories by amount descending for chart display
       return Array.from(spendingMap, ([category, amount]) => ({ category, amount }))
                   .sort((a, b) => b.amount - a.amount);
     }

    onMount(loadData); // Load data when the component mounts

    // For filtering
    let searchQuery = "";
    let selectedCategory = "All";
    let selectedStore = "All";

    // Dynamically generate categories and stores from loaded expenses
    $: availableCategories = ["All", ...new Set(expenses.map(e => e.category))];
    $: availableStores = ["All", ...new Set(expenses.map(e => e.store))];

    // Filtered items based on search query and filters
    $: filteredExpenses = expenses.filter(item => {
      const matchesSearch = searchQuery === "" ||
        item.category.toLowerCase().includes(searchQuery.toLowerCase()) ||
        item.store.toLowerCase().includes(searchQuery.toLowerCase()) ||
        item.date.includes(searchQuery); // Allow searching by date

      const matchesCategory = selectedCategory === "All" || item.category === selectedCategory;
      const matchesStore = selectedStore === "All" || item.store === selectedStore;

      return matchesSearch && matchesCategory && matchesStore;
    });

    // Calculate totals based on filtered expenses
    $: totalSpent = filteredExpenses.reduce((sum, expense) => sum + expense.amount, 0).toFixed(2);
    $: totalItems = filteredExpenses.reduce((sum, expense) => sum + expense.items, 0);

    // --- Add Expense Functionality ---
    let showAddExpenseModal = false;
    function openAddExpenseModal() {
        showAddExpenseModal = true;
        // Potentially pass existing expense data if editing
    }
    function closeAddExpenseModal() {
        showAddExpenseModal = false;
    }
    async function handleSaveExpense(event: CustomEvent<Expense>) { // Assuming modal returns a full Expense object
        const newExpense = event.detail;
        isLoading = true;
        error = null;
        try {
            if (!db) {
                throw new Error("Database not initialized");
            }
            // Replace with your actual INSERT statement and table name
            await db.execute(
                "INSERT INTO expenses (date, category, amount, store, items) VALUES ($1, $2, $3, $4, $5)",
                [newExpense.date, newExpense.category, newExpense.amount, newExpense.store, newExpense.items]
            );
            // Refresh data after adding
            await loadData();
            closeAddExpenseModal();
        } catch (err) {
            console.error("Error saving expense:", err);
            error = `Failed to save expense: ${err}`;
            isLoading = false; // Keep modal open or show error within modal?
        }
        // No finally isLoading = false here, loadData handles it
    }

    // --- Delete Expense Functionality ---
    async function deleteExpense(id: number) {
        if (!db) {
            error = "Database not initialized.";
            return;
        }
        if (!confirm(`Are you sure you want to delete expense ID ${id}?`)) {
            return;
        }
        isLoading = true;
        error = null;
        try {
            // Replace with your actual DELETE statement and table name
            await db.execute("DELETE FROM expenses WHERE id = $1", [id]);
            // Refresh data after deleting
            await loadData();
        } catch (err) {
            console.error(`Error deleting expense with id: ${id}:`, err);
            error = `Failed to delete expense: ${err}`;
        } finally {
            isLoading = false; // Ensure loading state is reset even on error
        }
    }

    // --- Pagination ---
    let currentPage = 1;
    const itemsPerPage = 10; // Or make this configurable
    $: totalPages = Math.ceil(filteredExpenses.length / itemsPerPage);
    $: paginatedExpenses = filteredExpenses.slice(
      (currentPage - 1) * itemsPerPage,
      currentPage * itemsPerPage
    );
    function changePage(page: number) {
      if (page >= 1 && page <= totalPages) {
        currentPage = page;
      }
    }

    import AddEditExpenseModal from '../../components/AddEditExpenseModal.svelte'; // Import the modal

</script>

<!-- Import Add/Edit Modal Component -->
<!-- Assuming you have a component like AddEditExpenseModal.svelte -->
<!-- import AddEditExpenseModal from '../../components/AddEditExpenseModal.svelte'; -->

<div class="fade-in container mx-auto px-4 py-8 w-full min-h-screen p-0">
  <div class="max-w-7xl ms-auto mt-10">
    <div class="flex justify-between items-center mb-8">
      <h1 class="text-3xl font-bold">Finance Analytics</h1>
      <!-- Update button to open your modal -->
      <button class="btn btn-secondary" on:click={openAddExpenseModal}>Add Expense</button>
    </div>

    {#if isLoading}
      <div class="text-center py-10">Loading finance data...</div>
    {:else if error}
       <div class="alert alert-error shadow-lg mb-8">
         <div>
           <span>{error}</span>
         </div>
       </div>
    {/if}

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
          <div class="h-64 mt-4 overflow-y-auto"> <!-- Added overflow -->
            {#if categorySpending.length === 0}
              <p class="text-center text-gray-500">No spending data available.</p>
            {:else}
              {@const maxAmount = Math.max(...categorySpending.map(c => c.amount), 1)} <!-- Avoid division by zero -->
              <div class="space-y-2">
                {#each categorySpending as item}
                  <div class="flex items-center">
                    <div class="w-24 truncate" title={item.category}>{item.category}</div> <!-- Added truncate -->
                    <div class="flex-grow">
                      <div class="h-6 bg-base-300 rounded-full overflow-hidden">
                        <!-- Added transition -->
                        <div
                          class="h-full rounded-full bg-primary transition-all duration-500"
                          style="width: {((item.amount / maxAmount) * 100)}%">
                        </div>
                      </div>
                    </div>
                    <div class="w-20 text-right">${item.amount.toFixed(2)}</div> <!-- Increased width -->
                  </div>
                {/each}
              </div>
            {/if}
          </div>
        </div>
      </div>

      <!-- Budget Overview (Example, needs real budget data) -->
      <div class="card bg-base-100 shadow-xl">
        <div class="card-body">
          <h2 class="card-title">Budget Overview</h2>
          {#if !isLoading && !error && financeStats.length > 0} <!-- Ensure data is loaded before calculating -->
            {@const budget = 450} <!-- Example Budget -->
            {@const spentStat = financeStats.find(s => s.title === "Spent This Month")}
            {@const spent = spentStat && spentStat.value !== 'Error' ? parseFloat(spentStat.value.replace('$', '')) : 0}
            {@const percentage = budget > 0 ? Math.min(Math.round((spent / budget) * 100), 100) : 0}
            <div class="flex flex-col items-center justify-center h-64">
              <!-- Moved const declarations to be direct children of #if -->
                <div class="relative w-40 h-40">
                  <div class="absolute inset-0 flex items-center justify-center">
                <span class="text-2xl font-bold">{percentage}%</span>
              </div>
              <svg viewBox="0 0 36 36" class="circular-chart">
                <path class="circle-bg"
                  d="M18 2.0845 a 15.9155 15.9155 0 0 1 0 31.831 a 15.9155 15.9155 0 0 1 0 -31.831"
                   /> <!-- Corrected closing tag -->
                <path class="circle"
                  d="M18 2.0845 a 15.9155 15.9155 0 0 1 0 31.831 a 15.9155 15.9155 0 0 1 0 -31.831"
                  stroke-dasharray="{percentage}, 100" />
              </svg>
            </div>
            <div class="mt-4 text-center">
              <p class="text-lg">${spent.toFixed(2)} of ${budget.toFixed(2)}</p>
              <p class="text-sm opacity-75">Monthly budget usage</p>
            </div>
          </div>
          {:else}
             <!-- Show loading or placeholder state for budget overview -->
             <div class="flex flex-col items-center justify-center h-64">
               <p class="text-gray-500">Loading budget data...</p>
             </div>
          {/if} <!-- Close #if !isLoading -->
        </div>
      </div>
    </div> <!-- Close grid grid-cols-1 lg:grid-cols-3 -->

    <!-- Filters and Search -->
    <div class="bg-base-100 shadow-xl rounded-lg p-4 mb-8">
      <div class="flex flex-col md:flex-row gap-4">
        <div class="form-control flex-grow">
          <div class="input-group">
            <input
              type="text"
              placeholder="Search by category, store, date..."
              class="input input-bordered w-full"
              bind:value={searchQuery}
            />
          </div>
        </div>

        <div class="form-control">
          <select class="select select-bordered" bind:value={selectedCategory}>
            {#each availableCategories as category}
              <option value={category}>{category}</option>
            {/each}
          </select>
        </div>

        <div class="form-control">
          <select class="select select-bordered" bind:value={selectedStore}>
            {#each availableStores as store}
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
              {#if paginatedExpenses.length === 0}
                <tr>
                  <td colspan="6" class="text-center py-4">
                    {#if expenses.length === 0}
                        No expenses recorded yet.
                    {:else}
                        No expenses match your current filters.
                    {/if}
                  </td>
                </tr>
              {:else}
                {#each paginatedExpenses as expense (expense.id)}
                  <tr class="hover"> <!-- Added hover effect -->
                    <td>{expense.date}</td>
                    <td>{expense.category}</td>
                    <td>{expense.store}</td>
                    <td>{expense.items}</td>
                    <td>${expense.amount.toFixed(2)}</td>
                    <td>
                      <div class="flex gap-2">
                        <!-- Add Edit button functionality later -->
                        <button class="btn btn-xs btn-ghost btn-info">Edit</button>
                        <button class="btn btn-xs btn-error btn-ghost" on:click={() => deleteExpense(expense.id)}>Delete</button>
                      </div>
                    </td>
                  </tr>
                {/each}
              {/if}
            </tbody>
            {#if filteredExpenses.length > 0}
            <tfoot>
              <tr>
                <th colspan="3">Filtered Totals</th>
                <th>{totalItems}</th>
                <th>${totalSpent}</th>
                <th></th>
              </tr>
            </tfoot>
            {/if}
          </table>
        </div>
      </div>
    </div>

    <!-- Summary & Pagination -->
    <div class="flex flex-col md:flex-row justify-between items-center mt-4">
      <div class="text-sm mb-4 md:mb-0">
         Showing {paginatedExpenses.length > 0 ? (currentPage - 1) * itemsPerPage + 1 : 0}
          to {Math.min(currentPage * itemsPerPage, filteredExpenses.length)}
          of {filteredExpenses.length} expenses (Total: {expenses.length})
      </div>

      {#if totalPages > 1}
        <div class="join">
          <button
            class="join-item btn btn-sm"
            on:click={() => changePage(currentPage - 1)}
            disabled={currentPage === 1}>
            «
          </button>
          {#each Array(totalPages) as _, i}
             <!-- Consider showing fewer page numbers for many pages -->
            <button
              class="join-item btn btn-sm {currentPage === i + 1 ? 'btn-active' : ''}"
              on:click={() => changePage(i + 1)}>
              {i + 1}
            </button>
          {/each}
          <button
            class="join-item btn btn-sm"
            on:click={() => changePage(currentPage + 1)}
            disabled={currentPage === totalPages}>
            »
          </button>
        </div>
      {/if}
    </div>

  </div> <!-- Close max-w-7xl -->

  <!-- Modal Placeholder -->
  {#if showAddExpenseModal}
    <!-- Replace with your actual modal component instance -->
    <!-- Example: <AddEditExpenseModal showModal={showAddExpenseModal} on:close={closeAddExpenseModal} on:save={handleSaveExpense} /> -->
     <div class="modal modal-open">
       <div class="modal-box">
         <h3 class="font-bold text-lg">Add Expense (Placeholder)</h3>
         <p class="py-4">Replace this with your AddEditExpenseModal component.</p>
         <div class="modal-action">
           <button class="btn" on:click={closeAddExpenseModal}>Close</button>
         </div>
       </div>
     </div>
  {/if}

</div> <!-- Close fade-in container -->

<style>
  .circular-chart {
    width: 100%;
    height: 100%;
    transform: rotate(-90deg);
  }
  .circle-bg {
    fill: none;
    stroke: #eee; /* Background circle color */
    stroke-width: 3.8;
  }
  .circle {
    fill: none;
    stroke: #4338ca; /* Progress circle color - Indigo */
    stroke-width: 2.8;
    stroke-linecap: round;
    animation: progress 1s ease-out forwards;
  }

  @keyframes progress {
    0% {
      stroke-dasharray: 0 100;
    }
  }

  /* Add other styles as needed */

</style>
