<script lang="ts">
  import { onDestroy } from 'svelte';
  import { errorStore, detectErrorPatterns } from '$lib/stores/enhanced-error-store';
  import { ErrorSeverity, ErrorCategory, type AppError } from '$lib/types/errors';

  export let autoHideTimeout = 5000; // Auto-hide after 5 seconds
  export let maxErrors = 5; // Maximum number of errors to show at once
  export let showAnalytics = false; // Show error analytics

  let errors: AppError[] = [];
  let timeouts = new Map<string, number>();
  let errorPatterns: any = {};
  let systemHealth: 'healthy' | 'degraded' | 'unhealthy' = 'healthy';

  // Subscribe to enhanced error store
  const unsubscribe = errorStore.subscribe(state => {
    errors = state.errors.slice(0, maxErrors);
    systemHealth = state.systemHealth;

    // Setup auto-dismiss for new errors
    state.errors.forEach(error => {
      if (!timeouts.has(error.id) && shouldAutoDismiss(error)) {
        setupAutoDismiss(error);
      }
    });

    // Update error patterns
    errorPatterns = detectErrorPatterns();
  });

  // Determine if error should auto-dismiss
  function shouldAutoDismiss(error: AppError): boolean {
    return autoHideTimeout > 0 &&
           error.severity !== ErrorSeverity.CRITICAL &&
           error.recoverable &&
           !errorPatterns.cascadingFailures; // Don't auto-dismiss during cascading failures
  }

  // Setup auto-dismiss with race condition prevention
  function setupAutoDismiss(error: AppError) {
    if (autoHideTimeout > 0) {
      // Clear any existing timeout for this error
      if (timeouts.has(error.id)) {
        clearTimeout(timeouts.get(error.id));
      }

      // Adjust timeout based on severity and system health
      let effectiveTimeout = autoHideTimeout;
      if (error.severity === ErrorSeverity.ERROR) {
        effectiveTimeout *= 2; // Keep errors visible longer
      }
      if (systemHealth === 'unhealthy') {
        effectiveTimeout *= 1.5; // Keep errors visible longer when system is unhealthy
      }

      // Set new timeout
      const timeoutId = setTimeout(() => {
        dismissError(error);
        timeouts.delete(error.id);
      }, effectiveTimeout);

      // Store timeout ID
      timeouts.set(error.id, timeoutId);
    }
  }

  // Dismiss a specific error
  function dismissError(error: AppError) {
    errorStore.dismissError(error.id);

    // Clear timeout if it exists
    if (timeouts.has(error.id)) {
      clearTimeout(timeouts.get(error.id));
      timeouts.delete(error.id);
    }
  }

  // Dismiss all errors
  function dismissAllErrors() {
    errorStore.clearAllErrors();

    // Clear all timeouts
    timeouts.forEach(timeoutId => clearTimeout(timeoutId));
    timeouts.clear();
  }

  // Get CSS classes for error severity
  function getSeverityClasses(severity: ErrorSeverity): string {
    switch (severity) {
      case ErrorSeverity.INFO:
        return 'bg-blue-600 border-blue-500 text-blue-100';
      case ErrorSeverity.WARNING:
        return 'bg-yellow-600 border-yellow-500 text-yellow-100';
      case ErrorSeverity.ERROR:
        return 'bg-red-600 border-red-500 text-red-100';
      case ErrorSeverity.CRITICAL:
        return 'bg-red-800 border-red-700 text-red-100 shadow-lg shadow-red-900/50';
      default:
        return 'bg-gray-600 border-gray-500 text-gray-100';
    }
  }

  // Get icon for error severity
  function getSeverityIcon(severity: ErrorSeverity): string {
    switch (severity) {
      case ErrorSeverity.INFO:
        return 'ℹ️';
      case ErrorSeverity.WARNING:
        return '⚠️';
      case ErrorSeverity.ERROR:
        return '❌';
      case ErrorSeverity.CRITICAL:
        return '🚨';
      default:
        return '❓';
    }
  }

  // Get category color
  function getCategoryColor(category: ErrorCategory): string {
    switch (category) {
      case ErrorCategory.API:
        return 'bg-purple-600';
      case ErrorCategory.NETWORK:
        return 'bg-orange-600';
      case ErrorCategory.TIMEOUT:
        return 'bg-yellow-600';
      case ErrorCategory.VALIDATION:
        return 'bg-blue-600';
      case ErrorCategory.ENVIRONMENT:
        return 'bg-green-600';
      case ErrorCategory.PERMISSION:
        return 'bg-red-600';
      case ErrorCategory.DATA:
        return 'bg-indigo-600';
      default:
        return 'bg-gray-600';
    }
  }

  // Format timestamp
  function formatTimestamp(timestamp: number): string {
    const now = Date.now();
    const diff = now - timestamp;

    if (diff < 60000) { // Less than 1 minute
      return 'Just now';
    } else if (diff < 3600000) { // Less than 1 hour
      return `${Math.floor(diff / 60000)}m ago`;
    } else {
      return new Date(timestamp).toLocaleTimeString();
    }
  }

  // Get recovery suggestion
  function getRecoverySuggestion(error: AppError): string | null {
    switch (error.category) {
      case ErrorCategory.NETWORK:
        return 'Check your internet connection and try again';
      case ErrorCategory.TIMEOUT:
        return 'The operation took too long. Try again or check system performance';
      case ErrorCategory.PERMISSION:
        return 'Check your permissions or try running as administrator';
      case ErrorCategory.ENVIRONMENT:
        return 'This feature requires the desktop application';
      case ErrorCategory.VALIDATION:
        return 'Please check your input and try again';
      default:
        return error.recoverable ? 'Try refreshing the page or restarting the application' : null;
    }
  }

  // Clean up timeouts when component is destroyed
  onDestroy(() => {
    if (unsubscribe) unsubscribe();

    // Clear all timeouts
    timeouts.forEach(timeoutId => clearTimeout(timeoutId));
    timeouts.clear();
  });
</script>

<!-- Error notifications container -->
{#if errors.length > 0}
  <div class="fixed top-4 right-4 z-50 space-y-2 max-w-md">
    <!-- System health indicator for multiple errors -->
    {#if errors.length > 3 || systemHealth !== 'healthy'}
      <div class="bg-gray-800 border border-gray-600 rounded-lg p-3 text-sm">
        <div class="flex items-center justify-between mb-2">
          <div class="flex items-center space-x-2">
            <div class="w-2 h-2 rounded-full {systemHealth === 'healthy' ? 'bg-green-400' : systemHealth === 'degraded' ? 'bg-yellow-400' : 'bg-red-400'}"></div>
            <span class="text-gray-300 font-medium">System Status: {systemHealth}</span>
          </div>
          <button
            on:click={dismissAllErrors}
            class="text-gray-400 hover:text-gray-200 text-xs"
          >
            Clear All
          </button>
        </div>

        {#if errorPatterns.cascadingFailures}
          <p class="text-yellow-300 text-xs">⚡ Detecting cascading failures - automatic recovery in progress</p>
        {/if}

        {#if errorPatterns.highErrorRate}
          <p class="text-red-300 text-xs">🔥 High error rate detected - system may be overloaded</p>
        {/if}

        {#if errorPatterns.repeatedErrors.length > 0}
          <p class="text-orange-300 text-xs">🔄 Repeated errors: {errorPatterns.repeatedErrors.join(', ')}</p>
        {/if}
      </div>
    {/if}

    <!-- Individual error notifications -->
    {#each errors as error (error.id)}
      <div
        class="border-l-4 rounded-lg shadow-lg transition-all duration-300 ease-in-out {getSeverityClasses(error.severity)}"
        role="alert"
        aria-live="polite"
      >
        <div class="p-4">
          <div class="flex items-start justify-between">
            <div class="flex items-start space-x-2 flex-1">
              <span class="text-lg flex-shrink-0 mt-0.5">
                {getSeverityIcon(error.severity)}
              </span>
              <div class="flex-1 min-w-0">
                <div class="flex items-center space-x-2 mb-1">
                  <h4 class="font-medium text-sm">
                    {error.message}
                  </h4>
                  <span class="text-xs opacity-75">
                    {formatTimestamp(error.timestamp)}
                  </span>
                </div>

                {#if error.details}
                  <p class="text-xs mt-1 opacity-90 line-clamp-2">
                    {error.details}
                  </p>
                {/if}

                <div class="flex items-center space-x-2 mt-2">
                  <span class="inline-block text-xs px-2 py-1 rounded {getCategoryColor(error.category)} bg-opacity-80">
                    {error.category}
                  </span>

                  {#if error.context?.command}
                    <span class="inline-block text-xs px-2 py-1 rounded bg-black bg-opacity-20">
                      {error.context.command}
                    </span>
                  {/if}

                  {#if !error.recoverable}
                    <span class="inline-block text-xs px-2 py-1 rounded bg-red-600 bg-opacity-80">
                      Non-recoverable
                    </span>
                  {/if}
                </div>

                <!-- Recovery suggestion -->
                {#if getRecoverySuggestion(error)}
                  <p class="text-xs mt-2 opacity-80 italic">
                    💡 {getRecoverySuggestion(error)}
                  </p>
                {/if}

                <!-- Context information for debugging -->
                {#if showAnalytics && error.context}
                  <details class="mt-2">
                    <summary class="text-xs cursor-pointer opacity-70 hover:opacity-100">
                      Debug Info
                    </summary>
                    <pre class="text-xs mt-1 opacity-60 overflow-x-auto">{JSON.stringify(error.context, null, 2)}</pre>
                  </details>
                {/if}
              </div>
            </div>

            <!-- Dismiss button -->
            <button
              on:click={() => dismissError(error)}
              class="ml-2 flex-shrink-0 text-white hover:text-gray-200 transition-colors"
              aria-label="Dismiss error"
            >
              <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12"></path>
              </svg>
            </button>
          </div>

          <!-- Progress bar for auto-dismiss -->
          {#if shouldAutoDismiss(error) && timeouts.has(error.id)}
            <div class="mt-3 w-full bg-black bg-opacity-20 rounded-full h-1">
              <div
                class="bg-white bg-opacity-60 h-1 rounded-full transition-all duration-100 ease-linear"
                style="animation: shrink {autoHideTimeout * (error.severity === ErrorSeverity.ERROR ? 2 : 1) * (systemHealth === 'unhealthy' ? 1.5 : 1)}ms linear forwards;"
              ></div>
            </div>
          {/if}
        </div>
      </div>
    {/each}
  </div>
{/if}

<style>
  @keyframes shrink {
    from {
      width: 100%;
    }
    to {
      width: 0%;
    }
  }

  .line-clamp-2 {
    display: -webkit-box;
    -webkit-line-clamp: 2;
    -webkit-box-orient: vertical;
    overflow: hidden;
  }
</style>
