<script lang="ts">
  import { createEventDispatcher, onMount, setContext } from 'svelte';
  import Database from "@tauri-apps/plugin-sql";

  export let recipeToEdit: any; // Recipe data passed as a prop

  interface InventoryItem {
    id: number;
    name: string;
    unit: string;
  }

  interface SelectedIngredient extends InventoryItem {
    quantity: number;
    recipe_ingredient_id?: number; // To track existing ingredients for updates
  }

  const dispatch = createEventDispatcher();
  let db: Database | null = null;
  let inventoryItems: InventoryItem[] = [];
  let selectedIngredients: SelectedIngredient[] = [];
  let error: string | null = null;
  let recipeName = '';
  let description = '';
  let sellingPrice: number | null = null;
  let recipeImageFile: File | null = null;
  let recipeImagePreview: string | null = null;
  let existingImage: Uint8Array | null = null;

  onMount(async () => {
    try {
      db = await Database.load("sqlite:inventory.db");
      const items = await db.select<InventoryItem[]>("SELECT id, name, unit FROM items ORDER BY name ASC");
      inventoryItems = items;

      if (recipeToEdit) {
        recipeName = recipeToEdit.name;
        description = recipeToEdit.description || '';
        sellingPrice = recipeToEdit.selling_price;
        existingImage = recipeToEdit.image; // Store existing image data
        if (existingImage) {
          recipeImagePreview = blobToDataURL(existingImage);
        }

        // Fetch existing ingredients for this recipe
        const existingRecipeIngredients = await db.select<any[]>(
          "SELECT ri.id as recipe_ingredient_id, i.id, i.name, i.unit, ri.quantity FROM recipe_ingredients ri JOIN items i ON ri.item_id = i.id WHERE ri.recipe_id = $1",
          [recipeToEdit.id]
        );
        selectedIngredients = existingRecipeIngredients.map(ing => ({
          id: ing.id,
          name: ing.name,
          unit: ing.unit,
          quantity: ing.quantity,
          recipe_ingredient_id: ing.recipe_ingredient_id
        }));
      }
    } catch (e) {
      console.error("Error loading DB or recipe data:", e);
      error = `Failed to load data: ${e}`;
    }
  });

  function blobToDataURL(blobData: Uint8Array | null | undefined): string {
    if (!blobData || blobData.length === 0) {
      return '/placeholder.png';
    }
    try {
      const base64String = btoa(String.fromCharCode(...blobData));
      return `data:image/png;base64,${base64String}`;
    } catch (e) {
      console.error("Error converting blob to Data URL:", e);
      return '/placeholder.png';
    }
  }

  function addIngredient(item: InventoryItem) {
    if (!selectedIngredients.some(ing => ing.id === item.id)) {
      selectedIngredients = [...selectedIngredients, { ...item, quantity: 1 }];
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
      error = "กรุณาใส่ราคาขายที่ถูกต้อง";
      return;
    }

    let imageData: Uint8Array | null = existingImage; // Default to existing image
    if (recipeImageFile) { // If a new file is selected, use it
      const arrayBuffer = await recipeImageFile.arrayBuffer();
      imageData = new Uint8Array(arrayBuffer);
    }

    try {
      // 1. Update recipes table
      await db.execute(
        "UPDATE recipes SET name = $1, description = $2, selling_price = $3, image = $4 WHERE id = $5",
        [recipeName, description, sellingPrice, imageData, recipeToEdit.id]
      );

      // 2. Update recipe_ingredients table
      // First, remove all existing ingredients for this recipe to handle deletions/changes easily
      await db.execute("DELETE FROM recipe_ingredients WHERE recipe_id = $1", [recipeToEdit.id]);

      // Then, insert the current list of selected ingredients
      for (const ingredient of selectedIngredients) {
        if (ingredient.quantity > 0) {
          await db.execute(
            "INSERT INTO recipe_ingredients (recipe_id, item_id, quantity, unit) VALUES ($1, $2, $3, $4)",
            [recipeToEdit.id, ingredient.id, ingredient.quantity, ingredient.unit]
          );
        }
      }
      console.log('Recipe updated successfully with ID:', recipeToEdit.id);
      dispatch('recipeedited');
      closeModal();
    } catch (e) {
      console.error("Error updating recipe:", e);
      error = `Failed to update recipe: ${e}`;
    }
  }
</script>

<div class="fixed inset-0 bg-black bg-opacity-50 flex justify-center items-center z-50">
  <div class="bg-base-100 p-6 rounded-lg shadow-xl w-full max-w-md">
    <h2 class="text-2xl font-bold mb-4">แก้ไขสูตรอาหาร</h2>

    {#if error}
      <div class="alert alert-error mb-4">
        <svg xmlns="http://www.w3.org/2000/svg" class="stroke-current shrink-0 h-6 w-6" fill="none" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M10 14l2-2m0 0l2-2m-2 2l-2 2m2-2l2 2m7-2a9 9 0 11-18 0 9 9 0 0118 0z" /></svg>
        <span>{error}</span>
      </div>
    {/if}

    <form on:submit|preventDefault={handleSubmit} class="space-y-4">
      <div>
        <label for="editRecipeName" class="block text-sm font-medium mb-1">ชื่อสูตรอาหาร:</label>
        <input type="text" id="editRecipeName" bind:value={recipeName} class="input input-bordered w-full" required />
      </div>

      <div>
        <label for="editSellingPrice" class="block text-sm font-medium mb-1">ราคาขาย (บาท):</label>
        <input type="number" step="0.01" min="0" id="editSellingPrice" bind:value={sellingPrice} class="input input-bordered w-full" required />
      </div>

      <div>
        <label for="edit-ingredient-select" class="block text-sm font-medium mb-1">เลือกส่วนผสม:</label>
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
        <label for="editDescription" class="block text-sm font-medium mb-1">คำอธิบาย/ขั้นตอนการทำ:</label>
        <textarea id="editDescription" bind:value={description} rows="4" class="textarea textarea-bordered w-full"></textarea>
      </div>

      <div>
        <label for="editRecipeImage" class="block text-sm font-medium mb-1">รูปภาพสูตรอาหาร (ถ้ามี):</label>
        <input
          type="file"
          id="editRecipeImage"
          class="file-input file-input-bordered w-full"
          accept="image/*"
          on:change={(e) => {
            const files = (e.target as HTMLInputElement).files;
            if (files && files.length > 0) {
              recipeImageFile = files[0];
              recipeImagePreview = URL.createObjectURL(recipeImageFile);
            } else {
              recipeImageFile = null;
              // If no new file is selected, keep the existing preview (or clear if existing was also null)
              recipeImagePreview = existingImage ? blobToDataURL(existingImage) : null;
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
        <button type="submit" class="btn btn-primary">บันทึกการแก้ไข</button>
      </div>
    </form>
  </div>
</div>
