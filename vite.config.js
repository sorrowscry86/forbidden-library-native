import { sveltekit } from '@sveltejs/kit/vite';
import { defineConfig, loadEnv } from 'vite';

export default defineConfig(({ mode }) => {
  const env = loadEnv(mode, process.cwd(), '');
  return ({
  plugins: [sveltekit()],

  // Vite options tailored for Tauri development
  clearScreen: false,

  // Tauri expects a fixed port, fail if that port is not available
  server: {
    port: 1430,
    strictPort: true,
    watch: {
      // 3. tell vite to ignore watching `src-tauri`
      ignored: ["**/src-tauri/**"],
    },
  },

  // Build options for production
  build: {
    // Tauri supports es2021
    target: process.env.TAURI_PLATFORM == "windows" ? "chrome105" : "safari13",
    // don't minify for debug builds
    minify: !process.env.TAURI_DEBUG ? "esbuild" : false,
    // produce sourcemaps for debug builds
    sourcemap: !!process.env.TAURI_DEBUG,
  },

  // Environment variables available to the frontend
  define: {
    __APP_VERSION__: JSON.stringify(process.env.npm_package_version),
    __SENTRY_DSN__: JSON.stringify(env.SENTRY_DSN || ''),
    __SENTRY_TRACES_SAMPLE_RATE__: JSON.stringify(env.SENTRY_TRACES_SAMPLE_RATE || '0'),
    __SENTRY_PROFILES_SAMPLE_RATE__: JSON.stringify(env.SENTRY_PROFILES_SAMPLE_RATE || '0'),
    __APP_ENV__: JSON.stringify(env.NODE_ENV || process.env.NODE_ENV || 'development')
  },

  test: {
    include: ['src/**/*.{test,spec}.{js,ts}']
  }
});
});
