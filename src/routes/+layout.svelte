<script>
    import Navbar from "../components/navbar.svelte"; // This contains both navbar and sidebar
    import "../style.css";
    import Titlebar from '../components/titlebar.svelte';
</script>


<main class="mt-7" data-theme="retro">
    <Titlebar />
    <Navbar />
    
    <!-- Main content area - adjusts for sidebar space automatically -->
    <div class="flex-1"> <!-- pt-16 for mobile navbar height -->
        <slot />
    </div>
</main>
