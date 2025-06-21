<script lang="ts">
    import { onMount } from 'svelte';
    import Database from '@tauri-apps/plugin-sql';
    import AddItemModal from '../../components/additemmodal.svelte'; // Changed to PascalCase
    let showAddItemModal = false;
    const openModal = () => { showAddItemModal = true; };

    interface Item {
      id: number;
      name: string;
      category: string;
      quantity: number;
      unit: string;
      cost: number;
      cost_per_unit: number;
    }
  
    let isLoadingItems = true;
    let items: Item[] = [];
    let name = '';
    let category = '';
    let quantity = 0;
    let unit = '';
    let error = '';
    let cost = 0;
    let costperunit = 0;
  
    async function getItems() {
      try {
        const db = await Database.load("sqlite:inventory.db");
        const dbItems = await db.select<Item[]>("SELECT id, name, category, quantity, unit, cost, cost_per_unit FROM items");
        error = '';
        items = dbItems;
        isLoadingItems = false;
      } catch (err) {
        console.log(err);
        error = "Failed to get items - check console";
      }
    }
  
    async function deleteItem(id: number) {
      try {
        const db = await Database.load("sqlite:inventory.db");
        await db.execute(`DELETE FROM items WHERE id = ${id}`);
  
        error = '';
        items = items.filter(item => item.id !== id);
      } catch (err) {
        console.log(err);
        error = "มีสูตรอาหารที่ใช้วัตถุดิบนี้อยู่ กรุณาลบสูตรอาหารก่อนที่จะลบวัตถุดิบนี้";
      }
    }
  
    onMount(() => {
      getItems();
    });
async function handleSaveNewItem(event: CustomEvent<{ name: string; quantity: number; unit: string; category: string; cost: number; costperunit: number }>) { // Add category, cost, costperunit to event detail type
      const { name, quantity, unit, category, cost, costperunit } = event.detail; // Destructure category, cost, costperunit
      const newItem: Item = { // Ensure newItem conforms to Item interface
        id: Date.now(), // Consider using a more robust ID generation if needed
        name,
        category: category, // Use the received category
        quantity,
        unit,
        cost,
        costperunit // Use the received costperunit
      };
      try {
        const db = await Database.load("sqlite:inventory.db");
        // Use parameterized query to prevent SQL injection vulnerabilities
        await db.execute(
          "INSERT INTO items (id, name, category, quantity, unit, cost, cost_per_unit) VALUES ($1, $2, $3, $4, $5, $6, $7)",
          [newItem.id, newItem.name, newItem.category, newItem.quantity, newItem.unit, newItem.cost, newItem.costperunit]
        );
        items = [...items, newItem]; // Update local state
      } catch (err) {
        console.log(err);
        error = "Failed to add item - check console";
      }
}
  
    // For filtering
    let searchQuery = "";
    let selectedCategory = "All";
    
    // Available categories
    const categories = ["All", "Dairy", "Bakery", "Fruit", "Vegetables", "Meat", "Grains", "Dry product", "Other"]; 
  
    // Pagination state
    let currentPage = 1;
    const itemsPerPage = 10;
  
    // Filtered items based on search query and filters
    $: filteredItems = items.filter(item => {
      const matchesSearch = searchQuery === "" ||
        item.name.toLowerCase().includes(searchQuery.toLowerCase()) ||
        item.category.toLowerCase().includes(searchQuery.toLowerCase());
      
      const matchesCategory = selectedCategory === "All" || item.category === selectedCategory || (selectedCategory === "Other" && !categories.slice(1).includes(item.category)); // Handle 'Other' category
      
      return matchesSearch && matchesCategory;
    });
  
    // Calculate total pages
    $: totalPages = Math.ceil(filteredItems.length / itemsPerPage);
  
    // Get items for the current page
    $: paginatedItems = filteredItems.slice(
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
        <h1 class="text-3xl font-bold">Inventory Management</h1>
        <button class="btn btn-secondary" on:click={openModal}>Add New Item</button>
      </div>
      
      
      <!-- Filters and Search -->
      <div class="bg-base-100 shadow-xl rounded-lg p-4 mb-8">
        <div class="flex flex-col md:flex-row gap-4">
          <div class="form-control flex-grow">
            <div class="input-group">
              <input 
                type="text" 
                placeholder="Search items..." 
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
          
        </div>
      </div>
      
      <!-- Inventory Table -->
      <div class="card bg-base-100 shadow-xl">
        <div class="card-body">
          <div class="overflow-x-auto">
            <table class="table table-zebra w-full">
              <thead>
                <tr>
                  <th>Name</th>
                  <th>Category</th>
                  <th>Quantity</th>
                  <th>Unit</th>
                  <th>Cost</th>
                  <th>Cost/Unit</th>
                  <th>Actions</th>
                </tr>
              </thead>
              <tbody>
                {#if isLoadingItems}
                  <tr>
                    <td colspan="5" class="text-center py-4">Loading items...</td>
                  </tr>
                {:else if filteredItems.length === 0}
                  <tr>
                    <td colspan="5" class="text-center py-4">No items match your search or no items in inventory.</td>
                  </tr>
                {:else}
                  {#each paginatedItems as item (item.id)}
                    <tr class="hover">
                      <td>{item.name}</td>
                      <td>{item.category || 'N/A'}</td> <!-- Handle potential null/empty category -->
                      <td>{item.quantity}</td>
                      <td>{item.unit}</td>
                      <td>{item.cost}</td>
                      <td>{item.cost_per_unit}</td>
                      <td>
                        <div class="flex gap-2">
                          <button class="btn btn-xs btn-secondary">Edit</button> 
                          <button class="btn btn-xs btn-error" on:click={() => deleteItem(item.id)}>Delete</button>
                        </div>
                      </td>
                    </tr>
                  {/each}
                {/if}
              </tbody>
              {#if error}
                <tfoot>
                  <tr>
                    <td colspan="5" class="text-error text-center">{error}</td>
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
          Showing {paginatedItems.length > 0 ? (currentPage - 1) * itemsPerPage + 1 : 0}
          to {Math.min(currentPage * itemsPerPage, filteredItems.length)}
          of {filteredItems.length} items (Total: {items.length})
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
<AddItemModal showModal={showAddItemModal as any} on:close={() => showAddItemModal = false} on:save={handleSaveNewItem} />
  </div>
