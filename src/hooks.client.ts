// Workflow-Update: Frontend Sentry Initialization for SPA WebView (VOI-76)
// Initializes Sentry on the client side only, respecting telemetry opt-in and env vars.

import * as Sentry from '@sentry/sveltekit';
// Minimal local types to avoid extra dependency on '@sentry/types'
type Breadcrumb = { data?: Record<string, unknown> };
type SentryEvent = { breadcrumbs?: Breadcrumb[] } & Record<string, unknown>;

// Build-time injected constants from Vite define()
declare const __SENTRY_DSN__: string;
declare const __SENTRY_TRACES_SAMPLE_RATE__: string;
declare const __SENTRY_PROFILES_SAMPLE_RATE__: string;
declare const __APP_VERSION__: string;
declare const __APP_ENV__: string;

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
    integrations: [],
    // Basic scrubbing policy; can be extended in VOI-78
  beforeSend(event: SentryEvent) {
      // Remove file paths if accidentally present
      if (event.breadcrumbs) {
  event.breadcrumbs = event.breadcrumbs.map((b: Breadcrumb) => {
          if (b?.data && typeof b.data === 'object') {
            delete (b.data as Record<string, unknown>).filePath;
          }
          return b;
        });
      }
      return event;
    }
  });
})();
