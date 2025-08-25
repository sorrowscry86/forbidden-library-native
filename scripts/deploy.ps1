# Forbidden Library Deployment Management Script
# PowerShell script for managing the Docker deployment

param(
    [Parameter(Mandatory=$false)]
    [ValidateSet("start", "stop", "restart", "status", "logs", "health", "update")]
    [string]$Action = "status"
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
            Write-ColorOutput "✅ Health Check: PASSING" $Green
            return $true
        }
    }
    catch {
        Write-ColorOutput "❌ Health Check: FAILING" $Red
        return $false
    }
}

function Show-Status {
    Write-ColorOutput "`n🔍 Checking Forbidden Library Status..." $Blue
    
    # Check if container is running
    $container = docker ps --filter "name=forbidden-library" --format "table {{.Names}}\t{{.Status}}\t{{.Ports}}"
    if ($container -and $container -notmatch "NAMES") {
        Write-ColorOutput "`n📦 Container Status:" $Yellow
        Write-Host $container
    } else {
        Write-ColorOutput "❌ Container is not running" $Red
        return
    }
    
    # Test health endpoint
    Test-ApplicationHealth
    
    # Show access information
    Write-ColorOutput "`n🌐 Access Information:" $Yellow
    Write-ColorOutput "   Main Application: http://localhost:8080" $Green
    Write-ColorOutput "   Health Check: http://localhost:8080/health" $Green
}

function Start-Application {
    Write-ColorOutput "`n🚀 Starting Forbidden Library..." $Blue
    docker-compose up -d
    if ($LASTEXITCODE -eq 0) {
        Write-ColorOutput "✅ Application started successfully!" $Green
        Start-Sleep -Seconds 3
        Show-Status
    } else {
        Write-ColorOutput "❌ Failed to start application" $Red
    }
}

function Stop-Application {
    Write-ColorOutput "`n🛑 Stopping Forbidden Library..." $Blue
    docker-compose down
    if ($LASTEXITCODE -eq 0) {
        Write-ColorOutput "✅ Application stopped successfully!" $Green
    } else {
        Write-ColorOutput "❌ Failed to stop application" $Red
    }
}

function Restart-Application {
    Write-ColorOutput "`n🔄 Restarting Forbidden Library..." $Blue
    docker-compose restart
    if ($LASTEXITCODE -eq 0) {
        Write-ColorOutput "✅ Application restarted successfully!" $Green
        Start-Sleep -Seconds 3
        Show-Status
    } else {
        Write-ColorOutput "❌ Failed to restart application" $Red
    }
}

function Show-Logs {
    Write-ColorOutput "`n📋 Showing application logs..." $Blue
    docker-compose logs -f --tail=50
}

function Update-Application {
    Write-ColorOutput "`n🔄 Updating Forbidden Library..." $Blue
    docker-compose down
    docker-compose up -d --build
    if ($LASTEXITCODE -eq 0) {
        Write-ColorOutput "✅ Application updated successfully!" $Green
        Start-Sleep -Seconds 5
        Show-Status
    } else {
        Write-ColorOutput "❌ Failed to update application" $Red
    }
}

# Main execution
Write-ColorOutput "🐳 Forbidden Library Deployment Manager" $Blue
Write-ColorOutput "=====================================" $Blue

switch ($Action) {
    "start" { Start-Application }
    "stop" { Stop-Application }
    "restart" { Restart-Application }
    "status" { Show-Status }
    "logs" { Show-Logs }
    "health" { Test-ApplicationHealth }
    "update" { Update-Application }
    default { Show-Status }
}

Write-ColorOutput "`n📖 Usage: .\deploy.ps1 -Action [start|stop|restart|status|logs|health|update]" $Yellow