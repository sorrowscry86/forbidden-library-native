import { describe, it, expect, beforeEach, afterEach, vi } from 'vitest';
import { get } from 'svelte/store';
import { errorStore, addError, addTimeoutError, addNetworkError, addValidationError, addApiError } from './error-store';
import { AppError, ErrorCategory, ErrorSeverity } from '$lib/types/errors';

describe('error-store', () => {
  beforeEach(() => {
    // Clear all errors before each test
    errorStore.clearAllErrors();
  });

  afterEach(() => {
    // Clean up after each test
    errorStore.destroy();
  });

  describe('addError', () => {
    it('should add error to the store', () => {
      const error = errorStore.addError({
        message: 'Test error',
        details: 'Test details',
        category: ErrorCategory.API,
        severity: ErrorSeverity.ERROR
      });

      const state = get(errorStore);
      expect(state.errors).toHaveLength(1);
      expect(state.errors[0]).toBe(error);
      expect(state.lastError).toBe(error);
    });

    it('should accept AppError instance', () => {
      const appError = new AppError({
        message: 'Test error',
        category: ErrorCategory.VALIDATION,
        severity: ErrorSeverity.WARNING
      });

      errorStore.addError(appError);

      const state = get(errorStore);
      expect(state.errors).toHaveLength(1);
      expect(state.errors[0]).toBe(appError);
    });

    it('should maintain error order (newest first)', () => {
      const error1 = errorStore.addError({
        message: 'First error',
        category: ErrorCategory.API,
        severity: ErrorSeverity.ERROR
      });

      const error2 = errorStore.addError({
        message: 'Second error',
        category: ErrorCategory.NETWORK,
        severity: ErrorSeverity.ERROR
      });

      const state = get(errorStore);
      expect(state.errors).toHaveLength(2);
      expect(state.errors[0]).toBe(error2);
      expect(state.errors[1]).toBe(error1);
      expect(state.lastError).toBe(error2);
    });

    it('should log error to console', () => {
      const consoleErrorSpy = vi.spyOn(console, 'error').mockImplementation(() => {});

      errorStore.addError({
        message: 'Test error',
        category: ErrorCategory.API,
        severity: ErrorSeverity.ERROR
      });

      expect(consoleErrorSpy).toHaveBeenCalledWith(
        expect.stringContaining('[api]'), // lowercase as shown in the error store implementation
        expect.any(Object)
      );

      consoleErrorSpy.mockRestore();
    });
  });

  describe('dismissError', () => {
    it('should remove error by ID', () => {
      const error1 = errorStore.addError({
        message: 'Error 1',
        category: ErrorCategory.API,
        severity: ErrorSeverity.ERROR
      });

      const error2 = errorStore.addError({
        message: 'Error 2',
        category: ErrorCategory.NETWORK,
        severity: ErrorSeverity.ERROR
      });

      errorStore.dismissError(error1.id);

      const state = get(errorStore);
      expect(state.errors).toHaveLength(1);
      expect(state.errors[0]).toBe(error2);
    });

    it('should update lastError when dismissed error was last', () => {
      const error1 = errorStore.addError({
        message: 'Error 1',
        category: ErrorCategory.API,
        severity: ErrorSeverity.ERROR
      });

      const error2 = errorStore.addError({
        message: 'Error 2',
        category: ErrorCategory.NETWORK,
        severity: ErrorSeverity.ERROR
      });

      errorStore.dismissError(error2.id);

      const state = get(errorStore);
      expect(state.lastError).toBe(error1);
    });

    it('should set lastError to null when all errors dismissed', () => {
      const error = errorStore.addError({
        message: 'Error',
        category: ErrorCategory.API,
        severity: ErrorSeverity.ERROR
      });

      errorStore.dismissError(error.id);

      const state = get(errorStore);
      expect(state.lastError).toBeNull();
      expect(state.errors).toHaveLength(0);
    });
  });

  describe('markErrorAsHandled', () => {
    it('should mark error as handled', () => {
      const error = errorStore.addError({
        message: 'Test error',
        category: ErrorCategory.API,
        severity: ErrorSeverity.ERROR
      });

      expect(error.handled).toBe(false);

      errorStore.markErrorAsHandled(error.id);

      expect(error.handled).toBe(true);
    });

    it('should not affect other errors', () => {
      const error1 = errorStore.addError({
        message: 'Error 1',
        category: ErrorCategory.API,
        severity: ErrorSeverity.ERROR
      });

      const error2 = errorStore.addError({
        message: 'Error 2',
        category: ErrorCategory.NETWORK,
        severity: ErrorSeverity.ERROR
      });

      errorStore.markErrorAsHandled(error1.id);

      expect(error1.handled).toBe(true);
      expect(error2.handled).toBe(false);
    });
  });

  describe('clearAllErrors', () => {
    it('should remove all errors', () => {
      errorStore.addError({
        message: 'Error 1',
        category: ErrorCategory.API,
        severity: ErrorSeverity.ERROR
      });

      errorStore.addError({
        message: 'Error 2',
        category: ErrorCategory.NETWORK,
        severity: ErrorSeverity.ERROR
      });

      errorStore.clearAllErrors();

      const state = get(errorStore);
      expect(state.errors).toHaveLength(0);
      expect(state.lastError).toBeNull();
    });
  });

  describe('clearErrorsByCategory', () => {
    it('should remove errors only from specified category', () => {
      const apiError = errorStore.addError({
        message: 'API Error',
        category: ErrorCategory.API,
        severity: ErrorSeverity.ERROR
      });

      const networkError = errorStore.addError({
        message: 'Network Error',
        category: ErrorCategory.NETWORK,
        severity: ErrorSeverity.ERROR
      });

      errorStore.clearErrorsByCategory(ErrorCategory.API);

      const state = get(errorStore);
      expect(state.errors).toHaveLength(1);
      expect(state.errors[0]).toBe(networkError);
    });

    it('should update lastError correctly', () => {
      errorStore.addError({
        message: 'API Error',
        category: ErrorCategory.API,
        severity: ErrorSeverity.ERROR
      });

      const networkError = errorStore.addError({
        message: 'Network Error',
        category: ErrorCategory.NETWORK,
        severity: ErrorSeverity.ERROR
      });

      errorStore.clearErrorsByCategory(ErrorCategory.NETWORK);

      const state = get(errorStore);
      expect(state.lastError?.message).toBe('API Error');
    });
  });

  describe('getErrorsBySeverity', () => {
    it('should return errors of specified severity', () => {
      errorStore.addError({
        message: 'Error',
        category: ErrorCategory.API,
        severity: ErrorSeverity.ERROR
      });

      errorStore.addError({
        message: 'Warning',
        category: ErrorCategory.VALIDATION,
        severity: ErrorSeverity.WARNING
      });

      errorStore.addError({
        message: 'Critical',
        category: ErrorCategory.DATA,
        severity: ErrorSeverity.CRITICAL
      });

      const errors = errorStore.getErrorsBySeverity(ErrorSeverity.ERROR);
      expect(errors).toHaveLength(1);
      expect(errors[0].message).toBe('Error');
    });

    it('should return empty array when no errors match', () => {
      errorStore.addError({
        message: 'Warning',
        category: ErrorCategory.VALIDATION,
        severity: ErrorSeverity.WARNING
      });

      const errors = errorStore.getErrorsBySeverity(ErrorSeverity.CRITICAL);
      expect(errors).toHaveLength(0);
    });
  });

  describe('init and cleanup', () => {
    it('should clean up old handled errors', async () => {
      vi.useFakeTimers();

      const cleanup = errorStore.init(100, 1000); // 1 second max age

      const error = errorStore.addError({
        message: 'Test error',
        category: ErrorCategory.API,
        severity: ErrorSeverity.ERROR
      });

      errorStore.markErrorAsHandled(error.id);

      // Fast-forward time by 61 seconds (cleanup runs every 60 seconds)
      vi.advanceTimersByTime(61000);

      const state = get(errorStore);
      expect(state.errors).toHaveLength(0);

      cleanup();
      vi.useRealTimers();
    });

    it('should keep unhandled errors', async () => {
      vi.useFakeTimers();

      const cleanup = errorStore.init(100, 1000);

      errorStore.addError({
        message: 'Unhandled error',
        category: ErrorCategory.API,
        severity: ErrorSeverity.ERROR
      });

      // Fast-forward time
      vi.advanceTimersByTime(61000);

      const state = get(errorStore);
      expect(state.errors).toHaveLength(1);

      cleanup();
      vi.useRealTimers();
    });

    it('should limit total number of errors', async () => {
      vi.useFakeTimers();

      const cleanup = errorStore.init(5, 3600000); // Max 5 errors

      // Add 10 errors
      for (let i = 0; i < 10; i++) {
        const error = errorStore.addError({
          message: `Error ${i}`,
          category: ErrorCategory.API,
          severity: ErrorSeverity.ERROR
        });
        errorStore.markErrorAsHandled(error.id);
      }

      // Fast-forward time
      vi.advanceTimersByTime(61000);

      const state = get(errorStore);
      expect(state.errors.length).toBeLessThanOrEqual(5);

      cleanup();
      vi.useRealTimers();
    });
  });

  describe('convenience functions', () => {
    it('addTimeoutError should create timeout error', () => {
      const error = addTimeoutError('test_command', 5000);

      expect(error.category).toBe(ErrorCategory.TIMEOUT);
      expect(error.severity).toBe(ErrorSeverity.WARNING);
      expect(error.message).toBe('Operation timed out');
      expect(error.context).toMatchObject({
        command: 'test_command',
        timeoutMs: 5000
      });
    });

    it('addNetworkError should create network error', () => {
      const originalError = new Error('Connection failed');
      const error = addNetworkError('Failed to connect', originalError);

      expect(error.category).toBe(ErrorCategory.NETWORK);
      expect(error.severity).toBe(ErrorSeverity.ERROR);
      expect(error.message).toBe('Network error');
      expect(error.details).toBe('Failed to connect');
      expect(error.originalError).toBe(originalError);
    });

    it('addValidationError should create validation error', () => {
      const error = addValidationError('Invalid input', { field: 'email' });

      expect(error.category).toBe(ErrorCategory.VALIDATION);
      expect(error.severity).toBe(ErrorSeverity.WARNING);
      expect(error.message).toBe('Validation error');
      expect(error.context).toMatchObject({ field: 'email' });
    });

    it('addApiError should create API error', () => {
      const originalError = new Error('API failed');
      const error = addApiError('get_messages', originalError);

      expect(error.category).toBe(ErrorCategory.API);
      expect(error.severity).toBe(ErrorSeverity.ERROR);
      expect(error.message).toBe('API call failed');
      expect(error.context).toMatchObject({ command: 'get_messages' });
    });
  });

  describe('destroy', () => {
    it('should clean up all resources', () => {
      errorStore.addError({
        message: 'Test error',
        category: ErrorCategory.API,
        severity: ErrorSeverity.ERROR
      });

      errorStore.destroy();

      const state = get(errorStore);
      expect(state.errors).toHaveLength(0);
      expect(state.lastError).toBeNull();
    });
  });
});
