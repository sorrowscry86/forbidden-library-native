# Quick Deployment Test Script
# Tests the Forbidden Library deployment without external dependencies

param(
    [Parameter(Mandatory=$false)]
    [ValidateSet("basic", "full", "performance")]
    [string]$TestType = "basic"
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

function Test-HealthCheck {
    Write-ColorOutput "📋 Testing Health Check..." $Blue
    
    try {
        $response = Invoke-WebRequest -Uri "http://localhost:8080/health" -UseBasicParsing -TimeoutSec 10
        $content = $response.Content.Trim()
        
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
    Write-ColorOutput "📋 Testing Main Application..." $Blue
    
    try {
        $startTime = Get-Date
        $response = Invoke-WebRequest -Uri "http://localhost:8080/" -UseBasicParsing -TimeoutSec 10
        $endTime = Get-Date
        $loadTime = ($endTime - $startTime).TotalMilliseconds
        
        if ($response.StatusCode -eq 200) {
            Write-ColorOutput "  ✅ Main application loaded successfully" $Green
            Write-ColorOutput "  📊 Load time: $loadTime ms" $Yellow
            Write-ColorOutput "  📄 Content length: $($response.Content.Length) bytes" $Yellow
            Write-ColorOutput "  🏷️  Content type: $($response.Headers.'Content-Type')" $Yellow
            
            # Check for key content indicators
            $hasTitle = $response.Content -match '<title>Forbidden Library</title>'
            $hasTheme = $response.Content -match 'data-theme="dark"'
            $hasSvelteKit = $response.Content -match '_app/immutable'
            
            if ($hasTitle) { Write-ColorOutput "  ✅ Page title found" $Green }
            if ($hasTheme) { Write-ColorOutput "  ✅ Dark theme configured" $Green }
            if ($hasSvelteKit) { Write-ColorOutput "  ✅ SvelteKit assets detected" $Green }
            
            return @{
                Success = $true
                LoadTime = $loadTime
                ContentLength = $response.Content.Length
                HasTitle = $hasTitle
                HasTheme = $hasTheme
                HasSvelteKit = $hasSvelteKit
            }
        } else {
            Write-ColorOutput "  ❌ Main application failed: Status $($response.StatusCode)" $Red
            return @{ Success = $false }
        }
    }
    catch {
        Write-ColorOutput "  ❌ Main application error: $($_.Exception.Message)" $Red
        return @{ Success = $false }
    }
}

function Test-Performance {
    Write-ColorOutput "📋 Testing Performance..." $Blue
    
    $results = @()
    
    # Run multiple tests to get average
    for ($i = 1; $i -le 5; $i++) {
        try {
            $startTime = Get-Date
            $response = Invoke-WebRequest -Uri "http://localhost:8080/" -UseBasicParsing -TimeoutSec 10
            $endTime = Get-Date
            $loadTime = ($endTime - $startTime).TotalMilliseconds
            
            $results += $loadTime
            Write-ColorOutput "  Test $i`: $loadTime ms" $Yellow
        }
        catch {
            Write-ColorOutput "  ❌ Performance test $i failed" $Red
            return $false
        }
    }
    
    $averageTime = ($results | Measure-Object -Average).Average
    $minTime = ($results | Measure-Object -Minimum).Minimum
    $maxTime = ($results | Measure-Object -Maximum).Maximum
    
    Write-ColorOutput "  📊 Performance Results:" $Blue
    Write-ColorOutput "    Average: $([math]::Round($averageTime, 2)) ms" $Yellow
    Write-ColorOutput "    Minimum: $minTime ms" $Yellow
    Write-ColorOutput "    Maximum: $maxTime ms" $Yellow
    
    # Performance rating
    if ($averageTime -lt 100) {
        Write-ColorOutput "  🚀 Performance: EXCELLENT (< 100ms)" $Green
    } elseif ($averageTime -lt 500) {
        Write-ColorOutput "  ⚡ Performance: GOOD (< 500ms)" $Green
    } elseif ($averageTime -lt 1000) {
        Write-ColorOutput "  ⚠️  Performance: ACCEPTABLE (< 1000ms)" $Yellow
    } else {
        Write-ColorOutput "  ❌ Performance: NEEDS OPTIMIZATION (> 1000ms)" $Red
    }
    
    return $true
}

function Test-ContainerStatus {
    Write-ColorOutput "📋 Testing Container Status..." $Blue
    
    try {
        $container = docker ps --filter "name=forbidden-library" --format "table {{.Names}}\t{{.Status}}\t{{.Ports}}"
        
        if ($container -and $container -notmatch "NAMES") {
            Write-ColorOutput "  ✅ Container is running" $Green
            Write-ColorOutput "  📊 Container Info:" $Yellow
            $container | ForEach-Object { Write-ColorOutput "    $_" $Yellow }
            return $true
        } else {
            Write-ColorOutput "  ❌ Container is not running" $Red
            return $false
        }
    }
    catch {
        Write-ColorOutput "  ❌ Container status check failed: $($_.Exception.Message)" $Red
        return $false
    }
}

function Show-TestSummary {
    param([hashtable]$Results)
    
    Write-ColorOutput "`n📊 Test Summary" $Blue
    Write-ColorOutput "=============" $Blue
    
    $passed = 0
    $total = $Results.Count
    
    foreach ($test in $Results.Keys) {
        if ($Results[$test]) {
            Write-ColorOutput "✅ $test" $Green
            $passed++
        } else {
            Write-ColorOutput "❌ $test" $Red
        }
    }
    
    Write-ColorOutput "`n📈 Results: $passed/$total tests passed" $Blue
    
    if ($passed -eq $total) {
        Write-ColorOutput "🎉 All tests passed! Application is ready for use." $Green
    } else {
        Write-ColorOutput "⚠️  Some tests failed. Please check the application." $Yellow
    }
}

# Main execution
Write-ColorOutput "🧪 Forbidden Library Quick Test" $Blue
Write-ColorOutput "=============================" $Blue

$testResults = @{}

# Always run basic tests
$testResults["Health Check"] = Test-HealthCheck
$testResults["Main Application"] = (Test-MainApplication).Success
$testResults["Container Status"] = Test-ContainerStatus

# Run additional tests based on type
if ($TestType -eq "full" -or $TestType -eq "performance") {
    $testResults["Performance"] = Test-Performance
}

# Show summary
Show-TestSummary -Results $testResults

Write-ColorOutput "`n🌐 Application URL: http://localhost:8080" $Green
Write-ColorOutput "📖 Usage: .\quick-test.ps1 -TestType [basic|full|performance]" $Yellow