<script lang="ts">
	import { onMount, onDestroy } from 'svelte';
	import { invokeWithRetry, invokeWithValidation, ms, validateCreateConversation, validateGetConversations } from '$lib/services/api';
	import { getEnvironment, isTauriAvailable, getEnvironmentInfo } from '$lib/utils/tauri-detection';
	import { safeInvoke } from '$lib/utils/enhanced-tauri-detection';
	import { errorStore } from '$lib/stores/error-store';
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

	// Initialize error store cleanup
	let cleanupErrorStore: (() => void) | null = null;

	onMount(async () => {
		// Initialize error store with cleanup
		cleanupErrorStore = errorStore.init();
		await loadConversations();
		
		// Show desktop welcome notification if running in desktop mode
		if (isTauriAvailable()) {
			try {
				await safeInvoke('show_notification', {
					title: 'Forbidden Library',
					body: 'Desktop application loaded successfully! Enjoy native features.',
					icon: null
				});
			} catch (error) {
				console.log('Notification not available:', error);
			}
		}
	});

	onDestroy(() => {
		// Clean up error store
		if (cleanupErrorStore) {
			cleanupErrorStore();
		}
	});

	function clearError() {
		error = null;
	}

	async function loadConversations() {
		try {
			loading = true;
			error = null;

			const args = { limit: 50, offset: 0 };
			conversations = await invokeWithValidation<Conversation[]>(
				'get_conversations',
				args,
				validateGetConversations,
				ms(8)
			);
		} catch (err) {
			if (err instanceof AppError) {
				// Add to global error store for notification display
				errorStore.addError(err);
				error = err.getUserMessage();
			} else {
				// Handle unexpected errors
				const appError = errorStore.addError({
					message: 'Failed to load conversations',
					details: String(err),
					category: ErrorCategory.DATA,
					severity: ErrorSeverity.ERROR,
					originalError: err
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
			const args = {
				title: 'New Conversation',
				persona_id: null
			};

			const newConversation = await invokeWithValidation<Conversation>(
				'create_conversation',
				args,
				validateCreateConversation,
				ms(8)
			);

			conversations = [newConversation, ...conversations];
			selectedConversation = newConversation;
			error = null; // Clear any previous errors
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
					originalError: err
				});
				error = appError.getUserMessage();
			}
			console.error('Error creating conversation:', err);
		}
	}

	function selectConversation(conversation: Conversation | null) {
		selectedConversation = conversation;
	}
</script>

<!-- Error Notifications -->
<ErrorNotification />

<!-- Environment Indicator -->
{#if environment === 'web'}
	<div class="bg-yellow-600 text-white px-4 py-2 text-center text-sm">
		🌐 Web Mode - Running in browser.
		<button class="underline ml-2" on:click={() => alert('To access full features, run: pnpm run tauri:dev')}>>
			Install Desktop App
		</button>
	</div>
{/if}

<div class="flex h-full">
	<!-- Sidebar -->
	<div class="w-80 bg-gray-800 border-r border-gray-700 flex flex-col">
		<!-- New Conversation Button -->
		<div class="p-4 border-b border-gray-700">
			<button
				on:click={createNewConversation}
				class="w-full bg-purple-600 hover:bg-purple-700 text-white font-medium py-2 px-4 rounded-lg transition-colors flex items-center justify-center space-x-2"
			>
				<svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
					<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 4v16m8-8H4"></path>
				</svg>
				<span>New Conversation</span>
			</button>
		</div>

		<!-- Conversation List -->
		<div class="flex-1 overflow-y-auto">
			{#if loading}
				<div class="p-4 text-center text-gray-400">
					<div class="animate-spin rounded-full h-8 w-8 border-b-2 border-purple-600 mx-auto"></div>
					<p class="mt-2">Loading conversations...</p>
				</div>
			{:else if error}
				<div class="p-4 text-center text-red-400">
					<p>{error}</p>
					<div class="mt-3 space-y-2">
						<button
							on:click={loadConversations}
							class="block w-full text-purple-400 hover:text-purple-300 underline"
							disabled={loading}
						>
							{loading ? 'Retrying...' : 'Retry'}
						</button>
						<button
							on:click={clearError}
							class="block w-full text-gray-400 hover:text-gray-300 text-sm"
						>
							Dismiss
						</button>
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
				<div class="text-center">
					<div class="w-16 h-16 bg-gray-700 rounded-full flex items-center justify-center mx-auto mb-4">
						<svg class="w-8 h-8" fill="none" stroke="currentColor" viewBox="0 0 24 24">
							<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M8 12h.01M12 12h.01M16 12h.01M21 12c0 4.418-4.03 8-9 8a9.863 9.863 0 01-4.255-.949L3 20l1.395-3.72C3.512 15.042 3 13.574 3 12c0-4.418 4.03-8 9-8s9 3.582 9 8z"></path>
						</svg>
					</div>
					<h2 class="text-xl font-semibold mb-2">Welcome to Forbidden Library</h2>
					<p class="text-gray-500 mb-4">Select a conversation or create a new one to get started</p>

					{#if environment === 'web'}
						<div class="bg-gray-700 rounded-lg p-4 max-w-md mx-auto">
							<p class="text-sm text-gray-300 mb-2">
								🌐 You're running in web mode. For the full experience with local storage,
								file system access, and enhanced privacy controls, run the desktop application.
							</p>
							<button
								class="text-purple-400 hover:text-purple-300 text-sm underline"
								on:click={() => alert('Run: pnpm run tauri:dev')}
							>
								How to install desktop app
							</button>
						</div>
					{/if}
				</div>
			</div>
		{/if}
	</div>
</div>

