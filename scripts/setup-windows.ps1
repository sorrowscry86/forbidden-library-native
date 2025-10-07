# Forbidden Library Native - Windows Setup Script
# PowerShell script to set up the development environment on Windows

$ErrorActionPreference = "Stop"

# Colors for output
$Red = "`e[31m"
$Green = "`e[32m"
$Yellow = "`e[33m"
$Blue = "`e[34m"
$Reset = "`e[0m"

function Write-ColorOutput {
    param([string]$Message, [string]$Color = $Reset)
    Write-Host "$Color$Message$Reset"
}

Write-ColorOutput "`n🚀 Forbidden Library Native - Windows Setup`n" $Blue
Write-ColorOutput "Setting up development environment for Windows...`n" $Yellow

# Check for Rust installation
Write-ColorOutput "Checking Rust installation..." $Blue
if (-not (Get-Command rustc -ErrorAction SilentlyContinue)) {
    Write-ColorOutput "❌ Rust is not installed." $Red
    Write-ColorOutput "Installing Rust..." $Yellow
    
    try {
        Invoke-WebRequest -Uri https://win.rustup.rs/x86_64 -OutFile rustup-init.exe
        .\rustup-init.exe -y
        Remove-Item rustup-init.exe
        $env:Path = "$env:USERPROFILE\.cargo\bin;$env:Path"
        Write-ColorOutput "✅ Rust installed successfully!" $Green
    } catch {
        Write-ColorOutput "❌ Failed to install Rust: $($_.Exception.Message)" $Red
        exit 1
    }
} else {
    $rustVersion = rustc --version
    Write-ColorOutput "✅ Rust is already installed: $rustVersion" $Green
}

# Check for Node.js
Write-ColorOutput "`nChecking Node.js installation..." $Blue
if (-not (Get-Command node -ErrorAction SilentlyContinue)) {
    Write-ColorOutput "❌ Node.js is not installed." $Red
    Write-ColorOutput "Please install Node.js from https://nodejs.org/" $Yellow
    Write-ColorOutput "Recommended: Node.js 18.x or higher" $Yellow
    exit 1
} else {
    $nodeVersion = node --version
    Write-ColorOutput "✅ Node.js is installed: $nodeVersion" $Green
}

# Check for pnpm
Write-ColorOutput "`nChecking pnpm installation..." $Blue
if (-not (Get-Command pnpm -ErrorAction SilentlyContinue)) {
    Write-ColorOutput "❌ pnpm is not installed." $Red
    Write-ColorOutput "Installing pnpm globally..." $Yellow
    
    try {
        npm install -g pnpm
        Write-ColorOutput "✅ pnpm installed successfully!" $Green
    } catch {
        Write-ColorOutput "❌ Failed to install pnpm: $($_.Exception.Message)" $Red
        exit 1
    }
} else {
    $pnpmVersion = pnpm --version
    Write-ColorOutput "✅ pnpm is already installed: $pnpmVersion" $Green
}

# Check for WebView2 Runtime (required for Tauri on Windows)
Write-ColorOutput "`nChecking WebView2 Runtime..." $Blue
$webView2Path = "${env:ProgramFiles(x86)}\Microsoft\EdgeWebView\Application"
if (Test-Path $webView2Path) {
    Write-ColorOutput "✅ WebView2 Runtime is installed" $Green
} else {
    Write-ColorOutput "⚠️  WebView2 Runtime might not be installed" $Yellow
    Write-ColorOutput "Tauri requires WebView2 Runtime to run on Windows." $Yellow
    Write-ColorOutput "Download from: https://developer.microsoft.com/en-us/microsoft-edge/webview2/" $Yellow
}

# Check for Microsoft C++ Build Tools
Write-ColorOutput "`nChecking for Microsoft C++ Build Tools..." $Blue
if (Get-Command cl.exe -ErrorAction SilentlyContinue) {
    Write-ColorOutput "✅ Microsoft C++ Build Tools are installed" $Green
} else {
    Write-ColorOutput "⚠️  Microsoft C++ Build Tools might not be installed" $Yellow
    Write-ColorOutput "Required for building native Rust dependencies." $Yellow
    Write-ColorOutput "Install from: https://visualstudio.microsoft.com/visual-cpp-build-tools/" $Yellow
    Write-ColorOutput "Select 'Desktop development with C++' workload" $Yellow
}

# Install project dependencies
Write-ColorOutput "`nInstalling project dependencies..." $Blue
try {
    pnpm install
    Write-ColorOutput "✅ Dependencies installed successfully!" $Green
} catch {
    Write-ColorOutput "❌ Failed to install dependencies: $($_.Exception.Message)" $Red
    exit 1
}

# Configure Windows-specific Tauri settings
Write-ColorOutput "`nConfiguring Windows-specific settings..." $Blue
$configFile = "src-tauri\tauri.conf.json"

if (Test-Path $configFile) {
    try {
        $config = Get-Content $configFile | ConvertFrom-Json
        
        # Ensure Windows bundle configuration exists
        if (-not $config.tauri.bundle) {
            $config.tauri.bundle = @{}
        }
        
        if (-not $config.tauri.bundle.windows) {
            $config.tauri.bundle.windows = @{
                webviewInstallMode = "downloadBootstrapper"
                digestAlgorithm = "sha256"
            }
        } else {
            $config.tauri.bundle.windows.webviewInstallMode = "downloadBootstrapper"
        }
        
        $config | ConvertTo-Json -Depth 100 | Set-Content $configFile
        Write-ColorOutput "✅ Windows configuration updated!" $Green
    } catch {
        Write-ColorOutput "⚠️  Could not update configuration: $($_.Exception.Message)" $Yellow
    }
} else {
    Write-ColorOutput "⚠️  tauri.conf.json not found at expected location" $Yellow
}

# Display setup summary
Write-ColorOutput "`n" + "=" * 60 $Blue
Write-ColorOutput "Setup Complete!" $Green
Write-ColorOutput "=" * 60 + "`n" $Blue

Write-ColorOutput "Next steps:" $Yellow
Write-ColorOutput "  1. Run 'pnpm run tauri dev' to start development server" $Reset
Write-ColorOutput "  2. Run 'pnpm run tauri build' to create production build" $Reset
Write-ColorOutput "`n" + "=" * 60 + "`n" $Blue

Write-ColorOutput "For troubleshooting, check:" $Yellow
Write-ColorOutput "  - Rust installation: rustc --version" $Reset
Write-ColorOutput "  - Node.js version: node --version" $Reset
Write-ColorOutput "  - pnpm version: pnpm --version" $Reset
Write-ColorOutput "`n" + "=" * 60 + "`n" $Blue

Write-ColorOutput "VoidCat RDC - Excellence Protocol Active" $Green
Write-ColorOutput "Contact: SorrowsCry86@voidcat.org | Support: CashApp $WykeveTF`n" $Blue
