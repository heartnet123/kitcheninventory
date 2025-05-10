<script lang="ts">
  // ... โค้ด script ของคุณทั้งหมดอยู่ที่นี่ ...
  import { open } from '@tauri-apps/plugin-dialog';
  import { convertFileSrc } from '@tauri-apps/api/core'; 
  import { onMount } from 'svelte';

  let isOpen = false;
  let itemName = '';
  let selectedFilePath: string | null = null;
let error: string | null = null;
let imageSrc: string | null = null;
let dialogElement: HTMLDialogElement; // เพิ่มตัวแปรสำหรับอ้างอิง dialog

// Reactive statement สำหรับควบคุมการเปิด/ปิด dialog
$: if (dialogElement) {
  if (isOpen) {
    dialogElement.showModal();
  } else {
    // ตรวจสอบว่า dialog เปิดอยู่ก่อนที่จะพยายามปิด
    if (dialogElement.open) {
       dialogElement.close();
    }
  }
}

// Derived variable for filename
$: selectedFileName = selectedFilePath ? selectedFilePath.split(/[\\/]/).pop() : '';

async function selectFile() {
  error = null;
    imageSrc = null;
    selectedFilePath = null;
    try {
      const file = await open({
        multiple: false,
        directory: false,
        filters: [{
          name: 'Images',
          extensions: ['png', 'jpg', 'jpeg', 'webp', 'gif', 'bmp']
        }]
      });

      if (typeof file === 'string') {
        selectedFilePath = file;
        imageSrc = convertFileSrc(file); // ใช้ convertFileSrc ที่ import มา
        console.log('Selected image file:', selectedFilePath);
        console.log('Image source for display:', imageSrc);
      } else {
         console.log('No file selected or dialog cancelled.');
      }
    } catch (err) {
      console.error("Error opening file dialog:", err);
      error = `Failed to open file dialog: ${err}`;
    }
  }

  function openModal() {
    isOpen = true;
  }

  function closeModal() {
    isOpen = false;
  }

  function addItem() {
    console.log("Adding item:", itemName);
    console.log("Associated file:", selectedFilePath);
    closeModal();
    itemName = '';
    selectedFilePath = null;
    imageSrc = null;
    error = null;
  }
</script>

<!-- ส่วน HTML ที่จะแสดงผล -->
<div class="max-w-7xl ms-auto mt-10 p-4 space-y-4">
  <!-- ปุ่มสำหรับเปิด Modal -->
  <button class="btn" on:click={openModal}>Open Add Item Modal</button>

  <!-- ปุ่มสำหรับเลือกไฟล์รูปภาพ -->
  <button class="btn btn-secondary" on:click={selectFile}>Select Image File</button>

  <!-- แสดง path ของไฟล์ที่เลือก -->
  {#if selectedFilePath}
    <p class="text-sm">Selected File Path: {selectedFilePath}</p>
  {/if}

  <!-- แสดงรูปภาพที่เลือก -->
  {#if imageSrc}
    <div class="mt-4">
      <p class="text-sm font-medium">Selected Image Preview:</p>
      <img src={imageSrc} alt="Selected Preview" class="max-w-xs max-h-64 border rounded mt-2" />
    </div>
  {/if}

  <!-- แสดงข้อผิดพลาด (ถ้ามี) -->
  {#if error}
      <div class="alert alert-error">
        <span>{error}</span>
      </div>
  {/if}
</div>

<!-- Modal สำหรับเพิ่ม Item -->
<dialog id="my_modal_1" class="modal" bind:this={dialogElement}>
  <div class="modal-box">
    <h3 class="text-lg font-bold">Add New Item</h3>
    <form method="dialog" on:submit|preventDefault={addItem}>
      <button type="button" class="btn btn-sm btn-circle btn-ghost absolute right-2 top-2" on:click={closeModal}>✕</button>
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
      <!-- แสดงข้อมูลรูปภาพที่เลือกใน Modal -->
      {#if selectedFilePath}
        <div class="py-2">
          <p class="text-sm">Associated Image: {selectedFileName}</p>
          {#if imageSrc}
             <img src={imageSrc} alt="Selected Preview" class="max-w-xs max-h-24 border rounded mt-1" />
          {/if}
          <p class="text-xs text-gray-500 mt-1">Note: Image selection happens outside the modal.</p>
        </div>
      {/if}
      <div class="modal-action">
        <button type="button" class="btn" on:click={closeModal}>Cancel</button> 
        <button type="submit" class="btn btn-primary">Add</button> 
      </div>
    </form>
  </div>
   <form method="dialog" class="modal-backdrop"> 
    <button type="button" on:click={closeModal}>close</button>
  </form>
</dialog>

<!-- ส่วน Style (ถ้ามี) -->
<style>
  /* เพิ่ม style ที่จำเป็น */
  .alert {
    padding: 1rem;
    border-radius: 0.25rem;
    margin-bottom: 1rem;
  }
  .alert-error {
    color: #721c24;
    background-color: #f8d7da;
    border-color: #f5c6cb;
  }
  .btn-secondary {
    /* ตัวอย่าง style */
    background-color: #6c757d;
    color: white;
  }
  .btn-secondary:hover {
    background-color: #5a6268;
  }
</style>
