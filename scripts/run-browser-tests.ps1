# Forbidden Library Browser Automation Testing
# PowerShell script to run Selenium-based browser tests

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

function Install-PythonDependencies {
    Write-ColorOutput "📦 Installing Python dependencies..." $Blue
    
    try {
        # Check if Python is available
        $pythonVersion = python --version 2>&1
        if ($LASTEXITCODE -ne 0) {
            Write-ColorOutput "❌ Python is not installed or not in PATH" $Red
            Write-ColorOutput "Please install Python from https://python.org/" $Yellow
            return $false
        }
        
        Write-ColorOutput "Python version: $pythonVersion" $Green
        
        # Install required packages
        Write-ColorOutput "Installing Selenium..." $Yellow
        python -m pip install selenium webdriver-manager
        
        if ($LASTEXITCODE -eq 0) {
            Write-ColorOutput "✅ Python dependencies installed successfully!" $Green
            return $true
        } else {
            Write-ColorOutput "❌ Failed to install Python dependencies" $Red
            return $false
        }
    }
    catch {
        Write-ColorOutput "❌ Error installing dependencies: $($_.Exception.Message)" $Red
        return $false
    }
}

function Setup-TestEnvironment {
    Write-ColorOutput "🔧 Setting up test environment..." $Blue
    
    # Check if application is running
    if (-not (Test-ApplicationHealth)) {
        Write-ColorOutput "❌ Cannot setup tests - application is not accessible" $Red
        Write-ColorOutput "Please start the application first: docker-compose up -d" $Yellow
        return $false
    }
    
    # Install Python dependencies
    if (-not (Install-PythonDependencies)) {
        return $false
    }
    
    Write-ColorOutput "✅ Test environment setup complete!" $Green
    return $true
}

function Run-BrowserTests {
    param([bool]$Headless = $false)
    
    Write-ColorOutput "🧪 Running browser automation tests..." $Blue
    
    # Check if application is running
    if (-not (Test-ApplicationHealth)) {
        Write-ColorOutput "❌ Cannot run tests - application is not accessible" $Red
        Write-ColorOutput "Please start the application first: docker-compose up -d" $Yellow
        return $false
    }
    
    # Change to scripts directory
    Push-Location "scripts"
    
    try {
        # Run the browser test script
        $headlessFlag = if ($Headless) { "--headless" } else { "" }
        Write-ColorOutput "Starting browser tests...$($headlessFlag)" $Yellow
        
        python browser-test.py $headlessFlag
        
        if ($LASTEXITCODE -eq 0) {
            Write-ColorOutput "✅ All browser tests passed!" $Green
            return $true
        } else {
            Write-ColorOutput "❌ Some browser tests failed" $Red
            return $false
        }
    }
    catch {
        Write-ColorOutput "❌ Browser test execution failed: $($_.Exception.Message)" $Red
        return $false
    }
    finally {
        Pop-Location
    }
}

function Show-Help {
    Write-ColorOutput "`n📖 Browser Testing Commands:" $Blue
    Write-ColorOutput "  .\run-browser-tests.ps1 -Action setup" $Yellow
    Write-ColorOutput "  .\run-browser-tests.ps1 -Action install" $Yellow
    Write-ColorOutput "  .\run-browser-tests.ps1 -Action test" $Yellow
    Write-ColorOutput "  .\run-browser-tests.ps1 -Action test-headless" $Yellow
    Write-ColorOutput "`n📖 Prerequisites:" $Blue
    Write-ColorOutput "  - Python 3.7+ installed" $Yellow
    Write-ColorOutput "  - Application running on http://localhost:8080" $Yellow
    Write-ColorOutput "  - Chrome browser installed" $Yellow
}

# Main execution
Write-ColorOutput "🧪 Forbidden Library Browser Automation Testing" $Blue
Write-ColorOutput "=============================================" $Blue

switch ($Action) {
    "setup" { 
        if (Setup-TestEnvironment) {
            Write-ColorOutput "`n🎉 Setup complete! Ready to run tests." $Green
        } else {
            Write-ColorOutput "`n❌ Setup failed. Please check the errors above." $Red
            exit 1
        }
    }
    "install" { 
        if (Install-PythonDependencies) {
            Write-ColorOutput "`n🎉 Dependencies installed! Ready to run tests." $Green
        } else {
            Write-ColorOutput "`n❌ Installation failed. Please check the errors above." $Red
            exit 1
        }
    }
    "test" { 
        if (Run-BrowserTests -Headless $false) {
            Write-ColorOutput "`n🎉 Browser tests completed successfully!" $Green
        } else {
            Write-ColorOutput "`n❌ Browser tests failed. Please check the errors above." $Red
            exit 1
        }
    }
    "test-headless" { 
        if (Run-BrowserTests -Headless $true) {
            Write-ColorOutput "`n🎉 Browser tests completed successfully!" $Green
        } else {
            Write-ColorOutput "`n❌ Browser tests failed. Please check the errors above." $Red
            exit 1
        }
    }
    default { 
        Show-Help
        if (Test-ApplicationHealth) {
            Write-ColorOutput "`n✅ Application is running. Ready for testing!" $Green
        }
    }
}

Write-ColorOutput "`n🌐 Application URL: http://localhost:8080" $Green