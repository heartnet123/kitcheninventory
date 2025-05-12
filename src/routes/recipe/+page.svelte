<script lang="ts">
  import { onMount } from 'svelte';
  import Database from "@tauri-apps/plugin-sql";
  import "../../style.css";
  import AddRecipeModal from '../../components/AddRecipeModal.svelte';
  import EditRecipeModal from '../../components/EditRecipeModal.svelte'; // Import EditRecipeModal

  interface Recipe {
    id: number;
    name: string;
    description?: string;
    ingredients_count: number; // This might become redundant or be calculated
    ingredients: RecipeIngredient[]; // To store detailed ingredients
    selling_price: number;
    cost: number;
    profit: number;
    profit_margin: number;
    image: Uint8Array;
  }

  interface RecipeIngredient {
    item_id: number;
    name: string;
    quantity: number;
    unit: string;
  }

  // Helper function to convert Uint8Array (Blob) to Base64 Data URL
  function blobToDataURL(blobData: Uint8Array | null | undefined): string {
    if (!blobData || blobData.length === 0) {
      // Return a placeholder or default image URL if blob is empty or null
      return '/cow.png'; // Adjust path as needed
    }
    try {
      // Simple conversion assuming the blob is directly convertible
      // Note: You might need a more robust conversion depending on the exact format
      const base64String = btoa(String.fromCharCode(...blobData));
      // Assuming PNG format, adjust if needed (e.g., 'image/jpeg')
      // You might need to store the image type in the database as well
      return `data:image/png;base64,${base64String}`; 
    } catch (e) {
      console.error("Error converting blob to Data URL:", e);
      return '/cow.png'; // Fallback on error
    }
  }


  let recipes: Recipe[] = [];
  let searchTerm = "";
  let db: Database | null = null;
  let error: string | null = null;
  let showAddRecipeModal = false;
  let showEditRecipeModal = false; // State for edit modal
  let recipeToEdit: Recipe | null = null; // State for recipe being edited

  function openModal() {
    showAddRecipeModal = true;
  }

  function closeModal() {
    showAddRecipeModal = false;
    getRecipes(); // Refresh recipes when add modal closes
  }

  function openEditModal(recipe: Recipe) {
    recipeToEdit = recipe;
    showEditRecipeModal = true;
  }

  function closeEditModal() {
    showEditRecipeModal = false;
    recipeToEdit = null;
    getRecipes(); // Refresh recipes when edit modal closes
  }

  async function deleteRecipe(id: number) {
    if (!db) {
      error = "Database not initialized.";
      return;
    }
    try {
      console.log(`Attempting to delete recipe with id: ${id}`);
      await db.execute("DELETE FROM recipes WHERE id = $1", [id]);
      console.log(`Recipe with id: ${id} deleted successfully.`);
      // Remove the recipe from the local array to update the UI
      recipes = recipes.filter(recipe => recipe.id !== id);
    } catch (e) {
      console.error(`Error deleting recipe with id: ${id}:`, e);
      error = `Failed to delete recipe: ${e}`;
    }
  }

  async function getRecipes() {
    try {
      console.log("Attempting to load database...");
      db = await Database.load("sqlite:inventory.db");
      console.log("Database loaded successfully.");

      console.log("Executing query to fetch recipes...");
      const mainRecipes = await db.select<Recipe[]>("SELECT id, name, description, selling_price, image FROM recipes"); // Removed ingredients_count for now
      console.log("Query successful, fetched main recipes:", mainRecipes);

      const recipesWithIngredients: Recipe[] = [];
      for (const recipe of mainRecipes) {
        const ingredients = await db.select<RecipeIngredient[]>(
          "SELECT ri.item_id, i.name, ri.quantity, ri.unit FROM recipe_ingredients ri JOIN items i ON ri.item_id = i.id WHERE ri.recipe_id = $1",
          [recipe.id]
        );
        recipesWithIngredients.push({
          ...recipe,
          ingredients: ingredients,
          ingredients_count: ingredients.length // Recalculate or use this
        });
      }
      recipes = recipesWithIngredients;
      console.log("Final recipes with ingredients:", recipes);
    } catch (e) {
      console.error("Error loading database or fetching recipes:", e);
      error = `Failed to load recipes: ${e}`;
    }
  }


  onMount(() => {
    getRecipes();
  });

</script>

<svelte:head>
  <title>Recipe Management</title>
</svelte:head>



<div class="fade-in container mx-auto px-4 py-8 w-full min-h-screen p-0" data-theme="retro">
  <div class="max-w-7xl ms-auto mt-10">
    <div class="flex justify-between items-center mb-6">
      <h1 class="text-3xl font-bold">Recipe Management</h1>
      <button class="bg-blue-500 text-white px-4 py-2 rounded-md hover:bg-blue-600 transition" on:click={openModal}>
        + Add Recipe
      </button>
    </div>

    <div class="mb-6">
      <input 
        class="w-full p-3 rounded-md border border-gray-300 focus:outline-none focus:ring-2 focus:ring-blue-500"
        placeholder="Search recipes..." 
        type="text"
        bind:value={searchTerm}
      />
    </div>

    {#if error}
      <div class="text-red-500 p-4 bg-red-100 border border-red-400 rounded">
        {error}
      </div>
    {:else if recipes.length === 0}
      <p class="text-center text-gray-500 mt-8">No recipes</p>
    {:else}
      <div class="grid grid-cols-1 md:grid-cols-2 gap-6">
        {#each recipes as recipe (recipe.id)}
          <div class="bg-white rounded-lg shadow-md overflow-hidden hover:shadow-lg transition">
            <img
              alt="{recipe.name}"
              class="w-full h-48 object-cover"
              src={blobToDataURL(recipe.image)} 
            />
            <!-- Call the conversion function -->
            <div class="p-4">
              <h2 class="text-xl font-bold mb-2">{recipe.name}</h2>
              <p class="text-gray-600 mb-1">({recipe.ingredients.length} ingredients)</p>
              <div class="mb-3 max-h-24 overflow-y-auto text-sm">
                <ul class="list-disc list-inside pl-2">
                  {#each recipe.ingredients as ingredient}
                    <li>{ingredient.name}: {ingredient.quantity} {ingredient.unit}</li>
                  {/each}
                </ul>
              </div>
              
              <div class="flex justify-between items-center mb-4">
                <div>
                  <p class="text-gray-600">Selling Price</p>
                  <!-- Updated field name -->
                  <p class="text-lg font-semibold">{recipe.selling_price} บาท</p> 
                </div>
                <div>
                  <p class="text-gray-600">Cost</p>
                  <p class="text-lg font-semibold">{recipe.cost} บาท</p>
                </div>
                <div>
                  <p class="text-gray-600">Profit</p>
                  <p class="text-lg font-semibold text-green-500">
                    <!-- Updated field names -->
                    {recipe.profit} บาท ({recipe.profit_margin}%)
                  </p>
                </div>
              </div>
              <!-- Moved button section inside p-4 div -->
              <div class="flex justify-between items-center mt-4"> 
                <button class="text-gray-600 flex items-center hover:text-gray-800">
                  <i class="fas fa-info-circle mr-2"></i>
                View Details
              </button>
              <div class="flex space-x-2">
                <button class="text-blue-500 hover:text-blue-700" aria-label="Edit Recipe" on:click={() => openEditModal(recipe)}>
                  <i class="fas fa-edit"></i>
                </button>
                <button class="text-red-500 hover:text-red-700" aria-label="Delete Recipe" on:click={() => deleteRecipe(recipe.id)}>
                  <i class="fas fa-trash"></i>
                </button>
              </div>
            </div>
          </div>
        </div>
      {/each}
    </div>
  {/if}
  </div>

  {#if showAddRecipeModal}
    <AddRecipeModal on:close={closeModal} on:recipesaved={getRecipes} />
  {/if}

  {#if showEditRecipeModal && recipeToEdit}
    <EditRecipeModal {recipeToEdit} on:close={closeEditModal} on:recipeedited={getRecipes} />
  {/if}
</div>

<style>
  /* You can add custom styles here if needed */
</style>
