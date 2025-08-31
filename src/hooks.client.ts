// Workflow-Update: Frontend Sentry Initialization for SPA WebView (VOI-76)
// Initializes Sentry on the client side only, respecting telemetry opt-in and env vars.

import * as Sentry from '@sentry/sveltekit';

// Simple localStorage-backed flag for now; can be replaced by a settings store later (VOI-78)
function isTelemetryEnabled(): boolean {
  try {
    const raw = localStorage.getItem('telemetry.enabled');
    if (raw === null) return false; // default off until VOI-78 finalizes policy
    return raw === 'true';
  } catch {
    return false;
  }
}

// Initialize Sentry if DSN is available and telemetry is enabled
(() => {
  const dsn = __SENTRY_DSN__;
  const traces = parseFloat(__SENTRY_TRACES_SAMPLE_RATE__ || '0');
  const profiles = parseFloat(__SENTRY_PROFILES_SAMPLE_RATE__ || '0');

  if (!dsn) return; // No DSN available — skip init
  if (!isTelemetryEnabled()) return; // Respect opt-in

  Sentry.init({
    dsn,
    release: __APP_VERSION__,
    environment: __APP_ENV__,
    tracesSampleRate: isNaN(traces) ? 0 : traces,
    profilesSampleRate: isNaN(profiles) ? 0 : profiles,
    integrations: [
      Sentry.browserTracingIntegration(),
      Sentry.replayIntegration({
        maskAllText: true,
        blockAllMedia: true,
      }),
    ],
    replaysSessionSampleRate: 0.1,
    replaysOnErrorSampleRate: 1.0,
    
    // Basic scrubbing policy; can be extended in VOI-78
    beforeSend(event: any): any {
      // Remove file paths if accidentally present
      if (event.breadcrumbs) {
        event.breadcrumbs = event.breadcrumbs.map((b: any) => {
          if (b?.data && typeof b.data === 'object') {
            delete b.data.filePath;
          }
          return b;
        });
      }
      return event;
    }
  });
})();