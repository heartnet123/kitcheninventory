<script lang="ts">
  import { createEventDispatcher, onMount } from 'svelte';

  // Updated Expense type to match the new database structure
  type Expense = {
    id?: number;
    record_date: string;
    amount: number;
    quantity: number;
    record_type: 'Expense' | 'Income';
    description: string;
    recipe_id?: number;
  };
  
  // Prop for existing expense data (if editing)
  export let expense: Expense | null = null;
  // Prop for recipes, now including selling_price
  export let recipes: Array<{ id: number; name: string; selling_price: number }> = [];

  // Categories and stores props are removed as these fields are no longer in the Expense type

  let formData: Expense = {
    record_date: new Date().toISOString().split('T')[0], // Default to today
    amount: 0,
    quantity: 1,
    record_type: 'Expense', // Default record_type
    description: '',
    recipe_id: undefined // Optional, so can be undefined
  };
  
  let isEditing = false;
  let title = "เพิ่มรายการใหม่"; // "Add New Item"
  let isAmountReadOnly = false;

  // Reactive statements
  $: if (formData.record_type === 'Expense') {
    formData.recipe_id = undefined;
    isAmountReadOnly = false; // Amount should be editable for expenses
  }

  $: if (formData.record_type === 'Income' && formData.recipe_id && recipes.length > 0) {
    const selectedRecipe = recipes.find(r => r.id === formData.recipe_id);
    if (selectedRecipe) {
      formData.amount = selectedRecipe.selling_price * formData.quantity;
      isAmountReadOnly = true;
    } else {
      isAmountReadOnly = false; // Recipe not found, allow manual input
    }
  } else if (formData.record_type === 'Income' && !formData.recipe_id) {
    isAmountReadOnly = false; // No recipe selected for income, allow manual input
  }

  // Recalculate amount if quantity changes for an income item with a recipe
  $: if (formData.record_type === 'Income' && formData.recipe_id && formData.quantity > 0 && recipes.length > 0) {
    const selectedRecipe = recipes.find(r => r.id === formData.recipe_id);
    if (selectedRecipe) {
      formData.amount = selectedRecipe.selling_price * formData.quantity;
    }
  }


  onMount(() => {
    if (expense) {
      isEditing = true;
      title = `แก้ไขรายการ (${expense.description})`; // "Edit Item (ID: ...)"
      // Ensure amount and quantity are numbers
      formData = { 
        ...expense, 
        amount: Number(expense.amount) || 0,
        quantity: Number(expense.quantity) || 1 
      }; 
    }
    // Removed default category/store logic
  });

  const dispatch = createEventDispatcher();

  function closeModal() {
    dispatch('close'); 
  }

  function handleSubmit() {
    // Updated validation
    if (!formData.record_date || !formData.description || formData.amount <= 0 || formData.quantity <= 0) {
      alert('กรุณากรอกข้อมูลให้ครบถ้วนและถูกต้อง'); 
      return;
    }
    console.log("Submitting form data:", formData); // Debug 
    dispatch('save', formData); 
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
          <label class="label" for="expense-record-date">
            <span class="label-text">วันที่</span>
          </label>
          <input type="date" id="expense-record-date" class="input input-bordered w-full" bind:value={formData.record_date} required />
        </div>

        <div class="form-control">
          <label class="label" for="expense-description">
            <span class="label-text">รายละเอียด</span>
          </label>
          <textarea id="expense-description" class="textarea textarea-bordered w-full" bind:value={formData.description} required rows="3"></textarea>
        </div>
        
        <div class="form-control">
          <label class="label" for="expense-record-type">
            <span class="label-text">ประเภทรายการ</span>
          </label>
          <select id="expense-record-type" class="select select-bordered w-full" bind:value={formData.record_type} required>
            <option value="Expense">รายจ่าย</option>
            <option value="Income">รายรับ</option>
          </select>
        </div>

        <div class="grid grid-cols-2 gap-4">
          <div class="form-control">
            <label class="label" for="expense-amount">
              <span class="label-text">จำนวนเงิน (บาท)</span>
            </label>
            <input type="number" id="expense-amount" step="0.01" min="0.01" class="input input-bordered w-full" bind:value={formData.amount} required readonly={isAmountReadOnly} />
          </div>

          <div class="form-control">
            <label class="label" for="expense-quantity">
              <span class="label-text">จำนวน</span>
            </label>
            <input type="number" id="expense-quantity" step="1" min="1" class="input input-bordered w-full" bind:value={formData.quantity} required />
          </div>
        </div>

        {#if formData.record_type === 'Income'}
        <div class="form-control">
          <label class="label" for="expense-recipe-id">
            <span class="label-text">สูตรอาหาร (ถ้ามี)</span>
          </label>
          <select id="expense-recipe-id" class="select select-bordered w-full" bind:value={formData.recipe_id}>
            <option value={undefined}>เลือกสูตรอาหาร (ถ้ามี)</option>
            {#each recipes as recipe}
              <option value={recipe.id}>{recipe.name}</option>
            {/each}
          </select>
        </div>
        {/if}
      </div>
      
      <div class="modal-action mt-6">
        <button type="button" class="btn btn-ghost" on:click={closeModal}>ยกเลิก</button>
        <button type="submit" class="btn btn-primary">{isEditing ? 'บันทึกการเปลี่ยนแปลง' : 'เพิ่มรายการ'}</button>
      </div>
    </form>
  </div>
    
  <!-- Click outside to close -->
  <form method="dialog" class="modal-backdrop" on:submit|preventDefault={closeModal}>
    <button type="submit">close</button>
  </form>
</div>
