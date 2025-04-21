<script lang="ts">
    import { onMount } from 'svelte';
    import Database from '@tauri-apps/plugin-sql';
    import Additemmodal from '../../components/additemmodal.svelte';
    let showAddItemModal = false;
    const openModal = () => { showAddItemModal = true; };
  

    interface Item {
      id: number;
      name: string;
      category: string;
      quantity: number;
      unit: string;
    }
  
    let isLoadingItems = true;
    let items: Item[] = [];
    let name = '';
    let category = '';
    let quantity = 0;
    let unit = '';
    let error = '';
  
    async function getItems() {
      try {
        const db = await Database.load("sqlite:inventory.db");
        const dbItems = await db.select<Item[]>("SELECT id, name, category, quantity, unit FROM items");
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
        error = "Failed to delete item - check console";
      }
    }
  
    onMount(() => {
      getItems();
    });
async function handleSaveNewItem(event: CustomEvent<{ name: string; quantity: number; unit: string}>) {
      const { name, quantity, unit } = event.detail;
      const newItem = {
        id: Date.now(),
        name,
        category: '',
        quantity,
        unit
      };
      try {
        const db = await Database.load("sqlite:inventory.db");
        await db.execute(`INSERT INTO items (id, name, category, quantity, unit) VALUES (${newItem.id}, '${newItem.name}', '${newItem.category}', ${newItem.quantity}, '${newItem.unit}')`);
        items = [...items, newItem];
      } catch (err) {
        console.log(err);
        error = "Failed to add item - check console";
      }
}
  
    // For filtering
    let searchQuery = "";
    let selectedCategory = "All";
    
    // Available categories (Consider fetching these from the DB or defining them based on expected data)
    const categories = ["All", "Dairy", "Bakery", "Fruit", "Vegetables", "Meat", "Grains", "Other"]; // Added 'Other' as a fallback
  
    // Filtered items based on search query and filters
    $: filteredItems = items.filter(item => {
      const matchesSearch = searchQuery === "" || 
        item.name.toLowerCase().includes(searchQuery.toLowerCase()) ||
        item.category.toLowerCase().includes(searchQuery.toLowerCase());
      
      const matchesCategory = selectedCategory === "All" || item.category === selectedCategory || (selectedCategory === "Other" && !categories.slice(1).includes(item.category)); // Handle 'Other' category
      
      return matchesSearch && matchesCategory;
    });
  </script>
  
  <!-- Remove the main tag since it's in the layout -->
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
          
          <!-- Removed Status Filter -->
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
                  {#each filteredItems as item (item.id)} <!-- Corrected key syntax -->
                    <tr class="hover"> <!-- Added hover effect -->
                      <td>{item.name}</td>
                      <td>{item.category || 'N/A'}</td> <!-- Handle potential null/empty category -->
                      <td>{item.quantity}</td>
                      <td>{item.unit}</td>
                      <td>
                        <div class="flex gap-2">
                          <button class="btn btn-xs btn-secondary">Edit</button> <!-- Changed to btn-xs -->
                          <button class="btn btn-xs btn-error" on:click={() => deleteItem(item.id)}>Delete</button> <!-- Changed to btn-xs and removed btn-primary -->
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
          Showing {filteredItems.length} of {items.length} items
        </div>
        
        <div class="join">
          <input
            class="join-item btn btn-square"
            type="radio"
            name="options"
            aria-label="1"
            checked />
          <input class="join-item btn btn-square" type="radio" name="options" aria-label="2" />
          <input class="join-item btn btn-square" type="radio" name="options" aria-label="3" />
          <input class="join-item btn btn-square" type="radio" name="options" aria-label="4" />
        </div>
      </div>
    </div>
<Additemmodal showModal={showAddItemModal as any} on:close={() => showAddItemModal = false} on:save={handleSaveNewItem} />
  </div>
