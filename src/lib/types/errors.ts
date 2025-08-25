/**
 * Comprehensive error handling types for the Forbidden Library
 * Provides structured error categorization and severity levels
 */

export enum ErrorCategory {
  API = 'api',
  TIMEOUT = 'timeout',
  ENVIRONMENT = 'environment',
  VALIDATION = 'validation',
  DATA = 'data',
  NETWORK = 'network',
  PERMISSION = 'permission',
  UNKNOWN = 'unknown'
}

export enum ErrorSeverity {
  INFO = 'info',
  WARNING = 'warning',
  ERROR = 'error',
  CRITICAL = 'critical'
}

export interface AppErrorOptions {
  message: string;
  details?: string;
  category: ErrorCategory;
  severity: ErrorSeverity;
  originalError?: Error | unknown;
  timestamp?: number;
  context?: Record<string, unknown>;
  recoverable?: boolean;
}

export class AppError extends Error {
  public readonly id: string;
  public readonly details?: string;
  public readonly category: ErrorCategory;
  public readonly severity: ErrorSeverity;
  public readonly originalError?: Error | unknown;
  public readonly timestamp: number;
  public readonly context?: Record<string, unknown>;
  public readonly recoverable: boolean;
  public handled: boolean = false;

  constructor(options: AppErrorOptions) {
    super(options.message);
    this.name = 'AppError';
    this.id = `error_${Date.now()}_${Math.random().toString(36).substr(2, 9)}`;
    this.details = options.details;
    this.category = options.category;
    this.severity = options.severity;
    this.originalError = options.originalError;
    this.timestamp = options.timestamp || Date.now();
    this.context = options.context;
    this.recoverable = options.recoverable ?? true;

    // Maintain proper stack trace
    if (Error.captureStackTrace) {
      Error.captureStackTrace(this, AppError);
    }
  }

  /**
   * Mark this error as handled to prevent duplicate processing
   */
  markAsHandled(): void {
    this.handled = true;
  }

  /**
   * Convert error to a plain object for serialization
   */
  toJSON(): Record<string, unknown> {
    return {
      id: this.id,
      name: this.name,
      message: this.message,
      details: this.details,
      category: this.category,
      severity: this.severity,
      timestamp: this.timestamp,
      context: this.context,
      recoverable: this.recoverable,
      handled: this.handled,
      stack: this.stack
    };
  }

  /**
   * Create a user-friendly error message
   */
  getUserMessage(): string {
    switch (this.category) {
      case ErrorCategory.TIMEOUT:
        return 'The operation took too long to complete. Please try again.';
      case ErrorCategory.NETWORK:
        return 'Network connection issue. Please check your internet connection.';
      case ErrorCategory.PERMISSION:
        return 'Permission denied. Please check your access rights.';
      case ErrorCategory.VALIDATION:
        return this.details || 'Invalid input provided.';
      case ErrorCategory.ENVIRONMENT:
        return 'Feature not available in current environment.';
      default:
        return this.message || 'An unexpected error occurred.';
    }
  }
}

/**
 * Helper function to create AppError from unknown error
 */
export function createAppError(
  error: unknown,
  category: ErrorCategory = ErrorCategory.UNKNOWN,
  severity: ErrorSeverity = ErrorSeverity.ERROR,
  context?: Record<string, unknown>
): AppError {
  if (error instanceof AppError) {
    return error;
  }

  const message = error instanceof Error ? error.message : String(error);
  const details = error instanceof Error ? error.stack : undefined;

  return new AppError({
    message,
    details,
    category,
    severity,
    originalError: error,
    context
  });
}
