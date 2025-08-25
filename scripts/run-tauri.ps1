# Forbidden Library - Tauri Desktop Application Runner
# PowerShell script to run the full Tauri desktop application

param(
    [Parameter(Mandatory=$false)]
    [ValidateSet("dev", "build", "preview")]
    [string]$Mode = "dev"
)

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

function Test-Prerequisites {
    Write-ColorOutput "🔧 Checking prerequisites..." $Blue
    
    # Check if Node.js is available
    $nodeVersion = node --version 2>$null
    if ($LASTEXITCODE -ne 0) {
        Write-ColorOutput "❌ Node.js is not installed or not in PATH" $Red
        Write-ColorOutput "Please install Node.js from https://nodejs.org/" $Yellow
        return $false
    }
    Write-ColorOutput "✅ Node.js version: $nodeVersion" $Green
    
    # Check if Rust is available
    $rustVersion = rustc --version 2>$null
    if ($LASTEXITCODE -ne 0) {
        Write-ColorOutput "❌ Rust is not installed or not in PATH" $Red
        Write-ColorOutput "Please install Rust from https://rustup.rs/" $Yellow
        return $false
    }
    Write-ColorOutput "✅ Rust version: $rustVersion" $Green
    
    # Check if pnpm is available
    $pnpmVersion = pnpm --version 2>$null
    if ($LASTEXITCODE -ne 0) {
        Write-ColorOutput "❌ pnpm is not installed" $Red
        Write-ColorOutput "Installing pnpm..." $Yellow
        npm install -g pnpm
        if ($LASTEXITCODE -ne 0) {
            Write-ColorOutput "❌ Failed to install pnpm" $Red
            return $false
        }
    }
    Write-ColorOutput "✅ pnpm version: $pnpmVersion" $Green
    
    return $true
}

function Install-Dependencies {
    Write-ColorOutput "📦 Installing dependencies..." $Blue
    
    try {
        # Install frontend dependencies
        Write-ColorOutput "Installing frontend dependencies..." $Yellow
        pnpm install
        
        if ($LASTEXITCODE -ne 0) {
            Write-ColorOutput "❌ Failed to install frontend dependencies" $Red
            return $false
        }
        
        Write-ColorOutput "✅ Dependencies installed successfully!" $Green
        return $true
        
    } catch {
        Write-ColorOutput "❌ Error installing dependencies: $($_.Exception.Message)" $Red
        return $false
    }
}

function Start-TauriApp {
    param([string]$Mode)
    
    Write-ColorOutput "🚀 Starting Tauri desktop application..." $Blue
    
    try {
        switch ($Mode) {
            "dev" {
                Write-ColorOutput "Starting in development mode..." $Yellow
                pnpm run tauri:dev
            }
            "build" {
                Write-ColorOutput "Building for production..." $Yellow
                pnpm run tauri:build
            }
            "preview" {
                Write-ColorOutput "Starting preview mode..." $Yellow
                pnpm run tauri:preview
            }
        }
        
        if ($LASTEXITCODE -eq 0) {
            Write-ColorOutput "✅ Tauri application completed successfully!" $Green
        } else {
            Write-ColorOutput "❌ Tauri application failed" $Red
        }
        
    } catch {
        Write-ColorOutput "❌ Error starting Tauri application: $($_.Exception.Message)" $Red
    }
}

# Main execution
Write-ColorOutput "🖥️ Forbidden Library - Tauri Desktop Application" $Blue
Write-ColorOutput "=============================================" $Blue

# Check prerequisites
if (-not (Test-Prerequisites)) {
    Write-ColorOutput "`n❌ Prerequisites check failed. Please install required tools." $Red
    exit 1
}

# Install dependencies
if (-not (Install-Dependencies)) {
    Write-ColorOutput "`n❌ Dependency installation failed." $Red
    exit 1
}

# Start Tauri application
Start-TauriApp -Mode $Mode

Write-ColorOutput "`n📋 Available commands:" $Blue
Write-ColorOutput "  .\scripts\run-tauri.ps1 -Mode dev     # Development mode" $Yellow
Write-ColorOutput "  .\scripts\run-tauri.ps1 -Mode build   # Build for production" $Yellow
Write-ColorOutput "  .\scripts\run-tauri.ps1 -Mode preview # Preview mode" $Yellow
Write-ColorOutput "`n🌐 Web mode: http://localhost:8080" $Green
Write-ColorOutput "🖥️ Desktop mode: Run the Tauri application" $Green