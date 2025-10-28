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
  <div class="flex items-center space-x-4 mb-4">
    <a
      href="/"
      class="p-2 text-gray-400 hover:text-gray-700 transition-colors rounded-lg hover:bg-gray-100"
      title="Back to Conversations"
    >
      <svg class="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24">
        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M10 19l-7-7m0 0l7-7m-7 7h18"></path>
      </svg>
    </a>
    <h1 class="text-xl font-semibold">Sentry Frontend Validation</h1>
  </div>
  <p class="text-sm text-gray-500">Use these buttons to emit test events from the WebView.</p>
  <div class="space-x-2">
    <button class="px-3 py-2 bg-blue-600 text-white rounded" on:click={triggerHandledError}>Trigger handled error</button>
    <button class="px-3 py-2 bg-red-600 text-white rounded" on:click={triggerUnhandledError}>Trigger unhandled error</button>
  </div>
</div>

<style>
  /* minimal styling */
</style>
