/**
 * Enhanced API service with advanced error handling, recovery, and resilience patterns
 */

import { invoke } from '@tauri-apps/api';
import { safeInvoke, createMockConversations, createMockConversation, isTauriAvailable } from '$lib/utils/enhanced-tauri-detection';
import { AppError, ErrorCategory, ErrorSeverity } from '$lib/types/errors';
import { errorStore } from '$lib/stores/enhanced-error-store';

// Circuit breaker states
enum CircuitState {
  CLOSED = 'closed',
  OPEN = 'open',
  HALF_OPEN = 'half_open'
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

  async execute<T>(operation: () => Promise<T>): Promise<T> {
    if (this.state === CircuitState.OPEN) {
      if (Date.now() - this.lastFailureTime > this.config.recoveryTimeout) {
        this.state = CircuitState.HALF_OPEN;
        this.successCount = 0;
      } else {
        throw new AppError({
          message: 'Circuit breaker is open',
          details: 'Too many recent failures, circuit breaker is protecting the system',
          category: ErrorCategory.API,
          severity: ErrorSeverity.WARNING,
          recoverable: true,
          context: { circuitState: this.state, failureCount: this.failureCount }
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
      if (this.successCount >= 3) { // Require 3 successes to close
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
      successCount: this.successCount
    };
  }
}

// Circuit breakers for different commands
const circuitBreakers = new Map<string, CircuitBreaker>();

function getCircuitBreaker(command: string): CircuitBreaker {
  if (!circuitBreakers.has(command)) {
    circuitBreakers.set(command, new CircuitBreaker({
      failureThreshold: 5,
      recoveryTimeout: 30000, // 30 seconds
      monitoringPeriod: 60000  // 1 minute
    }));
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
  retryableErrors: [
    ErrorCategory.TIMEOUT,
    ErrorCategory.NETWORK,
    ErrorCategory.API
  ]
};

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
              timestamp: new Date().toISOString()
            } as T;
          default:
            throw new AppError({
              message: 'Feature not available in web mode',
              details: `The command ${command} requires the desktop application. Please install and run the desktop version for full functionality.`,
              category: ErrorCategory.ENVIRONMENT,
              severity: ErrorSeverity.WARNING,
              context: { command, environment: 'web', availableCommands: ['get_conversations', 'create_conversation', 'get_messages', 'send_message'] }
            });
        }
      });
    }

    // Progressive timeout: increase timeout for subsequent retries
    const effectiveTimeout = progressiveTimeout ? timeoutMs : timeoutMs;
    let timeoutHandle: ReturnType<typeof setTimeout> | undefined;

    const timeoutPromise = new Promise<never>((_, reject) => {
      timeoutHandle = setTimeout(() => {
        reject(new AppError({
          message: 'Operation timed out',
          details: `The command ${command} exceeded the timeout of ${effectiveTimeout}ms`,
          category: ErrorCategory.TIMEOUT,
          severity: ErrorSeverity.WARNING,
          context: { command, timeoutMs: effectiveTimeout, args }
        }));
      }, effectiveTimeout);
    });

    try {
      const result = await Promise.race([
        invoke<T>(command, args as any),
        timeoutPromise,
      ]) as T;

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
          details: `The command ${command} took too long to respond (>${effectiveTimeout}ms). This might indicate network issues or server overload.`,
          category: ErrorCategory.TIMEOUT,
          severity: ErrorSeverity.WARNING,
          originalError: error,
          context: { command, timeoutMs: effectiveTimeout, circuitBreakerState: circuitBreaker.getState() }
        });
      } else if (errorMessage.includes('not available') || errorMessage.includes('not found')) {
        throw new AppError({
          message: 'Feature not available',
          details: `The command ${command} is not available in this environment or version`,
          category: ErrorCategory.ENVIRONMENT,
          severity: ErrorSeverity.ERROR,
          originalError: error,
          context: { command, environment: isTauriAvailable() ? 'tauri' : 'web' }
        });
      } else if (errorMessage.includes('permission') || errorMessage.includes('denied') || errorMessage.includes('unauthorized')) {
        throw new AppError({
          message: 'Permission denied',
          details: `Access denied for command ${command}. Please check your permissions or authentication.`,
          category: ErrorCategory.PERMISSION,
          severity: ErrorSeverity.ERROR,
          originalError: error,
          context: { command }
        });
      } else if (errorMessage.includes('network') || errorMessage.includes('connection') || errorMessage.includes('fetch')) {
        throw new AppError({
          message: 'Network error',
          details: `Network connectivity issue while executing ${command}`,
          category: ErrorCategory.NETWORK,
          severity: ErrorSeverity.ERROR,
          originalError: error,
          context: { command }
        });
      } else {
        throw new AppError({
          message: 'API operation failed',
          details: errorMessage || `Unknown error occurred while executing ${command}`,
          category: ErrorCategory.API,
          severity: ErrorSeverity.ERROR,
          originalError: error,
          context: { command, args, circuitBreakerMetrics: circuitBreaker.getMetrics() }
        });
      }
    } finally {
      if (timeoutHandle) clearTimeout(timeoutHandle);
    }
  });
}

export function ms(seconds: number): number {
  return Math.round(seconds * 1000);
}

// Enhanced validation with schema support
interface ValidationSchema {
  [key: string]: {
    type: 'string' | 'number' | 'boolean' | 'object' | 'array';
    required?: boolean;
    min?: number;
    max?: number;
    pattern?: RegExp;
    validator?: (value: any) => boolean | string;
  };
}

export async function invokeWithSchemaValidation<T>(
  command: string,
  args: unknown,
  schema: ValidationSchema,
  timeoutMs: number = 8000
): Promise<T> {
  // Enhanced schema validation
  const validationResult = validateWithSchema(args, schema);
  if (validationResult !== true) {
    throw new AppError({
      message: 'Validation failed',
      details: validationResult,
      category: ErrorCategory.VALIDATION,
      severity: ErrorSeverity.WARNING,
      context: { command, args, schema }
    });
  }

  return invokeWithEnhancedTimeout<T>(command, args as Record<string, unknown>, timeoutMs);
}

function validateWithSchema(args: unknown, schema: ValidationSchema): boolean | string {
  if (!args || typeof args !== 'object') {
    return 'Invalid arguments: expected object';
  }

  const data = args as Record<string, any>;

  for (const [key, rules] of Object.entries(schema)) {
    const value = data[key];

    // Check required fields
    if (rules.required && (value === undefined || value === null)) {
      return `Missing required field: ${key}`;
    }

    // Skip validation for optional undefined fields
    if (value === undefined || value === null) continue;

    // Type validation
    if (rules.type === 'string' && typeof value !== 'string') {
      return `Field ${key} must be a string`;
    }
    if (rules.type === 'number' && typeof value !== 'number') {
      return `Field ${key} must be a number`;
    }
    if (rules.type === 'boolean' && typeof value !== 'boolean') {
      return `Field ${key} must be a boolean`;
    }
    if (rules.type === 'object' && (typeof value !== 'object' || Array.isArray(value))) {
      return `Field ${key} must be an object`;
    }
    if (rules.type === 'array' && !Array.isArray(value)) {
      return `Field ${key} must be an array`;
    }

    // Range validation
    if (rules.min !== undefined) {
      if (typeof value === 'string' && value.length < rules.min) {
        return `Field ${key} must be at least ${rules.min} characters long`;
      }
      if (typeof value === 'number' && value < rules.min) {
        return `Field ${key} must be at least ${rules.min}`;
      }
    }

    if (rules.max !== undefined) {
      if (typeof value === 'string' && value.length > rules.max) {
        return `Field ${key} must be at most ${rules.max} characters long`;
      }
      if (typeof value === 'number' && value > rules.max) {
        return `Field ${key} must be at most ${rules.max}`;
      }
    }

    // Pattern validation
    if (rules.pattern && typeof value === 'string' && !rules.pattern.test(value)) {
      return `Field ${key} does not match required pattern`;
    }

    // Custom validator
    if (rules.validator) {
      const customResult = rules.validator(value);
      if (customResult !== true) {
        return typeof customResult === 'string' ? customResult : `Field ${key} failed custom validation`;
      }
    }
  }

  return true;
}

// Enhanced retry with intelligent backoff and error analysis
export async function invokeWithIntelligentRetry<T>(
  command: string,
  args?: Record<string, unknown>,
  config: Partial<RetryConfig> = {}
): Promise<T> {
  const retryConfig = { ...DEFAULT_RETRY_CONFIG, ...config };
  let lastError: AppError | Error;
  let attempt = 0;

  while (attempt <= retryConfig.maxRetries) {
    try {
      // Progressive timeout: increase timeout for each retry
      const timeoutMs = Math.min(
        8000 * Math.pow(1.5, attempt),
        30000 // Max 30 seconds
      );

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

      console.warn(`Command ${command} failed (attempt ${attempt}/${retryConfig.maxRetries + 1}), retrying in ${Math.round(delay)}ms...`, {
        error: error instanceof AppError ? error.toJSON() : error,
        retryConfig
      });

      // Track retry attempt
      errorStore.trackRetry(command, attempt, error instanceof AppError ? error : undefined);

      // Wait before retrying
      await new Promise(resolve => setTimeout(resolve, delay));
    }
  }

  // All retries failed, throw the last error with enhanced context
  if (lastError instanceof AppError) {
    throw new AppError({
      ...lastError,
      message: `${lastError.message} (after ${retryConfig.maxRetries} retries)`,
      context: {
        ...lastError.context,
        totalAttempts: attempt,
        retryConfig
      }
    });
  } else {
    throw new AppError({
      message: `Command failed after ${retryConfig.maxRetries} retries`,
      details: lastError.message,
      category: ErrorCategory.API,
      severity: ErrorSeverity.ERROR,
      originalError: lastError,
      context: { command, args, totalAttempts: attempt, retryConfig }
    });
  }
}

// Convenience function that combines validation and intelligent retry
export async function invokeWithValidation<T>(
  command: string,
  args: unknown,
  validator: (args: unknown) => boolean | string,
  timeoutMs: number = 8000
): Promise<T> {
  // Validate arguments first
  const validationResult = validator(args);
  if (validationResult !== true) {
    const errorMessage = typeof validationResult === 'string'
      ? validationResult
      : 'Invalid arguments provided';

    throw new AppError({
      message: 'Validation failed',
      details: errorMessage,
      category: ErrorCategory.VALIDATION,
      severity: ErrorSeverity.WARNING,
      context: { command, args }
    });
  }

  // Use intelligent retry for validated requests
  return invokeWithIntelligentRetry<T>(command, args as Record<string, unknown>, {
    maxRetries: 2, // Fewer retries for validated requests
    baseDelayMs: 1000
  });
}

// Legacy compatibility - enhanced version of the original function
export async function invokeWithRetry<T>(
  command: string,
  args?: Record<string, unknown>,
  timeoutMs: number = 8000,
  maxRetries: number = 3,
  baseDelayMs: number = 1000
): Promise<T> {
  return invokeWithIntelligentRetry<T>(command, args, {
    maxRetries,
    baseDelayMs,
    retryableErrors: [ErrorCategory.TIMEOUT, ErrorCategory.NETWORK, ErrorCategory.API]
  });
}

// Enhanced validation functions with better error messages
export function validateCreateConversation(args: unknown): boolean | string {
  const schema: ValidationSchema = {
    title: {
      type: 'string',
      required: false,
      min: 1,
      max: 200,
      validator: (value: string) => {
        if (value && value.trim().length === 0) {
          return 'Title cannot be empty or only whitespace';
        }
        return true;
      }
    },
    persona_id: {
      type: 'number',
      required: false,
      min: 1,
      validator: (value: number) => {
        if (value !== null && (!Number.isInteger(value) || value < 1)) {
          return 'Persona ID must be a positive integer';
        }
        return true;
      }
    }
  };

  return validateWithSchema(args, schema);
}

export function validateGetConversations(args: unknown): boolean | string {
  const schema: ValidationSchema = {
    limit: {
      type: 'number',
      required: false,
      min: 1,
      max: 1000
    },
    offset: {
      type: 'number',
      required: false,
      min: 0
    }
  };

  return validateWithSchema(args, schema);
}

export function validateSendMessage(args: unknown): boolean | string {
  const schema: ValidationSchema = {
    conversation_id: {
      type: 'number',
      required: true,
      min: 1
    },
    content: {
      type: 'string',
      required: true,
      min: 1,
      max: 50000,
      validator: (value: string) => {
        if (value.trim().length === 0) {
          return 'Message content cannot be empty or only whitespace';
        }
        return true;
      }
    },
    role: {
      type: 'string',
      required: false,
      validator: (value: string) => {
        if (value && !['user', 'assistant', 'system'].includes(value)) {
          return 'Role must be one of: user, assistant, system';
        }
        return true;
      }
    }
  };

  return validateWithSchema(args, schema);
}

// Export circuit breaker metrics for monitoring
export function getCircuitBreakerMetrics(): Record<string, any> {
  const metrics: Record<string, any> = {};
  circuitBreakers.forEach((breaker, command) => {
    metrics[command] = breaker.getMetrics();
  });
  return metrics;
}

// Health check function
export async function healthCheck(): Promise<{
  status: 'healthy' | 'degraded' | 'unhealthy';
  details: Record<string, any>;
}> {
  const metrics = getCircuitBreakerMetrics();
  const openCircuits = Object.values(metrics).filter((m: any) => m.state === CircuitState.OPEN).length;
  const totalCircuits = Object.keys(metrics).length;

  let status: 'healthy' | 'degraded' | 'unhealthy';
  if (openCircuits === 0) {
    status = 'healthy';
  } else if (openCircuits < totalCircuits / 2) {
    status = 'degraded';
  } else {
    status = 'unhealthy';
  }

  return {
    status,
    details: {
      circuitBreakers: metrics,
      openCircuits,
      totalCircuits,
      environment: isTauriAvailable() ? 'tauri' : 'web',
      timestamp: new Date().toISOString()
    }
  };
}
