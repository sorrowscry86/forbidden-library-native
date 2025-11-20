<script lang="ts">
	import { onMount, onDestroy } from 'svelte';
	import { invokeWithTimeout, ms } from '$lib/services/api';
	import { fade, slide } from '$lib/utils/animations';
	import { announceToScreenReader } from '$lib/utils/accessibility';
	import LoadingStates from './LoadingStates.svelte';
	import SearchHighlight from './SearchHighlight.svelte';
	import { createEventDispatcher } from 'svelte';

	// Props
	export let isOpen = false;
	export let onClose: (() => void) | null = null;

	const dispatch = createEventDispatcher();

	// Search state
	let query = '';
	let searchMode: 'all' | 'messages' | 'titles' | 'phrases' = 'all';
	let results: SearchResult[] = [];
	let suggestions: string[] = [];
	let loading = false;
	let error: string | null = null;
	let selectedIndex = 0;
	let showFilters = false;

	// Filter state
	let filters = {
		persona_id: null as number | null,
		date_from: null as string | null,
		date_to: null as string | null,
		archived: null as boolean | null,
		min_tokens: null as number | null,
		max_tokens: null as number | null
	};

	// Search result type
	interface SearchResult {
		conversation_id: number;
		message_id: number | null;
		title: string;
		content: string;
		relevance_score: number;
		created_at: string;
		snippet: string;
	}

	// Debounce timer
	let debounceTimer: ReturnType<typeof setTimeout> | null = null;
	let suggestionsTimer: ReturnType<typeof setTimeout> | null = null;

	// Search input element
	let searchInput: HTMLInputElement;

	onMount(() => {
		if (isOpen && searchInput) {
			searchInput.focus();
		}
	});

	onDestroy(() => {
		if (debounceTimer) clearTimeout(debounceTimer);
		if (suggestionsTimer) clearTimeout(suggestionsTimer);
	});

	// Focus search input when panel opens
	$: if (isOpen && searchInput) {
		setTimeout(() => searchInput.focus(), 100);
	}

	// Watch query changes for suggestions
	$: if (query.length >= 2) {
		loadSuggestions();
	} else {
		suggestions = [];
	}

	async function performSearch() {
		if (!query.trim()) {
			results = [];
			return;
		}

		try {
			loading = true;
			error = null;

			let searchResults: SearchResult[] = [];

			switch (searchMode) {
				case 'all':
					searchResults = await invokeWithTimeout(
						'search_messages',
						{
							query: query.trim(),
							persona_id: filters.persona_id,
							date_from: filters.date_from,
							date_to: filters.date_to,
							archived: filters.archived,
							min_tokens: filters.min_tokens,
							max_tokens: filters.max_tokens,
							limit: 50
						},
						ms(10)
					);
					break;

				case 'messages':
					searchResults = await invokeWithTimeout(
						'search_messages',
						{
							query: query.trim(),
							persona_id: filters.persona_id,
							date_from: filters.date_from,
							date_to: filters.date_to,
							archived: filters.archived,
							min_tokens: filters.min_tokens,
							max_tokens: filters.max_tokens,
							limit: 50
						},
						ms(10)
					);
					break;

				case 'titles':
					searchResults = await invokeWithTimeout(
						'search_titles',
						{
							query: query.trim(),
							limit: 50
						},
						ms(10)
					);
					break;

				case 'phrases':
					searchResults = await invokeWithTimeout(
						'search_phrases',
						{
							phrase: query.trim(),
							limit: 50
						},
						ms(10)
					);
					break;
			}

			results = searchResults;
			selectedIndex = 0;

			// Announce results to screen readers
			announceToScreenReader(`Found ${results.length} results for ${query}`);
		} catch (err) {
			error = `Search failed: ${String(err)}`;
			results = [];
			console.error('Search error:', err);
		} finally {
			loading = false;
		}
	}

	async function loadSuggestions() {
		if (suggestionsTimer) clearTimeout(suggestionsTimer);

		suggestionsTimer = setTimeout(async () => {
			try {
				suggestions = await invokeWithTimeout(
					'get_search_suggestions',
					{
						partial_query: query.trim(),
						limit: 5
					},
					ms(5)
				);
			} catch (err) {
				console.error('Failed to load suggestions:', err);
				suggestions = [];
			}
		}, 300);
	}

	function handleInput() {
		if (debounceTimer) clearTimeout(debounceTimer);

		debounceTimer = setTimeout(() => {
			performSearch();
		}, 400);
	}

	function handleKeyDown(event: KeyboardEvent) {
		switch (event.key) {
			case 'Escape':
				event.preventDefault();
				handleClose();
				break;

			case 'ArrowDown':
				event.preventDefault();
				selectedIndex = Math.min(selectedIndex + 1, results.length - 1);
				scrollToSelected();
				break;

			case 'ArrowUp':
				event.preventDefault();
				selectedIndex = Math.max(selectedIndex - 1, 0);
				scrollToSelected();
				break;

			case 'Enter':
				event.preventDefault();
				if (results[selectedIndex]) {
					selectResult(results[selectedIndex]);
				}
				break;
		}
	}

	function scrollToSelected() {
		const element = document.getElementById(`search-result-${selectedIndex}`);
		if (element) {
			element.scrollIntoView({ block: 'nearest', behavior: 'smooth' });
		}
	}

	function selectResult(result: SearchResult) {
		dispatch('select', {
			conversationId: result.conversation_id,
			messageId: result.message_id
		});
		handleClose();
	}

	function handleClose() {
		if (onClose) {
			onClose();
		}
		isOpen = false;
	}

	function clearFilters() {
		filters = {
			persona_id: null,
			date_from: null,
			date_to: null,
			archived: null,
			min_tokens: null,
			max_tokens: null
		};
		performSearch();
	}

	function formatDate(dateStr: string): string {
		try {
			const date = new Date(dateStr);
			return date.toLocaleDateString('en-US', {
				month: 'short',
				day: 'numeric',
				year: 'numeric',
				hour: '2-digit',
				minute: '2-digit'
			});
		} catch {
			return dateStr;
		}
	}

	function getRelevanceColor(score: number): string {
		if (score >= -2) return 'text-green-400';
		if (score >= -5) return 'text-yellow-400';
		return 'text-gray-400';
	}
</script>

{#if isOpen}
	<!-- Backdrop -->
	<div
		class="fixed inset-0 bg-black bg-opacity-50 z-40"
		on:click={handleClose}
		on:keydown={(e) => e.key === 'Escape' && handleClose()}
		transition:fade={{ duration: 200 }}
		role="button"
		tabindex="-1"
		aria-label="Close search"
	></div>

	<!-- Search Panel -->
	<div
		class="fixed top-20 left-1/2 transform -translate-x-1/2 w-full max-w-3xl bg-gray-800 rounded-lg shadow-2xl z-50 overflow-hidden"
		transition:slide={{ duration: 300, axis: 'y', distance: -20 }}
		role="dialog"
		aria-label="Search conversations and messages"
	>
		<!-- Search Header -->
		<div class="p-4 border-b border-gray-700">
			<div class="flex items-center space-x-3 mb-3">
				<!-- Search Icon -->
				<svg class="w-5 h-5 text-gray-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
					<path
						stroke-linecap="round"
						stroke-linejoin="round"
						stroke-width="2"
						d="M21 21l-6-6m2-5a7 7 0 11-14 0 7 7 0 0114 0z"
					></path>
				</svg>

				<!-- Search Input -->
				<input
					bind:this={searchInput}
					bind:value={query}
					on:input={handleInput}
					on:keydown={handleKeyDown}
					type="text"
					placeholder="Search conversations and messages..."
					class="flex-1 bg-transparent text-white placeholder-gray-400 outline-none text-lg"
					aria-label="Search query"
					autocomplete="off"
				/>

				<!-- Filter Toggle -->
				<button
					on:click={() => (showFilters = !showFilters)}
					class="p-2 text-gray-400 hover:text-white hover:bg-gray-700 rounded-lg transition-colors"
					title="Toggle filters"
					aria-label="Toggle filters"
				>
					<svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
						<path
							stroke-linecap="round"
							stroke-linejoin="round"
							stroke-width="2"
							d="M3 4a1 1 0 011-1h16a1 1 0 011 1v2.586a1 1 0 01-.293.707l-6.414 6.414a1 1 0 00-.293.707V17l-4 4v-6.586a1 1 0 00-.293-.707L3.293 7.293A1 1 0 013 6.586V4z"
						></path>
					</svg>
				</button>

				<!-- Close Button -->
				<button
					on:click={handleClose}
					class="p-2 text-gray-400 hover:text-white hover:bg-gray-700 rounded-lg transition-colors"
					title="Close (Esc)"
					aria-label="Close search"
				>
					<svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
						<path
							stroke-linecap="round"
							stroke-linejoin="round"
							stroke-width="2"
							d="M6 18L18 6M6 6l12 12"
						></path>
					</svg>
				</button>
			</div>

			<!-- Search Mode Tabs -->
			<div class="flex space-x-2">
				<button
					on:click={() => {
						searchMode = 'all';
						performSearch();
					}}
					class="px-3 py-1 rounded-lg text-sm transition-colors {searchMode === 'all'
						? 'bg-purple-600 text-white'
						: 'text-gray-400 hover:text-white hover:bg-gray-700'}"
				>
					All
				</button>
				<button
					on:click={() => {
						searchMode = 'messages';
						performSearch();
					}}
					class="px-3 py-1 rounded-lg text-sm transition-colors {searchMode === 'messages'
						? 'bg-purple-600 text-white'
						: 'text-gray-400 hover:text-white hover:bg-gray-700'}"
				>
					Messages
				</button>
				<button
					on:click={() => {
						searchMode = 'titles';
						performSearch();
					}}
					class="px-3 py-1 rounded-lg text-sm transition-colors {searchMode === 'titles'
						? 'bg-purple-600 text-white'
						: 'text-gray-400 hover:text-white hover:bg-gray-700'}"
				>
					Titles
				</button>
				<button
					on:click={() => {
						searchMode = 'phrases';
						performSearch();
					}}
					class="px-3 py-1 rounded-lg text-sm transition-colors {searchMode === 'phrases'
						? 'bg-purple-600 text-white'
						: 'text-gray-400 hover:text-white hover:bg-gray-700'}"
				>
					Exact Phrases
				</button>
			</div>
		</div>

		<!-- Advanced Filters -->
		{#if showFilters}
			<div class="p-4 border-b border-gray-700 bg-gray-750" transition:slide={{ duration: 200 }}>
				<div class="grid grid-cols-2 gap-3">
					<div>
						<label class="block text-xs text-gray-400 mb-1">Date From</label>
						<input
							type="date"
							bind:value={filters.date_from}
							on:change={performSearch}
							class="w-full bg-gray-700 text-white text-sm border border-gray-600 rounded px-2 py-1 focus:outline-none focus:ring-2 focus:ring-purple-600"
						/>
					</div>
					<div>
						<label class="block text-xs text-gray-400 mb-1">Date To</label>
						<input
							type="date"
							bind:value={filters.date_to}
							on:change={performSearch}
							class="w-full bg-gray-700 text-white text-sm border border-gray-600 rounded px-2 py-1 focus:outline-none focus:ring-2 focus:ring-purple-600"
						/>
					</div>
					<div>
						<label class="block text-xs text-gray-400 mb-1">Min Tokens</label>
						<input
							type="number"
							bind:value={filters.min_tokens}
							on:change={performSearch}
							placeholder="Any"
							class="w-full bg-gray-700 text-white text-sm border border-gray-600 rounded px-2 py-1 focus:outline-none focus:ring-2 focus:ring-purple-600"
						/>
					</div>
					<div>
						<label class="block text-xs text-gray-400 mb-1">Max Tokens</label>
						<input
							type="number"
							bind:value={filters.max_tokens}
							on:change={performSearch}
							placeholder="Any"
							class="w-full bg-gray-700 text-white text-sm border border-gray-600 rounded px-2 py-1 focus:outline-none focus:ring-2 focus:ring-purple-600"
						/>
					</div>
				</div>
				<div class="mt-3 flex justify-end">
					<button
						on:click={clearFilters}
						class="text-sm text-purple-400 hover:text-purple-300 underline"
					>
						Clear Filters
					</button>
				</div>
			</div>
		{/if}

		<!-- Search Results -->
		<div class="max-h-96 overflow-y-auto">
			{#if loading}
				<div class="p-8">
					<LoadingStates variant="spinner" size="medium" message="Searching..." />
				</div>
			{:else if error}
				<div class="p-8 text-center">
					<div class="text-red-400 mb-2">
						<svg class="w-12 h-12 mx-auto mb-2" fill="none" stroke="currentColor" viewBox="0 0 24 24">
							<path
								stroke-linecap="round"
								stroke-linejoin="round"
								stroke-width="2"
								d="M12 8v4m0 4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z"
							></path>
						</svg>
						<p class="text-sm">{error}</p>
					</div>
					<button
						on:click={performSearch}
						class="text-purple-400 hover:text-purple-300 text-sm underline"
					>
						Try Again
					</button>
				</div>
			{:else if !query.trim()}
				<div class="p-8 text-center text-gray-400">
					<svg class="w-16 h-16 mx-auto mb-3 opacity-50" fill="none" stroke="currentColor" viewBox="0 0 24 24">
						<path
							stroke-linecap="round"
							stroke-linejoin="round"
							stroke-width="2"
							d="M21 21l-6-6m2-5a7 7 0 11-14 0 7 7 0 0114 0z"
						></path>
					</svg>
					<p class="text-lg font-medium mb-2">Search your conversations</p>
					<p class="text-sm text-gray-500">Start typing to find messages, conversations, and more</p>
					<div class="mt-4 text-xs text-gray-500 space-y-1">
						<p>💡 Use quotes for exact phrases: "hello world"</p>
						<p>💡 Search by date, tokens, or persona using filters</p>
					</div>
				</div>
			{:else if results.length === 0}
				<div class="p-8 text-center text-gray-400">
					<svg class="w-16 h-16 mx-auto mb-3 opacity-50" fill="none" stroke="currentColor" viewBox="0 0 24 24">
						<path
							stroke-linecap="round"
							stroke-linejoin="round"
							stroke-width="2"
							d="M9.172 16.172a4 4 0 015.656 0M9 10h.01M15 10h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z"
						></path>
					</svg>
					<p class="text-lg font-medium mb-2">No results found</p>
					<p class="text-sm text-gray-500">Try adjusting your search or filters</p>
				</div>
			{:else}
				<div class="divide-y divide-gray-700">
					{#each results as result, index (result.message_id || result.conversation_id)}
						<button
							id="search-result-{index}"
							on:click={() => selectResult(result)}
							class="w-full text-left p-4 hover:bg-gray-750 transition-colors {index === selectedIndex
								? 'bg-gray-750 border-l-2 border-purple-600'
								: ''}"
						>
							<div class="flex items-start justify-between mb-2">
								<h3 class="font-medium text-white">
									<SearchHighlight text={result.title} query={query} />
								</h3>
								<span class="text-xs {getRelevanceColor(result.relevance_score)} ml-2">
									{result.relevance_score.toFixed(1)}
								</span>
							</div>

							{#if result.snippet}
								<div class="text-sm text-gray-300 mb-2 line-clamp-2">
									{@html result.snippet}
								</div>
							{/if}

							<div class="flex items-center text-xs text-gray-500 space-x-3">
								<span>{formatDate(result.created_at)}</span>
								{#if result.message_id}
									<span class="flex items-center">
										<svg class="w-3 h-3 mr-1" fill="none" stroke="currentColor" viewBox="0 0 24 24">
											<path
												stroke-linecap="round"
												stroke-linejoin="round"
												stroke-width="2"
												d="M8 12h.01M12 12h.01M16 12h.01M21 12c0 4.418-4.03 8-9 8a9.863 9.863 0 01-4.255-.949L3 20l1.395-3.72C3.512 15.042 3 13.574 3 12c0-4.418 4.03-8 9-8s9 3.582 9 8z"
											></path>
										</svg>
										Message
									</span>
								{:else}
									<span class="flex items-center">
										<svg class="w-3 h-3 mr-1" fill="none" stroke="currentColor" viewBox="0 0 24 24">
											<path
												stroke-linecap="round"
												stroke-linejoin="round"
												stroke-width="2"
												d="M7 8h10M7 12h4m1 8l-4-4H5a2 2 0 01-2-2V6a2 2 0 012-2h14a2 2 0 012 2v8a2 2 0 01-2 2h-3l-4 4z"
											></path>
										</svg>
										Conversation
									</span>
								{/if}
							</div>
						</button>
					{/each}
				</div>

				<!-- Results Footer -->
				<div class="p-3 border-t border-gray-700 bg-gray-800 text-center text-xs text-gray-500">
					Showing {results.length} result{results.length !== 1 ? 's' : ''}
					{#if results.length >= 50}
						(limited to 50)
					{/if}
				</div>
			{/if}
		</div>

		<!-- Keyboard Shortcuts Help -->
		<div class="p-3 border-t border-gray-700 bg-gray-800 flex items-center justify-between text-xs text-gray-500">
			<div class="flex items-center space-x-4">
				<span class="flex items-center">
					<kbd class="px-2 py-1 bg-gray-700 rounded text-gray-300 mr-1">↑↓</kbd> Navigate
				</span>
				<span class="flex items-center">
					<kbd class="px-2 py-1 bg-gray-700 rounded text-gray-300 mr-1">Enter</kbd> Select
				</span>
				<span class="flex items-center">
					<kbd class="px-2 py-1 bg-gray-700 rounded text-gray-300 mr-1">Esc</kbd> Close
				</span>
			</div>
			<span>
				Powered by FTS5
			</span>
		</div>
	</div>
{/if}

<style>
	/* Line clamp utility */
	.line-clamp-2 {
		display: -webkit-box;
		-webkit-line-clamp: 2;
		-webkit-box-orient: vertical;
		overflow: hidden;
	}

	/* Custom scrollbar for results */
	.overflow-y-auto::-webkit-scrollbar {
		width: 8px;
	}

	.overflow-y-auto::-webkit-scrollbar-track {
		background: rgba(31, 41, 55, 0.5);
	}

	.overflow-y-auto::-webkit-scrollbar-thumb {
		background: rgba(107, 114, 128, 0.5);
		border-radius: 4px;
	}

	.overflow-y-auto::-webkit-scrollbar-thumb:hover {
		background: rgba(107, 114, 128, 0.7);
	}

	/* Enhanced kbd styling */
	kbd {
		font-family: ui-monospace, monospace;
		font-size: 0.75rem;
		box-shadow: 0 1px 2px rgba(0, 0, 0, 0.3);
	}

	/* Search result highlight from FTS5 */
	:global(mark) {
		background-color: rgba(168, 85, 247, 0.3);
		color: #e9d5ff;
		padding: 0.1rem 0.2rem;
		border-radius: 2px;
	}
</style>
