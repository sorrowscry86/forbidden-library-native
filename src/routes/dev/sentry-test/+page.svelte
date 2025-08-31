<script lang="ts">
  import * as Sentry from '@sentry/sveltekit';

  // Dev-only Sentry validation route (VOI-79)
  function triggerHandledError() {
    try {
      throw new Error('Sentry handled test error (frontend)');
    } catch (e) {
      Sentry.captureException(e);
      console.error('Handled error for Sentry test:', e);
    }
  }

  function triggerUnhandledError() {
    // This will bubble as an unhandled error
    setTimeout(() => {
      throw new Error('Sentry unhandled test error (frontend)');
    }, 100);
  }

  function triggerPerformanceEvent() {
    // In Sentry v8, use startSpan instead of startTransaction
    Sentry.startSpan(
      {
        name: 'Test Performance Transaction',
        op: 'navigation'
      },
      () => {
        // Simulate some work
        setTimeout(() => {
          console.log('Performance transaction sent to Sentry');
        }, 1000);
      }
    );
  }

  function addBreadcrumb() {
    Sentry.addBreadcrumb({
      message: 'Test breadcrumb from validation harness',
      level: 'info',
      category: 'test'
    });
    console.log('Breadcrumb added');
  }

  function captureMessage() {
    Sentry.captureMessage('Test message from frontend validation harness', 'info');
    console.log('Message sent to Sentry');
  }
</script>

<div class="p-6 space-y-4 max-w-2xl">
  <h1 class="text-2xl font-bold text-white mb-4">Sentry Frontend Validation Harness</h1>
  <p class="text-sm text-gray-400 mb-6">
    Use these buttons to emit test events from the WebView to validate Sentry integration.
    Check your browser console and Sentry dashboard for results.
  </p>
  
  <div class="bg-gray-800 rounded-lg p-4 space-y-3">
    <h2 class="text-lg font-semibold text-white">Error Testing</h2>
    <div class="space-x-2 space-y-2">
      <button 
        class="px-4 py-2 bg-blue-600 hover:bg-blue-700 text-white rounded transition-colors"
        on:click={triggerHandledError}
      >
        Trigger Handled Error
      </button>
      <button 
        class="px-4 py-2 bg-red-600 hover:bg-red-700 text-white rounded transition-colors"
        on:click={triggerUnhandledError}
      >
        Trigger Unhandled Error
      </button>
    </div>
  </div>

  <div class="bg-gray-800 rounded-lg p-4 space-y-3">
    <h2 class="text-lg font-semibold text-white">Performance Testing</h2>
    <div class="space-x-2 space-y-2">
      <button 
        class="px-4 py-2 bg-purple-600 hover:bg-purple-700 text-white rounded transition-colors"
        on:click={triggerPerformanceEvent}
      >
        Trigger Performance Transaction
      </button>
    </div>
  </div>

  <div class="bg-gray-800 rounded-lg p-4 space-y-3">
    <h2 class="text-lg font-semibold text-white">Telemetry Testing</h2>
    <div class="space-x-2 space-y-2">
      <button 
        class="px-4 py-2 bg-green-600 hover:bg-green-700 text-white rounded transition-colors"
        on:click={addBreadcrumb}
      >
        Add Breadcrumb
      </button>
      <button 
        class="px-4 py-2 bg-yellow-600 hover:bg-yellow-700 text-white rounded transition-colors"
        on:click={captureMessage}
      >
        Capture Message
      </button>
    </div>
  </div>

  <div class="bg-gray-700 rounded-lg p-4 mt-6">
    <h3 class="text-sm font-semibold text-gray-300 mb-2">Instructions:</h3>
    <ol class="text-sm text-gray-400 space-y-1">
      <li>1. Enable telemetry in browser console: <code class="bg-gray-800 px-1 rounded">localStorage.setItem('telemetry.enabled', 'true')</code></li>
      <li>2. Reload the page</li>
      <li>3. Use the buttons above to test different Sentry events</li>
      <li>4. Check your Sentry dashboard for the events</li>
    </ol>
  </div>
</div>

<style>
  code {
    font-family: 'Monaco', 'Menlo', 'Ubuntu Mono', monospace;
  }
</style>