// src/lib/stores/environment.ts
import { writable } from 'svelte/store';

export type AppEnvironment = 'tauri' | 'web' | 'unknown';

export const environment = writable<AppEnvironment>('unknown');
