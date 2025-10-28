# Quick Start Guide - Windows Users

## 🚀 Get Started in 5 Minutes

This guide will help you get Forbidden Library Native up and running on Windows quickly.

## Prerequisites Check

Before you start, ensure you have:

- [ ] **Windows 10 or 11** (64-bit)
- [ ] **Administrator access** (for initial setup)
- [ ] **Internet connection** (for downloading dependencies)

## Step 1: Automated Setup

Open **PowerShell** and run:

```powershell
# Clone the repository
git clone https://github.com/sorrowscry86/forbidden-library-native.git
cd forbidden-library-native

# Run the automated setup script
.\scripts\setup-windows.ps1
```

The script will automatically:
- ✅ Install Rust (if needed)
- ✅ Verify Node.js
- ✅ Install pnpm
- ✅ Check WebView2 Runtime
- ✅ Install project dependencies
- ✅ Configure Windows settings

## Step 2: Start Development

```powershell
# Start the application in development mode
pnpm run tauri dev
```

That's it! The application should open automatically.

## Step 3: Choose Your AI Provider (Optional)

### Option A: LM Studio (Local, Privacy-First)

1. **Download LM Studio**: https://lmstudio.ai/
2. **Download a model** (e.g., Mistral 7B)
3. **Start the server** in LM Studio (Developer → Start Server)
4. **Configure in Forbidden Library**:
   - Provider: LM Studio
   - Port: 1234
   - No API key needed

### Option B: Ollama (Local, Command-Line)

```powershell
# Install Ollama
winget install Ollama.Ollama

# Pull a model
ollama pull mistral

# Ollama runs automatically
```

Configure in Forbidden Library:
- Provider: Ollama
- Port: 11434
- No API key needed

### Option C: OpenAI (Cloud)

1. **Get API key** from https://platform.openai.com/
2. **Configure in Forbidden Library**:
   - Provider: OpenAI
   - API Key: your-key-here
   - Model: gpt-3.5-turbo or gpt-4

## Troubleshooting

### Issue: "WebView2 Not Found"

**Solution**:
```powershell
# Download and install WebView2
Start-Process "https://developer.microsoft.com/en-us/microsoft-edge/webview2/"
```

### Issue: "Build Tools Not Found"

**Solution**:
```powershell
# Install Visual Studio Build Tools
winget install Microsoft.VisualStudio.2022.BuildTools
```

Select "Desktop development with C++" during installation.

### Issue: "Permission Denied"

**Solution**:
```powershell
# Run PowerShell as Administrator
Start-Process powershell -Verb runAs
```

### Issue: "Port Already in Use"

If port 1234 (LM Studio) or 11434 (Ollama) is already in use:

**LM Studio**:
- Change port in LM Studio settings
- Update in Forbidden Library settings

**Ollama**:
```powershell
# Stop Ollama service
Stop-Service Ollama

# Start on different port (not supported, use default)
# Or: Kill process using port 11434
```

## Build for Production

```powershell
# Build the application
pnpm run tauri build

# Find the installer at:
# src-tauri\target\release\bundle\msi\
```

## Next Steps

- 📖 Read [Windows Compatibility Guide](./docs/WINDOWS_COMPATIBILITY.md)
- 🤖 Learn about [AI Providers](./docs/AI_PROVIDERS.md)
- 🛠️ Check out [Contributing Guidelines](./CONTRIBUTING.md)

## Getting Help

- **Email**: SorrowsCry86@voidcat.org
- **CashApp**: $WykeveTF
- **GitHub Issues**: [Report a bug](https://github.com/sorrowscry86/forbidden-library-native/issues)

---

**VoidCat RDC - Excellence Protocol Active**

Enjoy your privacy-first AI conversation manager!
