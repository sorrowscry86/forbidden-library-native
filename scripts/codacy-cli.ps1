# Workflow-Update: Codacy CLI helper (Docker-based) for Windows PowerShell
# Runs Codacy Analysis CLI inside a container, mounting the repository root.
# Usage examples:
#   powershell -ExecutionPolicy Bypass -File scripts/codacy-cli.ps1
#   powershell -ExecutionPolicy Bypass -File scripts/codacy-cli.ps1 -Tool trivy
#   powershell -ExecutionPolicy Bypass -File scripts/codacy-cli.ps1 -Pull

param(
    [string]$Tool = "",
    [switch]$Pull,
    [switch]$SoftFail
)

Set-StrictMode -Version Latest
$ErrorActionPreference = "Stop"

# Determine workspace root as the parent of this script's folder
$ScriptDir = Split-Path -Parent $MyInvocation.MyCommand.Path
$WorkspaceRoot = Split-Path -Parent $ScriptDir

if (-not (Test-Path $WorkspaceRoot)) {
    Write-Error "Workspace root not found at: $WorkspaceRoot"
    exit 1
}

# Image reference: prefer Docker Hub public image; GHCR can be restricted
$Image = "codacy/codacy-analysis-cli:latest"

function Test-DockerAvailable {
    try {
        $null = docker --version 2>$null
    } catch {
        Write-Error "Docker does not appear to be installed or available on PATH. Please install Docker Desktop and try again."
        exit 1
    }
}

Test-DockerAvailable

if ($Pull) {
    Write-Host "Pulling Codacy Analysis CLI image: $Image"
    docker pull $Image
}

# Build the docker run arguments (mount at a Linux path inside the container)
$containerMount = "/workspace"
$vol = "$WorkspaceRoot`:$containerMount"
$workdir = $containerMount

$cliArgs = @(
    "run","--rm",
    "-v", "/var/run/docker.sock:/var/run/docker.sock",
    "-v", $vol,
    "-w", $workdir,
    $Image,
    "analyze",
    "--directory", $workdir
)

if ($Tool -and $Tool.Trim().Length -gt 0) {
    $cliArgs += @("--tool", $Tool)
}

if ($SoftFail) {
    # Do not fail the task if Codacy finds issues; return code 0
    $cliArgs += @("--fail-on-issues", "false")
}

Write-Host "Executing Codacy Analysis CLI via Docker..."
Write-Host "docker $($cliArgs -join ' ')"

docker @cliArgs
if (-not $SoftFail) {
    exit $LASTEXITCODE
} else {
    Write-Host "SoftFail enabled: ignoring Codacy CLI exit code $LASTEXITCODE"
    exit 0
}
