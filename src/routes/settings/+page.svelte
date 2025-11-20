<script lang="ts">
    import { onMount } from 'svelte';
    import { invokeWithTimeout, ms } from '$lib/services/api';
    import { getEnvironment } from '$lib/utils/tauri-detection';
    import ProviderSelector from '$lib/components/ProviderSelector.svelte';
    import LoadingStates from '$lib/components/LoadingStates.svelte';

    let loading = true;
    let environment = getEnvironment();
    let selectedProvider = 'openai';
    let selectedModel = '';
    let showConfig = false;
    let saveSuccess = false;
    let saveError: string | null = null;

    onMount(async () => {
        await loadSettings();
    });

    async function loadSettings() {
        try {
            loading = true;

            if (environment === 'tauri') {
                // Load saved provider configuration
                const config = await invokeWithTimeout('get_api_config', { provider: selectedProvider }, ms(8));
                if (config && config.length > 0) {
                    // Configuration loaded
                    console.log('Loaded config:', config);
                }
            }
        } catch (error) {
            console.error('Failed to load settings:', error);
        } finally {
            loading = false;
        }
    }

    async function handleSaveConfiguration(event: CustomEvent<{ provider: string; config: any; model?: string }>) {
        const { provider, config, model } = event.detail;

        try {
            saveSuccess = false;
            saveError = null;

            if (environment === 'tauri') {
                await invokeWithTimeout('store_api_config', {
                    provider,
                    api_key: config.api_key || '',
                    base_url: config.base_url || config.endpoint || null
                }, ms(8));

                saveSuccess = true;
                setTimeout(() => saveSuccess = false, 3000);
            } else {
                saveError = 'Settings cannot be saved in web mode';
            }
        } catch (error) {
            console.error('Failed to save settings:', error);
            saveError = 'Error saving settings: ' + error;
        }
    }

    function handleProviderSelected(event: CustomEvent<{ providerId: string }>) {
        selectedProvider = event.detail.providerId;
        loadSettings();
    }

    function handleModelSelected(event: CustomEvent<{ model: string }>) {
        selectedModel = event.detail.model;
    }
</script>

<div class="container mx-auto px-6 py-8 max-w-4xl">
    <div class="flex items-center space-x-4 mb-6">
        <a
            href="/"
            class="p-2 text-gray-400 hover:text-white transition-colors rounded-lg hover:bg-gray-700"
            title="Back to Conversations"
        >
            <svg class="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M10 19l-7-7m0 0l7-7m-7 7h18"></path>
            </svg>
        </a>
        <h1 class="text-2xl font-bold text-white">Settings</h1>
    </div>
    
    {#if loading}
        <div class="flex justify-center py-8">
            <LoadingStates variant="spinner" size="large" message="Loading settings..." />
        </div>
    {:else}
        <!-- AI Provider Configuration -->
        <div class="bg-gray-800 rounded-lg p-6 mb-8">
            <div class="flex items-center justify-between mb-6">
                <div>
                    <h2 class="text-xl font-semibold text-white">AI Provider Configuration</h2>
                    <p class="text-sm text-gray-400 mt-1">Choose and configure your AI provider</p>
                </div>
                {#if saveSuccess}
                    <div class="flex items-center space-x-2 text-green-400 animate-fade-in">
                        <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 13l4 4L19 7"></path>
                        </svg>
                        <span class="text-sm font-medium">Saved!</span>
                    </div>
                {/if}
            </div>

            <ProviderSelector
                bind:selectedProvider
                bind:selectedModel
                bind:showConfig
                on:providerSelected={handleProviderSelected}
                on:modelSelected={handleModelSelected}
                on:saveConfiguration={handleSaveConfiguration}
            />

            {#if saveError}
                <div class="mt-4 bg-red-900/20 border border-red-700 rounded-lg p-3 text-center">
                    <p class="text-sm text-red-400">⚠️ {saveError}</p>
                </div>
            {/if}

            {#if environment === 'web'}
                <div class="mt-4 bg-yellow-900/20 border border-yellow-700 rounded-lg p-3 text-center">
                    <p class="text-sm text-yellow-400">
                        🌐 Web mode: Settings cannot be saved. Install the desktop app for full functionality.
                    </p>
                </div>
            {/if}
        </div>

        <!-- Application Information -->
        <div class="bg-gray-800 rounded-lg p-6 mb-8">
            <h2 class="text-xl font-semibold text-white mb-4">Application Information</h2>

            <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
                <div>
                    <h3 class="text-sm font-medium text-gray-400 mb-2">Environment</h3>
                    <p class="text-white flex items-center">
                        {environment === 'tauri' ? '🖥️ Desktop Application' : '🌐 Web Browser'}
                    </p>
                </div>

                <div>
                    <h3 class="text-sm font-medium text-gray-400 mb-2">Version</h3>
                    <p class="text-white">2.0.0</p>
                </div>

                <div>
                    <h3 class="text-sm font-medium text-gray-400 mb-2">Developer</h3>
                    <p class="text-white">VoidCat RDC</p>
                </div>

                <div>
                    <h3 class="text-sm font-medium text-gray-400 mb-2">Contact</h3>
                    <p class="text-white">SorrowsCry86@voidcat.org</p>
                </div>
            </div>
        </div>

        <!-- Keyboard Shortcuts Reference -->
        <div class="bg-gray-800 rounded-lg p-6">
            <h2 class="text-xl font-semibold text-white mb-4">Keyboard Shortcuts</h2>

            <div class="grid grid-cols-1 md:grid-cols-2 gap-3 text-sm">
                <div class="flex items-center justify-between p-2 bg-gray-700 rounded">
                    <span class="text-gray-300">New Conversation</span>
                    <kbd class="px-2 py-1 bg-gray-800 rounded text-gray-400">Ctrl+N</kbd>
                </div>

                <div class="flex items-center justify-between p-2 bg-gray-700 rounded">
                    <span class="text-gray-300">Search</span>
                    <kbd class="px-2 py-1 bg-gray-800 rounded text-gray-400">Ctrl+K</kbd>
                </div>

                <div class="flex items-center justify-between p-2 bg-gray-700 rounded">
                    <span class="text-gray-300">Settings</span>
                    <kbd class="px-2 py-1 bg-gray-800 rounded text-gray-400">Ctrl+,</kbd>
                </div>

                <div class="flex items-center justify-between p-2 bg-gray-700 rounded">
                    <span class="text-gray-300">Close Panel</span>
                    <kbd class="px-2 py-1 bg-gray-800 rounded text-gray-400">Esc</kbd>
                </div>
            </div>
        </div>
    {/if}
</div>
