import { sveltekit } from '@sveltejs/kit/vite';
import { defineConfig } from 'vite';

export default defineConfig(async () => ({
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
  },

  test: {
    include: ['src/**/*.{test,spec}.{js,ts}']
  }
}));
