/**
 * Platform detection and adaptation utilities for Forbidden Library
 * 
 * This module provides cross-platform utilities for the SvelteKit frontend,
 * ensuring consistent behavior across Windows, macOS, and Linux.
 */

export interface PlatformInfo {
  isWindows: boolean;
  isMac: boolean;
  isLinux: boolean;
  platform: string;
}

/**
 * Platform detection utilities
 */
export const isPlatform = {
  windows: (): boolean => {
    if (typeof navigator !== 'undefined') {
      return navigator.platform.indexOf('Win') > -1;
    }
    return false;
  },
  
  mac: (): boolean => {
    if (typeof navigator !== 'undefined') {
      return navigator.platform.indexOf('Mac') > -1;
    }
    return false;
  },
  
  linux: (): boolean => {
    if (typeof navigator !== 'undefined') {
      return navigator.platform.indexOf('Linux') > -1;
    }
    return false;
  }
};

/**
 * Get detailed platform information
 */
export function getPlatformInfo(): PlatformInfo {
  return {
    isWindows: isPlatform.windows(),
    isMac: isPlatform.mac(),
    isLinux: isPlatform.linux(),
    platform: typeof navigator !== 'undefined' ? navigator.platform : 'Unknown'
  };
}

/**
 * Convert forward slashes to platform-specific path separators
 * Note: This is primarily for display purposes. The backend handles actual file paths.
 */
export function getPlatformSpecificPath(path: string): string {
  if (isPlatform.windows()) {
    return path.replace(/\//g, '\\');
  }
  return path;
}

/**
 * Get platform-specific command from a map of commands
 */
export function getPlatformSpecificCommand(commands: {
  windows: string;
  mac: string;
  linux: string;
}): string {
  if (isPlatform.windows()) return commands.windows;
  if (isPlatform.mac()) return commands.mac;
  return commands.linux;
}

/**
 * Get platform-specific keyboard shortcut display
 */
export function getPlatformShortcut(shortcut: {
  windows: string;
  mac: string;
  linux?: string;
}): string {
  if (isPlatform.mac()) return shortcut.mac;
  if (isPlatform.linux() && shortcut.linux) return shortcut.linux;
  return shortcut.windows;
}

/**
 * Format keyboard shortcuts for display
 * Converts modifier keys to platform-specific symbols
 */
export function formatShortcut(shortcut: string): string {
  if (isPlatform.mac()) {
    return shortcut
      .replace(/Ctrl/gi, '⌘')
      .replace(/Alt/gi, '⌥')
      .replace(/Shift/gi, '⇧')
      .replace(/Enter/gi, '↵');
  }
  return shortcut;
}

/**
 * Get platform-specific line ending
 */
export function getPlatformLineEnding(): string {
  return isPlatform.windows() ? '\r\n' : '\n';
}

/**
 * Check if the app is running in Tauri (native) or browser
 */
export function isTauriApp(): boolean {
  return typeof window !== 'undefined' && '__TAURI__' in window;
}

/**
 * Get platform-specific file dialog filters
 */
export function getFileFilters(type: 'text' | 'image' | 'all' = 'all'): string[] {
  const filters = {
    text: ['*.txt', '*.md', '*.json'],
    image: ['*.png', '*.jpg', '*.jpeg', '*.gif', '*.webp'],
    all: ['*']
  };
  
  return filters[type] || filters.all;
}

/**
 * Platform-specific default paths
 */
export const platformDefaults = {
  get documentsDir(): string {
    if (isPlatform.windows()) return 'Documents';
    if (isPlatform.mac()) return 'Documents';
    return 'Documents';
  },
  
  get downloadsDir(): string {
    if (isPlatform.windows()) return 'Downloads';
    if (isPlatform.mac()) return 'Downloads';
    return 'Downloads';
  },
  
  get desktopDir(): string {
    if (isPlatform.windows()) return 'Desktop';
    if (isPlatform.mac()) return 'Desktop';
    return 'Desktop';
  }
};

/**
 * Get user-friendly platform name
 */
export function getPlatformName(): string {
  if (isPlatform.windows()) return 'Windows';
  if (isPlatform.mac()) return 'macOS';
  if (isPlatform.linux()) return 'Linux';
  return 'Unknown';
}

/**
 * Check if a feature is available on the current platform
 */
export function isPlatformFeatureAvailable(feature: string): boolean {
  const platformFeatures: Record<string, boolean> = {
    'system-tray': true, // Available on all desktop platforms with Tauri
    'global-shortcuts': true,
    'notifications': true,
    'file-associations': isPlatform.windows() || isPlatform.mac(),
    'auto-updater': true
  };
  
  return platformFeatures[feature] ?? false;
}
