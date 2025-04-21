<script>
  let isOpen = false;
  let itemName = '';
  let itemQuantity = 0; 
  function openModal() {
    isOpen = true;
  }

  function closeModal() {
    isOpen = false;
  }
  
  function addItem() {
    console.log("Adding item:", itemName);
    closeModal();
  }
</script>
<div class="max-w-7xl ms-auto mt-10">
<button class="btn" on:click={openModal}>open modal</button>
</div>
<dialog id="my_modal_1" class="modal" open={isOpen} on:close={closeModal}>
  <div class="modal-box">
    <h3 class="text-lg font-bold">Add New Item</h3>
    <form on:submit|preventDefault={addItem}>
      <div class="py-4">
        <label for="itemName" class="block text-gray-700 text-sm font-bold mb-2">
          Item Name:
        </label>
        <input
          type="text"
          id="itemName"
          class="shadow appearance-none border rounded w-full py-2 px-3 text-gray-700 leading-tight focus:outline-none focus:shadow-outline"
          bind:value={itemName}
          placeholder="Enter item name"
          required
        />
      </div>
      <div class="modal-action">
        <button type="button" class="btn" on:click={closeModal}>Cancel</button>
        <button type="submit" class="btn btn-primary">Add</button>
      </div>
    </form>
  </div>
</dialog>

<style>
  /* Style to center the modal */
  .modal {
    display: none; /* Hidden by default */
    position: fixed; /* Stay in place */
    z-index: 1; /* Sit on top */
    left: 0;
    top: 0;
    width: 100%; /* Full width */
    height: 100%; /* Full height */
    overflow: auto; /* Enable scroll if needed */
    background-color: rgba(0,0,0,0.4); /* Black w/ opacity */
    display: flex; /* Enable flexbox for centering */
    align-items: center; /* Vertically center content */
    justify-content: center; /* Horizontally center content */
  }

  .modal[open] {
    display: flex;
  }

  .modal-box {
    background-color: #fefefe;
    padding: 20px;
    border: 1px solid #888;
    width: 80%; /* Adjust width as needed */
    max-width: 500px; /* Optional: set a maximum width */
    border-radius: .5rem;
  }

  .modal-action {
    display: flex;
    justify-content: flex-end;
  }

  .btn {
    /* Basic button styling - you might want to use your existing CSS */
    padding: 0.5rem 1rem;
    border: none;
    border-radius: 0.25rem;
    cursor: pointer;
    font-size: 1rem;
    background-color: #007bff;
    color: white;
  }

  .btn:hover {
    background-color: #0056b3;
  }
</style>
