param(
    [Parameter(Mandatory = $true)] [string]$SourceRepoUrl,
    [string]$SourceBranch = "main",
    [string]$Prefix = "",
    [ValidateSet("merge","subtree")] [string]$Strategy = "merge"
)

Set-StrictMode -Version Latest
$ErrorActionPreference = "Stop"

function Run($cmd) {
    Write-Host "→ $cmd"
    $out = Invoke-Expression $cmd
    return $out
}

# Validate we are inside a git repo
try { Run "git rev-parse --is-inside-work-tree | Out-Null" } catch {
    Write-Error "This folder is not a git repository. Initialize or open the target repo first."; exit 1
}

# Derive a remote name from URL host/path
$remoteName = ([uri]$SourceRepoUrl).Segments[-1].Trim('/')
if (-not $remoteName) { $remoteName = "source-remote" }
$remoteName = $remoteName -replace "\.git$",""

# Add and fetch the source repository
Run "git remote add $remoteName `"$SourceRepoUrl`"" 2>$null
Run "git fetch $remoteName --prune"

# Create a merge branch
$mergeBranch = "merge/$remoteName"
Run "git checkout -b $mergeBranch"

if ($Strategy -eq "merge" -and [string]::IsNullOrWhiteSpace($Prefix)) {
    # Direct merge with unrelated histories allowed
    Run "git merge --allow-unrelated-histories $remoteName/$SourceBranch -m `"Merge $remoteName/$SourceBranch into current repository (allow unrelated histories)`""
} elseif ($Strategy -eq "subtree") {
    if ([string]::IsNullOrWhiteSpace($Prefix)) { $Prefix = $remoteName }
    # Subtree merge: keep source under a directory prefix
    # 1) Read the source branch into a subtree
    Run "git read-tree --prefix=$Prefix/ -u $remoteName/$SourceBranch"
    # 2) Commit the subtree import
    Run "git commit -m `"Import $remoteName/$SourceBranch into $Prefix/ (subtree)`""
} else {
    # Fallback: move the source tree into Prefix via sparse checkout and merge
    if ([string]::IsNullOrWhiteSpace($Prefix)) { $Prefix = $remoteName }
    Write-Host "Using fallback subtree-like merge into $Prefix/"
    $tmpDir = Join-Path $env:TEMP ("repo-merge-" + [Guid]::NewGuid())
    New-Item -ItemType Directory -Path $tmpDir | Out-Null
    try {
        Run "git clone --branch $SourceBranch --depth 1 `"$SourceRepoUrl`" `"$tmpDir`""
        Run "robocopy `"$tmpDir`" `"$Prefix`" /MIR /XD .git > `$null"
        Run "git add `"$Prefix`""
        Run "git commit -m `"Import $remoteName/$SourceBranch into $Prefix/ (copy)`""
    } finally {
        Remove-Item -Recurse -Force $tmpDir -ErrorAction SilentlyContinue
    }
}

Write-Host "Merge complete on branch $mergeBranch. Review changes, resolve conflicts if any, then push or open a PR." -ForegroundColor Green
