# Codacy Analysis CLI (Docker) — Windows Setup

This project uses Codacy Analysis CLI via Docker. No local install is required.

## Prerequisites
- Docker Desktop installed and running
- PowerShell (use `powershell.exe` as the terminal)

## Quick Start
- Pull the CLI image (optional, the first run will pull automatically):

```powershell
powershell -ExecutionPolicy Bypass -File scripts/codacy-cli.ps1 -Pull
```

- Run a full analysis:

```powershell
powershell -ExecutionPolicy Bypass -File scripts/codacy-cli.ps1
```

- Run a security scan using Trivy:

```powershell
powershell -ExecutionPolicy Bypass -File scripts/codacy-cli.ps1 -Tool trivy
```

## VS Code Tasks
Use the Command Palette > Tasks: Run Task and pick:
- `Codacy: Analyze (Docker)`
- `Codacy: Security Scan (Trivy)`
 - `Codacy: Analyze (Soft)`
 - `Codacy: Metrics Only`

## Notes
- The script runs Codacy Analysis CLI in Docker and mounts the repository root as the working directory.
- If Docker isn't available, the script will stop with an error.

### Configuration and Troubleshooting
- A minimal `.codacyrc` has been added to stabilize CLI behavior by disabling tool runs by default. Enable tools incrementally as needed.
- If you still see messages about missing patterns or configuration, prefer the `Codacy: Metrics Only` or `Codacy: Analyze (Soft)` tasks.
- For strict CI gating, use the default `Codacy: Analyze (Docker)` task which will exit non-zero on issues.

