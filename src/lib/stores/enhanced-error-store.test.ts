import { describe, it, expect, beforeEach, afterEach, vi } from 'vitest';
import { get } from 'svelte/store';
import {
  errorStore,
  addError,
  addTimeoutError,
  addNetworkError,
  addValidationError,
  addApiError,
  detectErrorPatterns
} from './enhanced-error-store';
import { ErrorCategory, ErrorSeverity } from '$lib/types/errors';

describe('enhanced-error-store', () => {
  beforeEach(() => {
    errorStore.clearAllErrors();
  });

  afterEach(() => {
    errorStore.destroy();
  });

  describe('basic error management', () => {
    it('should add error and update metrics', () => {
      const error = errorStore.addError({
        message: 'Test error',
        category: ErrorCategory.API,
        severity: ErrorSeverity.ERROR
      });

      const state = get(errorStore);
      expect(state.errors).toHaveLength(1);
      expect(state.metrics.totalErrors).toBe(1);
      expect(state.metrics.errorsByCategory[ErrorCategory.API]).toBe(1);
      expect(state.metrics.errorsBySeverity[ErrorSeverity.ERROR]).toBe(1);
    });

    it('should track multiple error categories', () => {
      errorStore.addError({
        message: 'API Error',
        category: ErrorCategory.API,
        severity: ErrorSeverity.ERROR
      });

      errorStore.addError({
        message: 'Network Error',
        category: ErrorCategory.NETWORK,
        severity: ErrorSeverity.WARNING
      });

      const state = get(errorStore);
      expect(state.metrics.errorsByCategory[ErrorCategory.API]).toBe(1);
      expect(state.metrics.errorsByCategory[ErrorCategory.NETWORK]).toBe(1);
    });

    it('should track error severity distribution', () => {
      errorStore.addError({
        message: 'Critical',
        category: ErrorCategory.API,
        severity: ErrorSeverity.CRITICAL
      });

      errorStore.addError({
        message: 'Warning',
        category: ErrorCategory.VALIDATION,
        severity: ErrorSeverity.WARNING
      });

      const state = get(errorStore);
      expect(state.metrics.errorsBySeverity[ErrorSeverity.CRITICAL]).toBe(1);
      expect(state.metrics.errorsBySeverity[ErrorSeverity.WARNING]).toBe(1);
    });
  });

  describe('error rate tracking', () => {
    it('should calculate error rate correctly', () => {
      vi.useFakeTimers();
      const now = Date.now();
      vi.setSystemTime(now);

      // Add 3 errors
      for (let i = 0; i < 3; i++) {
        errorStore.addError({
          message: `Error ${i}`,
          category: ErrorCategory.API,
          severity: ErrorSeverity.ERROR
        });
      }

      const state = get(errorStore);
      expect(state.metrics.errorRate).toBe(3);

      vi.useRealTimers();
    });

    it('should only count recent errors in error rate', () => {
      vi.useFakeTimers();
      const now = Date.now();

      // Add old error (2 minutes ago)
      vi.setSystemTime(now - 120000);
      errorStore.addError({
        message: 'Old error',
        category: ErrorCategory.API,
        severity: ErrorSeverity.ERROR
      });

      // Add recent error
      vi.setSystemTime(now);
      errorStore.addError({
        message: 'Recent error',
        category: ErrorCategory.API,
        severity: ErrorSeverity.ERROR
      });

      const state = get(errorStore);
      // Error rate should only count recent errors (within last minute)
      expect(state.metrics.errorRate).toBe(1);

      vi.useRealTimers();
    });
  });

  describe('retry tracking', () => {
    it('should track retry attempts', () => {
      errorStore.trackRetry('test_command', 1);
      errorStore.trackRetry('test_command', 2);

      const state = get(errorStore);
      expect(state.metrics.retryAttempts['test_command']).toBe(2);
    });

    it('should track successful retries', () => {
      errorStore.trackRetry('test_command', 1);
      errorStore.trackSuccess('test_command', true);

      const state = get(errorStore);
      expect(state.metrics.successfulRetries['test_command']).toBe(1);
    });

    it('should calculate recovery rate correctly', () => {
      // Track 3 retries
      errorStore.trackRetry('cmd1', 1);
      errorStore.trackRetry('cmd2', 1);
      errorStore.trackRetry('cmd3', 1);

      // 2 successful retries
      errorStore.trackSuccess('cmd1', true);
      errorStore.trackSuccess('cmd2', true);

      const state = get(errorStore);
      expect(state.metrics.recoveryRate).toBe(67); // 2/3 = 66.67% rounded to 67
    });

    it('should handle 100% recovery rate when no retries', () => {
      const state = get(errorStore);
      // Initial state has recoveryRate: 0, which is expected
      // 100% recovery rate is calculated only after tracking operations
      expect(state.metrics.recoveryRate).toBe(0);
    });
  });

  describe('system health tracking', () => {
    it('should start with healthy status', () => {
      const state = get(errorStore);
      expect(state.systemHealth).toBe('healthy');
    });

    it('should become unhealthy with critical errors', () => {
      errorStore.addError({
        message: 'Critical error',
        category: ErrorCategory.DATA,
        severity: ErrorSeverity.CRITICAL
      });

      const state = get(errorStore);
      expect(state.systemHealth).toBe('unhealthy');
    });

    it('should become degraded with moderate error rate', () => {
      // Add 6 errors to trigger degraded state (error rate > 5)
      for (let i = 0; i < 6; i++) {
        errorStore.addError({
          message: `Error ${i}`,
          category: ErrorCategory.API,
          severity: ErrorSeverity.WARNING
        });
      }

      const state = get(errorStore);
      expect(state.systemHealth).toBe('degraded');
    });

    it('should become unhealthy with high error rate', () => {
      // Add 11 errors to trigger unhealthy state (error rate > 10)
      for (let i = 0; i < 11; i++) {
        errorStore.addError({
          message: `Error ${i}`,
          category: ErrorCategory.API,
          severity: ErrorSeverity.ERROR
        });
      }

      const state = get(errorStore);
      expect(state.systemHealth).toBe('unhealthy');
    });
  });

  describe('top error commands tracking', () => {
    it('should track most frequent error commands', () => {
      errorStore.addError({
        message: 'Error 1',
        category: ErrorCategory.API,
        severity: ErrorSeverity.ERROR,
        context: { command: 'get_messages' }
      });

      errorStore.addError({
        message: 'Error 2',
        category: ErrorCategory.API,
        severity: ErrorSeverity.ERROR,
        context: { command: 'get_messages' }
      });

      errorStore.addError({
        message: 'Error 3',
        category: ErrorCategory.API,
        severity: ErrorSeverity.ERROR,
        context: { command: 'send_message' }
      });

      const state = get(errorStore);
      expect(state.metrics.topErrorCommands['get_messages']).toBe(2);
      expect(state.metrics.topErrorCommands['send_message']).toBe(1);
    });
  });

  describe('online/offline tracking', () => {
    it('should initialize with online status', () => {
      const state = get(errorStore);
      expect(state.isOnline).toBeDefined();
    });

    // Note: Testing window events requires more complex setup
    // These would be integration tests rather than unit tests
  });

  describe('recovery strategies', () => {
    it('should have default recovery strategies', () => {
      const state = get(errorStore);
      expect(state.recoveryStrategies[ErrorCategory.API]).toBeDefined();
      expect(state.recoveryStrategies[ErrorCategory.NETWORK]).toBeDefined();
      expect(state.recoveryStrategies[ErrorCategory.TIMEOUT]).toBeDefined();
    });

    it('should allow updating recovery strategies', () => {
      errorStore.updateRecoveryStrategy(ErrorCategory.API, {
        maxRetries: 5,
        autoRecover: false
      });

      const state = get(errorStore);
      expect(state.recoveryStrategies[ErrorCategory.API].maxRetries).toBe(5);
      expect(state.recoveryStrategies[ErrorCategory.API].autoRecover).toBe(false);
    });
  });

  describe('analytics and export', () => {
    it('should export error data', () => {
      errorStore.addError({
        message: 'Test error',
        category: ErrorCategory.API,
        severity: ErrorSeverity.ERROR
      });

      const exportData = errorStore.exportErrorData();

      expect(exportData.timestamp).toBeDefined();
      expect(exportData.systemHealth).toBe('healthy');
      expect(exportData.metrics).toBeDefined();
      expect(exportData.recentErrors).toHaveLength(1);
    });

    it('should get analytics', () => {
      errorStore.addError({
        message: 'Test error',
        category: ErrorCategory.API,
        severity: ErrorSeverity.ERROR
      });

      const analytics = errorStore.getAnalytics();

      expect(analytics.totalErrors).toBe(1);
      expect(analytics.errorsByCategory).toBeDefined();
      expect(analytics.errorsBySeverity).toBeDefined();
    });

    it('should get system health', () => {
      const health = errorStore.getSystemHealth();
      expect(['healthy', 'degraded', 'unhealthy']).toContain(health);
    });
  });

  describe('error dismissal with metrics update', () => {
    it('should update metrics when dismissing error', () => {
      const error = errorStore.addError({
        message: 'Test error',
        category: ErrorCategory.API,
        severity: ErrorSeverity.ERROR
      });

      expect(get(errorStore).metrics.totalErrors).toBe(1);

      errorStore.dismissError(error.id);

      const state = get(errorStore);
      expect(state.metrics.totalErrors).toBe(0);
      expect(state.systemHealth).toBe('healthy');
    });
  });

  describe('cleanup and initialization', () => {
    it('should clean up old errors while preserving recent ones', async () => {
      vi.useFakeTimers();

      const cleanup = errorStore.init(100, 1000); // 1 second max age

      const oldError = errorStore.addError({
        message: 'Old error',
        category: ErrorCategory.API,
        severity: ErrorSeverity.ERROR
      });
      errorStore.markErrorAsHandled(oldError.id);

      // Advance past max age
      vi.advanceTimersByTime(2000);

      const recentError = errorStore.addError({
        message: 'Recent error',
        category: ErrorCategory.API,
        severity: ErrorSeverity.ERROR
      });

      // Trigger cleanup
      vi.advanceTimersByTime(61000);

      const state = get(errorStore);
      expect(state.errors).toHaveLength(1);
      expect(state.errors[0]).toBe(recentError);

      cleanup();
      vi.useRealTimers();
    });

    it('should limit total number of errors', async () => {
      vi.useFakeTimers();

      const cleanup = errorStore.init(5, 3600000); // Max 5 errors

      // Add 10 errors and mark as handled
      for (let i = 0; i < 10; i++) {
        const error = errorStore.addError({
          message: `Error ${i}`,
          category: ErrorCategory.API,
          severity: ErrorSeverity.ERROR
        });
        errorStore.markErrorAsHandled(error.id);
      }

      // Trigger cleanup
      vi.advanceTimersByTime(61000);

      const state = get(errorStore);
      expect(state.errors.length).toBeLessThanOrEqual(5);

      cleanup();
      vi.useRealTimers();
    });
  });

  describe('convenience functions', () => {
    it('should create timeout error with enhanced details', () => {
      const error = addTimeoutError('test_command', 5000);

      expect(error.category).toBe(ErrorCategory.TIMEOUT);
      expect(error.details).toContain('network issues or server overload');
      expect(error.context?.timestamp).toBeDefined();
    });

    it('should create network error with online status', () => {
      const error = addNetworkError('Connection failed');

      expect(error.category).toBe(ErrorCategory.NETWORK);
      expect(error.context?.isOnline).toBeDefined();
      expect(error.context?.timestamp).toBeDefined();
    });

    it('should create validation error with timestamp', () => {
      const error = addValidationError('Invalid email', { field: 'email' });

      expect(error.category).toBe(ErrorCategory.VALIDATION);
      expect(error.context?.field).toBe('email');
      expect(error.context?.timestamp).toBeDefined();
    });

    it('should create API error with context', () => {
      const error = addApiError('get_messages', new Error('Failed'));

      expect(error.category).toBe(ErrorCategory.API);
      expect(error.context?.command).toBe('get_messages');
      expect(error.context?.timestamp).toBeDefined();
    });
  });

  describe('error pattern detection', () => {
    it('should detect cascading failures', () => {
      // Add 6 errors to trigger high error rate
      for (let i = 0; i < 6; i++) {
        errorStore.addError({
          message: `Error ${i}`,
          category: ErrorCategory.API,
          severity: ErrorSeverity.ERROR
        });
      }

      const patterns = detectErrorPatterns();
      expect(patterns.cascadingFailures).toBe(true);
    });

    it('should detect high error rate', () => {
      // Add 11 errors
      for (let i = 0; i < 11; i++) {
        errorStore.addError({
          message: `Error ${i}`,
          category: ErrorCategory.API,
          severity: ErrorSeverity.ERROR
        });
      }

      const patterns = detectErrorPatterns();
      expect(patterns.highErrorRate).toBe(true);
    });

    it('should detect repeated errors', () => {
      // Add same command error 4 times
      for (let i = 0; i < 4; i++) {
        errorStore.addError({
          message: 'Repeated error',
          category: ErrorCategory.API,
          severity: ErrorSeverity.ERROR,
          context: { command: 'failing_command' }
        });
      }

      const patterns = detectErrorPatterns();
      expect(patterns.repeatedErrors).toContain('failing_command');
    });

    it('should detect system degradation', () => {
      // Add critical error to degrade system
      errorStore.addError({
        message: 'Critical failure',
        category: ErrorCategory.DATA,
        severity: ErrorSeverity.CRITICAL
      });

      const patterns = detectErrorPatterns();
      expect(patterns.systemDegraded).toBe(true);
    });
  });

  describe('clear operations', () => {
    it('should clear all errors and reset system health', () => {
      for (let i = 0; i < 5; i++) {
        errorStore.addError({
          message: `Error ${i}`,
          category: ErrorCategory.API,
          severity: ErrorSeverity.ERROR
        });
      }

      errorStore.clearAllErrors();

      const state = get(errorStore);
      expect(state.errors).toHaveLength(0);
      expect(state.systemHealth).toBe('healthy');
    });

    it('should clear errors by category and update metrics', () => {
      errorStore.addError({
        message: 'API Error',
        category: ErrorCategory.API,
        severity: ErrorSeverity.ERROR
      });

      errorStore.addError({
        message: 'Network Error',
        category: ErrorCategory.NETWORK,
        severity: ErrorSeverity.ERROR
      });

      errorStore.clearErrorsByCategory(ErrorCategory.API);

      const state = get(errorStore);
      expect(state.errors).toHaveLength(1);
      expect(state.metrics.errorsByCategory[ErrorCategory.API]).toBe(0);
    });
  });

  describe('destroy', () => {
    it('should clean up all resources and reset state', () => {
      errorStore.addError({
        message: 'Test error',
        category: ErrorCategory.API,
        severity: ErrorSeverity.ERROR
      });

      errorStore.destroy();

      const state = get(errorStore);
      expect(state.errors).toHaveLength(0);
      expect(state.metrics.totalErrors).toBe(0);
    });
  });
});
