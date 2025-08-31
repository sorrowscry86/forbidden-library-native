// See https://kit.svelte.dev/docs/types#app
// for information about these interfaces

import { ParameterizedString, RequiredPageData } from '@sveltejs/kit';

declare global {
	namespace App {
		// interface Error {}
		// interface Locals {}
		// interface PageData {}
		// interface Platform {}
	}

	// Vite env types
	interface ImportMetaEnv {
		readonly DEV: boolean;
		readonly PROD: boolean;
		readonly MODE: string;
		readonly NODE_ENV: string;
		readonly SENTRY_DSN?: string;
		readonly SENTRY_TRACES_SAMPLE_RATE?: string;
		readonly SENTRY_PROFILES_SAMPLE_RATE?: string;
	}

	interface ImportMeta {
		readonly env: ImportMetaEnv;
	}

	// Sentry build-time constants
	declare const __SENTRY_DSN__: string;
	declare const __SENTRY_TRACES_SAMPLE_RATE__: string;
	declare const __SENTRY_PROFILES_SAMPLE_RATE__: string;
	declare const __APP_VERSION__: string;
	declare const __APP_ENV__: string;

	// Error.captureStackTrace for Node.js environments
	interface ErrorConstructor {
		captureStackTrace?(thisArg: any, func: any): void;
	}
}

export {};