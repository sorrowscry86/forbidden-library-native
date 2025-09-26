// src/lib/sentry.ts
import * as Sentry from '@sentry/sveltekit';

const SENTRY_DSN =
  'https://b9f589b354fd05ee3e2c5d67f4bc3699@o4509552575053824.ingest.us.sentry.io/4509884862169088';

export function initSentry() {
  Sentry.init({
    dsn: SENTRY_DSN,
    tracesSampleRate: 1.0,
    environment: import.meta.env.MODE || 'development',
  });
}
