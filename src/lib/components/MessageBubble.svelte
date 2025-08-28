<script lang="ts">
	import type { Message } from '$lib/types/models';
	export let message: Message;

	function formatTime(dateString: string): string {
		const date = new Date(dateString);
		return date.toLocaleTimeString([], { hour: '2-digit', minute: '2-digit' });
	}

	function getRoleColor(role: string): string {
		switch (role.toLowerCase()) {
			case 'user':
				return 'bg-purple-600 text-white';
			case 'assistant':
				return 'bg-gray-700 text-gray-100';
			case 'system':
				return 'bg-red-600 text-white';
			default:
				return 'bg-gray-600 text-gray-100';
		}
	}

	function getRoleIcon(role: string): string {
		switch (role.toLowerCase()) {
			case 'user':
				return `<svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
					<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M16 7a4 4 0 11-8 0 4 4 0 018 0zM12 14a7 7 0 00-7 7h14a7 7 0 00-7-7z"></path>
				</svg>`;
			case 'assistant':
				return `<svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
					<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9.663 17h4.673M12 3v1m6.364 1.636l-.707.707M21 12h-1M4 12H3m3.343-5.657l-.707-.707m2.828 9.9a5 5 0 117.072 0l-.548.547A3.374 3.374 0 0014 18.469V19a2 2 0 11-4 0v-.531c0-.895-.356-1.754-.988-2.386l-.548-.547z"></path>
				</svg>`;
			case 'system':
				return `<svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
					<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 9v2m0 4h.01m-6.938 4h13.856c1.54 0 2.502-1.667 1.732-3L13.732 4c-.77-1.333-2.694-1.333-3.464 0L3.34 16c-.77 1.333.192 3 1.732 3z"></path>
				</svg>`;
			default:
				return `<svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
					<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M16 7a4 4 0 11-8 0 4 4 0 018 0zM12 14a7 7 0 00-7 7h14a7 7 0 00-7-7z"></path>
				</svg>`;
		}
	}

	function isUserMessage(role: string): boolean {
		return role.toLowerCase() === 'user';
	}
</script>

<div class="flex {isUserMessage(message.role) ? 'justify-end' : 'justify-start'}">
	<div class="max-w-xs lg:max-w-md xl:max-w-lg">
		<div class="flex items-start space-x-2 {isUserMessage(message.role) ? 'flex-row-reverse space-x-reverse' : ''}">
			<!-- Avatar -->
			<div class="flex-shrink-0 w-8 h-8 rounded-full {getRoleColor(message.role)} flex items-center justify-center">
				{@html getRoleIcon(message.role)}
			</div>

			<!-- Message Content -->
			<div class="flex flex-col {isUserMessage(message.role) ? 'items-end' : 'items-start'}">
				<!-- Message Bubble -->
				<div class="px-4 py-2 rounded-lg {getRoleColor(message.role)} {isUserMessage(message.role) ? 'rounded-br-sm' : 'rounded-bl-sm'}">
					<p class="text-sm whitespace-pre-wrap break-words">{message.content}</p>
				</div>

				<!-- Message Metadata -->
				<div class="flex items-center space-x-2 mt-1 text-xs text-gray-500">
					<span>{formatTime(message.created_at)}</span>

					{#if message.tokens_used}
						<span>•</span>
						<span>{message.tokens_used} tokens</span>
					{/if}

					{#if message.model_used && message.model_used !== 'mock-model'}
						<span>•</span>
						<span>{message.model_used}</span>
					{/if}
				</div>
			</div>
		</div>
	</div>
</div>

