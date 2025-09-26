import { invoke } from '@tauri-apps/api';
import { createMockConversations, createMockConversation } from '$lib/utils/enhanced-tauri-detection';
import { AppError, ErrorCategory, ErrorSeverity } from '$lib/types/errors';
import { environment } from '$lib/stores/environment';
import { get } from 'svelte/store';

/**
 * Enhanced invoke function with comprehensive error handling and categorization
 */
export async function invokeWithTimeout<T>(
  command: string,
  args?: Record<string, unknown>,
  timeoutMs: number = 8000
): Promise<T> {
  const currentEnvironment = get(environment);

  // If not in Tauri, use fallbacks
  if (currentEnvironment !== 'tauri') {
    switch (command) {
      case 'get_conversations':
        return createMockConversations() as T;
      case 'create_conversation':
        return createMockConversation() as T;
      case 'get_messages':
        return [] as T;
      case 'send_message':
        return { success: true, message: 'Demo message sent (web mode)' } as T;
      default:
        console.error(`No fallback available for command: ${command}`);
        throw new AppError({
          message: 'Feature not available',
          details: `The command ${command} is not available in web mode`,
          category: ErrorCategory.ENVIRONMENT,
          severity: ErrorSeverity.WARNING,
          context: { command, environment: 'web' },
        });
    }
  }

  let timeoutHandle: ReturnType<typeof setTimeout> | undefined;

  const timeoutPromise = new Promise<never>((_, reject) => {
    timeoutHandle = setTimeout(() => {
      reject(new Error(`TimeoutError: invoke(${command}) exceeded ${timeoutMs}ms`));
    }, timeoutMs);
  });

  try {
    const result = (await Promise.race([invoke<T>(command, args), timeoutPromise])) as T;
    return result;
  } catch (error) {
    // Enhanced error categorization
    if (error instanceof Error && error.message?.includes('timed out')) {
      throw new AppError({
        message: 'Operation timed out',
        details: `The command ${command} took too long to respond (>${timeoutMs}ms)`,
        category: ErrorCategory.TIMEOUT,
        severity: ErrorSeverity.WARNING,
        originalError: error,
        context: { command, timeoutMs },
      });
    } else if (error instanceof Error && error.message?.includes('not available')) {
      throw new AppError({
        message: 'Feature not available',
        details: `The command ${command} is not available in this environment`,
        category: ErrorCategory.ENVIRONMENT,
        severity: ErrorSeverity.ERROR,
        originalError: error,
        context: { command },
      });
    } else if (
      error instanceof Error &&
      (error.message?.includes('permission') || error.message?.includes('denied'))
    ) {
      throw new AppError({
        message: 'Permission denied',
        details: `Access denied for command ${command}`,
        category: ErrorCategory.PERMISSION,
        severity: ErrorSeverity.ERROR,
        originalError: error,
        context: { command },
      });
    } else {
      throw new AppError({
        message: 'API operation failed',
        details: error instanceof Error ? error.message : String(error),
        category: ErrorCategory.API,
        severity: ErrorSeverity.ERROR,
        originalError: error,
        context: { command, args },
      });
    }
  } finally {
    if (timeoutHandle) clearTimeout(timeoutHandle);
  }
}

export function ms(seconds: number): number {
  return Math.round(seconds * 1000);
}

/**
 * Validation wrapper for API calls with input validation
 */
export async function invokeWithValidation<T>(
  command: string,
  args: unknown,
  validator: (args: unknown) => boolean | string,
  timeoutMs: number = 8000
): Promise<T> {
  // Validate arguments before sending
  const validationResult = validator(args);
  if (validationResult !== true) {
    const errorMessage =
      typeof validationResult === 'string' ? validationResult : 'Invalid arguments provided';

    throw new AppError({
      message: 'Validation failed',
      details: errorMessage,
      category: ErrorCategory.VALIDATION,
      severity: ErrorSeverity.WARNING,
      context: { command, args },
    });
  }

  // Proceed with validated arguments
  return invokeWithTimeout<T>(command, args as Record<string, unknown>, timeoutMs);
}

/**
 * Retry wrapper with exponential backoff for failed operations
 */
export async function invokeWithRetry<T>(
  command: string,
  args?: Record<string, unknown>,
  timeoutMs: number = 8000,
  maxRetries: number = 3,
  baseDelayMs: number = 1000
): Promise<T> {
  let lastError: AppError | Error;

  for (let attempt = 0; attempt <= maxRetries; attempt++) {
    try {
      return await invokeWithTimeout<T>(command, args, timeoutMs);
    } catch (error) {
      lastError = error instanceof AppError ? error : new Error(String(error));

      // Don't retry on validation errors or environment errors
      if (
        error instanceof AppError &&
        (error.category === ErrorCategory.VALIDATION ||
          error.category === ErrorCategory.ENVIRONMENT ||
          error.category === ErrorCategory.PERMISSION)
      ) {
        throw error;
      }

      // If this was the last attempt, throw the error
      if (attempt === maxRetries) {
        break;
      }

      // Calculate delay with exponential backoff
      const delay = baseDelayMs * Math.pow(2, attempt);
      console.warn(
        `Command ${command} failed (attempt ${attempt + 1}/${maxRetries + 1}), retrying in ${delay}ms...`
      );

      // Wait before retrying
      await new Promise((resolve) => setTimeout(resolve, delay));
    }
  }

  // All retries failed, throw the last error
  throw lastError!;
}

// Validation functions for common API calls
export function validateCreateConversation(args: unknown): boolean | string {
  if (!args || typeof args !== 'object') return 'Invalid arguments: expected object';

  const { title, persona_id } = args as { title?: string; persona_id?: number | null };

  if (title !== undefined) {
    if (typeof title !== 'string') return 'Title must be a string';
    if (title.trim().length === 0) return 'Title cannot be empty';
    if (title.length > 200) return 'Title too long (max 200 characters)';
  }

  if (persona_id !== undefined && persona_id !== null && typeof persona_id !== 'number') {
    return 'Persona ID must be a number or null';
  }

  return true;
}

export function validateGetConversations(args: unknown): boolean | string {
  if (!args || typeof args !== 'object') return 'Invalid arguments: expected object';

  const { limit, offset } = args as { limit?: number; offset?: number };

  if (limit !== undefined) {
    if (typeof limit !== 'number' || limit < 1 || limit > 1000) {
      return 'Limit must be a number between 1 and 1000';
    }
  }

  if (offset !== undefined) {
    if (typeof offset !== 'number' || offset < 0) {
      return 'Offset must be a non-negative number';
    }
  }

  return true;
}

export function validateSendMessage(args: unknown): boolean | string {
  if (!args || typeof args !== 'object') return 'Invalid arguments: expected object';

  const { conversation_id, content, role } = args as {
    conversation_id?: number;
    content?: string;
    role?: string;
  };

  if (typeof conversation_id !== 'number' || conversation_id < 1) {
    return 'Conversation ID must be a positive number';
  }

  if (typeof content !== 'string' || content.trim().length === 0) {
    return 'Message content cannot be empty';
  }

  if (content.length > 50000) {
    return 'Message content too long (max 50,000 characters)';
  }

  if (role && !['user', 'assistant', 'system'].includes(role)) {
    return 'Invalid role: must be user, assistant, or system';
  }

  return true;
}
