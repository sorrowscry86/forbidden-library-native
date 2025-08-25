<script lang="ts">
  import { onDestroy } from 'svelte';
  import { errorStore } from '$lib/stores/error-store';
  import { ErrorSeverity, type AppError } from '$lib/types/errors';

  export let autoHideTimeout = 5000; // Auto-hide after 5 seconds
  export let maxErrors = 5; // Maximum number of errors to show at once

  let errors: AppError[] = [];
  let timeouts = new Map<string, number>();

  // Subscribe to error store
  const unsubscribe = errorStore.subscribe(state => {
    errors = state.errors.slice(0, maxErrors);

    // Setup auto-dismiss for new errors
    state.errors.forEach(error => {
      if (!timeouts.has(error.id) && shouldAutoDismiss(error)) {
        setupAutoDismiss(error);
      }
    });
  });

  // Determine if error should auto-dismiss
  function shouldAutoDismiss(error: AppError): boolean {
    return autoHideTimeout > 0 &&
           error.severity !== ErrorSeverity.CRITICAL &&
           error.recoverable;
  }

  // Setup auto-dismiss with race condition prevention
  function setupAutoDismiss(error: AppError) {
    if (autoHideTimeout > 0) {
      // Clear any existing timeout for this error
      if (timeouts.has(error.id)) {
        clearTimeout(timeouts.get(error.id));
      }

      // Set new timeout
      const timeoutId = setTimeout(() => {
        dismissError(error);
        timeouts.delete(error.id);
      }, autoHideTimeout);

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
        return 'bg-red-800 border-red-700 text-red-100';
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
    {#each errors as error (error.id)}
      <div
        class="border-l-4 p-4 rounded-lg shadow-lg transition-all duration-300 ease-in-out {getSeverityClasses(error.severity)}"
        role="alert"
        aria-live="polite"
      >
        <div class="flex items-start justify-between">
          <div class="flex items-start space-x-2 flex-1">
            <span class="text-lg flex-shrink-0 mt-0.5">
              {getSeverityIcon(error.severity)}
            </span>
            <div class="flex-1 min-w-0">
              <h4 class="font-medium text-sm">
                {error.message}
              </h4>
              {#if error.details}
                <p class="text-xs mt-1 opacity-90">
                  {error.details}
                </p>
              {/if}
              {#if error.category}
                <span class="inline-block text-xs px-2 py-1 rounded mt-2 bg-black bg-opacity-20">
                  {error.category}
                </span>
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
          <div class="mt-2 w-full bg-black bg-opacity-20 rounded-full h-1">
            <div
              class="bg-white bg-opacity-60 h-1 rounded-full transition-all duration-100 ease-linear"
              style="animation: shrink {autoHideTimeout}ms linear forwards;"
            ></div>
          </div>
        {/if}
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
</style>
