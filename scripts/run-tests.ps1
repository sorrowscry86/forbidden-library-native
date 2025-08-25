# Forbidden Library Deployment Testing Script
# PowerShell script to run Playwright tests

param(
    [Parameter(Mandatory=$false)]
    [ValidateSet("install", "test", "test-headless", "setup")]
    [string]$Action = "test"
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

function Test-ApplicationHealth {
    try {
        $response = Invoke-WebRequest -Uri "http://localhost:8080/health" -UseBasicParsing -TimeoutSec 5
        if ($response.StatusCode -eq 200) {
            Write-ColorOutput "✅ Application is running and healthy" $Green
            return $true
        }
    }
    catch {
        Write-ColorOutput "❌ Application is not accessible" $Red
        return $false
    }
}

function Install-TestDependencies {
    Write-ColorOutput "📦 Installing test dependencies..." $Blue
    
    # Change to scripts directory
    Push-Location "scripts"
    
    try {
        # Install Playwright
        Write-ColorOutput "Installing Playwright..." $Yellow
        npm install playwright
        
        # Install Playwright browsers
        Write-ColorOutput "Installing Playwright browsers..." $Yellow
        npx playwright install chromium
        
        Write-ColorOutput "✅ Test dependencies installed successfully!" $Green
    }
    catch {
        Write-ColorOutput "❌ Failed to install test dependencies" $Red
        throw
    }
    finally {
        Pop-Location
    }
}

function Setup-TestEnvironment {
    Write-ColorOutput "🔧 Setting up test environment..." $Blue
    
    # Check if Node.js is available
    try {
        $nodeVersion = node --version
        Write-ColorOutput "Node.js version: $nodeVersion" $Green
    }
    catch {
        Write-ColorOutput "❌ Node.js is not installed or not in PATH" $Red
        Write-ColorOutput "Please install Node.js from https://nodejs.org/" $Yellow
        exit 1
    }
    
    # Check if npm is available
    try {
        $npmVersion = npm --version
        Write-ColorOutput "npm version: $npmVersion" $Green
    }
    catch {
        Write-ColorOutput "❌ npm is not available" $Red
        exit 1
    }
    
    # Install dependencies
    Install-TestDependencies
}

function Run-Tests {
    Write-ColorOutput "🧪 Running deployment tests..." $Blue
    
    # Check if application is running
    if (-not (Test-ApplicationHealth)) {
        Write-ColorOutput "❌ Cannot run tests - application is not accessible" $Red
        Write-ColorOutput "Please start the application first: docker-compose up -d" $Yellow
        exit 1
    }
    
    # Change to scripts directory
    Push-Location "scripts"
    
    try {
        # Run the test script
        Write-ColorOutput "Starting Playwright tests..." $Yellow
        node test-deployment.js
        
        if ($LASTEXITCODE -eq 0) {
            Write-ColorOutput "✅ All tests passed!" $Green
        } else {
            Write-ColorOutput "❌ Some tests failed" $Red
            exit $LASTEXITCODE
        }
    }
    catch {
        Write-ColorOutput "❌ Test execution failed: $($_.Exception.Message)" $Red
        exit 1
    }
    finally {
        Pop-Location
    }
}

function Run-TestsHeadless {
    Write-ColorOutput "🧪 Running deployment tests (headless mode)..." $Blue
    
    # Check if application is running
    if (-not (Test-ApplicationHealth)) {
        Write-ColorOutput "❌ Cannot run tests - application is not accessible" $Red
        Write-ColorOutput "Please start the application first: docker-compose up -d" $Yellow
        exit 1
    }
    
    # Change to scripts directory
    Push-Location "scripts"
    
    try {
        # Run the test script with headless flag
        Write-ColorOutput "Starting Playwright tests (headless)..." $Yellow
        node test-deployment.js --headless
        
        if ($LASTEXITCODE -eq 0) {
            Write-ColorOutput "✅ All tests passed!" $Green
        } else {
            Write-ColorOutput "❌ Some tests failed" $Red
            exit $LASTEXITCODE
        }
    }
    catch {
        Write-ColorOutput "❌ Test execution failed: $($_.Exception.Message)" $Red
        exit 1
    }
    finally {
        Pop-Location
    }
}

# Main execution
Write-ColorOutput "🧪 Forbidden Library Deployment Testing" $Blue
Write-ColorOutput "=====================================" $Blue

switch ($Action) {
    "setup" { Setup-TestEnvironment }
    "install" { Install-TestDependencies }
    "test" { Run-Tests }
    "test-headless" { Run-TestsHeadless }
    default { Run-Tests }
}

Write-ColorOutput "`n📖 Usage: .\run-tests.ps1 -Action [setup|install|test|test-headless]" $Yellow