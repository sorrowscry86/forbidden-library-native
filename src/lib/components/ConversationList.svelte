<script lang="ts">
	import { createEventDispatcher } from 'svelte';
	import { invokeWithTimeout, ms } from '$lib/services/api';
	import type { Conversation } from '$lib/types/models';

	export let conversations: Conversation[] = [];
	export let selectedConversation: Conversation | null = null;

	const dispatch = createEventDispatcher();

	function selectConversation(conversation: Conversation) {
		dispatch('select', conversation);
	}

	async function deleteConversation(conversation: Conversation, event: MouseEvent) {
		event.stopPropagation();

		if (confirm(`Are you sure you want to delete "${conversation.title}"?`)) {
			try {
				await invokeWithTimeout('delete_conversation', { id: conversation.id }, ms(8));
				// Remove from local list
				conversations = conversations.filter(c => c.id !== conversation.id);

				// If this was the selected conversation, clear selection
				if (selectedConversation?.id === conversation.id) {
					dispatch('select', null);
				}
			} catch (error) {
				console.error('Failed to delete conversation:', error);
				alert('Failed to delete conversation. Please try again.');
			}
		}
	}

	function formatDate(dateString: string) {
		const date = new Date(dateString);
		const now = new Date();
		const diffTime = Math.abs(now.getTime() - date.getTime());
		const diffDays = Math.ceil(diffTime / (1000 * 60 * 60 * 24));

		if (diffDays === 1) {
			return 'Today';
		} else if (diffDays === 2) {
			return 'Yesterday';
		} else if (diffDays <= 7) {
			return `${diffDays - 1} days ago`;
		} else {
			return date.toLocaleDateString();
		}
	}

	function truncateTitle(title: string, maxLength = 30) {
		return title.length > maxLength ? title.substring(0, maxLength) + '...' : title;
	}
</script>

<div class="p-2">
	{#each conversations as conversation (conversation.id)}
		<div
			class="conversation-item p-3 rounded-lg cursor-pointer transition-colors mb-2 group relative {selectedConversation?.id === conversation.id ? 'bg-purple-600 text-white' : 'hover:bg-gray-700 text-gray-300'}"
			on:click={() => selectConversation(conversation)}
			role="button"
			tabindex="0"
			on:keydown={(e) => e.key === 'Enter' && selectConversation(conversation)}
		>
			<div class="flex items-start justify-between">
				<div class="flex-1 min-w-0">
					<h3 class="font-medium text-sm truncate" title={conversation.title}>
						{truncateTitle(conversation.title)}
					</h3>
					<p class="text-xs opacity-75 mt-1">
						{formatDate(conversation.updated_at)}
					</p>
				</div>

				<button
					class="delete-btn opacity-0 group-hover:opacity-100 p-1 rounded hover:bg-red-600 transition-all ml-2"
					on:click={(e) => deleteConversation(conversation, e)}
					title="Delete conversation"
				>
					<svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
						<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 7l-.867 12.142A2 2 0 0116.138 21H7.862a2 2 0 01-1.995-1.858L5 7m5 4v6m4-6v6m1-10V4a1 1 0 00-1-1h-4a1 1 0 00-1 1v3M4 7h16"></path>
					</svg>
				</button>
			</div>

			{#if conversation.archived}
				<div class="mt-2">
					<span class="inline-flex items-center px-2 py-1 rounded-full text-xs bg-yellow-600 text-yellow-100">
						<svg class="w-3 h-3 mr-1" fill="none" stroke="currentColor" viewBox="0 0 24 24">
							<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 8l6 6 6-6"></path>
						</svg>
						Archived
					</span>
				</div>
			{/if}
		</div>
	{:else}
		<div class="text-center text-gray-500 py-8">
			<svg class="w-12 h-12 mx-auto mb-4 opacity-50" fill="none" stroke="currentColor" viewBox="0 0 24 24">
				<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M8 12h.01M12 12h.01M16 12h.01M21 12c0 4.418-4.03 8-9 8a9.863 9.863 0 01-4.255-.949L3 20l1.395-3.72C3.512 15.042 3 13.574 3 12c0-4.418 4.03-8 9-8s9 3.582 9 8z"></path>
			</svg>
			<p class="text-sm">No conversations yet</p>
			<p class="text-xs opacity-75 mt-1">Create your first conversation to get started</p>
		</div>
	{/each}
</div>

<style>
	.conversation-item:hover .delete-btn {
		opacity: 1;
	}
</style>
