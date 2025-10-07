# Windows Compatibility Guide

## Overview

Forbidden Library Native is designed to work seamlessly across Windows, macOS, and Linux. This guide focuses on Windows-specific setup, configuration, and troubleshooting.

## Prerequisites

### Required Software

1. **Windows 10/11** (64-bit)
2. **Microsoft C++ Build Tools**
   - Download from: https://visualstudio.microsoft.com/visual-cpp-build-tools/
   - Select "Desktop development with C++" workload
3. **WebView2 Runtime** (Required for Tauri)
   - Usually pre-installed on Windows 11
   - Download from: https://developer.microsoft.com/en-us/microsoft-edge/webview2/
4. **Rust** (automatically installed by setup script)
   - Download manually from: https://rustup.rs/
5. **Node.js 18+**
   - Download from: https://nodejs.org/

### Optional Tools

- **Git for Windows**: https://git-scm.com/download/win
- **Windows Terminal**: Recommended for better PowerShell experience

## Quick Setup

### Automated Setup (Recommended)

Run the automated setup script from PowerShell:

```powershell
# Clone the repository
git clone https://github.com/sorrowscry86/forbidden-library-native.git
cd forbidden-library-native

# Run Windows setup script
.\scripts\setup-windows.ps1
```

The script will:
- ✅ Check and install Rust if needed
- ✅ Verify Node.js installation
- ✅ Install pnpm package manager
- ✅ Check for WebView2 Runtime
- ✅ Verify C++ Build Tools
- ✅ Install project dependencies
- ✅ Configure Windows-specific settings

### Manual Setup

If you prefer manual setup:

```powershell
# Install Rust
winget install Rustlang.Rustup

# Install pnpm
npm install -g pnpm

# Install dependencies
pnpm install

# Run development server
pnpm run tauri dev
```

## Development

### Running the Application

```powershell
# Development mode with hot reload
pnpm run tauri dev

# Build for production
pnpm run tauri build
```

### Platform-Specific Paths

The application automatically handles Windows paths:

- **App Data**: `%APPDATA%\Forbidden Library`
- **Database**: `%APPDATA%\Forbidden Library\database.sqlite`
- **Logs**: `%APPDATA%\Forbidden Library\logs`

### Environment Variables

Create a `.env` file in the project root:

```env
# Optional: Custom app data directory
APPDATA=C:\Users\YourUsername\AppData\Roaming

# Optional: Enable debug logging
RUST_LOG=debug
```

## Windows-Specific Features

### File System Operations

The application uses native Windows file dialogs and paths:

```typescript
// Frontend automatically detects Windows
import { isPlatform } from '$lib/utils/platform';

if (isPlatform.windows()) {
  // Windows-specific behavior
  const path = getPlatformSpecificPath('Documents\\MyFile.txt');
}
```

### PowerShell Integration

Windows-specific features can use PowerShell commands:

```rust
// Backend example
use forbidden_library_native::platform::windows;

let result = windows::run_powershell_command("Get-Process");
```

### Keyboard Shortcuts

Windows uses standard shortcuts:
- **Ctrl+N**: New conversation
- **Ctrl+S**: Save
- **Ctrl+Q**: Quit
- **Alt+F4**: Close window

## Troubleshooting

### Common Issues

#### 1. WebView2 Not Found

**Error**: `WebView2 Runtime not installed`

**Solution**:
```powershell
# Download and install WebView2 Runtime
Start-Process "https://developer.microsoft.com/en-us/microsoft-edge/webview2/"
```

#### 2. Compilation Errors

**Error**: `link.exe not found` or `MSVC not found`

**Solution**:
```powershell
# Install Visual Studio Build Tools
winget install Microsoft.VisualStudio.2022.BuildTools

# Or download from:
# https://visualstudio.microsoft.com/visual-cpp-build-tools/
```

#### 3. Database Lock Errors

**Error**: `Database is locked`

**Solution**:
- Close other instances of the application
- Check Windows Task Manager for orphaned processes
- Delete lock file: `%APPDATA%\Forbidden Library\database.sqlite-wal`

#### 4. Permission Denied Errors

**Error**: `Access is denied`

**Solutions**:
```powershell
# Run PowerShell as Administrator
Start-Process powershell -Verb runAs

# Or adjust folder permissions
icacls "%APPDATA%\Forbidden Library" /grant Users:F /T
```

### Platform-Specific Error Messages

The application provides Windows-specific error suggestions:

```rust
// Example error with Windows-specific suggestion
if error.contains("Access is denied") {
    println!("Try running as administrator or check file permissions.");
}
```

## Building for Distribution

### Creating Windows Installer

```powershell
# Build MSI installer
pnpm run tauri build

# Output location
.\src-tauri\target\release\bundle\msi\
```

### Code Signing (Optional)

For production releases:

```powershell
# Set environment variables
$env:TAURI_PRIVATE_KEY = "your-private-key"
$env:TAURI_KEY_PASSWORD = "your-password"

# Build signed application
pnpm run tauri build
```

### Bundle Configuration

The Windows bundle is configured in `src-tauri/tauri.conf.json`:

```json
{
  "bundle": {
    "windows": {
      "webviewInstallMode": "downloadBootstrapper",
      "digestAlgorithm": "sha256"
    }
  }
}
```

## Performance Optimization

### Windows-Specific Optimizations

1. **Startup Time**:
   - Application targets sub-second launch on Windows
   - Uses lazy loading for heavy modules

2. **Memory Usage**:
   - Optimized for Windows memory management
   - Automatic cleanup of resources

3. **File I/O**:
   - Uses Windows native file APIs
   - Buffered I/O for better performance

### Monitoring Performance

```powershell
# Check application performance
pnpm run tauri dev

# Monitor in Task Manager
# Look for: Forbidden Library.exe
```

## Security Considerations

### Windows Defender

Add exception for development:

```powershell
# Add folder to Windows Defender exclusions
Add-MpPreference -ExclusionPath "C:\path\to\forbidden-library-native"
```

### File System Security

- Database is encrypted using SQLCipher
- Sensitive data never touches Windows page file
- Secure deletion of temporary files

## CI/CD on Windows

The project includes GitHub Actions workflows for Windows:

- **windows-compatibility.yml**: Windows-specific tests
- **ci.yml**: Cross-platform builds including Windows

### Local CI Testing

```powershell
# Run tests locally
cargo test --target x86_64-pc-windows-msvc

# Run frontend tests
pnpm run test
```

## Additional Resources

### Documentation
- [Tauri Windows Guide](https://tauri.app/v1/guides/building/windows)
- [Rust on Windows](https://www.rust-lang.org/tools/install)
- [pnpm Documentation](https://pnpm.io/)

### Support
- **Email**: SorrowsCry86@voidcat.org
- **CashApp**: $WykeveTF
- **GitHub Issues**: [Report a bug](https://github.com/sorrowscry86/forbidden-library-native/issues)

### VoidCat RDC
Excellence Protocol Active - Production-grade desktop application development.
