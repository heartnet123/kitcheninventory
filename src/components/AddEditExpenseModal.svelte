<script lang="ts">
  import { createEventDispatcher, onMount } from 'svelte';

  // Define the type for the expense prop (optional, for editing)
  type Expense = { id?: number; date: string; category: string; amount: number; store: string; items: number };
  
  // Prop for existing expense data (if editing)
  export let expense: Expense | null = null; 
  // Props for available categories and stores
  export let categories: string[] = [];
  export let stores: string[] = [];

  let formData: Expense = {
    date: new Date().toISOString().split('T')[0], // Default to today
    category: categories.length > 1 ? categories[1] : '', // Default to first actual category
    amount: 0,
    store: stores.length > 1 ? stores[1] : '', // Default to first actual store
    items: 1
  };
  
  let isEditing = false;
  let title = "Add New Expense";

  onMount(() => {
    if (expense) {
      isEditing = true;
      title = `Edit Expense (ID: ${expense.id})`;
      // Ensure amount is a number, handle potential null/undefined from prop
      formData = { ...expense, amount: Number(expense.amount) || 0 }; 
    } else {
       // Ensure default category/store exists if lists are provided
       if (!categories.includes(formData.category) && categories.length > 1) {
           formData.category = categories[1];
       }
       if (!stores.includes(formData.store) && stores.length > 1) {
           formData.store = stores[1];
       }
    }
  });

  const dispatch = createEventDispatcher();

  function closeModal() {
    dispatch('close'); 
  }

  function handleSubmit() {
    // Basic validation (can be expanded)
    if (!formData.date || !formData.category || !formData.store || formData.amount <= 0 || formData.items <= 0) {
      alert('Please fill in all fields correctly.');
      return;
    }
    console.log("Submitting form data:", formData); // Debug log
    dispatch('save', formData); // Dispatch 'save' event with the form data
    // closeModal(); // Optionally close modal after save, or let parent handle it
  }

  function handleKeydown(event: KeyboardEvent) {
    if (event.key === 'Escape') {
      closeModal();
    }
  }
</script>

<svelte:window on:keydown={handleKeydown}/>

<div class="modal modal-open" role="dialog" aria-modal="true" aria-labelledby="add-edit-expense-modal-title">
  <div class="modal-box w-11/12 max-w-lg">
    <button 
      class="btn btn-sm btn-circle btn-ghost absolute right-2 top-2" 
      on:click={closeModal}
      aria-label="Close"
    >✕</button>
    <h3 class="font-bold text-lg mb-4" id="add-edit-expense-modal-title">{title}</h3>
    
    <form on:submit|preventDefault={handleSubmit}>
      <div class="space-y-4">
        <div class="form-control">
          <label class="label" for="expense-date">
            <span class="label-text">Date</span>
          </label>
          <input type="date" id="expense-date" class="input input-bordered w-full" bind:value={formData.date} required />
        </div>

        <div class="form-control">
          <label class="label" for="expense-category">
            <span class="label-text">Category</span>
          </label>
          <select id="expense-category" class="select select-bordered w-full" bind:value={formData.category} required>
            {#each categories.filter(c => c !== 'All') as category (category)}
              <option value={category}>{category}</option>
            {/each}
          </select>
        </div>

        <div class="form-control">
          <label class="label" for="expense-store">
            <span class="label-text">Store</span>
          </label>
           <select id="expense-store" class="select select-bordered w-full" bind:value={formData.store} required>
             {#each stores.filter(s => s !== 'All') as store (store)}
               <option value={store}>{store}</option>
             {/each}
           </select>
        </div>
        
        <div class="grid grid-cols-2 gap-4">
          <div class="form-control">
            <label class="label" for="expense-amount">
              <span class="label-text">Amount ($)</span>
            </label>
            <input type="number" id="expense-amount" step="0.01" min="0.01" class="input input-bordered w-full" bind:value={formData.amount} required />
          </div>

          <div class="form-control">
            <label class="label" for="expense-items">
              <span class="label-text">Items</span>
            </label>
            <input type="number" id="expense-items" step="1" min="1" class="input input-bordered w-full" bind:value={formData.items} required />
          </div>
        </div>

      </div>
      
      <div class="modal-action mt-6">
        <button type="button" class="btn btn-ghost" on:click={closeModal}>Cancel</button>
        <button type="submit" class="btn btn-primary">{isEditing ? 'Save Changes' : 'Add Expense'}</button>
      </div>
    </form>
  </div>
    
  <!-- Click outside to close -->
  <form method="dialog" class="modal-backdrop" on:submit|preventDefault={closeModal}>
    <button type="submit">close</button>
  </form>
</div>
