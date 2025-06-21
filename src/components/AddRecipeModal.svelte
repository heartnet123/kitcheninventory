<script lang="ts">
  import { createEventDispatcher, onMount } from 'svelte';
  import Database from "@tauri-apps/plugin-sql"

  interface InventoryItem {
    id: number;
    name: string;
    unit: string;
    cost_per_unit?: number;
  }

  interface SelectedIngredient extends InventoryItem {
    quantity: number;
    cost_per_unit: number;
  }

  const dispatch = createEventDispatcher();
  let db: Database | null = null;
  let inventoryItems: InventoryItem[] = [];
  let selectedIngredients: SelectedIngredient[] = [];
  let error: string | null = null;
  let recipeName = '';
  let description = ''; // Changed from instructions to match DB column 'description'
  let sellingPrice: number | null = null; // Added state for selling price
  let recipeImageFile: File | null = null; // To store the selected image file
  let recipeImagePreview: string | null = null; // To store the image preview URL
  
  // Profit calculation variables
  let recipeCost: number = 0;
  let profit: number = 0;
  let profitMargin: number = 0;
  let isProfit: boolean = false;

  onMount(async () => {
    try {
      db = await Database.load("sqlite:inventory.db");
      const items = await db.select<InventoryItem[]>("SELECT id, name, unit, cost_per_unit FROM items ORDER BY name ASC");
      inventoryItems = items;
    } catch (e) {
      console.error("Error loading DB or fetching items:", e);
      error = `Failed to load inventory items: ${e}`;
    }
  });

  function addIngredient(item: InventoryItem) {
    if (!selectedIngredients.some(ing => ing.id === item.id)) {
      selectedIngredients = [...selectedIngredients, {
        ...item,
        quantity: 1,
        cost_per_unit: item.cost_per_unit || 0
      }]; // Default quantity to 1
    }
  }

  function removeIngredient(itemId: number) {
    selectedIngredients = selectedIngredients.filter(ing => ing.id !== itemId);
  }

  // Reactive calculations for profit
  $: {
    recipeCost = selectedIngredients.reduce((total, ingredient) => {
      return total + (ingredient.quantity * ingredient.cost_per_unit);
    }, 0);
  }

  $: {
    if (sellingPrice !== null && sellingPrice > 0) {
      profit = sellingPrice - recipeCost;
      profitMargin = (profit / sellingPrice) * 100;
      isProfit = profit > 0;
    } else {
      profit = 0;
      profitMargin = 0;
      isProfit = false;
    }
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

    let imageData: Uint8Array | null = null;
    if (recipeImageFile) {
      const arrayBuffer = await recipeImageFile.arrayBuffer();
      imageData = new Uint8Array(arrayBuffer);
    }

    console.log('Submitting recipe:', {
      name: recipeName,
      description: description, // Use description here
      selling_price: sellingPrice,
      image_present: !!recipeImageFile, // Log if an image is present
      image_data_length: imageData ? imageData.length : 0, // Shows size of byte array if imageData is not null
      ingredients: selectedIngredients
    });
    console.log('Image data:', imageData); // ต้องเป็น Uint8Array ที่มีข้อมูล
    try {
      // --- Database Insertion Logic ---
      // 1. Insert into recipes table - Added selling_price, recipe_cost, profit, profit_margin and image
      const recipeInsertResult = await db.execute(
        "INSERT INTO recipes (name, description, selling_price, recipe_cost, profit, profit_margin, image) VALUES ($1, $2, $3, $4, $5, $6, $7)",
        [recipeName, description, sellingPrice, recipeCost, profit, profitMargin, imageData]
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
        dispatch('recipesaved'); // Dispatch event to indicate recipe was saved
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
        <label for="ingredient-select" class="block text-sm font-medium mb-1">เลือกส่วนผสม:</label>
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
          <label for="quantity-input" class="block text-sm font-medium mb-1">ปริมาณส่วนผสมที่เลือก:</label>
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

      <!-- Cost and Profit Calculation Section -->
      {#if selectedIngredients.length > 0}
        <div class="bg-base-200 p-4 rounded-lg space-y-3">
          <h3 class="text-lg font-semibold">การคำนวณต้นทุนและกำไร</h3>
          
          <!-- Cost Breakdown -->
          <div class="space-y-2">
            <h4 class="text-sm font-medium text-gray-600">รายละเอียดต้นทุน:</h4>
            {#each selectedIngredients as ingredient (ingredient.id)}
              <div class="flex justify-between text-sm bg-white p-2 rounded">
                <span>{ingredient.name} ({ingredient.quantity} {ingredient.unit})</span>
                <span>{(ingredient.quantity * ingredient.cost_per_unit).toFixed(2)} บาท</span>
              </div>
            {/each}
          </div>

          <!-- Total Cost -->
          <div class="flex justify-between font-semibold border-t pt-2">
            <span>รวมต้นทุนทั้งหมด:</span>
            <span>{recipeCost.toFixed(2)} บาท</span>
          </div>

          <!-- Profit Calculation -->
          {#if sellingPrice !== null && sellingPrice > 0}
            <div class="space-y-2 border-t pt-3">
              <div class="flex justify-between">
                <span>ราคาขาย:</span>
                <span>{sellingPrice.toFixed(2)} บาท</span>
              </div>
              <div class="flex justify-between">
                <span>ต้นทุน:</span>
                <span>{recipeCost.toFixed(2)} บาท</span>
              </div>
              <div class="flex justify-between font-semibold {isProfit ? 'text-green-600' : profit === 0 ? 'text-yellow-600' : 'text-red-600'}">
                <span>กำไร:</span>
                <span>{profit.toFixed(2)} บาท ({profitMargin.toFixed(1)}%)</span>
              </div>
              
              <!-- Profit Status Indicator -->
              <div class="text-center mt-2">
                {#if isProfit}
                  <div class="badge badge-success gap-2">
                    🟢 มีกำไร
                  </div>
                {:else if profit === 0}
                  <div class="badge badge-warning gap-2">
                    🟡 คุ้มทุน
                  </div>
                {:else}
                  <div class="badge badge-error gap-2">
                    🔴 ขาดทุน
                  </div>
                {/if}
              </div>
            </div>
          {/if}
        </div>
      {/if}

      <div>
        <label for="description" class="block text-sm font-medium mb-1">คำอธิบาย/ขั้นตอนการทำ:</label>
        <textarea id="description" bind:value={description} rows="4" class="textarea textarea-bordered w-full"></textarea>
      </div>

      <div>
        <label for="recipeImage" class="block text-sm font-medium mb-1">รูปภาพสูตรอาหาร (ถ้ามี):</label>
        <input
          type="file"
          id="recipeImage"
          class="file-input file-input-bordered w-full"
          accept="image/*"
          on:change={(e) => {
            const files = (e.target as HTMLInputElement).files;
            if (files && files.length > 0) {
              recipeImageFile = files[0];
              recipeImagePreview = URL.createObjectURL(recipeImageFile); // Create a preview URL
            } else {
              recipeImageFile = null;
              recipeImagePreview = null;
            }
          }}
        />
        {#if recipeImagePreview}
          <div class="mt-2">
            <img src={recipeImagePreview} alt="Recipe preview" class="max-h-40 rounded border" />
          </div>
        {/if}
      </div>

      <div class="flex justify-end gap-2 pt-4">
        <button type="button" class="btn btn-ghost" on:click={closeModal}>ยกเลิก</button>
        <button type="submit" class="btn btn-primary">บันทึกสูตรอาหาร</button>
      </div>
    </form>
  </div>
</div>
