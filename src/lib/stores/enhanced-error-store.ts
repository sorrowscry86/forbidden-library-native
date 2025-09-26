/**
 * Enhanced global error store with analytics, recovery strategies, and intelligent error management
 */

import { writable, type Writable } from 'svelte/store';
import { AppError, ErrorCategory, ErrorSeverity, type AppErrorOptions } from '$lib/types/errors';

// Error analytics and metrics
interface ErrorMetrics {
  totalErrors: number;
  errorsByCategory: Record<ErrorCategory, number>;
  errorsBySeverity: Record<ErrorSeverity, number>;
  recentErrors: AppError[];
  errorRate: number; // errors per minute
  recoveryRate: number; // percentage of errors that were recovered from
  topErrorCommands: Record<string, number>;
  retryAttempts: Record<string, number>;
  successfulRetries: Record<string, number>;
}

// Recovery strategy configuration
interface RecoveryStrategy {
  category: ErrorCategory;
  maxRetries: number;
  backoffMs: number;
  autoRecover: boolean;
  userNotification: boolean;
  fallbackAction?: () => Promise<void>;
}

// Enhanced error store state
export interface EnhancedErrorStoreState {
  errors: AppError[];
  lastError: AppError | null;
  metrics: ErrorMetrics;
  recoveryStrategies: Record<ErrorCategory, RecoveryStrategy>;
  isOnline: boolean;
  systemHealth: 'healthy' | 'degraded' | 'unhealthy';
}

// Default recovery strategies
const DEFAULT_RECOVERY_STRATEGIES: Record<ErrorCategory, RecoveryStrategy> = {
  [ErrorCategory.API]: {
    category: ErrorCategory.API,
    maxRetries: 3,
    backoffMs: 1000,
    autoRecover: true,
    userNotification: true,
  },
  [ErrorCategory.TIMEOUT]: {
    category: ErrorCategory.TIMEOUT,
    maxRetries: 2,
    backoffMs: 2000,
    autoRecover: true,
    userNotification: false,
  },
  [ErrorCategory.NETWORK]: {
    category: ErrorCategory.NETWORK,
    maxRetries: 5,
    backoffMs: 3000,
    autoRecover: true,
    userNotification: true,
  },
  [ErrorCategory.ENVIRONMENT]: {
    category: ErrorCategory.ENVIRONMENT,
    maxRetries: 0,
    backoffMs: 0,
    autoRecover: false,
    userNotification: true,
  },
  [ErrorCategory.VALIDATION]: {
    category: ErrorCategory.VALIDATION,
    maxRetries: 0,
    backoffMs: 0,
    autoRecover: false,
    userNotification: true,
  },
  [ErrorCategory.DATA]: {
    category: ErrorCategory.DATA,
    maxRetries: 1,
    backoffMs: 1000,
    autoRecover: true,
    userNotification: true,
  },
  [ErrorCategory.PERMISSION]: {
    category: ErrorCategory.PERMISSION,
    maxRetries: 0,
    backoffMs: 0,
    autoRecover: false,
    userNotification: true,
  },
  [ErrorCategory.UNKNOWN]: {
    category: ErrorCategory.UNKNOWN,
    maxRetries: 1,
    backoffMs: 1000,
    autoRecover: false,
    userNotification: true,
  },
};

// Create enhanced error store
function createEnhancedErrorStore() {
  const initialState: EnhancedErrorStoreState = {
    errors: [],
    lastError: null,
    metrics: {
      totalErrors: 0,
      errorsByCategory: {} as Record<ErrorCategory, number>,
      errorsBySeverity: {} as Record<ErrorSeverity, number>,
      recentErrors: [],
      errorRate: 0,
      recoveryRate: 0,
      topErrorCommands: {},
      retryAttempts: {},
      successfulRetries: {},
    },
    recoveryStrategies: DEFAULT_RECOVERY_STRATEGIES,
    isOnline: typeof navigator !== 'undefined' ? navigator.onLine : true,
    systemHealth: 'healthy',
  };

  const { subscribe, update, set }: Writable<EnhancedErrorStoreState> = writable(initialState);

  let cleanupInterval: ReturnType<typeof setInterval> | null = null;
  let onlineListener: (() => void) | null = null;
  let offlineListener: (() => void) | null = null;

  // Track online/offline status
  function setupOnlineTracking() {
    if (typeof window !== 'undefined') {
      onlineListener = () => {
        update((state) => ({ ...state, isOnline: true }));
        console.log('Network connection restored');
      };

      offlineListener = () => {
        update((state) => ({ ...state, isOnline: false }));
        console.warn('Network connection lost');
      };

      window.addEventListener('online', onlineListener);
      window.addEventListener('offline', offlineListener);
    }
  }

  // Calculate error rate (errors per minute)
  function calculateErrorRate(errors: AppError[]): number {
    const now = Date.now();
    const oneMinuteAgo = now - 60000;
    const recentErrors = errors.filter((error) => error.timestamp > oneMinuteAgo);
    return recentErrors.length;
  }

  // Calculate recovery rate
  function calculateRecoveryRate(metrics: ErrorMetrics): number {
    const totalRetries = Object.values(metrics.retryAttempts).reduce(
      (sum, count) => sum + count,
      0
    );
    const successfulRetries = Object.values(metrics.successfulRetries).reduce(
      (sum, count) => sum + count,
      0
    );

    if (totalRetries === 0) return 100; // No retries needed = 100% success
    return Math.round((successfulRetries / totalRetries) * 100);
  }

  // Update system health based on error patterns
  function updateSystemHealth(
    state: EnhancedErrorStoreState
  ): 'healthy' | 'degraded' | 'unhealthy' {
    const { metrics, errors } = state;
    const recentCriticalErrors = errors.filter(
      (error) => error.severity === ErrorSeverity.CRITICAL && Date.now() - error.timestamp < 300000 // 5 minutes
    ).length;

    const errorRate = metrics.errorRate;
    const recoveryRate = metrics.recoveryRate;

    if (recentCriticalErrors > 0 || errorRate > 10 || recoveryRate < 50) {
      return 'unhealthy';
    } else if (errorRate > 5 || recoveryRate < 80) {
      return 'degraded';
    } else {
      return 'healthy';
    }
  }

  // Update metrics
  function updateMetrics(state: EnhancedErrorStoreState): ErrorMetrics {
    const { errors } = state;
    const now = Date.now();
    const recentErrors = errors.filter((error) => now - error.timestamp < 3600000); // 1 hour

    // Count errors by category
    const errorsByCategory = {} as Record<ErrorCategory, number>;
    Object.values(ErrorCategory).forEach((category) => {
      errorsByCategory[category] = errors.filter((error) => error.category === category).length;
    });

    // Count errors by severity
    const errorsBySeverity = {} as Record<ErrorSeverity, number>;
    Object.values(ErrorSeverity).forEach((severity) => {
      errorsBySeverity[severity] = errors.filter((error) => error.severity === severity).length;
    });

    // Top error commands
    const topErrorCommands = {} as Record<string, number>;
    errors.forEach((error) => {
      if (error.context?.command) {
        const command = error.context.command as string;
        topErrorCommands[command] = (topErrorCommands[command] || 0) + 1;
      }
    });

    return {
      totalErrors: errors.length,
      errorsByCategory,
      errorsBySeverity,
      recentErrors: recentErrors.slice(0, 50), // Keep last 50 recent errors
      errorRate: calculateErrorRate(errors),
      recoveryRate: calculateRecoveryRate(state.metrics),
      topErrorCommands,
      retryAttempts: state.metrics.retryAttempts,
      successfulRetries: state.metrics.successfulRetries,
    };
  }

  return {
    subscribe,

    /**
     * Add a new error with enhanced analytics
     */
    addError: (errorOptions: AppErrorOptions | AppError) => {
      const error = errorOptions instanceof AppError ? errorOptions : new AppError(errorOptions);

      update((state) => {
        const newErrors = [error, ...state.errors];
        const newMetrics = updateMetrics({ ...state, errors: newErrors });
        const newSystemHealth = updateSystemHealth({
          ...state,
          errors: newErrors,
          metrics: newMetrics,
        });

        // Log error with enhanced context
        console.error(`[${error.category}:${error.severity}] ${error.message}`, {
          details: error.details,
          context: error.context,
          originalError: error.originalError,
          systemHealth: newSystemHealth,
          errorRate: newMetrics.errorRate,
        });

        return {
          ...state,
          errors: newErrors,
          lastError: error,
          metrics: newMetrics,
          systemHealth: newSystemHealth,
        };
      });

      return error;
    },

    /**
     * Track a retry attempt
     */
    trackRetry: (command: string, _attempt: number, _error?: AppError) => {
      update((state) => ({
        ...state,
        metrics: {
          ...state.metrics,
          retryAttempts: {
            ...state.metrics.retryAttempts,
            [command]: (state.metrics.retryAttempts[command] || 0) + 1,
          },
        },
      }));
    },

    /**
     * Track a successful operation (including successful retries)
     */
    trackSuccess: (command: string, wasRetry: boolean = false) => {
      update((state) => {
        const newMetrics = { ...state.metrics };

        if (wasRetry) {
          newMetrics.successfulRetries = {
            ...newMetrics.successfulRetries,
            [command]: (newMetrics.successfulRetries[command] || 0) + 1,
          };
        }

        return {
          ...state,
          metrics: {
            ...newMetrics,
            recoveryRate: calculateRecoveryRate(newMetrics),
          },
        };
      });
    },

    /**
     * Remove a specific error by ID
     */
    dismissError: (errorId: string) => {
      update((state) => {
        const updatedErrors = state.errors.filter((error) => error.id !== errorId);
        const newMetrics = updateMetrics({ ...state, errors: updatedErrors });

        return {
          ...state,
          errors: updatedErrors,
          lastError: updatedErrors[0] || null,
          metrics: newMetrics,
          systemHealth: updateSystemHealth({
            ...state,
            errors: updatedErrors,
            metrics: newMetrics,
          }),
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
      update((state) => ({
        ...state,
        errors: [],
        lastError: null,
        systemHealth: 'healthy',
      }));
    },

    /**
     * Clear errors by category
     */
    clearErrorsByCategory: (category: ErrorCategory) => {
      update((state) => {
        const filteredErrors = state.errors.filter((error) => error.category !== category);
        const newMetrics = updateMetrics({ ...state, errors: filteredErrors });

        return {
          ...state,
          errors: filteredErrors,
          lastError: filteredErrors[0] || null,
          metrics: newMetrics,
          systemHealth: updateSystemHealth({
            ...state,
            errors: filteredErrors,
            metrics: newMetrics,
          }),
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
     * Get error analytics
     */
    getAnalytics: (): ErrorMetrics => {
      let metrics: ErrorMetrics = initialState.metrics;
      update((state) => {
        metrics = state.metrics;
        return state;
      });
      return metrics;
    },

    /**
     * Update recovery strategy for a category
     */
    updateRecoveryStrategy: (category: ErrorCategory, strategy: Partial<RecoveryStrategy>) => {
      update((state) => ({
        ...state,
        recoveryStrategies: {
          ...state.recoveryStrategies,
          [category]: { ...state.recoveryStrategies[category], ...strategy },
        },
      }));
    },

    /**
     * Get system health status
     */
    getSystemHealth: (): 'healthy' | 'degraded' | 'unhealthy' => {
      let health: 'healthy' | 'degraded' | 'unhealthy' = 'healthy';
      update((state) => {
        health = state.systemHealth;
        return state;
      });
      return health;
    },

    /**
     * Initialize the enhanced error store
     */
    init: (maxErrors = 200, maxAgeMs = 7200000) => {
      // 2 hours default
      setupOnlineTracking();

      // Enhanced cleanup with analytics preservation
      cleanupInterval = setInterval(() => {
        update((state) => {
          const now = Date.now();
          const cleanedErrors = state.errors
            .filter((error) => {
              // Keep unhandled errors and recent errors
              return !error.handled || now - error.timestamp < maxAgeMs;
            })
            .slice(0, maxErrors); // Limit total number of errors

          const newMetrics = updateMetrics({ ...state, errors: cleanedErrors });
          const newSystemHealth = updateSystemHealth({
            ...state,
            errors: cleanedErrors,
            metrics: newMetrics,
          });

          return {
            ...state,
            errors: cleanedErrors,
            lastError: cleanedErrors[0] || null,
            metrics: newMetrics,
            systemHealth: newSystemHealth,
          };
        });
      }, 60000); // Check every minute

      // Return cleanup function
      return () => {
        if (cleanupInterval) {
          clearInterval(cleanupInterval);
          cleanupInterval = null;
        }

        if (onlineListener && typeof window !== 'undefined') {
          window.removeEventListener('online', onlineListener);
        }

        if (offlineListener && typeof window !== 'undefined') {
          window.removeEventListener('offline', offlineListener);
        }
      };
    },

    /**
     * Export error data for analysis
     */
    exportErrorData: () => {
      let exportData: any = {};
      update((state) => {
        exportData = {
          timestamp: new Date().toISOString(),
          systemHealth: state.systemHealth,
          isOnline: state.isOnline,
          metrics: state.metrics,
          recentErrors: state.errors.slice(0, 50).map((error) => error.toJSON()),
          recoveryStrategies: state.recoveryStrategies,
        };
        return state;
      });
      return exportData;
    },

    /**
     * Destroy the store and clean up resources
     */
    destroy: () => {
      if (cleanupInterval) {
        clearInterval(cleanupInterval);
        cleanupInterval = null;
      }

      if (onlineListener && typeof window !== 'undefined') {
        window.removeEventListener('online', onlineListener);
      }

      if (offlineListener && typeof window !== 'undefined') {
        window.removeEventListener('offline', offlineListener);
      }

      set(initialState);
    },
  };
}

// Export the enhanced error store instance
export const errorStore = createEnhancedErrorStore();

// Enhanced convenience functions
export function addError(options: AppErrorOptions): AppError {
  return errorStore.addError(options);
}

export function addTimeoutError(command: string, timeoutMs: number): AppError {
  return errorStore.addError({
    message: 'Operation timed out',
    details: `The command ${command} took longer than ${timeoutMs}ms to complete. This might indicate network issues or server overload.`,
    category: ErrorCategory.TIMEOUT,
    severity: ErrorSeverity.WARNING,
    context: { command, timeoutMs, timestamp: Date.now() },
  });
}

export function addNetworkError(message: string, originalError?: unknown): AppError {
  return errorStore.addError({
    message: 'Network error',
    details: message,
    category: ErrorCategory.NETWORK,
    severity: ErrorSeverity.ERROR,
    originalError,
    context: {
      isOnline: typeof navigator !== 'undefined' ? navigator.onLine : true,
      timestamp: Date.now(),
    },
  });
}

export function addValidationError(message: string, context?: Record<string, unknown>): AppError {
  return errorStore.addError({
    message: 'Validation error',
    details: message,
    category: ErrorCategory.VALIDATION,
    severity: ErrorSeverity.WARNING,
    context: { ...context, timestamp: Date.now() },
  });
}

export function addApiError(command: string, originalError: unknown): AppError {
  return errorStore.addError({
    message: 'API call failed',
    details: `Command ${command} failed: ${originalError}`,
    category: ErrorCategory.API,
    severity: ErrorSeverity.ERROR,
    originalError,
    context: { command, timestamp: Date.now() },
  });
}

// Error pattern detection
export function detectErrorPatterns(): {
  cascadingFailures: boolean;
  highErrorRate: boolean;
  repeatedErrors: string[];
  systemDegraded: boolean;
} {
  const analytics = errorStore.getAnalytics();
  const systemHealth = errorStore.getSystemHealth();

  // Detect cascading failures (multiple errors in short time)
  const cascadingFailures = analytics.errorRate > 5;

  // Detect high error rate
  const highErrorRate = analytics.errorRate > 10;

  // Detect repeated errors
  const repeatedErrors = Object.entries(analytics.topErrorCommands)
    .filter(([_, count]) => count > 3)
    .map(([command, _]) => command);

  // System degraded
  const systemDegraded = systemHealth !== 'healthy';

  return {
    cascadingFailures,
    highErrorRate,
    repeatedErrors,
    systemDegraded,
  };
}
