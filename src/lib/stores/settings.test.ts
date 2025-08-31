import { describe, it, expect, vi, beforeEach } from 'vitest';
import { settings } from '../stores/settings';
import { get } from 'svelte/store';

// Mock localStorage
const localStorageMock = (() => {
  let store: { [key: string]: string } = {};
  return {
    getItem: vi.fn((key: string) => store[key] || null),
    setItem: vi.fn((key: string, value: string) => {
      store[key] = value;
    }),
    removeItem: vi.fn((key: string) => {
      delete store[key];
    }),
    clear: vi.fn(() => {
      store = {};
    })
  };
})();

// Mock browser environment
vi.mock('$app/environment', () => ({
  browser: true
}));

Object.defineProperty(global, 'localStorage', {
  value: localStorageMock,
  writable: true,
});

describe('Settings Store - Telemetry Controls', () => {
  beforeEach(() => {
    vi.clearAllMocks();
    localStorageMock.clear();
  });

  it('should default to telemetry disabled', () => {
    const defaultSettings = settings.load();
    expect(defaultSettings.telemetryEnabled).toBe(false);
  });

  it('should persist telemetry preference', () => {
    const testSettings = {
      telemetryEnabled: true,
      theme: 'dark' as const,
      apiProvider: 'openai',
      apiKey: 'test-key',
    };

    settings.save(testSettings);
    
    expect(localStorageMock.setItem).toHaveBeenCalledWith(
      'forbidden-library-settings', 
      JSON.stringify(testSettings)
    );
    expect(localStorageMock.setItem).toHaveBeenCalledWith(
      'telemetry.enabled', 
      'true'
    );
  });

  it('should load persisted telemetry preference', () => {
    const testData = {
      telemetryEnabled: true,
      theme: 'light',
      apiProvider: 'anthropic',
      apiKey: 'saved-key'
    };
    
    localStorageMock.setItem('forbidden-library-settings', JSON.stringify(testData));
    
    const loaded = settings.load();
    expect(loaded.telemetryEnabled).toBe(true);
    expect(loaded.theme).toBe('light');
    expect(loaded.apiProvider).toBe('anthropic');
  });

  it('should update specific setting', () => {
    // First set some initial data
    settings.save({
      telemetryEnabled: false,
      theme: 'dark',
      apiProvider: 'openai',
      apiKey: '',
    });

    // Then update just the telemetry setting
    settings.updateSetting('telemetryEnabled', true);
    
    expect(localStorageMock.setItem).toHaveBeenCalledWith(
      'telemetry.enabled', 
      'true'
    );
  });

  it('should handle localStorage errors gracefully', () => {
    // Mock localStorage to throw an error
    localStorageMock.getItem.mockImplementation(() => {
      throw new Error('Storage not available');
    });

    const loaded = settings.load();
    expect(loaded.telemetryEnabled).toBe(false); // Should return default
  });

  it('should handle malformed JSON in localStorage', () => {
    localStorageMock.getItem.mockReturnValue('{"invalid": json}');
    
    const loaded = settings.load();
    expect(loaded.telemetryEnabled).toBe(false); // Should return default
  });
});