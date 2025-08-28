<script lang="ts">
  // Dev-only Sentry validation route (VOI-79)
  function triggerHandledError() {
    try {
      throw new Error('Sentry handled test error (frontend)');
    } catch (e) {
      console.error('Handled error for Sentry test:', e);
    }
  }

  function triggerUnhandledError() {
    // This will bubble as an unhandled error
    throw new Error('Sentry unhandled test error (frontend)');
  }
</script>

<div class="p-6 space-y-4">
  <h1 class="text-xl font-semibold">Sentry Frontend Validation</h1>
  <p class="text-sm text-gray-500">Use these buttons to emit test events from the WebView.</p>
  <div class="space-x-2">
    <button class="px-3 py-2 bg-blue-600 text-white rounded" on:click={triggerHandledError}>Trigger handled error</button>
    <button class="px-3 py-2 bg-red-600 text-white rounded" on:click={triggerUnhandledError}>Trigger unhandled error</button>
  </div>
</div>

<style>
  /* minimal styling */
</style>
