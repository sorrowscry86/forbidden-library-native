/**
 * Global error store for managing application errors
 * Provides centralized error handling with automatic cleanup
 */

import { writable, type Writable } from 'svelte/store';
import { AppError, ErrorCategory, ErrorSeverity, type AppErrorOptions } from '$lib/types/errors';

export interface ErrorStoreState {
  errors: AppError[];
  lastError: AppError | null;
}

// Create the error store
function createErrorStore() {
  const { subscribe, update, set }: Writable<ErrorStoreState> = writable({
    errors: [],
    lastError: null,
  });

  let cleanupInterval: ReturnType<typeof setInterval> | null = null;

  return {
    subscribe,

    /**
     * Add a new error to the store
     */
    addError: (errorOptions: AppErrorOptions | AppError) => {
      const error = errorOptions instanceof AppError ? errorOptions : new AppError(errorOptions);

      update((state) => ({
        errors: [error, ...state.errors],
        lastError: error,
      }));

      // Log error for debugging
      console.error(`[${error.category}] ${error.message}`, {
        details: error.details,
        context: error.context,
        originalError: error.originalError,
      });

      return error;
    },

    /**
     * Remove a specific error by ID
     */
    dismissError: (errorId: string) => {
      update((state) => {
        const updatedErrors = state.errors.filter((error) => error.id !== errorId);
        return {
          errors: updatedErrors,
          lastError: updatedErrors[0] || null,
        };
      });
    },

    /**
     * Mark an error as handled
     */
    markErrorAsHandled: (errorId: string) => {
      update((state) => {
        const error = state.errors.find((e) => e.id === errorId);
        if (error) {
          error.markAsHandled();
        }
        return state;
      });
    },

    /**
     * Clear all errors
     */
    clearAllErrors: () => {
      set({ errors: [], lastError: null });
    },

    /**
     * Clear errors by category
     */
    clearErrorsByCategory: (category: ErrorCategory) => {
      update((state) => {
        const filteredErrors = state.errors.filter((error) => error.category !== category);
        return {
          errors: filteredErrors,
          lastError: filteredErrors[0] || null,
        };
      });
    },

    /**
     * Get errors by severity
     */
    getErrorsBySeverity: (severity: ErrorSeverity): AppError[] => {
      let errors: AppError[] = [];
      update((state) => {
        errors = state.errors.filter((error) => error.severity === severity);
        return state;
      });
      return errors;
    },

    /**
     * Initialize the error store with cleanup
     */
    init: (maxErrors = 100, maxAgeMs = 3600000) => {
      // 1 hour default
      // Clean up old errors periodically
      cleanupInterval = setInterval(() => {
        update((state) => {
          const now = Date.now();
          const cleanedErrors = state.errors
            .filter((error) => {
              // Keep unhandled errors and recent errors
              return !error.handled || now - error.timestamp < maxAgeMs;
            })
            .slice(0, maxErrors); // Limit total number of errors

          return {
            errors: cleanedErrors,
            lastError: cleanedErrors[0] || null,
          };
        });
      }, 60000); // Check every minute

      // Return cleanup function
      return () => {
        if (cleanupInterval) {
          clearInterval(cleanupInterval);
          cleanupInterval = null;
        }
      };
    },

    /**
     * Destroy the store and clean up resources
     */
    destroy: () => {
      if (cleanupInterval) {
        clearInterval(cleanupInterval);
        cleanupInterval = null;
      }
      set({ errors: [], lastError: null });
    },
  };
}

// Export the error store instance
export const errorStore = createErrorStore();

// Convenience functions for common error types
export function addError(options: AppErrorOptions): AppError {
  return errorStore.addError(options);
}

export function addTimeoutError(command: string, timeoutMs: number): AppError {
  return errorStore.addError({
    message: 'Operation timed out',
    details: `The command ${command} took longer than ${timeoutMs}ms to complete`,
    category: ErrorCategory.TIMEOUT,
    severity: ErrorSeverity.WARNING,
    context: { command, timeoutMs },
  });
}

export function addNetworkError(message: string, originalError?: unknown): AppError {
  return errorStore.addError({
    message: 'Network error',
    details: message,
    category: ErrorCategory.NETWORK,
    severity: ErrorSeverity.ERROR,
    originalError,
  });
}

export function addValidationError(message: string, context?: Record<string, unknown>): AppError {
  return errorStore.addError({
    message: 'Validation error',
    details: message,
    category: ErrorCategory.VALIDATION,
    severity: ErrorSeverity.WARNING,
    context,
  });
}

export function addApiError(command: string, originalError: unknown): AppError {
  return errorStore.addError({
    message: 'API call failed',
    details: `Command ${command} failed: ${originalError}`,
    category: ErrorCategory.API,
    severity: ErrorSeverity.ERROR,
    originalError,
    context: { command },
  });
}
