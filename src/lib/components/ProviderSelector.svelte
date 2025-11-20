<script lang="ts">
  /**
   * AI Provider Selector Component
   *
   * Allows users to select and configure AI providers for conversations
   */

  import { createEventDispatcher } from 'svelte';
  import { scale, fly } from '$lib/utils/animations';

  export let selectedProvider: string = '';
  export let apiKey: string = '';
  export let selectedModel: string = '';
  export let showConfig: boolean = false;

  const dispatch = createEventDispatcher();

  interface Provider {
    id: string;
    name: string;
    icon: string;
    description: string;
    requiresApiKey: boolean;
    models: string[];
    configFields?: Array<{
      id: string;
      label: string;
      type: 'text' | 'password' | 'select';
      placeholder?: string;
      required?: boolean;
    }>;
  }

  const providers: Provider[] = [
    {
      id: 'openai',
      name: 'OpenAI',
      icon: '🤖',
      description: 'GPT-4, GPT-3.5 Turbo, and more',
      requiresApiKey: true,
      models: [
        'gpt-4-turbo-preview',
        'gpt-4',
        'gpt-3.5-turbo',
        'gpt-3.5-turbo-16k'
      ],
      configFields: [
        { id: 'api_key', label: 'API Key', type: 'password', placeholder: 'sk-...', required: true },
        { id: 'organization', label: 'Organization ID (optional)', type: 'text', placeholder: 'org-...' }
      ]
    },
    {
      id: 'anthropic',
      name: 'Anthropic (Claude)',
      icon: '🎭',
      description: 'Claude 3.5 Sonnet, Opus, and Haiku',
      requiresApiKey: true,
      models: [
        'claude-3-5-sonnet-20241022',
        'claude-3-opus-20240229',
        'claude-3-sonnet-20240229',
        'claude-3-haiku-20240307'
      ],
      configFields: [
        { id: 'api_key', label: 'API Key', type: 'password', placeholder: 'sk-ant-...', required: true }
      ]
    },
    {
      id: 'google_gemini',
      name: 'Google Gemini',
      icon: '💎',
      description: 'Gemini 1.5 Pro and Flash',
      requiresApiKey: true,
      models: [
        'gemini-1.5-pro-latest',
        'gemini-1.5-flash-latest',
        'gemini-pro'
      ],
      configFields: [
        { id: 'api_key', label: 'API Key', type: 'password', placeholder: 'AIza...', required: true }
      ]
    },
    {
      id: 'azure_openai',
      name: 'Azure OpenAI',
      icon: '☁️',
      description: 'Azure-hosted OpenAI models',
      requiresApiKey: true,
      models: [],
      configFields: [
        { id: 'api_key', label: 'API Key', type: 'password', required: true },
        { id: 'endpoint', label: 'Endpoint', type: 'text', placeholder: 'https://your-resource.openai.azure.com', required: true },
        { id: 'deployment_name', label: 'Deployment Name', type: 'text', required: true },
        { id: 'api_version', label: 'API Version', type: 'text', placeholder: '2024-02-01', required: true }
      ]
    },
    {
      id: 'lmstudio',
      name: 'LM Studio',
      icon: '🖥️',
      description: 'Local models via LM Studio',
      requiresApiKey: false,
      models: [],
      configFields: [
        { id: 'base_url', label: 'Base URL', type: 'text', placeholder: 'http://localhost:1234/v1', required: true }
      ]
    },
    {
      id: 'ollama',
      name: 'Ollama',
      icon: '🦙',
      description: 'Local models via Ollama',
      requiresApiKey: false,
      models: ['llama3', 'mistral', 'codellama', 'phi', 'neural-chat'],
      configFields: [
        { id: 'base_url', label: 'Base URL', type: 'text', placeholder: 'http://localhost:11434', required: true }
      ]
    },
    {
      id: 'openai_compatible',
      name: 'OpenAI Compatible',
      icon: '🔌',
      description: 'Any OpenAI-compatible API',
      requiresApiKey: false,
      models: [],
      configFields: [
        { id: 'base_url', label: 'Base URL', type: 'text', placeholder: 'https://api.example.com/v1', required: true },
        { id: 'api_key', label: 'API Key (optional)', type: 'password' }
      ]
    }
  ];

  let formData: { [key: string]: string } = {};

  $: currentProvider = providers.find(p => p.id === selectedProvider);

  function selectProvider(providerId: string) {
    selectedProvider = providerId;
    showConfig = true;
    formData = {};
    dispatch('providerSelected', { providerId });
  }

  function selectModel(model: string) {
    selectedModel = model;
    dispatch('modelSelected', { model });
  }

  function saveConfiguration() {
    if (!currentProvider) return;

    // Validate required fields
    const requiredFields = currentProvider.configFields?.filter(f => f.required) || [];
    for (const field of requiredFields) {
      if (!formData[field.id]) {
        alert(`Please fill in the required field: ${field.label}`);
        return;
      }
    }

    dispatch('configSaved', {
      provider: selectedProvider,
      config: formData,
      model: selectedModel
    });
  }

  function cancelConfiguration() {
    showConfig = false;
    formData = {};
  }
</script>

<div class="provider-selector">
  {#if !showConfig}
    <div class="provider-grid" transition:scale={{ duration: 200 }}>
      <h3 class="provider-grid-title">Select AI Provider</h3>
      <div class="providers-list">
        {#each providers as provider, i}
          <button
            class="provider-card"
            class:selected={selectedProvider === provider.id}
            on:click={() => selectProvider(provider.id)}
            transition:fly={{ y: 20, delay: i * 50 }}
          >
            <div class="provider-icon">{provider.icon}</div>
            <div class="provider-info">
              <h4 class="provider-name">{provider.name}</h4>
              <p class="provider-description">{provider.description}</p>
            </div>
            {#if provider.requiresApiKey}
              <span class="provider-badge">API Key Required</span>
            {:else}
              <span class="provider-badge free">Free / Local</span>
            {/if}
          </button>
        {/each}
      </div>
    </div>
  {:else if currentProvider}
    <div class="provider-config" transition:scale={{ duration: 200 }}>
      <div class="config-header">
        <button class="back-button" on:click={cancelConfiguration} aria-label="Go back">
          <svg xmlns="http://www.w3.org/2000/svg" width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
            <path d="M19 12H5M12 19l-7-7 7-7"/>
          </svg>
        </button>
        <div class="config-title">
          <span class="config-icon">{currentProvider.icon}</span>
          <h3>Configure {currentProvider.name}</h3>
        </div>
      </div>

      <div class="config-form">
        {#if currentProvider.configFields}
          {#each currentProvider.configFields as field}
            <div class="form-group">
              <label for={field.id}>
                {field.label}
                {#if field.required}
                  <span class="required">*</span>
                {/if}
              </label>
              <input
                id={field.id}
                type={field.type}
                bind:value={formData[field.id]}
                placeholder={field.placeholder || ''}
                required={field.required}
                class="form-input"
              />
            </div>
          {/each}
        {/if}

        {#if currentProvider.models.length > 0}
          <div class="form-group">
            <label for="model">Model</label>
            <select id="model" bind:value={selectedModel} class="form-select">
              <option value="">Select a model...</option>
              {#each currentProvider.models as model}
                <option value={model}>{model}</option>
              {/each}
            </select>
          </div>
        {/if}
      </div>

      <div class="config-actions">
        <button class="btn btn-secondary" on:click={cancelConfiguration}>Cancel</button>
        <button class="btn btn-primary" on:click={saveConfiguration}>Save Configuration</button>
      </div>
    </div>
  {/if}
</div>

<style>
  .provider-selector {
    width: 100%;
    max-width: 800px;
    margin: 0 auto;
  }

  .provider-grid-title {
    font-size: 1.5rem;
    font-weight: 600;
    margin-bottom: 1.5rem;
    color: var(--text-primary, #111827);
  }

  .providers-list {
    display: grid;
    gap: 1rem;
    grid-template-columns: repeat(auto-fill, minmax(300px, 1fr));
  }

  .provider-card {
    position: relative;
    display: flex;
    flex-direction: column;
    gap: 0.75rem;
    padding: 1.5rem;
    background: var(--card-bg, white);
    border: 2px solid var(--border-color, #e5e7eb);
    border-radius: 0.75rem;
    cursor: pointer;
    transition: all 0.2s ease;
    text-align: left;
  }

  .provider-card:hover {
    border-color: var(--primary-color, #3b82f6);
    box-shadow: 0 4px 12px rgba(59, 130, 246, 0.1);
    transform: translateY(-2px);
  }

  .provider-card.selected {
    border-color: var(--primary-color, #3b82f6);
    background: var(--primary-bg, #eff6ff);
  }

  .provider-icon {
    font-size: 2rem;
  }

  .provider-info {
    flex: 1;
  }

  .provider-name {
    font-size: 1.125rem;
    font-weight: 600;
    margin-bottom: 0.25rem;
    color: var(--text-primary, #111827);
  }

  .provider-description {
    font-size: 0.875rem;
    color: var(--text-secondary, #6b7280);
    margin: 0;
  }

  .provider-badge {
    display: inline-block;
    padding: 0.25rem 0.75rem;
    font-size: 0.75rem;
    font-weight: 600;
    border-radius: 9999px;
    background: var(--badge-bg, #fef3c7);
    color: var(--badge-text, #92400e);
  }

  .provider-badge.free {
    background: var(--success-bg, #d1fae5);
    color: var(--success-text, #065f46);
  }

  .provider-config {
    background: var(--card-bg, white);
    border: 1px solid var(--border-color, #e5e7eb);
    border-radius: 0.75rem;
    padding: 1.5rem;
  }

  .config-header {
    display: flex;
    align-items: center;
    gap: 1rem;
    margin-bottom: 1.5rem;
  }

  .back-button {
    background: transparent;
    border: none;
    color: var(--text-secondary, #6b7280);
    cursor: pointer;
    padding: 0.5rem;
    border-radius: 0.375rem;
    transition: all 0.2s ease;
  }

  .back-button:hover {
    background: var(--hover-bg, #f3f4f6);
    color: var(--text-primary, #111827);
  }

  .config-title {
    display: flex;
    align-items: center;
    gap: 0.75rem;
  }

  .config-icon {
    font-size: 1.5rem;
  }

  .config-title h3 {
    font-size: 1.25rem;
    font-weight: 600;
    margin: 0;
    color: var(--text-primary, #111827);
  }

  .config-form {
    display: flex;
    flex-direction: column;
    gap: 1.25rem;
    margin-bottom: 1.5rem;
  }

  .form-group {
    display: flex;
    flex-direction: column;
    gap: 0.5rem;
  }

  .form-group label {
    font-size: 0.875rem;
    font-weight: 600;
    color: var(--text-primary, #111827);
  }

  .required {
    color: var(--danger-color, #ef4444);
  }

  .form-input,
  .form-select {
    padding: 0.75rem;
    border: 1px solid var(--border-color, #e5e7eb);
    border-radius: 0.5rem;
    font-size: 0.875rem;
    transition: all 0.2s ease;
    background: var(--input-bg, white);
    color: var(--text-primary, #111827);
  }

  .form-input:focus,
  .form-select:focus {
    outline: none;
    border-color: var(--primary-color, #3b82f6);
    box-shadow: 0 0 0 3px rgba(59, 130, 246, 0.1);
  }

  .config-actions {
    display: flex;
    gap: 1rem;
    justify-content: flex-end;
  }

  .btn {
    padding: 0.75rem 1.5rem;
    border: none;
    border-radius: 0.5rem;
    font-weight: 600;
    font-size: 0.875rem;
    cursor: pointer;
    transition: all 0.2s ease;
  }

  .btn-primary {
    background: var(--primary-color, #3b82f6);
    color: white;
  }

  .btn-primary:hover {
    background: var(--primary-hover, #2563eb);
    transform: translateY(-1px);
    box-shadow: 0 4px 12px rgba(59, 130, 246, 0.3);
  }

  .btn-secondary {
    background: var(--secondary-bg, #f3f4f6);
    color: var(--text-primary, #111827);
  }

  .btn-secondary:hover {
    background: var(--secondary-hover, #e5e7eb);
  }
</style>
