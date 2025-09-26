/**
 * Enhanced API service with advanced error handling, recovery, and resilience patterns
 */

import { invoke } from '@tauri-apps/api';
import {
  safeInvoke,
  createMockConversations,
  createMockConversation,
  isTauriAvailable,
} from '/utils/enhanced-tauri-detection';
import { AppError, ErrorCategory, ErrorSeverity } from '/types/errors';
import { errorStore } from '/stores/enhanced-error-store';

// Circuit breaker states
enum CircuitState {
  CLOSED = 'closed',
  OPEN = 'open',
  HALF_OPEN = 'half_open',
}

// Circuit breaker configuration
interface CircuitBreakerConfig {
  failureThreshold: number;
  recoveryTimeout: number;
  monitoringPeriod: number;
}

// Circuit breaker for each command
class CircuitBreaker {
  private state: CircuitState = CircuitState.CLOSED;
  private failureCount = 0;
  private lastFailureTime = 0;
  private successCount = 0;

  constructor(private config: CircuitBreakerConfig) {}

  async execute<T>(operation: () => Promise<T>, commandContext?: { command: string; args?: Record<string, unknown> }): Promise<T> {
    if (this.state === CircuitState.OPEN) {
      if (Date.now() - this.lastFailureTime > this.config.recoveryTimeout) {
        this.state = CircuitState.HALF_OPEN;
        this.successCount = 0;
      } else {
        const waitTimeMs = this.config.recoveryTimeout - (Date.now() - this.lastFailureTime);
        throw new AppError({
          message: 'Circuit breaker is open',
          details: Too many recent failures, circuit breaker is protecting the system. Will retry in  seconds.,
          category: ErrorCategory.API,
          severity: ErrorSeverity.WARNING,
          recoverable: true,
          context: {
            circuitState: this.state,
            failureCount: this.failureCount,
            recoveryTimeRemaining: waitTimeMs,
            ...(commandContext || {})
          },
        });
      }
    }

    try {
      const result = await operation();
      this.onSuccess();
      return result;
    } catch (error) {
      this.onFailure();
      throw error;
    }
  }

  private onSuccess() {
    this.failureCount = 0;
    if (this.state === CircuitState.HALF_OPEN) {
      this.successCount++;
      if (this.successCount >= 3) {
        // Require 3 successes to close
        this.state = CircuitState.CLOSED;
      }
    }
  }

  private onFailure() {
    this.failureCount++;
    this.lastFailureTime = Date.now();

    if (this.failureCount >= this.config.failureThreshold) {
      this.state = CircuitState.OPEN;
    }
  }

  getState(): CircuitState {
    return this.state;
  }

  getMetrics() {
    return {
      state: this.state,
      failureCount: this.failureCount,
      lastFailureTime: this.lastFailureTime,
      successCount: this.successCount,
    };
  }
}

// Circuit breakers for different commands
const circuitBreakers = new Map<string, CircuitBreaker>();

function getCircuitBreaker(command: string): CircuitBreaker {
  if (!circuitBreakers.has(command)) {
    circuitBreakers.set(
      command,
      new CircuitBreaker({
        failureThreshold: 5,
        recoveryTimeout: 30000, // 30 seconds
        monitoringPeriod: 60000, // 1 minute
      })
    );
  }
  return circuitBreakers.get(command)!;
}

// Enhanced retry configuration
interface RetryConfig {
  maxRetries: number;
  baseDelayMs: number;
  maxDelayMs: number;
  backoffMultiplier: number;
  jitterMs: number;
  retryableErrors: ErrorCategory[];
}

const DEFAULT_RETRY_CONFIG: RetryConfig = {
  maxRetries: 3,
  baseDelayMs: 1000,
  maxDelayMs: 30000,
  backoffMultiplier: 2,
  jitterMs: 100,
  retryableErrors: [ErrorCategory.TIMEOUT, ErrorCategory.NETWORK, ErrorCategory.API],
};

// Logging interface for better testability and flexibility
interface Logger {
  warn(message: string, context?: Record<string, unknown>): void;
  error(message: string, context?: Record<string, unknown>): void;
  info(message: string, context?: Record<string, unknown>): void;
}

// Default console logger implementation
const consoleLogger: Logger = {
  warn: (message, context) => console.warn(message, context),
  error: (message, context) => console.error(message, context),
  info: (message, context) => console.info(message, context),
};

// Utility function to calculate progressive timeout
function calculateProgressiveTimeout(
  baseTimeoutMs: number = 8000,
  attempt: number = 0,
  maxTimeoutMs: number = 30000
): number {
  return Math.min(
    baseTimeoutMs * Math.pow(1.5, attempt),
    maxTimeoutMs
  );
}

// Enhanced timeout with progressive timeout
export async function invokeWithEnhancedTimeout<T>(
  command: string,
  args?: Record<string, unknown>,
  timeoutMs: number = 8000,
  progressiveTimeout: boolean = true
): Promise<T> {
  // Use circuit breaker
  const circuitBreaker = getCircuitBreaker(command);

  return circuitBreaker.execute(async () => {
    // If Tauri is not available, use safe invoke with fallbacks
    if (!isTauriAvailable()) {
      return safeInvoke<T>(command, args, () => {
        // Enhanced fallbacks with better error messages
        switch (command) {
          case 'get_conversations':
            return createMockConversations() as T;
          case 'create_conversation':
            return createMockConversation() as T;
          case 'get_messages':
            return [] as T;
          case 'send_message':
            return {
              success: true,
              message: 'Demo message sent (web mode)',
              id: Date.now(),
              timestamp: new Date().toISOString(),
            } as T;
          default:
            throw new AppError({
              message: 'Feature not available in web mode',
              details: The command  requires the desktop application. Please install and run the desktop version for full functionality.,
              category: ErrorCategory.ENVIRONMENT,
              severity: ErrorSeverity.WARNING,
              context: {
                command,
                environment: 'web',
                availableCommands: [
                  'get_conversations',
                  'create_conversation',
                  'get_messages',
                  'send_message',
                ],
              },
            });
        }
      });
    }

    // Progressive timeout: increase timeout for subsequent retries
    const effectiveTimeout = progressiveTimeout ? timeoutMs : timeoutMs;
    let timeoutHandle: ReturnType<typeof setTimeout> | undefined;

    const timeoutPromise = new Promise<never>((_, reject) => {
      timeoutHandle = setTimeout(() => {
        reject(
          new AppError({
            message: 'Operation timed out',
            details: The command  exceeded the timeout of ms,
            category: ErrorCategory.TIMEOUT,
            severity: ErrorSeverity.WARNING,
            context: { command, timeoutMs: effectiveTimeout, args },
          })
        );
      }, effectiveTimeout);
    });

    try {
      const result = (await Promise.race([invoke<T>(command, args), timeoutPromise])) as T;

      // Track successful operation
      errorStore.trackSuccess(command);
      return result;
    } catch (error) {
      // Enhanced error categorization with more specific detection
      if (error instanceof AppError) {
        throw error;
      }

      const errorMessage = error instanceof Error ? error.message : String(error);

      // More sophisticated error categorization
      if (errorMessage.includes('timeout') || errorMessage.includes('timed out')) {
        throw new AppError({
          message: 'Operation timed out',
          details: The command  took too long to respond (>ms). This might indicate network issues or server overload.,
          category: ErrorCategory.TIMEOUT,
          severity: ErrorSeverity.WARNING,
          originalError: error,
          context: {
            command,
            timeoutMs: effectiveTimeout,
            circuitBreakerState: circuitBreaker.getState(),
          },
        });
      } else if (errorMessage.includes('not available') || errorMessage.includes('not found')) {
        throw new AppError({
          message: 'Feature not available',
          details: The command  is not available in this environment or version,
          category: ErrorCategory.ENVIRONMENT,
          severity: ErrorSeverity.ERROR,
          originalError: error,
          context: { command, environment: isTauriAvailable() ? 'tauri' : 'web' },
        });
      } else if (
        errorMessage.includes('permission') ||
        errorMessage.includes('denied') ||
        errorMessage.includes('unauthorized')
      ) {
        throw new AppError({
          message: 'Permission denied',
          details: Access denied for command . Please check your permissions or authentication.,
          category: ErrorCategory.PERMISSION,
          severity: ErrorSeverity.ERROR,
          originalError: error,
          context: { command },
        });
      } else if (
        errorMessage.includes('network') ||
        errorMessage.includes('connection') ||
        errorMessage.includes('fetch')
      ) {
        throw new AppError({
          message: 'Network error',
          details: Network connectivity issue while executing ,
          category: ErrorCategory.NETWORK,
          severity: ErrorSeverity.ERROR,
          originalError: error,
          context: { command },
        });
      } else {
        throw new AppError({
          message: 'API operation failed',
          details: errorMessage || Unknown error occurred while executing ,
          category: ErrorCategory.API,
          severity: ErrorSeverity.ERROR,
          originalError: error,
          context: { command, args, circuitBreakerMetrics: circuitBreaker.getMetrics() },
        });
      }
    } finally {
      if (timeoutHandle) clearTimeout(timeoutHandle);
    }
  }, { command, args });
}

export function ms(seconds: number): number {
  return Math.round(seconds * 1000);
}

// Predefined retry configuration presets for common scenarios
export const RETRY_PRESETS = {
  AGGRESSIVE: {
    maxRetries: 5,
    baseDelayMs: 500,
    maxDelayMs: 10000,
    backoffMultiplier: 1.5,
    jitterMs: 100,
    retryableErrors: [
      ErrorCategory.TIMEOUT,
      ErrorCategory.NETWORK,
      ErrorCategory.API
    ],
  },
  CONSERVATIVE: {
    maxRetries: 2,
    baseDelayMs: 1000,
    maxDelayMs: 5000,
    backoffMultiplier: 2,
    jitterMs: 50,
    retryableErrors: [
      ErrorCategory.TIMEOUT,
      ErrorCategory.NETWORK
    ],
  },
  MINIMAL: {
    maxRetries: 1,
    baseDelayMs: 1000,
    maxDelayMs: 2000,
    backoffMultiplier: 1,
    jitterMs: 0,
    retryableErrors: [
      ErrorCategory.TIMEOUT
    ],
  },
} as const;

// Enhanced retry with intelligent backoff and error analysis
export async function invokeWithIntelligentRetry<T>(
  command: string,
  args?: Record<string, unknown>,
  presetOrConfig: keyof typeof RETRY_PRESETS | Partial<RetryConfig> = 'CONSERVATIVE',
  logger: Logger = consoleLogger
): Promise<T> {
  // Determine if a preset was used
  const retryConfig = typeof presetOrConfig === 'string'
    ? RETRY_PRESETS[presetOrConfig] as RetryConfig
    : { ...DEFAULT_RETRY_CONFIG, ...presetOrConfig };

  let lastError: AppError | Error | undefined;
  let attempt = 0;

  while (attempt <= retryConfig.maxRetries) {
    try {
      // Progressive timeout: increase timeout for each retry
      const timeoutMs = calculateProgressiveTimeout(8000, attempt);

      return await invokeWithEnhancedTimeout<T>(command, args, timeoutMs, true);
    } catch (error) {
      lastError = error instanceof AppError ? error : new Error(String(error));
      attempt++;

      // Check if error is retryable
      if (error instanceof AppError && !retryConfig.retryableErrors.includes(error.category)) {
        throw error;
      }

      // If this was the last attempt, throw the error
      if (attempt > retryConfig.maxRetries) {
        break;
      }

      // Calculate delay with exponential backoff and jitter
      const baseDelay = Math.min(
        retryConfig.baseDelayMs * Math.pow(retryConfig.backoffMultiplier, attempt - 1),
        retryConfig.maxDelayMs
      );

      // Add jitter to prevent thundering herd
      const jitter = Math.random() * retryConfig.jitterMs;
      const delay = baseDelay + jitter;

      // Log retry attempt with context
      logger.warn(
        Command  failed (attempt /), retrying in ms...,
        {
          error: error instanceof AppError ? error.toJSON() : error,
          retryConfig,
        }
      );

      // Track retry attempt
      errorStore.trackRetry(command, attempt, error instanceof AppError ? error : undefined);

      // Wait before retrying
      await new Promise((resolve) => setTimeout(resolve, delay));
    }
  }

  // All retries failed, throw the last error with enhanced context
  if (lastError && lastError instanceof AppError) {
    throw new AppError({
      ...lastError,
      message: ${lastError.message} (after  retries),
      context: {
        ...lastError.context,
        totalAttempts: attempt,
        retryConfig,
      },
    });
  } else {
    throw new AppError({
      message: Command failed after  retries,
      details: lastError ? lastError.message : 'Unknown error',
      category: ErrorCategory.API,
      severity: ErrorSeverity.ERROR,
      originalError: lastError,
      context: { command, args, totalAttempts: attempt, retryConfig },
    });
  }
}

// Enhanced validation with schema support
interface ValidationSchema {
  [key: string]: {
    type: 'string' | 'number' | 'boolean' | 'object' | 'array';
    required?: boolean;
    min?: number;
    max?: number;
    pattern?: RegExp;
    validator?: (value: unknown) => boolean | string;
  };
}

// Improved validation result type for better type safety
interface ValidationSuccess {
  success: true;
}

interface ValidationFailure {
  success: false;
  message: string;
}

type ValidationResult = ValidationSuccess | ValidationFailure;

export async function invokeWithSchemaValidation<T>(
  command: string,
  args: unknown,
  schema: ValidationSchema,
  timeoutMs: number = 8000
): Promise<T> {
  // Enhanced schema validation
  const validationResult = validateWithSchema(args, schema);
  if (!validationResult.success) {
    throw new AppError({
      message: 'Validation failed',
      details: validationResult.message,
      category: ErrorCategory.VALIDATION,
      severity: ErrorSeverity.WARNING,
      context: { command, args, schema },
    });
  }

  return invokeWithEnhancedTimeout<T>(command, args as Record<string, unknown>, timeoutMs);
}

function validateWithSchema(args: unknown, schema: ValidationSchema): ValidationResult {
  if (!args || typeof args !== 'object') {
    return { success: false, message: 'Invalid arguments: expected object' };
  }

  const data = args as Record<string, unknown>;

  for (const [key, rules] of Object.entries(schema)) {
    const value = data[key];

    // Check required fields
    if (rules.required && (value === undefined || value === null)) {
      return { success: false, message: Missing required field:  };
    }

    // Skip validation for optional undefined fields
    if (value === undefined || value === null) continue;

    // Type validation
    if (rules.type === 'string' && typeof value !== 'string') {
      return { success: false, message: Field  must be a string };
    }
    if (rules.type === 'number' && typeof value !== 'number') {
      return { success: false, message: Field  must be a number };
    }
    if (rules.type === 'boolean' && typeof value !== 'boolean') {
      return { success: false, message: Field  must be a boolean };
    }
    if (rules.type === 'object' && (typeof value !== 'object' || Array.isArray(value))) {
      return { success: false, message: Field  must be an object };
    }
    if (rules.type === 'array' && !Array.isArray(value)) {
      return { success: false, message: Field  must be an array };
    }

    // Range validation
    if (rules.min !== undefined) {
      if (typeof value === 'string' && value.length < rules.min) {
        return { success: false, message: Field  must be at least  characters long };
      }
      if (typeof value === 'number' && value < rules.min) {
        return { success: false, message: Field  must be at least  };
      }
    }

    if (rules.max !== undefined) {
      if (typeof value === 'string' && value.length > rules.max) {
        return { success: false, message: Field  must be at most  characters long };
      }
      if (typeof value === 'number' && value > rules.max) {
        return { success: false, message: Field  must be at most  };
      }
    }

    // Pattern validation
    if (rules.pattern && typeof value === 'string' && !rules.pattern.test(value)) {
      return { success: false, message: Field  does not match required pattern };
    }

    // Custom validator
    if (rules.validator) {
      const customResult = rules.validator(value);
      if (customResult !== true) {
        return {
          success: false,
          message: typeof customResult === 'string'
            ? customResult
            : Field  failed custom validation
        };
      }
    }
  }

  return { success: true };
}

// Convenience function that combines validation and intelligent retry
export async function invokeWithValidation<T>(
  command: string,
  args: unknown,
  validator: (args: unknown) => ValidationResult | boolean | string
): Promise<T> {
  // Handle both old and new validation result formats
  const validationResult = validator(args);

  // Handle different validation result formats
  if (validationResult === true) {
    // Old format: boolean true means success
    // No action needed
  } else if (typeof validationResult === 'string') {
    // Old format: string means error message
    throw new AppError({
      message: 'Validation failed',
      details: validationResult,
      category: ErrorCategory.VALIDATION,
      severity: ErrorSeverity.WARNING,
      context: { command, args },
    });
  } else if (typeof validationResult === 'object') {
    // New format: ValidationResult object
    if (!validationResult.success) {
      throw new AppError({
        message: 'Validation failed',
        details: validationResult.message,
        category: ErrorCategory.VALIDATION,
        severity: ErrorSeverity.WARNING,
        context: { command, args },
      });
    }
  } else {
    // Boolean false (old format)
    throw new AppError({
      message: 'Validation failed',
      details: 'Invalid arguments provided',
      category: ErrorCategory.VALIDATION,
      severity: ErrorSeverity.WARNING,
      context: { command, args },
    });
  }

  // Use intelligent retry for validated requests
  return invokeWithIntelligentRetry<T>(command, args as Record<string, unknown>, {
    maxRetries: 2, // Fewer retries for validated requests
    baseDelayMs: 1000,
  });
}

// Legacy compatibility - enhanced version of the original function
export async function invokeWithRetry<T>(
  command: string,
  args?: Record<string, unknown>,
  maxRetries: number = 3,
  baseDelayMs: number = 1000
): Promise<T> {
  return invokeWithIntelligentRetry<T>(command, args, {
    maxRetries,
    baseDelayMs,
    retryableErrors: [ErrorCategory.TIMEOUT, ErrorCategory.NETWORK, ErrorCategory.API],
  });
}

// Enhanced validation functions with better error messages
export function validateCreateConversation(args: unknown): ValidationResult {
  const schema: ValidationSchema = {
    title: {
      type: 'string',
      required: false,
      min: 1,
      max: 200,
      validator: (value: unknown) => {
        if (typeof value === 'string' && value.trim().length === 0) {
          return 'Title cannot be empty or only whitespace';
        }
        return true;
      },
    },
    persona_id: {
      type: 'number',
      required: false,
      min: 1,
      validator: (value: unknown) => {
        if (value !== null && (typeof value !== 'number' || !Number.isInteger(value) || value < 1)) {
          return 'Persona ID must be a positive integer';
        }
        return true;
      },
    },
  };

  return validateWithSchema(args, schema);
}

export function validateGetConversations(args: unknown): ValidationResult {
  const schema: ValidationSchema = {
    limit: {
      type: 'number',
      required: false,
      min: 1,
      max: 1000,
    },
    offset: {
      type: 'number',
      required: false,
      min: 0,
    },
  };

  return validateWithSchema(args, schema);
}

export function validateSendMessage(args: unknown): ValidationResult {
  const schema: ValidationSchema = {
    conversation_id: {
      type: 'number',
      required: true,
      min: 1,
    },
    content: {
      type: 'string',
      required: true,
      min: 1,
      max: 50000,
      validator: (value: unknown) => {
        if (typeof value === 'string' && value.trim().length === 0) {
          return 'Message content cannot be empty or only whitespace';
        }
        return true;
      },
    },
    role: {
      type: 'string',
      required: false,
      validator: (value: unknown) => {
        if (typeof value === 'string' && value && !['user', 'assistant', 'system'].includes(value)) {
          return 'Role must be one of: user, assistant, system';
        }
        return true;
      },
    },
  };

  return validateWithSchema(args, schema);
}

// Export circuit breaker metrics for monitoring
export function getCircuitBreakerMetrics(command: string) {
  const circuitBreaker = getCircuitBreaker(command);
  return circuitBreaker.getMetrics();
}

// Export all circuit breaker metrics for system health monitoring
export function getAllCircuitBreakerMetrics() {
  const metrics: Record<string, ReturnType<CircuitBreaker['getMetrics']>> = {};

  for (const [command, breaker] of circuitBreakers.entries()) {
    metrics[command] = breaker.getMetrics();
  }

  return metrics;
}
