<script>
import '../app.css';
import { onMount } from 'svelte';
import { invokeWithTimeout, ms } from '/services/api';
import { page } from '/stores';

let appInfo = { name: '', version: '' };

onMount(async () => {
    try {
        appInfo = await invokeWithTimeout('get_app_version', undefined, ms(5));
    } catch (error) {
        console.error('Failed to get app version:', error);
    }
});
</script>

<div class="h-screen flex flex-col bg-gray-900 text-gray-100">
    <!-- Header -->
    <header class="bg-gray-800 border-b border-gray-700 px-6 py-4">
        <div class="flex items-center justify-between">
            <div class="flex items-center space-x-4">
                <div class="w-8 h-8 bg-purple-600 rounded-lg flex items-center justify-center">
                    <span class="text-white font-bold text-lg">F</span>
                </div>
                <div>
                    <h1 class="text-xl font-bold text-white">Forbidden Library</h1>
                    <p class="text-sm text-gray-400">v{appInfo.version}</p>
                </div>
            </div>

            <div class="flex items-center space-x-4">
                <nav class="flex items-center space-x-6 mr-6">
                    <a 
                        href="/" 
                        class="text-sm font-medium {.url.pathname === '/' ? 'text-white' : 'text-gray-400 hover:text-white'} transition-colors"
                    >
                        Conversations
                    </a>
                    <a 
                        href="/planning" 
                        class="text-sm font-medium {.url.pathname === '/planning' ? 'text-white' : 'text-gray-400 hover:text-white'} transition-colors"
                    >
                        Planning
                    </a>
                    <a 
                        href="/settings" 
                        class="text-sm font-medium {.url.pathname === '/settings' ? 'text-white' : 'text-gray-400 hover:text-white'} transition-colors"
                    >
                        Settings
                    </a>
                </nav>
                
                <button class="p-2 text-gray-400 hover:text-white transition-colors">
                    <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M10.325 4.317c.426-1.756 2.924-1.756 3.35 0a1.724 1.724 0 002.573 1.066c1.543-.94 3.31.826 2.37 2.37a1.724 1.724 0 001.065 2.572c1.756.426 1.756 2.924 0 3.35a1.724 1.724 0 00-1.066 2.573c.94 1.543-.826 3.31-2.37 2.37a1.724 1.724 0 00-2.572 1.065c-.426 1.756-2.924 1.756-3.35 0a1.724 1.724 0 00-2.573-1.066c-1.543.94-3.31-.826-2.37-2.37a1.724 1.724 0 00-1.065-2.572c-1.756-.426-1.756-2.924 0-3.35a1.724 1.724 0 001.066-2.573c-.94-1.543.826-3.31 2.37-2.37.996.608 2.296.07 2.572-1.065z"></path>
                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 12a3 3 0 11-6 0 3 3 0 016 0z"></path>
                    </svg>
                </button>
            </div>
        </div>
    </header>

    <!-- Main Content -->
    <main class="flex-1 overflow-hidden">
        <slot />
    </main>
</div>

