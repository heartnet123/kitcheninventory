<script lang="ts">
    import { onMount } from 'svelte';
    import Database from '@tauri-apps/plugin-sql';
    import AddEditExpenseModal from '../../components/AddEditExpenseModal.svelte';
    import { formatDate } from '../../lib/utils';

    interface Expense {
      id: number;
      record_date: string; // Changed from date
      amount: number;
      quantity: number;    // Was items
      record_type: 'Expense' | 'Income';
      description: string; // Added
      recipe_id?: number;   // Added (optional)
    }

    let financeStats: { title: string; value: string; desc: string; icon: string }[] = [];
    let expenses: Expense[] = [];
    let db: Database | null = null;
    let error: string | null = null;
    let isLoading = true;
    let recipes: Array<{ id: number; name: string; selling_price: number }> = [];

    async function loadData() {
      isLoading = true;
      error = null;
      try {
        db = await Database.load("sqlite:inventory.db");
        const fetchedExpenses = await db.select<Expense[]>("SELECT id, record_date, amount, quantity, record_type, description, recipe_id FROM financial_records ORDER BY record_date DESC");
        expenses = fetchedExpenses;

        const fetchedRecipes = await db.select<Array<{ id: number; name: string; selling_price: number }>>("SELECT id, name, selling_price FROM recipes");
        recipes = fetchedRecipes;

        financeStats = calculateFinanceStats(expenses);
      } catch (err) {
        error = `ไม่สามารถโหลดข้อมูลทางการเงินและสูตรอาหารได้: ${err}`;
        expenses = [];
        financeStats = [
          { title: "งบเริ่มต้น", value: "Error", desc: "N/A", icon: "💵" },
          { title: "ค่าใช้จ่ายทั้งหมด", value: "Error", desc: "N/A", icon: "📊" },
          { title: "เงินออม", value: "Error", desc: "N/A", icon: "💰" },
          { title: "ค่าอาหารเฉลี่ยต่อมื้อ", value: "Error", desc: "N/A", icon: "🍽️" },
        ];
      } finally {
        isLoading = false;
      }
    }

    function calculateFinanceStats(data: Expense[]) {
        const monthlyBudget = 5000;
        if (!data || data.length === 0) {
             return [
                { title: "งบประมาณ", value: `${monthlyBudget} บาท`, desc: "", icon: "💵" },
                { title: "ค่าใช้จ่ายทั้งหมด", value: "0 บาท", desc: "0% ของงบประมาณ", icon: "📊" },
                { title: "เงินออม", value: `${monthlyBudget} บาท`, desc: "100% คงเหลือ", icon: "💰" },
                { title: "ค่าอาหารเฉลี่ยต่อมื้อ", value: "0 บาท", desc: "จากค่าใช้จ่าย", icon: "🍽️" },
            ];
        }

      const totalSpent = data
        .filter(expense => expense.record_type === 'Expense')
        .reduce((sum, expense) => sum + expense.amount, 0);
      const savings = monthlyBudget - totalSpent;
      const expenseQuantityCount = data 
        .filter(expense => expense.record_type === 'Expense')
        .reduce((sum, exp) => sum + (exp.quantity || 0), 0);
      const avgCostPerMeal = expenseQuantityCount > 0 ? (totalSpent / expenseQuantityCount) : 0;

      return [
          { title: "งบประมาณเริ่มต้น", value: `${monthlyBudget} บาท`, desc: "", icon: "💵" },
          { title: "ค่าใช้จ่ายทั้งหมด", value: `${totalSpent} บาท`, desc: `${monthlyBudget > 0 ? ((totalSpent / monthlyBudget) * 100).toFixed(0) : 0}% ของงบประมาณ`, icon: "📊" },
          { title: "เงินออม", value: `${savings} บาท`, desc: `${monthlyBudget > 0 ? ((savings / monthlyBudget) * 100).toFixed(0) : 0}% คงเหลือ`, icon: "💰" },
          { title: "ค่าอาหารเฉลี่ยต่อมื้อ", value: `${avgCostPerMeal} บาท`, desc: "จากค่าใช้จ่าย", icon: "🍽️" },
        ];
    }

    onMount(loadData);


    let searchQuery = "";
    $: filteredExpenses = expenses.filter(item => {
      const matchesSearch = searchQuery === "" ||
        (item.description && item.description.toLowerCase().includes(searchQuery.toLowerCase())) || // Search in description
        (item.record_type && item.record_type.toLowerCase().includes(searchQuery.toLowerCase())) ||
        (item.record_date && item.record_date.includes(searchQuery)); // Search in record_date

      return matchesSearch;
    });

    $: totalSpentFiltered = filteredExpenses.reduce((sum, expense) => sum + (expense.amount || 0), 0);
    $: totalItemsFiltered = filteredExpenses.reduce((sum, expense) => sum + (expense.quantity || 0), 0); // Changed from items to quantity

    let showAddExpenseModal = false;
    let expenseToEdit: Expense | null = null;

    function openAddExpenseModal(expense: Expense | null = null) {
        expenseToEdit = expense;
        showAddExpenseModal = true;
    }
    function closeAddExpenseModal() {
        showAddExpenseModal = false;
        expenseToEdit = null;
    }
    async function adjustInventoryForRecipe(recipeId: number, quantitySold: number, operation: 'decrease' | 'increase') {
        if (!db) throw new Error("Database not initialized for inventory adjustment");

        const recipeItems = await db.select<Array<{ item_id: number; quantity: number }>>(
            "SELECT item_id, quantity FROM recipe_items WHERE recipe_id = $1",
            [recipeId]
        );

        for (const recipeItem of recipeItems) {
            const adjustmentQuantity = recipeItem.quantity * quantitySold;
            if (operation === 'decrease') {
                await db.execute(
                    "UPDATE inventory SET quantity = quantity - $1 WHERE id = $2",
                    [adjustmentQuantity, recipeItem.item_id]
                );
            } else { // increase
                await db.execute(
                    "UPDATE inventory SET quantity = quantity + $1 WHERE id = $2",
                    [adjustmentQuantity, recipeItem.item_id]
                );
            }
        }
    }

    async function handleSaveExpense(event: CustomEvent<Expense>) {
        const expenseData = event.detail;
        isLoading = true;
        error = null;
        try {
            if (!db) {
                throw new Error("Database not initialized");
            }

            let oldExpenseData: Expense | undefined = undefined;
            if (expenseData.id) {
                oldExpenseData = await db.select<Expense[]>("SELECT * FROM financial_records WHERE id = $1", [expenseData.id]).then(res => res[0]);
            }

            if (expenseData.id) {
                 await db.execute(
                   "UPDATE financial_records SET record_date = $1, amount = $2, quantity = $3, record_type = $4, description = $5, recipe_id = $6 WHERE id = $7",
                   [expenseData.record_date, expenseData.amount, expenseData.quantity, expenseData.record_type, expenseData.description, expenseData.recipe_id, expenseData.id]
                 );
                // Revert old inventory adjustment if recipe was changed or removed
                if (oldExpenseData && oldExpenseData.recipe_id && oldExpenseData.record_type === 'Income') {
                    if (oldExpenseData.recipe_id !== expenseData.recipe_id || expenseData.record_type !== 'Income') {
                         await adjustInventoryForRecipe(oldExpenseData.recipe_id, oldExpenseData.quantity, 'increase');
                    } else if (oldExpenseData.recipe_id === expenseData.recipe_id && oldExpenseData.quantity !== expenseData.quantity) {
                        // Adjust for quantity change
                        const quantityDifference = expenseData.quantity - oldExpenseData.quantity;
                        if (quantityDifference !== 0) {
                             await adjustInventoryForRecipe(expenseData.recipe_id, Math.abs(quantityDifference), quantityDifference > 0 ? 'decrease' : 'increase');
                        }
                    }
                }
            } else { // New expense
                await db.execute(
                    "INSERT INTO financial_records (record_date, amount, quantity, record_type, description, recipe_id) VALUES ($1, $2, $3, $4, $5, $6)",
                    [expenseData.record_date, expenseData.amount, expenseData.quantity, expenseData.record_type, expenseData.description, expenseData.recipe_id]
                );
            }

            // Adjust inventory for new/updated income record with recipe
            if (expenseData.record_type === 'Income' && expenseData.recipe_id) {
                // If it's an update and recipe/quantity didn't change in a way that was already handled
                if (!(oldExpenseData && oldExpenseData.recipe_id === expenseData.recipe_id && oldExpenseData.quantity === expenseData.quantity && oldExpenseData.record_type === 'Income')) {
                    await adjustInventoryForRecipe(expenseData.recipe_id, expenseData.quantity, 'decrease');
                }
            }
            
            await loadData();
            closeAddExpenseModal();
        } catch (err) {
            error = `ไม่สามารถบันทึกรายการได้: ${err}`;
        }
    }

    function editExpense(expense: Expense) {
        openAddExpenseModal(expense);
    }

    async function deleteExpense(id: number) {
        if (!db) {
            error = "Database not initialized.";
            return;
        }
        isLoading = true;
        error = null;
        try {
            const expenseToDelete = await db.select<Expense[]>("SELECT * FROM financial_records WHERE id = $1", [id]).then(res => res[0]);

            await db.execute("DELETE FROM financial_records WHERE id = $1", [id]);

            if (expenseToDelete && expenseToDelete.record_type === 'Income' && expenseToDelete.recipe_id) {
                await adjustInventoryForRecipe(expenseToDelete.recipe_id, expenseToDelete.quantity, 'increase');
            }

            await loadData();
        } catch (err) {
            error = `ไม่สามารถลบรายการได้: ${err}`;
        }
    }

    let currentPage = 1;
    const itemsPerPage = 10;
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

</script>

<div class="fade-in container mx-auto px-4 py-8 w-full min-h-screen p-0">
  <div class="max-w-7xl ms-auto mt-10">
    <div class="flex justify-between items-center mb-8">
      <h1 class="text-3xl font-bold">การเงิน</h1>
      <button class="btn btn-secondary" on:click={() => openAddExpenseModal()}>เพิ่มรายการ</button>
    </div>

    {#if isLoading}
      <div class="text-center py-10">กำลังโหลดข้อมูลทางการเงิน...</div>
    {:else if error}
       <div class="alert alert-error shadow-lg mb-8">
         <div>
           <span>{error}</span>
         </div>
       </div>
    {/if}

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

      <div class="card bg-base-100 shadow-xl lg:col-span-3"> 
        <div class="card-body">
          <h2 class="card-title">ภาพรวมงบประมาณ</h2>
          {#if !isLoading && !error && financeStats.length > 0}
            {@const budget = 5000}
            {@const spentStat = financeStats.find(s => s.title === "กำไรทั้งหมด")}
            {@const spent = spentStat && spentStat.value !== 'Error' ? parseFloat(spentStat.value.replace(' บาท', '').replace(',', '')) : 0}
            {@const percentage = budget > 0 ? Math.min(Math.round((spent / budget) * 100), 100) : 0}
            <div class="flex flex-col items-center justify-center h-64">
                <div class="relative w-40 h-40">
                  <div class="absolute inset-0 flex items-center justify-center">
                <span class="text-2xl font-bold">{percentage}%</span>
              </div>
              <svg viewBox="0 0 36 36" class="circular-chart">
                <path class="circle-bg"
                  d="M18 2.0845 a 15.9155 15.9155 0 0 1 0 31.831 a 15.9155 15.9155 0 0 1 0 -31.831"
                   />
                <path class="circle"
                  d="M18 2.0845 a 15.9155 15.9155 0 0 1 0 31.831 a 15.9155 15.9155 0 0 1 0 -31.831"
                  stroke-dasharray="{percentage}, 100" />
              </svg>
            </div>
            <div class="mt-4 text-center">
              <p class="text-lg">{spent} บาท จาก {budget} บาท</p>
              <p class="text-sm opacity-75">การใช้งานงบประมาณเริ่มต้น</p>
            </div>
          </div>
          {:else}
             <div class="flex flex-col items-center justify-center h-64">
               <p class="text-gray-500">กำลังโหลดข้อมูลงบประมาณ...</p>
             </div>
          {/if}
        </div>
      </div>
    </div>

    <div class="bg-base-100 shadow-xl rounded-lg p-4 mb-8">
      <div class="flex flex-col md:flex-row gap-4">
        <div class="form-control flex-grow">
          <div class="input-group">
            <input
              type="text"
              placeholder="ค้นหาตามรายละเอียด, ประเภท, วันที่..."
              class="input input-bordered w-full"
              bind:value={searchQuery}
            />
          </div>
        </div>

      </div>
    </div>

    <div class="card bg-base-100 shadow-xl">
      <div class="card-body">
        <h2 class="card-title mb-4">รายการล่าสุด</h2>
        <div class="overflow-x-auto">
          <table class="table table-zebra w-full">
            <thead>
              <tr>
                <th>วันที่</th>
                <th>ประเภท</th>
                <th>รายละเอียด</th>
                <th>จำนวน</th>
                <th>จำนวนเงิน</th>
                <th>การดำเนินการ</th>
              </tr>
            </thead>
            <tbody>
              {#if paginatedExpenses.length === 0}
                <tr>
                  <td colspan="6" class="text-center py-4">
                    {#if expenses.length === 0}
                        ยังไม่มีรายการบันทึก
                    {:else}
                        ไม่มีรายการที่ตรงกับการค้นหาปัจจุบัน
                    {/if}
                  </td>
                </tr>
              {:else}
                {#each paginatedExpenses as expense (expense.id)}
                  <tr class="hover">
                    <td>{formatDate(expense.record_date)}</td>
                    <td>{expense.record_type}</td>
                    <td>{expense.description}</td>
                    <td>{expense.quantity}</td>
                    <td>{(expense.amount || 0).toLocaleString()} บาท</td>
                    <td>
                      <div class="flex gap-2">
                        <button class="btn btn-xs btn-ghost btn-info" on:click={() => editExpense(expense)}>แก้ไข</button>
                        <button class="btn btn-xs btn-error btn-ghost" on:click={() => deleteExpense(expense.id)}>ลบ</button>
                      </div>
                    </td>
                  </tr>
                {/each}
              {/if}
            </tbody>
            {#if filteredExpenses.length > 0}
            <tfoot>
              <tr>
                <th colspan="3">ยอดรวมที่กรองแล้ว</th>
                <th>{totalItemsFiltered}</th>
                <th>{totalSpentFiltered} บาท</th>
                <th></th>
              </tr>
            </tfoot>
            {/if}
          </table>
        </div>
      </div>
    </div>

    <div class="flex flex-col md:flex-row justify-between items-center mt-4">
      <div class="text-sm mb-4 md:mb-0">
         แสดง {paginatedExpenses.length > 0 ? (currentPage - 1) * itemsPerPage + 1 : 0}
          ถึง {Math.min(currentPage * itemsPerPage, filteredExpenses.length)}
          จาก {filteredExpenses.length} รายการ (ทั้งหมด: {expenses.length})
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

  </div>

  {#if showAddExpenseModal}
    <AddEditExpenseModal
      expense={expenseToEdit}
      {recipes}
      on:close={closeAddExpenseModal}
      on:save={handleSaveExpense}
    />
  {/if}

</div>

<style>
  .circular-chart {
    width: 100%;
    height: 100%;
    transform: rotate(-90deg);
  }
  .circle-bg {
    fill: none;
    stroke: #eee;
    stroke-width: 3.8;
  }
  .circle {
    fill: none;
    stroke: #4338ca;
    stroke-width: 2.8;
    stroke-linecap: round;
    animation: progress 1s ease-out forwards;
  }

  @keyframes progress {
    0% {
      stroke-dasharray: 0 100;
    }
  }
</style>
