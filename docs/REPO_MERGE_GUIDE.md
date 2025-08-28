# Repository Merge Guide

This guide helps you merge another repository into this one while preserving history.

## Prerequisites

- Git installed and authenticated (SSH or HTTPS)
- Current repo checked out and clean (no uncommitted changes)

## Quick Options

1. Direct merge (histories unrelated):

```powershell
# Adds remote, fetches, merges source history into current branch
git remote add source <REPO_URL>
git fetch source --prune
git merge --allow-unrelated-histories source/main -m "Merge source/main"
```

1. Subtree import (keep under a folder):

```powershell
git remote add source <REPO_URL>
git fetch source --prune
git read-tree --prefix=vendor/<NAME>/ -u source/main
git commit -m "Import source/main into vendor/<NAME>/ (subtree)"
```

## Automated Helper (Windows PowerShell)

Use the script to streamline merge steps:

```powershell
# Strategy can be 'merge' (default) or 'subtree'
# Prefix is used only with 'subtree' to place files under that folder
powershell -ExecutionPolicy Bypass -File scripts/merge-repo.ps1 `
  -SourceRepoUrl "https://github.com/owner/repo.git" `
  -SourceBranch main `
  -Strategy subtree `
  -Prefix vendor/that-repo
```

After running, review changes on the created branch (e.g., `merge/<remote-name>`), resolve conflicts, and push a PR.

## Tips

- If the source repo has a `LICENSE`, keep both by placing the source one under the imported folder (`<prefix>/LICENSE`).
- Consider adding the imported path to `.codacyrc` `excludePaths` if you don’t want it analyzed initially.
- Update documentation and CI as needed to reflect the new structure.
