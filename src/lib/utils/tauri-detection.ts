/**
 * Tauri detection and fallback utilities
 * Provides graceful degradation when running in web browser vs Tauri desktop app
 */

// Environment cache to prevent repeated checks
let environmentCache: 'tauri' | 'web' | undefined;

// Check if we're running in Tauri environment with enhanced detection
export function isTauriAvailable(): boolean {
  try {
    // More robust detection with multiple checks
    return Boolean(
      typeof window !== 'undefined' &&
      window.__TAURI__ &&
      typeof window.__TAURI__.invoke === 'function' &&
      typeof window.__TAURI__.tauri === 'object'
    );
  } catch (error) {
    // Safely handle any unexpected errors in detection
    console.warn('Error detecting Tauri environment:', error);
    return false; // Default to web mode on detection failure
  }
}

// Check if we're running in development mode
export function isDevelopment(): boolean {
  return import.meta.env.DEV;
}

// Check if we're running in production mode
export function isProduction(): boolean {
  return import.meta.env.PROD;
}

// Get the current environment type with caching
export function getEnvironment(): 'tauri' | 'web' | 'unknown' {
  // Add caching to prevent repeated checks
  if (environmentCache !== undefined) return environmentCache;

  let result: 'tauri' | 'web' | 'unknown';
  if (isTauriAvailable()) {
    result = 'tauri';
  } else if (typeof window !== 'undefined') {
    result = 'web';
  } else {
    result = 'unknown';
  }

  // Cache the result (but only cache 'tauri' or 'web', not 'unknown')
  if (result !== 'unknown') {
    environmentCache = result;
  }

  return result;
}

// Clear cache when needed (e.g., for testing)
export function clearEnvironmentCache(): void {
  environmentCache = undefined;
}

// Create a mock conversation for web-only mode
export function createMockConversation() {
  return {
    id: 1,
    uuid: 'mock-conversation-1',
    title: 'Demo Conversation (Web Mode)',
    created_at: new Date().toISOString(),
    updated_at: new Date().toISOString(),
    persona_id: null,
    archived: false
  };
}

// Create mock conversations list for web-only mode
export function createMockConversations() {
  return [
    createMockConversation(),
    {
      id: 2,
      uuid: 'mock-conversation-2',
      title: 'Web Mode Demo',
      created_at: new Date(Date.now() - 86400000).toISOString(), // 1 day ago
      updated_at: new Date(Date.now() - 3600000).toISOString(), // 1 hour ago
      persona_id: null,
      archived: false
    }
  ];
}

// Safe invoke function that falls back to mock data in web mode
export async function safeInvoke<T>(
  command: string,
  args?: Record<string, unknown>,
  fallback?: () => T
): Promise<T> {
  if (isTauriAvailable()) {
    try {
      const { invoke } = await import('@tauri-apps/api');
      return await invoke<T>(command, args as any);
    } catch (error) {
      console.warn(`Tauri command ${command} failed:`, error);
      if (fallback) {
        return fallback();
      }
      throw error;
    }
  } else {
    console.warn(`Tauri not available, using fallback for command: ${command}`);
    if (fallback) {
      return fallback();
    }
    throw new Error(`Tauri command ${command} not available in web mode`);
  }
}

// Environment information for debugging
export function getEnvironmentInfo() {
  return {
    environment: getEnvironment(),
    isDevelopment: isDevelopment(),
    isProduction: isProduction(),
    isTauriAvailable: isTauriAvailable(),
    userAgent: typeof navigator !== 'undefined' ? navigator.userAgent : 'unknown',
    platform: typeof navigator !== 'undefined' ? navigator.platform : 'unknown'
  };
}


