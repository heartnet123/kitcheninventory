<script lang="ts">
  import { createEventDispatcher } from 'svelte';
  export let showModal: boolean = false;
  let itemName = "";
  let itemQuantity = "";
  let itemUnit = "";
  let itemCategory = ""; // Variable to hold the selected category
  const categories = ["Dairy", "Bakery", "Fruit", "Vegetables", "Meat", "Grains", "Dry product", "Other"]; // Define categories
  const dispatch = createEventDispatcher();

  function closeModal() {
    dispatch('close');
    itemName = "";
    itemQuantity = "";
    itemUnit = "";
    itemCategory = ""; // Reset item category
  }

  function submitItem() {
    // Dispatch category along with other item details
    dispatch('save', { name: itemName, quantity: itemQuantity, unit: itemUnit, category: itemCategory });
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
              type="text"
              id="itemQuantity"
              class="shadow appearance-none border rounded w-full py-2 px-3 text-gray-700 leading-tight focus:outline-none focus:shadow-outline"
              bind:value={itemQuantity}
              placeholder="0"
              required
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
  /* สามารถปรับแต่งสไตล์เพิ่มเติมได้ตามต้องการ */
</style>
