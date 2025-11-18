<script lang="ts">
	import { onMount, onDestroy } from 'svelte';
	import {
		invokeWithIntelligentRetry,
		invokeWithValidation,
		ms,
		validateCreateConversation,
		validateGetConversations,
		healthCheck,
		getCircuitBreakerMetrics
	} from '$lib/services/enhanced-api';
	import {
		getEnvironment,
		isTauriAvailable,
		getEnvironmentInfo,
		monitorEnvironmentChanges,
		environmentHealthCheck
	} from '$lib/utils/enhanced-tauri-detection';
	import { errorStore, detectErrorPatterns } from '$lib/stores/enhanced-error-store';
	import { AppError, ErrorCategory, ErrorSeverity } from '$lib/types/errors';
	import ConversationList from '$lib/components/ConversationList.svelte';
	import ChatInterface from '$lib/components/ChatInterface.svelte';
	import ErrorNotification from '$lib/components/ErrorNotification.svelte';
	import type { Conversation } from '$lib/types/models';

	let conversations: Conversation[] = [];
	let selectedConversation: Conversation | null = null;
	let loading = false;
	let error: string | null = null;
	let environment = getEnvironment();
	let environmentInfo = getEnvironmentInfo();
	let systemHealth: 'healthy' | 'degraded' | 'unhealthy' = 'healthy';
	let showDiagnostics = false;
	let diagnosticsData: {
		apiHealth?: ReturnType<typeof checkApiHealth> extends Promise<infer T> ? T : never;
		envHealth?: ReturnType<typeof checkEnvironmentHealth>;
		errorPatterns?: ReturnType<typeof detectErrorPatterns>;
		circuitBreakers?: ReturnType<typeof getCircuitBreakerMetrics>;
		errorAnalytics?: ReturnType<typeof errorStore.getAnalytics>;
		systemHealth?: 'healthy' | 'degraded' | 'unhealthy';
	} = {};

	// Enhanced state management
	let retryCount = 0;
	let lastLoadAttempt = 0;
	let isRetrying = false;
	let connectionStatus: 'online' | 'offline' | 'unknown' = 'unknown';

	// Initialize error store cleanup and monitoring
	let cleanupErrorStore: (() => void) | null = null;
	let cleanupEnvironmentMonitor: (() => void) | null = null;
	let healthCheckInterval: ReturnType<typeof setInterval> | null = null;

	onMount(async () => {
		// Initialize enhanced error store
		cleanupErrorStore = errorStore.init(200, 7200000); // 2 hours retention

		// Monitor environment changes
		cleanupEnvironmentMonitor = monitorEnvironmentChanges((newEnv) => {
			environment = newEnv;
			environmentInfo = getEnvironmentInfo();
			console.log(`Environment changed to: ${newEnv}`);
		});

		// Setup periodic health checks
		healthCheckInterval = setInterval(async () => {
			await performHealthCheck();
		}, 30000); // Every 30 seconds

		// Initial load with enhanced error handling
		await loadConversationsWithRecovery();

		// Initial health check
		await performHealthCheck();

		// Setup connection monitoring
		if (typeof window !== 'undefined') {
			updateConnectionStatus();
			window.addEventListener('online', updateConnectionStatus);
			window.addEventListener('offline', updateConnectionStatus);
		}
	});

	onDestroy(() => {
		// Clean up all resources
		if (cleanupErrorStore) {
			cleanupErrorStore();
		}
		if (cleanupEnvironmentMonitor) {
			cleanupEnvironmentMonitor();
		}
		if (healthCheckInterval) {
			clearInterval(healthCheckInterval);
		}
		if (typeof window !== 'undefined') {
			window.removeEventListener('online', updateConnectionStatus);
			window.removeEventListener('offline', updateConnectionStatus);
		}
	});

	function updateConnectionStatus() {
		connectionStatus = typeof navigator !== 'undefined'
			? (navigator.onLine ? 'online' : 'offline')
			: 'unknown';
	}

	async function performHealthCheck() {
		try {
			const [apiHealth, envHealth] = await Promise.all([
				healthCheck(),
				environmentHealthCheck()
			]);

			systemHealth = apiHealth.status === 'healthy' && envHealth.status === 'healthy'
				? 'healthy'
				: apiHealth.status === 'unhealthy' || envHealth.status === 'unhealthy'
				? 'unhealthy'
				: 'degraded';

			// Update diagnostics data
			diagnosticsData = {
				apiHealth,
				envHealth,
				errorPatterns: detectErrorPatterns(),
				circuitBreakers: getCircuitBreakerMetrics(),
				errorAnalytics: errorStore.getAnalytics(),
				timestamp: new Date().toISOString()
			};

		} catch (err) {
			console.warn('Health check failed:', err);
			systemHealth = 'degraded';
		}
	}

	function clearError() {
		error = null;
		retryCount = 0;
	}

	async function loadConversationsWithRecovery() {
		const maxRetries = 3;
		let attempt = 0;

		while (attempt <= maxRetries) {
			try {
				await loadConversations();
				retryCount = 0; // Reset on success
				isRetrying = false;
				return;
			} catch (err) {
				attempt++;
				retryCount = attempt;

				if (attempt > maxRetries) {
					isRetrying = false;
					throw err; // Re-throw after all retries failed
				}

				// Progressive delay
				const delay = Math.min(1000 * Math.pow(2, attempt - 1), 10000);
				console.log(`Retrying conversation load in ${delay}ms (attempt ${attempt}/${maxRetries})`);

				isRetrying = true;
				await new Promise(resolve => setTimeout(resolve, delay));
			}
		}
	}

	async function loadConversations() {
		try {
			loading = true;
			error = null;
			lastLoadAttempt = Date.now();

			const args = { limit: 50, offset: 0 };

			// Use intelligent retry for better resilience
			conversations = await invokeWithIntelligentRetry<Conversation[]>(
				'get_conversations',
				args,
				{
					maxRetries: 2,
					baseDelayMs: 1000,
					retryableErrors: [ErrorCategory.TIMEOUT, ErrorCategory.NETWORK, ErrorCategory.API]
				}
			);

			// Track successful operation
			errorStore.trackSuccess('get_conversations', retryCount > 0);

		} catch (err) {
			if (err instanceof AppError) {
				// Add to global error store for notification display
				errorStore.addError(err);
				error = err.getUserMessage();
			} else {
				// Handle unexpected errors with enhanced context
				const appError = errorStore.addError({
					message: 'Failed to load conversations',
					details: String(err),
					category: ErrorCategory.DATA,
					severity: ErrorSeverity.ERROR,
					originalError: err,
					context: {
						attempt: retryCount + 1,
						lastAttempt: lastLoadAttempt,
						environment,
						connectionStatus
					}
				});
				error = appError.getUserMessage();
			}
			console.error('Error loading conversations:', err);
		} finally {
			loading = false;
		}
	}

	async function createNewConversation() {
		try {
			loading = true;
			const args = {
				title: 'New Conversation',
				persona_id: null
			};

			// Enhanced validation and retry
			const newConversation = await invokeWithValidation<Conversation>(
				'create_conversation',
				args,
				validateCreateConversation,
				ms(10) // Slightly longer timeout for creation
			);

			conversations = [newConversation, ...conversations];
			selectedConversation = newConversation;
			error = null; // Clear any previous errors

			// Track successful operation
			errorStore.trackSuccess('create_conversation');

		} catch (err) {
			if (err instanceof AppError) {
				// Add to global error store for notification display
				errorStore.addError(err);
				error = err.getUserMessage();
			} else {
				// Handle unexpected errors
				const appError = errorStore.addError({
					message: 'Failed to create conversation',
					details: String(err),
					category: ErrorCategory.API,
					severity: ErrorSeverity.ERROR,
					originalError: err,
					context: {
						environment,
						connectionStatus,
						systemHealth
					}
				});
				error = appError.getUserMessage();
			}
			console.error('Error creating conversation:', err);
		} finally {
			loading = false;
		}
	}

	function selectConversation(conversation: Conversation | null) {
		selectedConversation = conversation;
	}

	async function handleRetry() {
		if (isRetrying) return; // Prevent multiple simultaneous retries
		await loadConversationsWithRecovery();
	}

	function toggleDiagnostics() {
		showDiagnostics = !showDiagnostics;
	}

	// Get status indicator color
	function getStatusColor(status: string): string {
		switch (status) {
			case 'healthy': return 'text-green-400';
			case 'degraded': return 'text-yellow-400';
			case 'unhealthy': return 'text-red-400';
			default: return 'text-gray-400';
		}
	}

	// Get connection status color
	function getConnectionColor(status: string): string {
		switch (status) {
			case 'online': return 'text-green-400';
			case 'offline': return 'text-red-400';
			default: return 'text-gray-400';
		}
	}
</script>

<!-- Error Notifications -->
<ErrorNotification />

<!-- Enhanced Environment and Status Indicators -->
<div class="bg-gray-900 border-b border-gray-700">
	<!-- Environment Indicator -->
	{#if environment === 'web'}
		<div class="bg-yellow-600 text-white px-4 py-2 text-center text-sm">
			🌐 Web Mode - Limited functionality.
			<a href="#" class="underline ml-2" onclick="alert('To access full features, run: pnpm run tauri:dev')">
				Install Desktop App
			</a>
		</div>
	{/if}

	<!-- Status Bar -->
	<div class="px-4 py-2 flex items-center justify-between text-sm">
		<div class="flex items-center space-x-4">
			<div class="flex items-center space-x-2">
				<span class="text-gray-400">System:</span>
				<span class="{getStatusColor(systemHealth)}">
					{systemHealth.charAt(0).toUpperCase() + systemHealth.slice(1)}
				</span>
			</div>

			<div class="flex items-center space-x-2">
				<span class="text-gray-400">Connection:</span>
				<span class="{getConnectionColor(connectionStatus)}">
					{connectionStatus.charAt(0).toUpperCase() + connectionStatus.slice(1)}
				</span>
			</div>

			<div class="flex items-center space-x-2">
				<span class="text-gray-400">Environment:</span>
				<span class="text-blue-400">{environment}</span>
			</div>

			{#if retryCount > 0}
				<div class="flex items-center space-x-2">
					<span class="text-gray-400">Retries:</span>
					<span class="text-orange-400">{retryCount}</span>
				</div>
			{/if}
		</div>

		<button
			on:click={toggleDiagnostics}
			class="text-gray-400 hover:text-gray-300 text-xs px-2 py-1 rounded border border-gray-600 hover:border-gray-500"
		>
			{showDiagnostics ? 'Hide' : 'Show'} Diagnostics
		</button>
	</div>

	<!-- Diagnostics Panel -->
	{#if showDiagnostics}
		<div class="px-4 py-3 bg-gray-800 border-t border-gray-700">
			<div class="text-xs text-gray-300 space-y-2">
				<div class="grid grid-cols-2 gap-4">
					<div>
						<h4 class="font-medium text-gray-200 mb-1">Environment Info</h4>
						<div class="space-y-1">
							<div>Confidence: {Math.round((environmentInfo.confidence || 0) * 100)}%</div>
							<div>Platform: {environmentInfo.platform}</div>
							<div>User Agent: {environmentInfo.userAgent.substring(0, 50)}...</div>
						</div>
					</div>

					<div>
						<h4 class="font-medium text-gray-200 mb-1">Error Analytics</h4>
						<div class="space-y-1">
							<div>Error Rate: {diagnosticsData.errorAnalytics?.errorRate || 0}/min</div>
							<div>Recovery Rate: {diagnosticsData.errorAnalytics?.recoveryRate || 100}%</div>
							<div>Total Errors: {diagnosticsData.errorAnalytics?.totalErrors || 0}</div>
						</div>
					</div>
				</div>

				{#if diagnosticsData.circuitBreakers && Object.keys(diagnosticsData.circuitBreakers).length > 0}
					<div>
						<h4 class="font-medium text-gray-200 mb-1">Circuit Breakers</h4>
						<div class="flex flex-wrap gap-2">
							{#each Object.entries(diagnosticsData.circuitBreakers) as [command, metrics]}
								<span class="px-2 py-1 rounded text-xs {metrics.state === 'closed' ? 'bg-green-600' : metrics.state === 'open' ? 'bg-red-600' : 'bg-yellow-600'}">
									{command}: {metrics.state}
								</span>
							{/each}
						</div>
					</div>
				{/if}
			</div>
		</div>
	{/if}
</div>

<div class="flex h-full">
	<!-- Sidebar -->
	<div class="w-80 bg-gray-800 border-r border-gray-700 flex flex-col">
		<!-- New Conversation Button -->
		<div class="p-4 border-b border-gray-700">
			<button
				on:click={createNewConversation}
				disabled={loading}
				class="w-full bg-purple-600 hover:bg-purple-700 disabled:bg-purple-800 disabled:cursor-not-allowed text-white font-medium py-2 px-4 rounded-lg transition-colors flex items-center justify-center space-x-2"
			>
				{#if loading}
					<div class="animate-spin rounded-full h-4 w-4 border-b-2 border-white"></div>
				{:else}
					<svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
						<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 4v16m8-8H4"></path>
					</svg>
				{/if}
				<span>{loading ? 'Creating...' : 'New Conversation'}</span>
			</button>
		</div>

		<!-- Conversation List -->
		<div class="flex-1 overflow-y-auto">
			{#if loading && conversations.length === 0}
				<div class="p-4 text-center text-gray-400">
					<div class="animate-spin rounded-full h-8 w-8 border-b-2 border-purple-600 mx-auto"></div>
					<p class="mt-2">
						{isRetrying ? `Retrying... (${retryCount}/3)` : 'Loading conversations...'}
					</p>
				</div>
			{:else if error}
				<div class="p-4 text-center text-red-400">
					<div class="mb-2">
						<svg class="w-8 h-8 mx-auto mb-2" fill="none" stroke="currentColor" viewBox="0 0 24 24">
							<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 9v2m0 4h.01m-6.938 4h13.856c1.54 0 2.502-1.667 1.732-2.5L13.732 4c-.77-.833-1.964-.833-2.732 0L3.732 16.5c-.77.833.192 2.5 1.732 2.5z"></path>
						</svg>
					</div>
					<p class="mb-3">{error}</p>

					{#if retryCount > 0}
						<p class="text-xs text-gray-500 mb-3">
							Failed after {retryCount} attempt{retryCount > 1 ? 's' : ''}
						</p>
					{/if}

					<div class="space-y-2">
						<button
							on:click={handleRetry}
							disabled={loading || isRetrying}
							class="block w-full text-purple-400 hover:text-purple-300 disabled:text-purple-600 underline"
						>
							{isRetrying ? `Retrying... (${retryCount}/3)` : loading ? 'Loading...' : 'Retry'}
						</button>
						<button
							on:click={clearError}
							class="block w-full text-gray-400 hover:text-gray-300 text-sm"
						>
							Dismiss
						</button>

						{#if systemHealth !== 'healthy'}
							<button
								on:click={performHealthCheck}
								class="block w-full text-yellow-400 hover:text-yellow-300 text-sm"
							>
								Check System Health
							</button>
						{/if}
					</div>
				</div>
			{:else}
				<ConversationList
					{conversations}
					{selectedConversation}
					on:select={(event) => selectConversation(event.detail)}
				/>
			{/if}
		</div>
	</div>

	<!-- Main Chat Area -->
	<div class="flex-1 flex flex-col">
		{#if selectedConversation}
			<ChatInterface conversation={selectedConversation} />
		{:else}
			<div class="flex-1 flex items-center justify-center text-gray-400">
				<div class="text-center max-w-md">
					<div class="w-16 h-16 bg-gray-700 rounded-full flex items-center justify-center mx-auto mb-4">
						<svg class="w-8 h-8" fill="none" stroke="currentColor" viewBox="0 0 24 24">
							<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M8 12h.01M12 12h.01M16 12h.01M21 12c0 4.418-4.03 8-9 8a9.863 9.863 0 01-4.255-.949L3 20l1.395-3.72C3.512 15.042 3 13.574 3 12c0-4.418 4.03-8 9-8s9 3.582 9 8z"></path>
						</svg>
					</div>
					<h2 class="text-xl font-semibold mb-2">Welcome to Forbidden Library</h2>
					<p class="text-gray-500 mb-4">Select a conversation or create a new one to get started</p>

					{#if environment === 'web'}
						<div class="bg-gray-700 rounded-lg p-4 mb-4">
							<p class="text-sm text-gray-300 mb-2">
								🌐 You're running in web mode. For the full experience with local storage,
								file system access, and enhanced privacy controls, run the desktop application.
							</p>
							<button
								class="text-purple-400 hover:text-purple-300 text-sm underline"
								onclick="alert('Run: pnpm run tauri:dev')"
							>
								How to install desktop app
							</button>
						</div>
					{/if}

					{#if systemHealth !== 'healthy'}
						<div class="bg-yellow-900 border border-yellow-600 rounded-lg p-3 text-sm">
							<div class="flex items-center space-x-2 mb-2">
								<svg class="w-4 h-4 text-yellow-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
									<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 9v2m0 4h.01m-6.938 4h13.856c1.54 0 2.502-1.667 1.732-2.5L13.732 4c-.77-.833-1.964-.833-2.732 0L3.732 16.5c-.77.833.192 2.5 1.732 2.5z"></path>
								</svg>
								<span class="text-yellow-300 font-medium">System Status: {systemHealth}</span>
							</div>
							<p class="text-yellow-200">
								Some features may be limited. The system is monitoring and attempting to recover automatically.
							</p>
						</div>
					{/if}
				</div>
			</div>
		{/if}
	</div>
</div>
