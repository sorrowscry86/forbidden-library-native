/**
 * Enhanced Tauri detection and environment management
 * Provides robust environment detection with fallback strategies and monitoring
 */

import { AppError, ErrorCategory, ErrorSeverity } from '$lib/types/errors';

// Environment cache with expiration
interface EnvironmentCache {
  environment: 'tauri' | 'web' | 'unknown';
  timestamp: number;
  confidence: number; // 0-1 scale
  checks: EnvironmentCheck[];
}

interface EnvironmentCheck {
  name: string;
  passed: boolean;
  details?: string;
}

let environmentCache: EnvironmentCache | undefined;
const CACHE_DURATION = 30000; // 30 seconds cache

// Enhanced Tauri detection with multiple verification methods
export function isTauriAvailable(): boolean {
  try {
    // Multiple checks for robust detection
    const checks: EnvironmentCheck[] = [
      {
        name: 'window_tauri_object',
        passed: Boolean(typeof window !== 'undefined' && window.__TAURI__)
      },
      {
        name: 'tauri_invoke_function',
        passed: Boolean(typeof window !== 'undefined' && window.__TAURI__ && typeof window.__TAURI__.invoke === 'function')
      },
      {
        name: 'tauri_api_object',
        passed: Boolean(typeof window !== 'undefined' && window.__TAURI__ && typeof window.__TAURI__.tauri === 'object')
      },
      {
        name: 'user_agent_tauri',
        passed: Boolean(typeof navigator !== 'undefined' && navigator.userAgent && navigator.userAgent.includes('Tauri'))
      }
    ];

    // Calculate confidence based on passed checks
    const passedChecks = checks.filter(check => check.passed).length;
    const confidence = passedChecks / checks.length;

    // Update cache
    if (typeof window !== 'undefined') {
      environmentCache = {
        environment: confidence >= 0.5 ? 'tauri' : 'web',
        timestamp: Date.now(),
        confidence,
        checks
      };
    }

    // Require at least 2 out of 4 checks to pass for Tauri detection
    return confidence >= 0.5;
  } catch (error) {
    console.warn('Error detecting Tauri environment:', error);

    // Update cache with error state
    environmentCache = {
      environment: 'unknown',
      timestamp: Date.now(),
      confidence: 0,
      checks: [{
        name: 'detection_error',
        passed: false,
        details: error instanceof Error ? error.message : String(error)
      }]
    };

    return false; // Default to web mode on detection failure
  }
}

// Check if we're running in development mode with enhanced detection
export function isDevelopment(): boolean {
  try {
    // Multiple ways to detect development mode
    return Boolean(
      import.meta.env.DEV ||
      import.meta.env.MODE === 'development' ||
      (typeof window !== 'undefined' && window.location.hostname === 'localhost') ||
      (typeof window !== 'undefined' && window.location.hostname === '127.0.0.1') ||
      (typeof window !== 'undefined' && window.location.port !== '')
    );
  } catch (error) {
    console.warn('Error detecting development mode:', error);
    return false;
  }
}

// Check if we're running in production mode
export function isProduction(): boolean {
  try {
    return Boolean(
      import.meta.env.PROD ||
      import.meta.env.MODE === 'production' ||
      (typeof window !== 'undefined' && window.location.protocol === 'https:' && !isDevelopment())
    );
  } catch (error) {
    console.warn('Error detecting production mode:', error);
    return false;
  }
}

// Enhanced environment detection with caching and confidence scoring
export function getEnvironment(): 'tauri' | 'web' | 'unknown' {
  // Check cache validity
  if (environmentCache && (Date.now() - environmentCache.timestamp) < CACHE_DURATION) {
    return environmentCache.environment;
  }

  // Perform fresh detection
  let result: 'tauri' | 'web' | 'unknown';

  if (typeof window === 'undefined') {
    result = 'unknown';
  } else if (isTauriAvailable()) {
    result = 'tauri';
  } else {
    result = 'web';
  }

  return result;
}

// Get detailed environment information with diagnostics
export function getEnvironmentInfo() {
  const environment = getEnvironment();
  const cache = environmentCache;

  return {
    environment,
    isDevelopment: isDevelopment(),
    isProduction: isProduction(),
    isTauriAvailable: isTauriAvailable(),
    confidence: cache?.confidence || 0,
    checks: cache?.checks || [],
    userAgent: typeof navigator !== 'undefined' ? navigator.userAgent : 'unknown',
    platform: typeof navigator !== 'undefined' ? navigator.platform : 'unknown',
    hostname: typeof window !== 'undefined' ? window.location.hostname : 'unknown',
    protocol: typeof window !== 'undefined' ? window.location.protocol : 'unknown',
    port: typeof window !== 'undefined' ? window.location.port : 'unknown',
    cacheAge: cache ? Date.now() - cache.timestamp : 0,
    timestamp: new Date().toISOString()
  };
}

// Clear cache when needed (e.g., for testing or environment changes)
export function clearEnvironmentCache(): void {
  environmentCache = undefined;
}

// Monitor environment changes (useful for hot reloading in development)
export function monitorEnvironmentChanges(callback: (newEnv: 'tauri' | 'web' | 'unknown') => void): () => void {
  let lastEnvironment = getEnvironment();

  const interval = setInterval(() => {
    clearEnvironmentCache(); // Force fresh detection
    const currentEnvironment = getEnvironment();

    if (currentEnvironment !== lastEnvironment) {
      console.log(`Environment changed from ${lastEnvironment} to ${currentEnvironment}`);
      callback(currentEnvironment);
      lastEnvironment = currentEnvironment;
    }
  }, 5000); // Check every 5 seconds

  // Return cleanup function
  return () => clearInterval(interval);
}

// Enhanced mock data with more realistic structure
export function createMockConversation() {
  return {
    id: Math.floor(Math.random() * 1000) + 1,
    uuid: `mock-conversation-${Date.now()}-${Math.random().toString(36).substr(2, 9)}`,
    title: 'Demo Conversation (Web Mode)',
    created_at: new Date().toISOString(),
    updated_at: new Date().toISOString(),
    persona_id: null,
    archived: false,
    message_count: Math.floor(Math.random() * 10),
    last_message_at: new Date(Date.now() - Math.random() * 86400000).toISOString()
  };
}

// Create mock conversations list with variety
export function createMockConversations() {
  const conversations = [];
  const titles = [
    'Demo Conversation (Web Mode)',
    'Web Mode Example',
    'Sample Chat Session',
    'Test Conversation',
    'Mock Discussion'
  ];

  for (let i = 0; i < 5; i++) {
    conversations.push({
      id: i + 1,
      uuid: `mock-conversation-${i + 1}-${Date.now()}`,
      title: titles[i] || `Conversation ${i + 1}`,
      created_at: new Date(Date.now() - (i * 86400000)).toISOString(), // Spread over days
      updated_at: new Date(Date.now() - (i * 3600000)).toISOString(), // Spread over hours
      persona_id: null,
      archived: false,
      message_count: Math.floor(Math.random() * 20) + 1,
      last_message_at: new Date(Date.now() - (i * 3600000)).toISOString()
    });
  }

  return conversations;
}

// Enhanced safe invoke with better error handling and fallback strategies
export async function safeInvoke<T>(
  command: string,
  args?: Record<string, unknown>,
  fallback?: () => T
): Promise<T> {
  const environment = getEnvironment();

  if (environment === 'tauri') {
    try {
      const { invoke } = await import('@tauri-apps/api');
      return await invoke<T>(command, args as any);
    } catch (error) {
      console.warn(`Tauri command ${command} failed:`, error);

      // If we have a fallback, use it
      if (fallback) {
        console.log(`Using fallback for command: ${command}`);
        return fallback();
      }

      // Re-throw as AppError for better error handling
      throw new AppError({
        message: 'Tauri command failed',
        details: `Command ${command} failed: ${error instanceof Error ? error.message : String(error)}`,
        category: ErrorCategory.API,
        severity: ErrorSeverity.ERROR,
        originalError: error,
        context: { command, args, environment }
      });
    }
  } else if (environment === 'web') {
    console.log(`Tauri not available, using fallback for command: ${command}`);

    if (fallback) {
      return fallback();
    }

    throw new AppError({
      message: 'Feature not available in web mode',
      details: `The command ${command} requires the desktop application. Please install and run the desktop version for full functionality.`,
      category: ErrorCategory.ENVIRONMENT,
      severity: ErrorSeverity.WARNING,
      context: {
        command,
        args,
        environment,
        suggestion: 'Install the desktop application for full functionality'
      }
    });
  } else {
    throw new AppError({
      message: 'Unknown environment',
      details: 'Unable to determine the current environment. This might indicate a configuration issue.',
      category: ErrorCategory.ENVIRONMENT,
      severity: ErrorSeverity.ERROR,
      context: { command, args, environment, environmentInfo: getEnvironmentInfo() }
    });
  }
}

// Capability detection - check what features are available
export async function detectCapabilities(): Promise<{
  tauri: boolean;
  fileSystem: boolean;
  notifications: boolean;
  clipboard: boolean;
  systemTray: boolean;
  globalShortcuts: boolean;
}> {
  const capabilities = {
    tauri: false,
    fileSystem: false,
    notifications: false,
    clipboard: false,
    systemTray: false,
    globalShortcuts: false
  };

  if (!isTauriAvailable()) {
    return capabilities;
  }

  try {
    // Test Tauri availability
    capabilities.tauri = true;

    // Test file system access
    try {
      const { readDir } = await import('@tauri-apps/api/fs');
      await readDir('/', { recursive: false });
      capabilities.fileSystem = true;
    } catch (error) {
      // File system not available or no permission
    }

    // Test notifications
    try {
      const { isPermissionGranted } = await import('@tauri-apps/api/notification');
      capabilities.notifications = await isPermissionGranted();
    } catch (error) {
      // Notifications not available
    }

    // Test clipboard
    try {
      const { readText } = await import('@tauri-apps/api/clipboard');
      await readText();
      capabilities.clipboard = true;
    } catch (error) {
      // Clipboard not available
    }

    // Note: System tray and global shortcuts are harder to test without side effects
    // We'll assume they're available if Tauri is available
    capabilities.systemTray = true;
    capabilities.globalShortcuts = true;

  } catch (error) {
    console.warn('Error detecting capabilities:', error);
  }

  return capabilities;
}

// Environment health check
export async function environmentHealthCheck(): Promise<{
  status: 'healthy' | 'degraded' | 'unhealthy';
  environment: string;
  issues: string[];
  capabilities: Record<string, boolean>;
}> {
  const issues: string[] = [];
  const environment = getEnvironment();
  const environmentInfo = getEnvironmentInfo();
  const capabilities = await detectCapabilities();

  // Check environment confidence
  if (environmentInfo.confidence < 0.8) {
    issues.push(`Low environment detection confidence: ${Math.round(environmentInfo.confidence * 100)}%`);
  }

  // Check for failed detection checks
  const failedChecks = environmentInfo.checks.filter(check => !check.passed);
  if (failedChecks.length > 0) {
    issues.push(`Failed environment checks: ${failedChecks.map(c => c.name).join(', ')}`);
  }

  // Check cache age
  if (environmentInfo.cacheAge > CACHE_DURATION * 2) {
    issues.push('Environment cache is stale');
  }

  // Determine overall health
  let status: 'healthy' | 'degraded' | 'unhealthy';
  if (issues.length === 0) {
    status = 'healthy';
  } else if (issues.length <= 2) {
    status = 'degraded';
  } else {
    status = 'unhealthy';
  }

  return {
    status,
    environment,
    issues,
    capabilities
  };
}
