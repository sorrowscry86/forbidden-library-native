# AI Provider Integration Guide

## Overview

Forbidden Library Native supports multiple AI providers, allowing you to choose between cloud services and local models. This gives you flexibility in terms of privacy, cost, and performance.

## Supported Providers

### 1. LM Studio (Local Models)

**Description**: Run models locally on your machine using LM Studio.

**Advantages**:
- ✅ Complete privacy (no data leaves your machine)
- ✅ No API costs
- ✅ Works offline
- ✅ Full control over model selection

**Setup**:
1. Download LM Studio from: https://lmstudio.ai/
2. Download your preferred model (e.g., Mistral, LLaMA, etc.)
3. Start the local server in LM Studio
4. Configure Forbidden Library to use LM Studio

**Default Configuration**:
- **Base URL**: `http://localhost:1234`
- **Port**: `1234` (configurable)
- **API Format**: OpenAI-compatible

### 2. Ollama (Local Models)

**Description**: Run open-source models locally using Ollama.

**Advantages**:
- ✅ Complete privacy
- ✅ No API costs
- ✅ Easy model management
- ✅ Command-line interface

**Setup**:
1. Install Ollama from: https://ollama.ai/
2. Pull a model: `ollama pull mistral`
3. Start Ollama service
4. Configure Forbidden Library to use Ollama

**Default Configuration**:
- **Base URL**: `http://localhost:11434`
- **Port**: `11434` (configurable)
- **API Format**: Ollama-specific

### 3. OpenAI-Compatible Endpoints

**Description**: Use any OpenAI API-compatible service (OpenAI, Azure OpenAI, custom endpoints).

**Advantages**:
- ✅ Cloud-based (no local resources needed)
- ✅ Access to latest models
- ✅ Scalable performance

**Setup**:
1. Get API key from your provider
2. Configure the base URL and API key in Forbidden Library

**Supported Providers**:
- OpenAI (GPT-3.5, GPT-4)
- Azure OpenAI
- OpenRouter
- Any OpenAI-compatible API

## Configuration

### Using the Frontend

```typescript
// Check if a provider is available
import { invoke } from '@tauri-apps/api/tauri';

const isAvailable = await invoke('check_ai_provider_availability', {
  provider_type: 'lm_studio',
  port: 1234
});

// List available models
const models = await invoke('list_ai_provider_models', {
  provider_type: 'ollama',
  port: 11434
});

// Send a request
const response = await invoke('send_ai_provider_request', {
  provider_type: 'lm_studio',
  model: 'mistral-7b',
  messages: [
    { role: 'user', content: 'Hello, how are you?' }
  ],
  port: 1234,
  temperature: 0.7,
  max_tokens: 500
});
```

### Using the Backend

```rust
use forbidden_library_native::ai_providers::{AIProvider, AIRequest, ChatMessage};

// Create a provider
let provider = AIProvider::lm_studio(Some(1234));

// Check availability
let available = provider.check_availability().await?;

// List models
let models = provider.list_models().await?;

// Send a request
let request = AIRequest {
    model: "mistral-7b".to_string(),
    messages: vec![
        ChatMessage {
            role: "user".to_string(),
            content: "Hello!".to_string(),
        }
    ],
    temperature: Some(0.7),
    max_tokens: Some(500),
    stream: false,
};

let response = provider.send_request(request).await?;
```

## Provider-Specific Configuration

### LM Studio

**Installation**:
```powershell
# Windows
winget install lmstudio

# Or download from https://lmstudio.ai/
```

**Configuration**:
```typescript
const lmStudioConfig = {
  provider_type: 'lm_studio',
  base_url: 'http://localhost:1234',
  port: 1234,
  // No API key needed for local server
};
```

**Recommended Models**:
- **Mistral 7B**: Fast, efficient, good quality
- **LLaMA 2 13B**: Better quality, more resources
- **Phi-2**: Lightweight, fast responses

### Ollama

**Installation**:
```powershell
# Windows (using winget)
winget install Ollama.Ollama

# Or download from https://ollama.ai/
```

**Model Management**:
```bash
# Pull a model
ollama pull mistral

# List installed models
ollama list

# Remove a model
ollama rm mistral
```

**Configuration**:
```typescript
const ollamaConfig = {
  provider_type: 'ollama',
  base_url: 'http://localhost:11434',
  port: 11434,
};
```

**Recommended Models**:
- **mistral**: General purpose, balanced
- **llama2**: Meta's LLaMA 2 model
- **codellama**: Specialized for code
- **neural-chat**: Conversational AI

### OpenAI / Custom Endpoints

**Configuration**:
```typescript
const openaiConfig = {
  provider_type: 'openai_compatible',
  base_url: 'https://api.openai.com',
  api_key: 'sk-...',  // Your API key
};
```

**Custom Endpoint Example**:
```typescript
const customConfig = {
  provider_type: 'openai_compatible',
  base_url: 'https://your-custom-endpoint.com',
  api_key: 'your-api-key',
};
```

## API Reference

### Commands

#### `check_ai_provider_availability`

Check if an AI provider is available and responding.

**Parameters**:
- `provider_type`: `'lm_studio' | 'ollama' | 'openai_compatible'`
- `base_url?`: Custom base URL (optional)
- `port?`: Custom port (optional)

**Returns**: `boolean`

#### `list_ai_provider_models`

List all available models from the provider.

**Parameters**:
- `provider_type`: `'lm_studio' | 'ollama' | 'openai_compatible'`
- `base_url?`: Custom base URL (optional)
- `port?`: Custom port (optional)

**Returns**: `string[]` - Array of model names

#### `send_ai_provider_request`

Send a chat completion request to the provider.

**Parameters**:
- `provider_type`: Provider type
- `model`: Model name
- `messages`: Array of chat messages
- `base_url?`: Custom base URL (optional)
- `port?`: Custom port (optional)
- `api_key?`: API key (for OpenAI-compatible)
- `temperature?`: Sampling temperature (0-1)
- `max_tokens?`: Maximum tokens in response

**Returns**: Object with:
- `content`: Response text
- `model`: Model used
- `tokens_used?`: Token count (if available)

## Best Practices

### Privacy & Security

1. **Local vs Cloud**:
   - Use local models (LM Studio, Ollama) for sensitive data
   - Use cloud APIs for non-sensitive, high-volume tasks

2. **API Keys**:
   - Never commit API keys to version control
   - Store in environment variables or secure storage
   - Use `.env.local` for development

3. **Data Handling**:
   - Local models: Data never leaves your machine
   - Cloud APIs: Review provider's data policy

### Performance

1. **Model Selection**:
   - **Fast responses**: Use smaller models (7B parameters)
   - **Better quality**: Use larger models (13B+ parameters)
   - **Specialized tasks**: Use task-specific models

2. **Resource Management**:
   - Local models require GPU/RAM
   - Monitor system resources
   - Close unused model servers

3. **Caching**:
   - Cache common responses
   - Reuse conversations for context

### Error Handling

```typescript
try {
  const response = await invoke('send_ai_provider_request', {
    provider_type: 'lm_studio',
    model: 'mistral-7b',
    messages: [{ role: 'user', content: 'Hello' }],
    port: 1234
  });
  console.log(response.content);
} catch (error) {
  if (error.includes('connection refused')) {
    console.error('Provider not running. Please start LM Studio.');
  } else {
    console.error('AI request failed:', error);
  }
}
```

## Troubleshooting

### LM Studio Issues

**Problem**: Connection refused
```powershell
# Check if LM Studio server is running
curl http://localhost:1234/v1/models

# Start the server in LM Studio
# Go to: Developer > Start Server
```

**Problem**: Model not loading
- Ensure model is downloaded in LM Studio
- Check available disk space
- Verify system resources (RAM/VRAM)

### Ollama Issues

**Problem**: Service not running
```powershell
# Check Ollama status
ollama list

# Start Ollama service (Windows)
Start-Service Ollama
```

**Problem**: Model not found
```bash
# Pull the model
ollama pull mistral

# Verify installation
ollama list
```

### OpenAI Issues

**Problem**: Invalid API key
- Verify key is correct
- Check API key permissions
- Ensure billing is set up (for OpenAI)

**Problem**: Rate limit exceeded
- Reduce request frequency
- Upgrade API plan
- Implement request queuing

## Platform-Specific Notes

### Windows

- Local models work best with NVIDIA GPUs
- Ensure WebView2 is installed for UI
- Use PowerShell for service management

### macOS

- Apple Silicon (M1/M2) provides excellent performance
- Use Metal acceleration for models
- LM Studio has native Apple Silicon support

### Linux

- CUDA support for NVIDIA GPUs
- ROCm support for AMD GPUs
- systemd for service management

## Examples

### Complete Workflow

```typescript
// 1. Check provider availability
const providers = ['lm_studio', 'ollama', 'openai_compatible'];
const available = [];

for (const provider of providers) {
  const isAvailable = await invoke('check_ai_provider_availability', {
    provider_type: provider
  });
  if (isAvailable) available.push(provider);
}

// 2. Select first available provider
const selectedProvider = available[0];

// 3. List models
const models = await invoke('list_ai_provider_models', {
  provider_type: selectedProvider
});

// 4. Send request with first model
const response = await invoke('send_ai_provider_request', {
  provider_type: selectedProvider,
  model: models[0],
  messages: [
    { role: 'system', content: 'You are a helpful assistant.' },
    { role: 'user', content: 'Explain quantum computing in simple terms.' }
  ],
  temperature: 0.7,
  max_tokens: 500
});

console.log('Response:', response.content);
console.log('Tokens used:', response.tokens_used);
```

## Resources

### Documentation
- [LM Studio Docs](https://lmstudio.ai/docs)
- [Ollama Documentation](https://github.com/ollama/ollama)
- [OpenAI API Reference](https://platform.openai.com/docs/api-reference)

### Models
- [Hugging Face](https://huggingface.co/models)
- [Ollama Model Library](https://ollama.ai/library)
- [LM Studio Model Hub](https://lmstudio.ai/models)

### Support
- **VoidCat RDC**: SorrowsCry86@voidcat.org
- **CashApp**: $WykeveTF
- **GitHub**: [Report Issues](https://github.com/sorrowscry86/forbidden-library-native/issues)
