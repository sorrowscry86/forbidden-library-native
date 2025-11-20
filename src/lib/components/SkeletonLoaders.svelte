<script lang="ts">
  export let variant: 'conversation' | 'message' | 'sidebar' | 'settings' = 'conversation';
  export let count: number = 1;
</script>

<div class="space-y-4" role="status" aria-label="Loading content">
  {#each Array(count) as _, i}
    {#if variant === 'conversation'}
      <!-- Conversation Item Skeleton -->
      <div class="flex items-start gap-3 p-3 rounded-lg border border-border bg-card animate-pulse">
        <div class="w-10 h-10 bg-muted rounded-full flex-shrink-0"></div>
        <div class="flex-1 space-y-2">
          <div class="h-4 bg-muted rounded w-3/4"></div>
          <div class="h-3 bg-muted rounded w-full"></div>
          <div class="h-3 bg-muted rounded w-2/3"></div>
          <div class="flex gap-2 mt-2">
            <div class="h-2 bg-muted rounded w-16"></div>
            <div class="h-2 bg-muted rounded w-12"></div>
          </div>
        </div>
      </div>
    {:else if variant === 'message'}
      <!-- Message Skeleton -->
      <div class="flex gap-3 p-4 rounded-lg border border-border bg-card animate-pulse">
        <div class="w-8 h-8 bg-muted rounded-full flex-shrink-0"></div>
        <div class="flex-1 space-y-3">
          <div class="h-3 bg-muted rounded w-16"></div>
          <div class="space-y-2">
            <div class="h-3 bg-muted rounded w-full"></div>
            <div class="h-3 bg-muted rounded w-11/12"></div>
            <div class="h-3 bg-muted rounded w-4/5"></div>
          </div>
        </div>
      </div>
    {:else if variant === 'sidebar'}
      <!-- Sidebar Item Skeleton -->
      <div class="flex items-center gap-2 px-3 py-2 rounded-md animate-pulse">
        <div class="w-4 h-4 bg-muted rounded"></div>
        <div class="h-3 bg-muted rounded flex-1"></div>
      </div>
    {:else if variant === 'settings'}
      <!-- Settings Section Skeleton -->
      <div class="space-y-4 p-4 border border-border rounded-lg animate-pulse">
        <div class="h-5 bg-muted rounded w-1/4"></div>
        <div class="space-y-3">
          <div class="flex items-center justify-between">
            <div class="h-3 bg-muted rounded w-1/3"></div>
            <div class="h-8 bg-muted rounded w-16"></div>
          </div>
          <div class="flex items-center justify-between">
            <div class="h-3 bg-muted rounded w-1/4"></div>
            <div class="h-8 bg-muted rounded w-24"></div>
          </div>
        </div>
      </div>
    {/if}
  {/each}
</div>

<style>
  @keyframes pulse {
    0%, 100% {
      opacity: 1;
    }
    50% {
      opacity: 0.5;
    }
  }

  .animate-pulse {
    animation: pulse 2s cubic-bezier(0.4, 0, 0.6, 1) infinite;
  }
</style>
