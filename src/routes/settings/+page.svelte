<script lang="ts">
    import { onMount } from 'svelte';
    import { invokeWithTimeout, ms } from '$lib/services/api';
    import { getEnvironment } from '$lib/utils/tauri-detection';
    
    let loading = true;
    let environment = getEnvironment();
    let apiKey = '';
    let selectedProvider = 'openai';
    let providers = [
        { id: 'openai', name: 'OpenAI' },
        { id: 'anthropic', name: 'Anthropic' },
        { id: 'google', name: 'Google AI' },
        { id: 'local', name: 'Local Model' }
    ];
    
    onMount(async () => {
        await loadSettings();
    });
    
    async function loadSettings() {
        try {
            loading = true;
            
            if (environment === 'tauri') {
                const config = await invokeWithTimeout('get_api_config', { provider: selectedProvider }, ms(8));
                if (config) {
                    apiKey = config[0] || '';
                }
            }
        } catch (error) {
            console.error('Failed to load settings:', error);
        } finally {
            loading = false;
        }
    }
    
    async function saveSettings() {
        try {
            if (environment === 'tauri') {
                await invokeWithTimeout('store_api_config', {
                    provider: selectedProvider,
                    api_key: apiKey,
                    base_url: null
                }, ms(8));
                
                alert('Settings saved successfully');
            } else {
                alert('Settings cannot be saved in web mode');
            }
        } catch (error) {
            console.error('Failed to save settings:', error);
            alert('Error saving settings: ' + error);
        }
    }
    
    function handleProviderChange() {
        loadSettings();
    }
</script>

<div class="container mx-auto px-6 py-8 max-w-4xl">
    <h1 class="text-2xl font-bold text-white mb-6">Settings</h1>
    
    {#if loading}
        <div class="flex justify-center py-8">
            <div class="animate-spin rounded-full h-8 w-8 border-b-2 border-purple-600"></div>
        </div>
    {:else}
        <div class="bg-gray-800 rounded-lg p-6 mb-8">
            <h2 class="text-xl font-semibold text-white mb-4">AI Provider Configuration</h2>
            
            <div class="mb-6">
                <label for="provider" class="block text-sm font-medium text-gray-400 mb-2">
                    Select Provider
                </label>
                <select
                    id="provider"
                    bind:value={selectedProvider}
                    on:change={handleProviderChange}
                    class="w-full bg-gray-700 text-white border border-gray-600 rounded-lg px-4 py-2 focus:outline-none focus:ring-2 focus:ring-purple-600 focus:border-transparent"
                >
                    {#each providers as provider}
                        <option value={provider.id}>{provider.name}</option>
                    {/each}
                </select>
            </div>
            
            <div class="mb-6">
                <label for="apiKey" class="block text-sm font-medium text-gray-400 mb-2">
                    API Key
                </label>
                <input
                    id="apiKey"
                    type="password"
                    bind:value={apiKey}
                    placeholder="Enter your API key"
                    class="w-full bg-gray-700 text-white placeholder-gray-500 border border-gray-600 rounded-lg px-4 py-2 focus:outline-none focus:ring-2 focus:ring-purple-600 focus:border-transparent"
                />
                <p class="mt-2 text-sm text-gray-500">
                    Your API key is stored securely and never shared with third parties.
                </p>
            </div>
            
            <div class="flex justify-end">
                <button
                    on:click={saveSettings}
                    class="bg-purple-600 hover:bg-purple-700 text-white px-4 py-2 rounded-lg transition-colors"
                >
                    Save Settings
                </button>
            </div>
            
            {#if environment === 'web'}
                <div class="mt-4 bg-gray-700 rounded-lg p-3 text-center">
                    <p class="text-sm text-gray-300">
                        ⚠️ Web mode: Settings cannot be saved. Install the desktop app for full functionality.
                    </p>
                </div>
            {/if}
        </div>
        
        <div class="bg-gray-800 rounded-lg p-6">
            <h2 class="text-xl font-semibold text-white mb-4">Application Information</h2>
            
            <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
                <div>
                    <h3 class="text-sm font-medium text-gray-400 mb-2">Environment</h3>
                    <p class="text-white">{environment === 'tauri' ? 'Desktop Application' : 'Web Browser'}</p>
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
    {/if}
</div>
