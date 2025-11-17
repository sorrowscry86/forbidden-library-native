import { describe, it, expect, beforeEach } from 'vitest';
import {
  AppError,
  ErrorCategory,
  ErrorSeverity,
  createAppError,
  type AppErrorOptions
} from './errors';

describe('errors', () => {
  describe('AppError', () => {
    const baseOptions: AppErrorOptions = {
      message: 'Test error',
      category: ErrorCategory.API,
      severity: ErrorSeverity.ERROR
    };

    it('should create an error with required fields', () => {
      const error = new AppError(baseOptions);

      expect(error.message).toBe('Test error');
      expect(error.category).toBe(ErrorCategory.API);
      expect(error.severity).toBe(ErrorSeverity.ERROR);
      expect(error.id).toBeDefined();
      expect(error.timestamp).toBeDefined();
      expect(error.handled).toBe(false);
      expect(error.recoverable).toBe(true); // default
    });

    it('should generate unique IDs for different errors', () => {
      const error1 = new AppError(baseOptions);
      const error2 = new AppError(baseOptions);

      expect(error1.id).not.toBe(error2.id);
    });

    it('should accept optional fields', () => {
      const originalError = new Error('Original');
      const error = new AppError({
        ...baseOptions,
        details: 'Additional details',
        originalError,
        context: { userId: '123' },
        recoverable: false,
        timestamp: 1234567890
      });

      expect(error.details).toBe('Additional details');
      expect(error.originalError).toBe(originalError);
      expect(error.context).toEqual({ userId: '123' });
      expect(error.recoverable).toBe(false);
      expect(error.timestamp).toBe(1234567890);
    });

    it('should have correct name', () => {
      const error = new AppError(baseOptions);
      expect(error.name).toBe('AppError');
    });

    it('should extend Error', () => {
      const error = new AppError(baseOptions);
      expect(error).toBeInstanceOf(Error);
      expect(error).toBeInstanceOf(AppError);
    });

    describe('markAsHandled', () => {
      it('should mark error as handled', () => {
        const error = new AppError(baseOptions);

        expect(error.handled).toBe(false);
        error.markAsHandled();
        expect(error.handled).toBe(true);
      });

      it('should be idempotent', () => {
        const error = new AppError(baseOptions);

        error.markAsHandled();
        error.markAsHandled();
        expect(error.handled).toBe(true);
      });
    });

    describe('toJSON', () => {
      it('should serialize to JSON object', () => {
        const error = new AppError({
          ...baseOptions,
          details: 'Test details',
          context: { key: 'value' }
        });

        const json = error.toJSON();

        expect(json).toMatchObject({
          id: error.id,
          name: 'AppError',
          message: 'Test error',
          details: 'Test details',
          category: ErrorCategory.API,
          severity: ErrorSeverity.ERROR,
          context: { key: 'value' },
          recoverable: true,
          handled: false
        });
        expect(json.timestamp).toBeDefined();
        expect(json.stack).toBeDefined();
      });

      it('should include handled state', () => {
        const error = new AppError(baseOptions);
        error.markAsHandled();

        const json = error.toJSON();
        expect(json.handled).toBe(true);
      });
    });

    describe('getUserMessage', () => {
      it('should return user-friendly message for timeout errors', () => {
        const error = new AppError({
          message: 'Timeout',
          category: ErrorCategory.TIMEOUT,
          severity: ErrorSeverity.WARNING
        });

        expect(error.getUserMessage()).toBe(
          'The operation took too long to complete. Please try again.'
        );
      });

      it('should return user-friendly message for network errors', () => {
        const error = new AppError({
          message: 'Network failed',
          category: ErrorCategory.NETWORK,
          severity: ErrorSeverity.ERROR
        });

        expect(error.getUserMessage()).toBe(
          'Network connection issue. Please check your internet connection.'
        );
      });

      it('should return user-friendly message for permission errors', () => {
        const error = new AppError({
          message: 'Access denied',
          category: ErrorCategory.PERMISSION,
          severity: ErrorSeverity.ERROR
        });

        expect(error.getUserMessage()).toBe(
          'Permission denied. Please check your access rights.'
        );
      });

      it('should return details for validation errors', () => {
        const error = new AppError({
          message: 'Validation failed',
          details: 'Email is required',
          category: ErrorCategory.VALIDATION,
          severity: ErrorSeverity.WARNING
        });

        expect(error.getUserMessage()).toBe('Email is required');
      });

      it('should return default message for validation errors without details', () => {
        const error = new AppError({
          message: 'Validation failed',
          category: ErrorCategory.VALIDATION,
          severity: ErrorSeverity.WARNING
        });

        expect(error.getUserMessage()).toBe('Invalid input provided.');
      });

      it('should return user-friendly message for environment errors', () => {
        const error = new AppError({
          message: 'Feature unavailable',
          category: ErrorCategory.ENVIRONMENT,
          severity: ErrorSeverity.WARNING
        });

        expect(error.getUserMessage()).toBe(
          'Feature not available in current environment.'
        );
      });

      it('should return original message for unknown errors', () => {
        const error = new AppError({
          message: 'Something went wrong',
          category: ErrorCategory.UNKNOWN,
          severity: ErrorSeverity.ERROR
        });

        expect(error.getUserMessage()).toBe('Something went wrong');
      });

      it('should return default message for API errors', () => {
        const error = new AppError({
          message: 'API call failed',
          category: ErrorCategory.API,
          severity: ErrorSeverity.ERROR
        });

        expect(error.getUserMessage()).toBe('API call failed');
      });
    });
  });

  describe('createAppError', () => {
    it('should return existing AppError instance', () => {
      const originalError = new AppError({
        message: 'Original error',
        category: ErrorCategory.API,
        severity: ErrorSeverity.ERROR
      });

      const result = createAppError(originalError);

      expect(result).toBe(originalError);
    });

    it('should create AppError from Error instance', () => {
      const error = new Error('Test error');
      const appError = createAppError(error);

      expect(appError).toBeInstanceOf(AppError);
      expect(appError.message).toBe('Test error');
      expect(appError.category).toBe(ErrorCategory.UNKNOWN);
      expect(appError.severity).toBe(ErrorSeverity.ERROR);
      expect(appError.originalError).toBe(error);
    });

    it('should create AppError from string', () => {
      const appError = createAppError('Simple error message');

      expect(appError).toBeInstanceOf(AppError);
      expect(appError.message).toBe('Simple error message');
      expect(appError.category).toBe(ErrorCategory.UNKNOWN);
      expect(appError.severity).toBe(ErrorSeverity.ERROR);
    });

    it('should accept custom category and severity', () => {
      const error = new Error('Network error');
      const appError = createAppError(
        error,
        ErrorCategory.NETWORK,
        ErrorSeverity.WARNING
      );

      expect(appError.category).toBe(ErrorCategory.NETWORK);
      expect(appError.severity).toBe(ErrorSeverity.WARNING);
    });

    it('should accept context', () => {
      const error = new Error('API error');
      const context = { endpoint: '/api/users', method: 'GET' };
      const appError = createAppError(
        error,
        ErrorCategory.API,
        ErrorSeverity.ERROR,
        context
      );

      expect(appError.context).toEqual(context);
    });

    it('should include stack trace from Error', () => {
      const error = new Error('Test error');
      const appError = createAppError(error);

      expect(appError.details).toBeDefined();
      expect(appError.details).toContain('Error: Test error');
    });

    it('should handle non-Error objects', () => {
      const appError = createAppError({ custom: 'object' });

      expect(appError.message).toBe('[object Object]');
      expect(appError.originalError).toEqual({ custom: 'object' });
    });

    it('should handle null', () => {
      const appError = createAppError(null);

      expect(appError.message).toBe('null');
    });

    it('should handle undefined', () => {
      const appError = createAppError(undefined);

      expect(appError.message).toBe('undefined');
    });

    it('should handle numbers', () => {
      const appError = createAppError(42);

      expect(appError.message).toBe('42');
    });
  });

  describe('ErrorCategory enum', () => {
    it('should have all expected categories', () => {
      expect(ErrorCategory.API).toBe('api');
      expect(ErrorCategory.TIMEOUT).toBe('timeout');
      expect(ErrorCategory.ENVIRONMENT).toBe('environment');
      expect(ErrorCategory.VALIDATION).toBe('validation');
      expect(ErrorCategory.DATA).toBe('data');
      expect(ErrorCategory.NETWORK).toBe('network');
      expect(ErrorCategory.PERMISSION).toBe('permission');
      expect(ErrorCategory.UNKNOWN).toBe('unknown');
    });
  });

  describe('ErrorSeverity enum', () => {
    it('should have all expected severities', () => {
      expect(ErrorSeverity.INFO).toBe('info');
      expect(ErrorSeverity.WARNING).toBe('warning');
      expect(ErrorSeverity.ERROR).toBe('error');
      expect(ErrorSeverity.CRITICAL).toBe('critical');
    });
  });
});
