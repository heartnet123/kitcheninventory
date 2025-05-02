<script lang="ts">
  import { createEventDispatcher } from 'svelte';

  // Define the type for the expense prop
  type Expense = { id: number; date: string; category: string; amount: number; store: string; items: number };
  
  // Prop to receive the selected expense data
  export let expense: Expense | null = null;
  
  const dispatch = createEventDispatcher();

  function closeModal() {
    dispatch('close'); // Dispatch a 'close' event when the modal should be closed
  }

  // Handle Escape key press to close modal
  function handleKeydown(event: KeyboardEvent) {
    if (event.key === 'Escape') {
      closeModal();
    }
  }
</script>

<svelte:window on:keydown={handleKeydown}/>

{#if expense}
  <div class="modal modal-open" role="dialog" aria-modal="true" aria-labelledby="expense-modal-title">
    <div class="modal-box">
      <button 
        class="btn btn-sm btn-circle btn-ghost absolute right-2 top-2" 
        on:click={closeModal}
        aria-label="Close"
      >✕</button>
      <h3 class="font-bold text-lg" id="expense-modal-title">Expense Details (ID: {expense.id})</h3>
      <div class="py-4 space-y-2">
        <p><strong>Date:</strong> {expense.date}</p>
        <p><strong>Category:</strong> {expense.category}</p>
        <p><strong>Store:</strong> {expense.store}</p>
        <p><strong>Amount:</strong> ${expense.amount.toFixed(2)}</p>
        <p><strong>Items:</strong> {expense.items}</p>
        
        <!-- Add more details or edit form here if needed -->
        
      </div>
      <div class="modal-action">
        <button class="btn" on:click={closeModal}>Close</button>
        <!-- Add other actions like Edit or Delete if required -->
        <!-- <button class="btn btn-primary">Edit</button> -->
      </div>
    </div>
    
    <!-- Click outside to close -->
    <form method="dialog" class="modal-backdrop" on:submit|preventDefault={closeModal}>
      <button type="submit">close</button>
    </form>
  </div>
{/if}
