<script lang="ts">
  export let type: 'spinner' | 'skeleton' | 'pulse' | 'dots' | 'bars' = 'spinner';
  export let size: 'sm' | 'md' | 'lg' | 'xl' = 'md';
  export let text: string = '';
  export let fullScreen: boolean = false;

  const sizeClasses = {
    sm: 'w-4 h-4',
    md: 'w-8 h-8',
    lg: 'w-12 h-12',
    xl: 'w-16 h-16'
  };

  const textSizeClasses = {
    sm: 'text-xs',
    md: 'text-sm',
    lg: 'text-base',
    xl: 'text-lg'
  };
</script>

{#if fullScreen}
  <div class="fixed inset-0 bg-background/80 backdrop-blur-sm flex items-center justify-center z-50">
    <div class="flex flex-col items-center gap-4">
      <svelte:self {type} {size} {text} fullScreen={false} />
    </div>
  </div>
{:else}
  <div class="flex flex-col items-center justify-center gap-3" role="status" aria-live="polite">
    {#if type === 'spinner'}
      <div class="{sizeClasses[size]} animate-spin">
        <svg class="w-full h-full text-primary" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24">
          <circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4"></circle>
          <path class="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"></path>
        </svg>
      </div>
    {:else if type === 'dots'}
      <div class="flex gap-2">
        <div class="w-3 h-3 bg-primary rounded-full animate-bounce" style="animation-delay: 0ms"></div>
        <div class="w-3 h-3 bg-primary rounded-full animate-bounce" style="animation-delay: 150ms"></div>
        <div class="w-3 h-3 bg-primary rounded-full animate-bounce" style="animation-delay: 300ms"></div>
      </div>
    {:else if type === 'bars'}
      <div class="flex gap-1 items-end h-8">
        <div class="w-2 bg-primary rounded-sm animate-pulse" style="animation-delay: 0ms; height: 40%"></div>
        <div class="w-2 bg-primary rounded-sm animate-pulse" style="animation-delay: 150ms; height: 70%"></div>
        <div class="w-2 bg-primary rounded-sm animate-pulse" style="animation-delay: 300ms; height: 100%"></div>
        <div class="w-2 bg-primary rounded-sm animate-pulse" style="animation-delay: 450ms; height: 60%"></div>
      </div>
    {:else if type === 'pulse'}
      <div class="{sizeClasses[size]} bg-primary rounded-full animate-pulse"></div>
    {:else if type === 'skeleton'}
      <div class="w-full space-y-3">
        <div class="h-4 bg-muted rounded animate-pulse"></div>
        <div class="h-4 bg-muted rounded animate-pulse w-5/6"></div>
        <div class="h-4 bg-muted rounded animate-pulse w-4/6"></div>
      </div>
    {/if}

    {#if text}
      <p class="text-muted-foreground {textSizeClasses[size]} font-medium">{text}</p>
    {/if}
  </div>
{/if}

<style>
  @keyframes bounce {
    0%, 100% {
      transform: translateY(0);
    }
    50% {
      transform: translateY(-50%);
    }
  }

  .animate-bounce {
    animation: bounce 0.6s ease-in-out infinite;
  }

  @keyframes spin {
    from {
      transform: rotate(0deg);
    }
    to {
      transform: rotate(360deg);
    }
  }

  .animate-spin {
    animation: spin 1s linear infinite;
  }

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
