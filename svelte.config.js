import { vitePreprocess } from '@sveltejs/vite-plugin-svelte';

// Dynamically import adapter-static with a safe fallback to satisfy editor tooling
let adapter;
try {
  ({ default: adapter } = await import('@sveltejs/adapter-static'));
} catch (e) {
  // Minimal no-op adapter fallback for language server environments
  adapter = function adapterStaticFallback() {
    return {
      name: 'adapter-static-fallback',
      adapt: () => {}
    };
  };
}

/** @type {import('@sveltejs/kit').Config} */
const config = {
  // Consult https://kit.svelte.dev/docs/integrations#preprocessors
  // for more information about preprocessors
  preprocess: vitePreprocess(),

  kit: {
    alias: {
      $lib: 'src/lib',
    },
    // adapter-static is required for Tauri
    adapter: adapter({
      // default options are shown. On some platforms
      // these options are set automatically — see below
      pages: 'build',
      assets: 'build',
      // Enable SPA fallback so dynamic routes resolve to index.html in Tauri
      fallback: 'index.html',
      precompress: false,
      // Keep strict; fallback handles dynamic routes for SPA-style apps
      strict: true
    }),

    // Prefer SPA + CSR inside Tauri; keep prerender warnings non-fatal
    prerender: {
      handleHttpError: 'warn'
    }
  }
};

export default config;
