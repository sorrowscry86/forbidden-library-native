<script lang="ts">
  import DesktopFeatures from '$lib/components/DesktopFeatures.svelte';
  import { onMount } from 'svelte';

  const title = 'Desktop Features - Forbidden Library';

  onMount(() => {
    document.title = title;
  });
</script>

<svelte:head>
  <title>{title}</title>
  <meta
    name="description"
    content="Explore native desktop features of the Forbidden Library application"
  />
</svelte:head>

<main class="desktop-page">
  <DesktopFeatures />
</main>

<style>
  .desktop-page {
    min-height: 100vh;
    background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
    padding: 2rem 0;
  }

  :global(.desktop-page .desktop-features) {
    background-color: white;
    border-radius: 1rem;
    box-shadow:
      0 20px 25px -5px rgba(0, 0, 0, 0.1),
      0 10px 10px -5px rgba(0, 0, 0, 0.04);
  }
</style>
