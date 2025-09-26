# Pre-Commit Cleanup Recommendations

This file contains recommendations for cleaning up the repository before the initial commit. Delete this file after completing all the tasks.

## Critical Issues (Must Fix)

1. **Remove Sensitive Information**
   - [ ] Remove the `.env` file from version control
   - [ ] Create a `.env.example` file with placeholder values instead

   ```
   # Sentry Configuration - VoidCat RDC
   # Production monitoring for Forbidden Library

   SENTRY_DSN=your_sentry_dsn_here
   ENVIRONMENT=development
   SENTRY_TRACES_SAMPLE_RATE=1.0
   SENTRY_PROFILES_SAMPLE_RATE=1.0

   # Development flags
   RUST_LOG=info
   RUST_BACKTRACE=1
   ```

2. **Remove Build Artifacts**
   - [ ] Ensure the `target` directory is not included in the initial commit
   - [ ] Ensure the `build` directory is not included in the initial commit
   - [ ] Ensure the `.svelte-kit` directory is not included in the initial commit

## Required Issues (Should Fix)

1. **Create Missing Documentation**
   - [ ] Create a `CONTRIBUTING.md` file with contribution guidelines
   - [ ] Create a `CHANGELOG.md` file with version history or at least a placeholder for the initial release

## Recommended Improvements (Nice to Have)

1. **Fix .gitignore**
   - [ ] Change Windows-style path `.github\instructions\codacy.instructions.md` to forward slashes: `.github/instructions/codacy.instructions.md`

2. **Improve Directory Structure**
   - [ ] Move the following documentation files from the root directory to the `docs` folder:
     - `Forbidden Library Blueprint v2.0 (Expanded).md`
     - `PHASE_3_IMPLEMENTATION_REPORT.md`
     - `PHASE_6_COMPLETION_REPORT.md`
     - `Project Checklist_ Forbidden Library - Ground-Up Implementation.md`
     - `THREAD_MIGRATION_PROMPT.md`
     - `Unified Agent Directives for Project_ Forbidden Library.md`

## Implementation Instructions

### For .env file:

```powershell
# Create .env.example with placeholder values
Copy-Item -Path "d:\Clones\GitHub\TechData\ForbiddenLibraryRework\.env" -Destination "d:\Clones\GitHub\TechData\ForbiddenLibraryRework\.env.example"
# Edit .env.example to replace sensitive values with placeholders
# Add .env to .gitignore if not already there
```

### For build artifacts:

```powershell
# These directories should already be in .gitignore
# Just make sure they're not included in the initial commit
```

### For missing documentation:

```powershell
# Create CONTRIBUTING.md and CHANGELOG.md files
New-Item -Path "d:\Clones\GitHub\TechData\ForbiddenLibraryRework\CONTRIBUTING.md" -ItemType File
New-Item -Path "d:\Clones\GitHub\TechData\ForbiddenLibraryRework\CHANGELOG.md" -ItemType File
```

### For .gitignore fix:

```powershell
# Edit .gitignore to fix Windows-style paths
```

### For moving documentation files:

```powershell
# Move documentation files to docs folder
Move-Item -Path "d:\Clones\GitHub\TechData\ForbiddenLibraryRework\Forbidden Library Blueprint v2.0 (Expanded).md" -Destination "d:\Clones\GitHub\TechData\ForbiddenLibraryRework\docs\"
Move-Item -Path "d:\Clones\GitHub\TechData\ForbiddenLibraryRework\PHASE_3_IMPLEMENTATION_REPORT.md" -Destination "d:\Clones\GitHub\TechData\ForbiddenLibraryRework\docs\"
Move-Item -Path "d:\Clones\GitHub\TechData\ForbiddenLibraryRework\PHASE_6_COMPLETION_REPORT.md" -Destination "d:\Clones\GitHub\TechData\ForbiddenLibraryRework\docs\"
Move-Item -Path "d:\Clones\GitHub\TechData\ForbiddenLibraryRework\Project Checklist_ Forbidden Library - Ground-Up Implementation.md" -Destination "d:\Clones\GitHub\TechData\ForbiddenLibraryRework\docs\"
Move-Item -Path "d:\Clones\GitHub\TechData\ForbiddenLibraryRework\THREAD_MIGRATION_PROMPT.md" -Destination "d:\Clones\GitHub\TechData\ForbiddenLibraryRework\docs\"
Move-Item -Path "d:\Clones\GitHub\TechData\ForbiddenLibraryRework\Unified Agent Directives for Project_ Forbidden Library.md" -Destination "d:\Clones\GitHub\TechData\ForbiddenLibraryRework\docs\"
```

## Final Checklist Before Initial Commit

- [ ] All critical issues resolved
- [ ] All required issues resolved
- [ ] All recommended improvements implemented (optional)
- [ ] This file deleted
