<script lang="ts">
  import type { Message } from '$lib/types/models';
  import { User, Bot, AlertTriangle } from 'lucide-svelte';
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

  function getRoleIcon(role: string) {
    switch (role.toLowerCase()) {
      case 'user':
        return User;
      case 'assistant':
        return Bot;
      case 'system':
        return AlertTriangle;
      default:
        return User;
    }
  }

  function isUserMessage(role: string): boolean {
    return role.toLowerCase() === 'user';
  }
</script>

<div class="flex {isUserMessage(message.role) ? 'justify-end' : 'justify-start'}">
  <div class="max-w-xs lg:max-w-md xl:max-w-lg">
    <div
      class="flex items-start space-x-2 {isUserMessage(message.role)
        ? 'flex-row-reverse space-x-reverse'
        : ''}"
    >
      <!-- Avatar -->
      <div
        class="flex-shrink-0 w-8 h-8 rounded-full {getRoleColor(
          message.role
        )} flex items-center justify-center"
      >
        <svelte:component this={getRoleIcon(message.role)} class="w-4 h-4" />
      </div>

      <!-- Message Content -->
      <div class="flex flex-col {isUserMessage(message.role) ? 'items-end' : 'items-start'}">
        <!-- Message Bubble -->
        <div
          class="px-4 py-2 rounded-lg {getRoleColor(message.role)} {isUserMessage(message.role)
            ? 'rounded-br-sm'
            : 'rounded-bl-sm'}"
        >
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
