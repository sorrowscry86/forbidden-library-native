/**
 * Tauri detection and fallback utilities
 * Provides graceful degradation when running in web browser vs Tauri desktop app
 */
import { environment } from '$lib/stores/environment';
import { getEnvironment, getEnvironmentInfo } from './enhanced-tauri-detection';

// Function to be called once from the root layout onMount
export function initializeEnvironment() {
  // Use the enhanced detection with multiple checks
  const detectedEnvironment = getEnvironment();
  environment.set(detectedEnvironment);

  console.log(`Environment initialized as: ${detectedEnvironment}`);

  // Add debugging info to help diagnose issues
  if (typeof window !== 'undefined' && detectedEnvironment === 'tauri') {
    console.log('Tauri API available:', Boolean(window.__TAURI__?.invoke));
  }

  // Log detailed environment info in development
  if (import.meta.env.DEV) {
    console.log('Environment details:', getEnvironmentInfo());
  }
}
