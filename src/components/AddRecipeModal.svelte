<script lang="ts">
  import { createEventDispatcher, onMount } from 'svelte';
  import Database from "@tauri-apps/plugin-sql";

  interface InventoryItem {
    id: number;
    name: string;
    unit: string;
  }

  interface SelectedIngredient extends InventoryItem {
    quantity: number;
  }

  const dispatch = createEventDispatcher();
  let db: Database | null = null;
  let inventoryItems: InventoryItem[] = [];
  let selectedIngredients: SelectedIngredient[] = [];
  let error: string | null = null;
  let recipeName = '';
  let description = ''; // Changed from instructions to match DB column 'description'
  let sellingPrice: number | null = null; // Added state for selling price

  onMount(async () => {
    try {
      db = await Database.load("sqlite:inventory.db");
      const items = await db.select<InventoryItem[]>("SELECT id, name, unit FROM items ORDER BY name ASC");
      inventoryItems = items;
    } catch (e) {
      console.error("Error loading DB or fetching items:", e);
      error = `Failed to load inventory items: ${e}`;
    }
  });

  function addIngredient(item: InventoryItem) {
    if (!selectedIngredients.some(ing => ing.id === item.id)) {
      selectedIngredients = [...selectedIngredients, { ...item, quantity: 1 }]; // Default quantity to 1
    }
  }

  function removeIngredient(itemId: number) {
    selectedIngredients = selectedIngredients.filter(ing => ing.id !== itemId);
  }

  function closeModal() {
    dispatch('close');
  }

  async function handleSubmit() {
    if (!db) {
      error = "Database not loaded.";
      return;
    }
    if (!recipeName.trim()) {
      error = "กรุณาใส่ชื่อสูตรอาหาร";
      return;
    }
     if (selectedIngredients.length === 0) {
      error = "กรุณาเลือกส่วนผสมอย่างน้อย 1 อย่าง";
      return;
    }
    if (sellingPrice === null || sellingPrice < 0) {
      error = "กรุณาใส่ราคาขายที่ถูกต้อง (ต้องไม่เป็นค่าว่างหรือติดลบ)";
      return;
    }

    console.log('Submitting recipe:', {
      name: recipeName,
      description: description, // Use description here
      selling_price: sellingPrice,
      ingredients: selectedIngredients
    });

    try {
      // --- Database Insertion Logic ---
      // 1. Insert into recipes table - Added selling_price
      const recipeInsertResult = await db.execute(
        "INSERT INTO recipes (name, description, selling_price) VALUES ($1, $2, $3)", // Added selling_price
        [recipeName, description, sellingPrice] // Use description and sellingPrice
      );

      if (recipeInsertResult.lastInsertId) {
        const recipeId = recipeInsertResult.lastInsertId;

        // 2. Insert into recipe_ingredients table
        for (const ingredient of selectedIngredients) {
          if (ingredient.quantity > 0) { // Ensure quantity is valid
             await db.execute(
              "INSERT INTO recipe_ingredients (recipe_id, item_id, quantity, unit) VALUES ($1, $2, $3, $4)",
              [recipeId, ingredient.id, ingredient.quantity, ingredient.unit]
            );
          }
        }
        console.log('Recipe added successfully with ID:', recipeId);
        closeModal(); // Close modal on success
      } else {
         throw new Error("Failed to get last insert ID for recipe.");
      }

    } catch (e) {
      console.error("Error saving recipe:", e);
      error = `Failed to save recipe: ${e}`;
    }
  }
</script>

<div class="fixed inset-0 bg-black bg-opacity-50 flex justify-center items-center z-50">
  <div class="bg-base-100 p-6 rounded-lg shadow-xl w-full max-w-md">
    <h2 class="text-2xl font-bold mb-4">เพิ่มสูตรอาหารใหม่</h2>

    {#if error}
      <div class="alert alert-error mb-4">
        <svg xmlns="http://www.w3.org/2000/svg" class="stroke-current shrink-0 h-6 w-6" fill="none" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M10 14l2-2m0 0l2-2m-2 2l-2 2m2-2l2 2m7-2a9 9 0 11-18 0 9 9 0 0118 0z" /></svg>
        <span>{error}</span>
      </div>
    {/if}

    <form on:submit|preventDefault={handleSubmit} class="space-y-4">
      <div>
        <label for="recipeName" class="block text-sm font-medium mb-1">ชื่อสูตรอาหาร:</label>
        <input type="text" id="recipeName" bind:value={recipeName} class="input input-bordered w-full" required />
      </div>

      <div>
        <label for="sellingPrice" class="block text-sm font-medium mb-1">ราคาขาย (บาท):</label>
        <input type="number" step="0.01" min="0" id="sellingPrice" bind:value={sellingPrice} class="input input-bordered w-full" required />
      </div>

      <div>
        <label class="block text-sm font-medium mb-1">เลือกส่วนผสม:</label>
        {#if inventoryItems.length > 0}
          <div class="max-h-40 overflow-y-auto border rounded p-2 mb-2 bg-base-200">
            {#each inventoryItems as item (item.id)}
              {@const isSelected = selectedIngredients.some(ing => ing.id === item.id)}
              <button
                type="button"
                class="btn btn-sm btn-outline w-full justify-start mb-1 {isSelected ? 'btn-active btn-primary' : ''}"
                on:click={() => isSelected ? removeIngredient(item.id) : addIngredient(item)}
              >
                {item.name} ({item.unit})
              </button>
            {/each}
          </div>
        {:else}
          <p class="text-sm text-gray-500">กำลังโหลดรายการวัตถุดิบ...</p>
        {/if}
      </div>

      {#if selectedIngredients.length > 0}
        <div>
          <label class="block text-sm font-medium mb-1">ปริมาณส่วนผสมที่เลือก:</label>
          <div class="space-y-2">
            {#each selectedIngredients as ingredient (ingredient.id)}
              <div class="flex items-center gap-2 p-2 border rounded bg-base-200">
                <span class="flex-grow">{ingredient.name}</span>
                <input
                  type="number"
                  step="any"
                  min="0.01"
                  bind:value={ingredient.quantity}
                  class="input input-sm input-bordered w-24"
                  required
                />
                <span class="w-16 text-right">{ingredient.unit}</span>
                 <button type="button" class="btn btn-xs btn-circle btn-error btn-outline" on:click={() => removeIngredient(ingredient.id)}>
                   ✕
                 </button>
              </div>
            {/each}
          </div>
        </div>
      {/if}


      <div>
        <label for="description" class="block text-sm font-medium mb-1">คำอธิบาย/ขั้นตอนการทำ:</label>
        <textarea id="description" bind:value={description} rows="4" class="textarea textarea-bordered w-full"></textarea>
      </div>

       <!-- Optional: Add Image Upload -->

      <div class="flex justify-end gap-2 pt-4">
        <button type="button" class="btn btn-ghost" on:click={closeModal}>ยกเลิก</button>
        <button type="submit" class="btn btn-primary">บันทึกสูตรอาหาร</button>
      </div>
    </form>
  </div>
</div>
