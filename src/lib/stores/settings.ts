// Workflow-Update: Settings store with telemetry preferences (VOI-78)
// Manages user settings including privacy controls for Sentry telemetry

import { writable } from 'svelte/store';
import { browser } from '$app/environment';

export interface Settings {
  telemetryEnabled: boolean;
  theme: 'dark' | 'light' | 'auto';
  apiProvider: string;
  apiKey: string;
  // Add more settings as needed
}

const defaultSettings: Settings = {
  telemetryEnabled: false, // Default to false for privacy
  theme: 'dark',
  apiProvider: 'openai',
  apiKey: '',
};

function createSettingsStore() {
  const { subscribe, set, update } = writable<Settings>(defaultSettings);

  return {
    subscribe,
    // Load settings from localStorage
    load: (): Settings => {
      if (!browser) return defaultSettings;
      
      try {
        const stored = localStorage.getItem('forbidden-library-settings');
        if (stored) {
          const parsed = JSON.parse(stored);
          return { ...defaultSettings, ...parsed };
        }
      } catch (error) {
        console.error('Failed to load settings:', error);
      }
      return defaultSettings;
    },
    
    // Save settings to localStorage
    save: (settings: Settings): void => {
      if (!browser) return;
      
      try {
        localStorage.setItem('forbidden-library-settings', JSON.stringify(settings));
        // Also update the legacy telemetry flag for backward compatibility
        localStorage.setItem('telemetry.enabled', settings.telemetryEnabled ? 'true' : 'false');
        set(settings);
      } catch (error) {
        console.error('Failed to save settings:', error);
      }
    },
    
    // Update specific setting
    updateSetting: <K extends keyof Settings>(key: K, value: Settings[K]): void => {
      update(current => {
        const updated = { ...current, [key]: value };
        if (browser) {
          try {
            localStorage.setItem('forbidden-library-settings', JSON.stringify(updated));
            // Also update the legacy telemetry flag for backward compatibility
            if (key === 'telemetryEnabled') {
              localStorage.setItem('telemetry.enabled', value ? 'true' : 'false');
            }
          } catch (error) {
            console.error('Failed to update setting:', error);
          }
        }
        return updated;
      });
    },
    
    // Initialize store with saved settings
    init: (): void => {
      if (!browser) return;
      
      const stored = createSettingsStore().load();
      set(stored);
    }
  };
}

export const settings = createSettingsStore();