# Forbidden Library - Comprehensive Test Battery
# PowerShell script for complete testing suite

param(
    [Parameter(Mandatory=$false)]
    [ValidateSet("browser", "health", "performance", "all")]
    [string]$TestType = "all"
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
    Write-ColorOutput "`n📋 Test 1: Application Health Check" $Blue
    try {
        $response = Invoke-WebRequest -Uri "http://localhost:8080/health" -UseBasicParsing -TimeoutSec 5
        $content = $response.Content.ToString().Trim()
        
        if ($response.StatusCode -eq 200 -and $content -eq "healthy") {
            Write-ColorOutput "  ✅ Health check passed" $Green
            return $true
        } else {
            Write-ColorOutput "  ❌ Health check failed: Status $($response.StatusCode), Content: '$content'" $Red
            return $false
        }
    }
    catch {
        Write-ColorOutput "  ❌ Health check error: $($_.Exception.Message)" $Red
        return $false
    }
}

function Test-MainApplication {
    Write-ColorOutput "`n📋 Test 2: Main Application" $Blue
    try {
        $startTime = Get-Date
        $response = Invoke-WebRequest -Uri "http://localhost:8080" -UseBasicParsing -TimeoutSec 10
        $endTime = Get-Date
        $loadTime = ($endTime - $startTime).TotalMilliseconds
        
        $content = $response.Content.ToString()
        
        Write-ColorOutput "  📄 Load time: $($loadTime.ToString('F2')) ms" $Yellow
        Write-ColorOutput "  📄 Content length: $($content.Length) characters" $Yellow
        
        # Check for key elements
        $hasTitle = $content -match "Forbidden Library"
        $hasTheme = $content -match 'data-theme="dark"'
        $hasSvelteKit = $content -match "data-sveltekit-preload-data"
        
        if ($hasTitle) { Write-ColorOutput "  ✅ Page title found" $Green }
        if ($hasTheme) { Write-ColorOutput "  ✅ Dark theme configured" $Green }
        if ($hasSvelteKit) { Write-ColorOutput "  ✅ SvelteKit detected" $Green }
        
        if ($hasTitle -and $hasTheme -and $hasSvelteKit) {
            Write-ColorOutput "  ✅ Main application test passed" $Green
            return $true
        } else {
            Write-ColorOutput "  ❌ Main application test failed" $Red
            return $false
        }
    }
    catch {
        Write-ColorOutput "  ❌ Main application error: $($_.Exception.Message)" $Red
        return $false
    }
}

function Test-Performance {
    Write-ColorOutput "`n📋 Test 3: Performance Testing" $Blue
    $times = @()
    
    try {
        for ($i = 1; $i -le 5; $i++) {
            $startTime = Get-Date
            $response = Invoke-WebRequest -Uri "http://localhost:8080" -UseBasicParsing -TimeoutSec 10
            $endTime = Get-Date
            $loadTime = ($endTime - $startTime).TotalMilliseconds
            $times += $loadTime
            
            Write-ColorOutput "  Test $i`: $($loadTime.ToString('F2')) ms" $Yellow
        }
        
        $avgTime = ($times | Measure-Object -Average).Average
        $minTime = ($times | Measure-Object -Minimum).Minimum
        $maxTime = ($times | Measure-Object -Maximum).Maximum
        
        Write-ColorOutput "  📊 Average: $($avgTime.ToString('F2')) ms" $Yellow
        Write-ColorOutput "  📊 Minimum: $($minTime.ToString('F2')) ms" $Yellow
        Write-ColorOutput "  📊 Maximum: $($maxTime.ToString('F2')) ms" $Yellow
        
        if ($avgTime -lt 100) {
            Write-ColorOutput "  🚀 Performance: EXCELLENT (< 100ms)" $Green
        } elseif ($avgTime -lt 500) {
            Write-ColorOutput "  ⚡ Performance: GOOD (< 500ms)" $Green
        } elseif ($avgTime -lt 1000) {
            Write-ColorOutput "  ⚠️ Performance: ACCEPTABLE (< 1000ms)" $Yellow
        } else {
            Write-ColorOutput "  ❌ Performance: NEEDS OPTIMIZATION (> 1000ms)" $Red
        }
        
        return $true
    }
    catch {
        Write-ColorOutput "  ❌ Performance test error: $($_.Exception.Message)" $Red
        return $false
    }
}

function Test-DockerContainer {
    Write-ColorOutput "`n📋 Test 4: Docker Container Status" $Blue
    try {
        $container = docker ps --filter "name=forbidden-library" --format "table {{.Names}}\t{{.Status}}\t{{.Ports}}" 2>$null
        
        if ($container -and $container -notmatch "NAMES") {
            Write-ColorOutput "  ✅ Container is running" $Green
            Write-ColorOutput "  📊 Container info:" $Yellow
            $container | ForEach-Object { Write-ColorOutput "    $_" $Yellow }
            return $true
        } else {
            Write-ColorOutput "  ❌ Container is not running" $Red
            return $false
        }
    }
    catch {
        Write-ColorOutput "  ❌ Docker test error: $($_.Exception.Message)" $Red
        return $false
    }
}

function Test-BrowserAutomation {
    Write-ColorOutput "`n📋 Test 5: Browser Automation" $Blue
    try {
        # Check if Python and dependencies are available
        $pythonVersion = python --version 2>$null
        if ($LASTEXITCODE -ne 0) {
            Write-ColorOutput "  ❌ Python is not available" $Red
            return $false
        }
        
        # Run browser automation test
        Push-Location "scripts"
        $browserTest = python browser-test.py --headless 2>&1
        $browserExitCode = $LASTEXITCODE
        Pop-Location
        
        if ($browserExitCode -eq 0) {
            Write-ColorOutput "  ✅ Browser automation test passed" $Green
            return $true
        } else {
            Write-ColorOutput "  ❌ Browser automation test failed" $Red
            Write-ColorOutput "  📄 Output: $browserTest" $Yellow
            return $false
        }
    }
    catch {
        Write-ColorOutput "  ❌ Browser automation error: $($_.Exception.Message)" $Red
        return $false
    }
}

function Show-TestSummary {
    param([hashtable]$Results)
    
    Write-ColorOutput "`n📊 Test Summary" $Blue
    Write-ColorOutput "=============" $Blue
    
    $passed = 0
    $total = $Results.Count
    
    foreach ($test in $Results.GetEnumerator()) {
        if ($test.Value) {
            Write-ColorOutput "✅ $($test.Key)" $Green
            $passed++
        } else {
            Write-ColorOutput "❌ $($test.Key)" $Red
        }
    }
    
    Write-ColorOutput "`n📈 Results: $passed/$total tests passed" $Blue
    
    if ($passed -eq $total) {
        Write-ColorOutput "🎉 All tests passed! Application is ready for use." $Green
    } else {
        Write-ColorOutput "⚠️ Some tests failed. Please check the application." $Yellow
    }
    
    return $passed -eq $total
}

# Main execution
Write-ColorOutput "🧪 Forbidden Library - Comprehensive Test Battery" $Blue
Write-ColorOutput "=============================================" $Blue

$testResults = @{}

switch ($TestType) {
    "health" {
        $testResults["Health Check"] = Test-ApplicationHealth
    }
    "performance" {
        $testResults["Health Check"] = Test-ApplicationHealth
        $testResults["Performance"] = Test-Performance
    }
    "browser" {
        $testResults["Health Check"] = Test-ApplicationHealth
        $testResults["Browser Automation"] = Test-BrowserAutomation
    }
    "all" {
        $testResults["Health Check"] = Test-ApplicationHealth
        $testResults["Main Application"] = Test-MainApplication
        $testResults["Performance"] = Test-Performance
        $testResults["Docker Container"] = Test-DockerContainer
        $testResults["Browser Automation"] = Test-BrowserAutomation
    }
}

$success = Show-TestSummary -Results $testResults

Write-ColorOutput "`n🌐 Application URL: http://localhost:8080" $Green
Write-ColorOutput "📁 Test files: scripts/browser-test.py, scripts/comprehensive-test.ps1" $Green

if (-not $success) {
    exit 1
}