<script lang="ts">
  import { createEventDispatcher } from 'svelte';
  export let showModal: boolean = false;
  let itemName = "";
  let itemQuantity: string = ""; // Ensure type for parsing
  let itemUnit = "";
  let itemCategory = ""; // Variable to hold the selected category
  let itemCost: string = ""; // Ensure type for parsing
  const categories = ["Dairy", "Bakery", "Fruit", "Vegetables", "Meat", "Grains", "Dry product", "Other"]; // Define categories
  const dispatch = createEventDispatcher();

  let itemCostPerUnit: number | string = ""; // Will be calculated

  $: {
    const costVal = parseFloat(itemCost);
    const quantityVal = parseFloat(itemQuantity);
    if (!isNaN(costVal) && !isNaN(quantityVal) && quantityVal > 0) {
      itemCostPerUnit = costVal / quantityVal;
    } else {
      itemCostPerUnit = ""; // Or some placeholder like 'N/A' or 0
    }
  }

  function closeModal() {
    dispatch('close');
    itemName = "";
    itemQuantity = "";
    itemUnit = "";
    itemCategory = ""; // Reset item category
    itemCost = "";
    itemCostPerUnit = "";
  }

  function submitItem() {
    const costVal = parseFloat(itemCost);
    const quantityVal = parseFloat(itemQuantity);
    let costPerUnitVal = 0;
    if (!isNaN(costVal) && !isNaN(quantityVal) && quantityVal > 0) {
      costPerUnitVal = costVal / quantityVal;
    }
    // Dispatch category, cost, and calculated costperunit along with other item details
    dispatch('save', { name: itemName, quantity: quantityVal, unit: itemUnit, category: itemCategory, cost: costVal, costperunit: costPerUnitVal });
    closeModal();
  }
</script>

{#if showModal}
  <div class="modal modal-open">
    <div class="modal-box">
      <h3 class="font-bold text-lg">Add New Inventory Item</h3>
      <form on:submit|preventDefault={submitItem}>
        <div class="form-control">
          <!-- svelte-ignore a11y_label_has_associated_control -->
          <div class="mt-5 mb-4">
            <label for="itemName" class="block text-gray-700 text-sm font-bold mb-2">
              ชื่อ :
            </label>
            <input
              type="text"
              id="itemName"
              class="shadow appearance-none border rounded w-full py-2 px-3 text-gray-700 leading-tight focus:outline-none focus:shadow-outline"
              bind:value={itemName}
              placeholder=""
              required
            />
          </div>

          <div class="mb-4">
            <label for="itemQuantity" class="block text-gray-700 text-sm font-bold mb-2">
              จำนวน :
            </label>
            <input
              id="itemQuantity"
              class="shadow appearance-none border rounded w-full py-2 px-3 text-gray-700 leading-tight focus:outline-none focus:shadow-outline"
              bind:value={itemQuantity}
              placeholder="0"
              required
              type="number"
              step="any"
            />
          </div>

          <div class="mb-4">
            <label for="itemUnit" class="block text-gray-700 text-sm font-bold mb-2">
              หน่วย :
            </label>
            <input
              type="text"
              id="itemUnit"
              class="shadow appearance-none border rounded w-full py-2 px-3 text-gray-700 leading-tight focus:outline-none focus:shadow-outline"
              bind:value={itemUnit}
              placeholder="Unit"
              required
            />
          </div>

          <div class="mb-4">
            <label for="itemCategory" class="block text-gray-700 text-sm font-bold mb-2">
              หมวดหมู่ :
            </label>
            <select
              id="itemCategory"
              class="shadow appearance-none border rounded w-full py-2 px-3 text-gray-700 leading-tight focus:outline-none focus:shadow-outline"
              bind:value={itemCategory}
              required
            >
              <option value="" disabled selected>Select a category</option>
              {#each categories as category}
                <option value={category}>{category}</option>
              {/each}
            </select>
          </div>

          <div class="mb-4">
            <label for="itemCost" class="block text-gray-700 text-sm font-bold mb-2">
              ราคา :
            </label>
            <input
              type="number"
              id="itemCost"
              class="shadow appearance-none border rounded w-full py-2 px-3 text-gray-700 leading-tight focus:outline-none focus:shadow-outline"
              bind:value={itemCost}
              placeholder="0.00"
              required
              step="0.01"
            />
          </div>

          <div class="mb-4">
            <label for="itemCostPerUnit" class="block text-gray-700 text-sm font-bold mb-2">
              ราคาต่อหน่วย (คำนวณอัตโนมัติ) :
            </label>
            <input
              type="text" 
              id="itemCostPerUnit"
              class="shadow appearance-none border rounded w-full py-2 px-3 text-gray-700 leading-tight focus:outline-none focus:shadow-outline bg-gray-100"
              bind:value={itemCostPerUnit}
              readonly 
            />
          </div>

        </div>
        <div class="modal-action">
          <button type="button" class="btn" on:click={closeModal}>Close</button>
          <button type="submit" class="btn btn-success">Submit</button>
        </div>
      </form>
    </div>
  </div>
{/if}

<style>

</style>
