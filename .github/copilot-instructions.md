---
description: AI rules derived by SpecStory from the project AI interaction history
globs: *
---

## PROJECT RULES & CODING STANDARDS

### GENERAL PRINCIPLES
*   Adhere to the principle of least astonishment. Code should behave in a way that is expected and easily understood by other developers.
*   Follow established naming conventions and project structure.
*   Write clean, modular, and well-documented code.
*   Prioritize code readability and maintainability.
*   All code must be reviewed before merging.

### CODE STYLE
*   Use consistent indentation (4 spaces).
*   Keep lines short (under 120 characters).
*   Use meaningful variable and function names.
*   Add comments to explain complex logic.
*   Follow language-specific coding style guides (e.g., PEP 8 for Python, Effective Java for Java).
*   Use `.editorconfig` to maintain consistent coding styles across the project.
*   Maintain both "Workflow-Update" style comments and standard comments where possible, as "Workflow-Update" style is more readable for human eyes.

### COMMIT MESSAGES
*   Use clear and concise commit messages.
*   Follow the conventional commits specification (e.g., `feat: add new feature`, `fix: correct a bug`).
*   Include a reference to the issue being addressed (e.g., `Fixes #123`).

## TECH STACK

### LANGUAGES
*   Rust (primary language for backend and core logic)
*   SvelteKit (primary language for frontend)
*   TypeScript (for frontend configuration and scripting)

### FRAMEWORKS & LIBRARIES
*   Tauri (for building native desktop applications)
*   Drizzle ORM (for database interactions)
*   SQLite (for local data persistence)
*   SQLCipher (for encrypted database storage)
*   Tailwind CSS (for frontend styling)
*   ESLint (for code linting)
*   Prettier (for code formatting)
*   Vitest (for frontend testing)
*   SQLCipher (for encrypted database storage)
*   rusqlcipher (potentially required crate for SQLCipher, check availability)
*   @tauri-apps/cli
*   @tauri-apps/api
*   @sveltejs/adapter-static
*   postcss
*   autoprefixer
*   vite
*   @tailwindcss/forms
*   @tailwindcss/typography
*   sentry-rust (for error tracking)

### TOOLS
*   VS Code (recommended IDE)
*   cargo (Rust package manager and build tool)
*   git (version control)
*   GitHub Copilot (AI code assistant)
*   Linear (Project Management)
*   Sentry
*   Claude Desktop (for MCP server configuration - see VOIDCAT RDC SPECIFIC RULES)

## AVAILABLE AI ASSISTANT TOOLS

### CORE DEVELOPMENT TOOLS
#### File & Workspace Operations
*   `create_directory`, `create_file`, `list_dir`, `read_file` - Basic file system operations
*   `replace_string_in_file`, `multi_replace_string_in_file` - Code editing (prefer multi_replace for efficiency)
*   `create_new_workspace`, `get_project_setup_info` - Project scaffolding and initialization
*   `get_changed_files` - Git status and diff information

#### Search & Navigation
*   `file_search` - Find files by glob patterns (exact filename matching)
*   `grep_search` - Fast text search with regex support (exact string/pattern matching)  
*   `semantic_search` - Natural language search across codebase (concept/meaning-based)
*   `get_search_view_results` - Access VS Code search view results
*   **Use Case Guide:** file_search for filenames, grep_search for exact text/patterns, semantic_search for concepts

#### Code Analysis & Testing  
*   `list_code_usages` - Find references, definitions, implementations of symbols
*   `test_search` - Find test files for source files and vice versa
*   `runTests`, `test_failure` - Execute and analyze test results
*   `get_errors` - Retrieve compile/lint errors from VS Code

#### Terminal & Task Execution
*   `run_in_terminal`, `get_terminal_output` - Execute shell commands with persistent sessions
*   `terminal_last_command`, `terminal_selection` - Access terminal state
*   `create_and_run_task`, `run_task`, `get_task_output` - VS Code task management
*   **Use Case Guide:** Use tasks for structured build/run operations, terminal for ad-hoc commands

### VS CODE INTEGRATION
*   `install_extension`, `vscode_searchExtensions_internal` - Extension management
*   `run_vscode_command` - Execute VS Code commands programmatically  
*   `get_vscode_api` - Access VS Code extension development documentation
*   `open_simple_browser` - Preview websites in VS Code's built-in browser

### NOTEBOOK DEVELOPMENT (JUPYTER)
*   `create_new_jupyter_notebook` - Generate new notebooks
*   `edit_notebook_file` - Modify notebook cells (insert/edit/delete)
*   `run_notebook_cell`, `read_notebook_cell_output` - Execute and read cell results
*   `copilot_getNotebookSummary` - Get notebook structure and cell metadata

### EXTERNAL SERVICE INTEGRATIONS

#### GitHub Integration (Comprehensive Suite)
*   **Repository Management:** Create, fork, branch, push multiple files
*   **Issue Management:** Create, list, filter, update, comment on issues
*   **Pull Request Management:** Create, review, merge, update PRs
*   **File Management:** Create/update files, read contents, list commits
*   **Search:** Find code, repositories, users across GitHub
*   **Activation Required:** Use `activate_github_*` tools to enable specific capabilities
*   **Recommendation:** Use the comprehensive GitHub tools instead of `github_repo`, as the former provides comprehensive repo/issue/PR/file management.

#### Linear Project Management
*   **Issue Management:** Create, update, retrieve, comment on Linear issues
*   **Project Management:** Create, update, manage Linear projects and labels
*   **Team Management:** Access team and user information
*   **Documentation:** Access Linear docs and help content
*   **Status/Cycle Management:** Manage issue statuses and development cycles
*   **Activation Required:** Use `activate_linear_*` tools to enable specific capabilities

#### Sentry Error Tracking  
*   **Issue Analysis:** AI-powered root cause analysis, detailed error information
*   **Project Management:** Create/update projects, teams, DSN management
*   **Event Management:** Search events, attachments, release tracking
*   **Documentation:** Access Sentry SDK docs and configuration guides
*   **Organization Management:** Multi-org and team coordination
*   **Trace Analysis:** Detailed performance trace analysis
*   **Activation Required:** Use `activate_sentry_*` tools to enable specific capabilities

### MCP SERVER INTEGRATIONS

#### Documentation & Knowledge
*   **Context7 Library Docs:** `mcp_context7_resolve-library-id`, `mcp_context7_get-library-docs` - Access up-to-date library documentation
*   **GitHub Repository Docs:** `activate_github_repository_tools` - Ask questions, read wikis, explore repo documentation  
*   **Knowledge Graph Management:** `activate_knowledge_graph_management_tools` - Create/manage entities, relations, observations

#### System Management
*   **Python Environment:** `activate_python_environment_tools` - Configure environments, install packages, get executable paths
*   **PowerShell Management:** `activate_powershell_management_tools` - Generate BigFix/Intune scripts, system info, secure command execution

### WEB CONTENT & RESEARCH
*   `fetch_webpage` - Retrieve and analyze web page content for research and documentation

### TOOL SELECTION GUIDELINES

#### When to Use Which Search Tool:
*   **File names/paths known:** `file_search` with glob patterns
*   **Exact text/code patterns:** `grep_search` with regex support  
*   **Conceptual/semantic queries:** `semantic_search` for meaning-based results
*   **VS Code search integration:** `get_search_view_results`

#### Terminal vs Tasks:
*   **Structured build/dev operations:** Use VS Code `tasks` (build, test, dev server)
*   **Ad-hoc commands/exploration:** Use `terminal` operations
*   **Background processes:** Both support background execution

#### File Editing Efficiency:
*   **Single edit:** `replace_string_in_file`
*   **Multiple edits:** `multi_replace_string_in_file` (preferred for efficiency)

#### MCP Tool Activation:
*   Many advanced tools require activation via `activate_*` functions before use
*   Activation tools unlock entire capability suites rather than individual functions

### IDENTIFIED TOOL REDUNDANCIES & OVERLAPS

#### Minimal True Redundancy:
*   `github_repo` vs comprehensive GitHub management suite - **Recommendation:** Use full GitHub suite for better capabilities. `github_repo` provides limited repository code search, while the full GitHub suite offers comprehensive repo/issue/PR/file management and should be used instead.

#### Complementary Tool Relationships:
*   **Search tools:** Each serves distinct use cases (filename vs content vs semantic)
*   **Execution tools:** Terminal for flexibility, tasks for structure
*   **External integrations:** Comprehensive suites work together, not redundant

### VS Code EXTENSIONS
*   svelte.svelte-vscode
*   rust-lang.rust-analyzer
*   tamasfe.even-better-toml
*   bradlc.vscode-tailwindcss
*   dbaeumer.vscode-eslint
*   esbenp.prettier-vscode
*   vadimcn.vscode-lldb

### VERSIONING
*   Use Semantic Versioning (SemVer).

## PROJECT DOCUMENTATION & CONTEXT SYSTEM

### REPOSITORY STRUCTURE
*   `src/`: Source code
*   `docs/`: Project documentation
*   `examples/`: Example usage
*   `tests/`: Automated tests
*   `README.md`: Project overview and setup instructions
*   `AI-Comms.md`: Central communication hub for AI agents
*   `CONTRIBUTING.md`: Guidelines for contributing to the project
*   `CHANGELOG.md`: Version tracking and change history
*   `THREAD_MIGRATION_PROMPT.md`: Used for storing a prompt to migrate the project to another AI agent.
*   `docs/linear/`: Drafts for Linear issues.
*   `docs/notion/`: Drafts for Notion pages.
*   `docs/linear/PHASE_5_Completion_Issues.md`: Draft Linear issues for Phase 5 completion.
*   `docs/linear/README.md`: Linear README.
*   `docs/notion/Phase5_Completion_Page.md`: Draft Notion page for Phase 5 completion.

### DOCUMENTATION STANDARDS
*   Use Markdown for all documentation.
*   Include code examples where appropriate.
*   Keep documentation up-to-date with code changes.
*   Document all public APIs.
*   Ensure proper blank lines around headings and lists in Markdown files to satisfy markdown lint rules.
*   Remove trailing spaces from Markdown files.
*   Ensure the first line of each markdown file is a heading.

### AI AGENT COMMUNICATION HUB
*   `AI-Comms.md` serves as the central communication hub for all AI agents working on the project.
*   It includes agent introductions, specializations, available tools, major project changes, and collaboration protocols.
*   All agents must register themselves and their capabilities in `AI-Comms.md`.
*   All major decisions and modifications must be documented in the change log within `AI-Comms.md`.

## WORKFLOW & RELEASE RULES

### BRANCHING STRATEGY
*   Use Gitflow or a similar branching model.
*   `main`: Stable, released code.
*   `develop`: Integration branch for ongoing development.
*   `feature/*`: Feature branches for new features.
*   `bugfix/*`: Bugfix branches for bug fixes.
*   Implement branch protection rules.

### PULL REQUESTS
*   Create pull requests for all code changes.
*   Assign reviewers to pull requests.
*   All pull requests must be approved before merging.
*   Address all comments and concerns raised during review.

### TESTING
*   Write unit tests for all code.
*   Run tests before submitting pull requests.
*   Ensure sufficient test coverage.

### RELEASE PROCESS
*   Create release branches from `develop`.
*   Update version numbers.
*   Tag releases in Git.
*   Publish releases to appropriate channels.

## DEBUGGING

### GENERAL DEBUGGING PRACTICES
*   AI assistant must start running things with timeouts.

### LOGGING
*   Use a consistent logging framework.
*   Log important events and errors.
*   Include sufficient context in log messages.

### DEBUGGING TOOLS
*   Use debuggers to step through code and inspect variables.
*   Use profiling tools to identify performance bottlenecks.

### ERROR HANDLING
*   Handle errors gracefully.
*   Provide informative error messages.
*   Avoid crashing the application.

### SYSTEMATIC PROBLEM-SOLVING PROTOCOL (Active)
1. Intake diagnostics
  - Use `get_errors` to collect current compile/lint errors, or consult the VS Code Problems panel.
  - For Rust: prefer `cargo check` via a task; for SvelteKit: `pnpm run check` and `pnpm run build`.
2. Get minimal context fast
  - Filename known → `file_search`.
  - Exact strings/symbols → `grep_search`.
  - Conceptual queries → `semantic_search`.
  - Only then `read_file` for focused sections (avoid over-reading).
3. Plan small, act immediately
  - Draft a 3–7 step plan mapping each requirement to a concrete change.
  - Prefer the smallest safe edit; preserve style and public APIs.
4. Implement with precision
  - Use `apply_patch` for edits; avoid unrelated reformatting.
  - Note 1–2 explicit assumptions if details are missing and proceed.
5. Validate with quality gates (green-before-done)
  - Build: Rust `cargo check`; Frontend `pnpm run build`.
  - Lint/Typecheck: `pnpm run check`; Rust `cargo clippy` (when configured).
  - Unit tests: Rust `cargo test`; Frontend `pnpm run test` (Vitest when present).
  - Smoke: run the Tauri dev app to sanity check key flows.
  - Report PASS/FAIL per gate and iterate up to three targeted fixes.
6. Document micro-changes
  - Update `.specstory/AI-Comms.md` with notable decisions when appropriate.

### Workspace Tasks (Forbidden Library)
The following VS Code tasks are available for quick, reproducible operations:
- `Install Node deps (pnpm)` → installs Node dependencies.
- `Reinstall Node deps (pnpm)` → reinstalls Node dependencies.
- `Typecheck Svelte (pnpm check)` → runs SvelteKit/TypeScript checks.
- `Typecheck Svelte (pnpm check) again` → re-runs checks after fixes.
- `Typecheck Svelte (pnpm check) final` → final verification.
- `Build Svelte (pnpm build)` → builds the SvelteKit app.
- `Build Svelte (pnpm build) after fixes` → rebuild after changes.
- `Run Tauri Dev` → starts the Tauri dev application.

Preferred order when stabilizing:
1) `Install Node deps (pnpm)` → 2) `Typecheck Svelte (pnpm check)` → 3) `Build Svelte (pnpm build)` → 4) `Run Tauri Dev`.

### Windows PowerShell notes
- Join commands with `;` (not `&&`).
- Set env vars with `$env:KEY = "value"` and reference as `$env:KEY`.
- When providing copyable commands, prefer one command per line.

### Rust/Tauri pitfall tip
- Tauri `State<T>` is injected as a parameter to `#[tauri::command]` functions; avoid constructing it manually (there is no `State::new`). If you encounter `no function or associated item named 'new' for 'tauri::State'`, refactor to accept `state: State<AppState>` as an argument instead of trying to create one.

## VOIDCAT RDC SPECIFIC RULES

### BRANDING
*   Maintain consistent VoidCat RDC branding in all materials.

### COMMUNICATION
*   Follow established communication protocols.
*   Maintain an authoritative and professional tone.
*   Maintain both "Workflow-Update" style comments and standard comments where possible, as "Workflow-Update" style is more readable for human eyes.

### QUALITY
*   Adhere to strict code excellence standards.
*   Perform self-assessments to ensure quality.
*   Excellence delivered as commanded.
*   VoidCat RDC Excellence Protocol Active (Contact: SorrowsCry86@voidcat.org | Support: CashApp $WykeveTF)

## FORBIDDEN LIBRARY RDC SPECIFIC RULES

### ARCHITECTURE
*   Transform Forbidden Library from web prototype to high-performance native desktop application using the Rust/Tauri/SvelteKit stack.
*   No legacy dependencies from the Node.js/React prototype should be used, except for reference.
*   All operations must go through the Drizzle ORM schema and Rust core.

### PERFORMANCE
*   Sub-second launch time.
*   60 FPS UI.
*   Negligible idle consumption.

### PRIVACY
*   Local-first application.
*   Offline-capable.
*   Encrypted SQLite database.
*   Local-first application.
*   Offline-capable.

### REPOSITORY CONFIGURATION
*   Configure GitHub repository for native Rust/Tauri project.
*   Implement proper `.gitignore` for Rust/Tauri build artifacts.
*   Rewrite `README.md` with Tauri-specific build instructions.
*   Initialize repository with descriptive naming.
*   Apply appropriate license (MIT default).

### CI/CD PIPELINE
*   Establish GitHub Actions workflow centered on Rust toolchain.
*   Include `cargo check`, `cargo clippy`, `cargo fmt`, `cargo test`.
*   Add Tauri application build stage for integration validation.

### CODE QUALITY & CONVENTION ENFORCEMENT
*   Integrate Rust/Tauri-appropriate linting (`cargo clippy`).
*   Configure code formatting (`cargo fmt`).
*   Generate `.editorconfig` for consistent coding styles.

### DOCUMENTATION & KNOWLEDGE SCAFFOLDING
*   Create `CONTRIBUTING.md` with Tauri-specific guidelines.
*   Establish `/docs` directory for comprehensive documentation.
*   Initialize `CHANGELOG.md` for version tracking.

### FOUNDATIONAL PHASE PROTOCOLS:

**Project Status:** Core modules and services are implemented and compiling; comprehensive testing framework is in place and validated.

**Next Step:** Pandora will now take over after successful completion of the dev environment and sentry integration, followed by cross-platform/user acceptance testing and release candidate preparation.

#### Phase 1: Repository Initialization & Scaffolding
*   ✅ Initialize repository with descriptive naming
*   ✅ Generate comprehensive .gitignore (Rust/Tauri artifacts)
*   ✅ Apply appropriate license (MIT default)
*   ✅ Create detailed README.md with Tauri build instructions
*   ✅ Establish develop/main branching strategy
*   ✅ Implement branch protection rules

#### Phase 2: Code Quality & Convention Enforcement
*   ✅ Integrate Rust/Tauri-appropriate linting (cargo clippy)
*   ✅ Configure code formatting (cargo fmt)
*   ✅ Establish CI/CD workflow with Rust toolchain focus
*   ✅ Generate .editorconfig for consistent coding styles

#### Phase 3: Documentation & Knowledge Scaffolding
*   Create CONTRIBUTING.md with Tauri-specific guidelines
*   Establish /docs directory for comprehensive documentation
*   Initialize CHANGELOG.md for version tracking

#### Phase 4: Core Implementation & Testing
*   ✅ Technology Stack Confirmation
*   ✅ Architectural Refactoring Plan
*   ✅ Repository Foundation & Development Environment
*   ✅ Establish New Development Environment
*   ✅ Build Rust Core Services
*   ✅ Establish Comprehensive Testing Suite

#### Phase 5: Frontend Migration & UI/UX Refinement
*   ✅ Frontend Migration to SvelteKit
*   Cross-platform and user acceptance testing
*   Preparing release candidates

#### Phase 6: Release Automation
*   Automate packaging, signing, and release workflows using GitHub Actions.
*   Generate signed multi-OS releases.
*   Manage signing keys securely.
*   Create release notes templates.
*   Update README with a Downloads section.
*   Implement a version bump script.

### EXTERNAL INTEGRATIONS
*   The AI assistant will synchronize the project checklist with Linear. This includes fetching teams, checking existing issues, and creating/updating them accordingly.
    *   This process involves:
        *   Fetching Linear workspace teams.
        *   Checking for existing issues.
        *   Creating or updating issues.
        *   Fetching the team’s issue statuses.
        *   Reading the full checklist file to extract tasks.
        *   Creating phase labels and a new Linear project.
        *   Batch-creating issues from the checklist with appropriate statuses and labels.
    *   When synchronizing with Linear, the AI assistant should aim to:
        *   Fetch Linear workspace teams to target the correct one.
        *   Create phase labels and a new Linear project.
        *   Batch-create issues from the checklist with appropriate statuses and labels to reflect the current plan.
        *   Create the remaining Linear issues for Phases 0–5 with correct statuses and labels.
    *   The AI assistant can also back-link Linear issue IDs into the project checklist markdown for traceability, or set up a tiny sync script to keep them in lockstep.
*   Sentry tools are active now

### RELEASE AUTOMATION (Phase 6)

*   Automate packaging, signing, and release workflows using GitHub Actions.
*   Generate signed multi-OS releases.
*   Manage signing keys securely.
*   Create release notes templates.
*   Update README with a Downloads section.
*   Implement a version bump script.
*   Configure Tauri config for bundling and signing.
*   Create draft RELEASE_NOTES_v1.0.md.
*   Added GitHub Actions workflow `release.yml`
  *   Triggers on tags (including RCs)
  *   Builds Tauri app on Windows/macOS/Linux
  *   Computes SHA256 checksums and uploads artifacts
  *   Creates a draft GitHub Release with collected assets
  *   Includes placeholders for:
    *   macOS: Developer ID cert import and notarization env
    *   Windows: PFX import
    *   Linux: GPG key import
  *   Note: Secrets must be added in your GitHub repo for full signing/notarization.
*   Enabled bundling in `tauri.conf.json` by toggling `"bundle.active": true`
*   Kept the schema valid by avoiding non-schema fields (signing-specific keys are handled via environment/secrets and build time, not the config)
*   Removed invalid `"theme": "auto"` error by using only supported keys
*   Added `POLICY_Key_Management.md` outlining issuance, storage, rotation, revocation, audit
*   Added `RELEASE_NOTES_TEMPLATE.md` with lint-safe formatting
*   Added `RELEASE_NOTES_v1.0.md` draft with system requirements and checksum instructions
*   Added `bump-version.mjs` to sync version across:
  *   `package.json`
  *   `tauri.conf.json` (`package.version`)
  *   `Cargo.toml` (`[workspace.package] version`)
*   Wired npm script:
  *   `version:bump`: `node scripts/bump-version.mjs`
*   Added “Downloads” section with checksum verification commands for PowerShell and bash/zsh

### SVELTEKIT CONFIGURATION
*   Configure SvelteKit for SPA static output (ssr=false, prerender=true) with adapter-static fallback.
  *   Update `svelte.config.js` to enable a SPA fallback:
    *   `adapter` now uses `fallback: 'index.html'` so dynamic routes are handled at runtime.
  *   Add `+layout.ts` with:
    *   `export const ssr = false;` to disable SSR (we’re running in a desktop webview)
    *   `export const prerender = true;` to keep static output generation
    *   These changes resolve adapter-static’s “dynamic routes” error for a Tauri app.

### ENVIRONMENT SETUP
*   Lightweight, reproducible version pinning is preferred over heavy virtualization.
    *   Rust: Use `rustup` and optionally a repo-level `rust-toolchain.toml` with `channel = "stable"` to keep contributors aligned.
    *   Node + pnpm: Pin the package manager via `"packageManager": "pnpm@8.15.0"` and declare engines `node >= 18`.
        *   Optionally add a `.node-version` file (e.g., `20.11.1`) if using tools like `nvm`/`asdf`.
        *   Optionally add a `.npmrc` with conservative settings for stricter reproducibility later (e.g., lockfile-enforced installs).

### TAURI AND SVELTEKIT INTEGRATION
*   Use `@sveltejs/adapter-static` for production; set build output to `build` to match Tauri’s `distDir`.
*   Keep Vite dev server stable for Tauri dev:
    *   In `vite.config.ts`: set `server: { strictPort: true }` to avoid port flapping.
    *   Tauri dev uses the running Vite server for `devPath`.
*   In `tauri.conf.json`:
    *   Dev: `build.beforeDevCommand` should run your frontend dev server. **Use `pnpm` instead of `npm` for this project.**
    *   Prod: `build.beforeBuildCommand` should build the SvelteKit app; `tauri.build.distDir` should point at `build`.
*   Use `@tauri-apps/api` for IPC in the SvelteKit layer:
    *   `import { invoke } from '@tauri-apps/api/tauri';`
    *   Create a thin TS service per domain (e.g., `src/lib/services/conversations.ts`) to centralize `invoke` calls and typing.
*   Ensure all UI IPC calls use timeouts and match Rust arg names. Use `invokeWithTimeout()` and align argument names to Rust (snake_case) and roles lowercase. Example:
    *   `create_conversation`: `{ title, persona_id }`
    *   `get_conversations`: `{ limit, offset }`
    *   `get_messages`: `{ conversation_id }`
    *   `add_message`: `{ conversation_id, role, content, tokens_used, model_used }`
    *   `send_ai_request`: `{ message, _persona_id, _conversation_id, model }` (keep underscores to match current rust signature)

### FRONTEND DEVELOPMENT
*   Implement `+layout.svelte` and wire `+page.svelte` to existing components (`ChatInterface.svelte`, `ConversationList.svelte`, `MessageBubble.svelte`).
*   Add `src/lib/stores/*` for conversations, messages, personas.
*   Add `src/lib/services/{conversations,personas,messages}.ts` that wrap `@tauri-apps/api` `invoke` calls with typed DTOs aligned to Rust models.
*   Create shared TS types in `src/lib/types.ts` mirroring Rust structs (ids, titles, timestamps).
*   Convert Svelte scripts to TypeScript where appropriate; resolve implicit any issues.

### VS CODE EXTENSION RECOMMENDATIONS
*   Paste the following to install/view the key extensions:
```vscode-extensions
svelte.svelte-vscode, rust-lang.rust-analyzer, tamasfe.even-better-toml, bradlc.vscode-tailwindcss, dbaeumer.vscode-eslint, esbenp.prettier-vscode, vadimcn.vscode-lldb
```
*   Avoid “TAUrine” (it’s for Samsung’s TAU, not Tauri).
*   The above set covers SvelteKit, Rust, TOML, Tailwind, lint/format, and Rust debugging.

---
description: Svelte best practices and patterns for modern web applications
globs: **/*.svelte, src/**/*.ts, src/**/*.js
---

# Svelte Best Practices

## Component Structure
- Keep components small and focused
- Use proper TypeScript integration
- Implement proper props typing
- Use proper event dispatching
- Keep markup clean and readable
- Use proper slot implementation

## Reactivity
- Use proper reactive declarations
- Implement proper stores
- Use proper reactive statements
- Handle derived values properly
- Use proper lifecycle functions
- Implement proper bindings

## State Management
- Use proper Svelte stores
- Keep stores modular
- Use proper derived stores
- Implement proper actions
- Handle async state properly
- Use proper store subscriptions

## Performance
- Use proper component lazy loading
- Implement proper transitions
- Use proper animations
- Avoid unnecessary reactivity
- Use proper event forwarding
- Implement proper key blocks

## Routing
- Use SvelteKit for routing
- Implement proper layouts
- Use proper route parameters
- Handle loading states properly
- Implement proper error pages
- Use proper navigation methods

## Forms
- Use proper form bindings
- Implement proper validation
- Handle form submission properly
- Show proper loading states
- Use proper error handling
- Implement proper form reset

## TypeScript Integration
- Use proper component types
- Implement proper prop types
- Use proper event types
- Handle proper type inference
- Use proper store types
- Implement proper action types

## Testing
- Write proper unit tests
- Implement proper component tests
- Use proper testing libraries
- Test stores properly
- Implement proper mocking
- Test async operations

## Best Practices
- Follow Svelte style guide
- Use proper naming conventions
- Keep components organized
- Implement proper error handling
- Use proper event handling
- Document complex logic

## Build and Tooling
- Use Vite for development
- Configure proper build setup
- Use proper environment variables
- Implement proper code splitting
- Use proper asset handling
- Configure proper optimization

### PLATFORM PREREQUISITES
*   Windows prerequisites (Tauri):
    *   Install Microsoft C++ Build Tools (Desktop development with C++).
    *   Ensure WebView2 Runtime is installed (required by Tauri on Windows).
*   macOS:
    *   Xcode Command Line Tools (`xcode-select --install`).
    *   Apple Developer certs for code signing later.
*   Linux:
    *   Common packages (examples): `libgtk-3-dev libwebkit2gtk-4.1-dev libayatana-appindicator3-dev librsvg2-dev` (exact names vary by distro).

### NODE TOOLING
*   Node-side tooling for SvelteKit + Tailwind + Tauri (if any are missing):
```powershell
npm i -D @tauri-apps/cli @tauri-apps/api @sveltejs/adapter-static eslint prettier tailwindcss postcss autoprefixer typescript vite
```

### FRONTEND PERFORMANCE TIPS
*   Virtualize long lists (conversation lists, file trees) with a lightweight virtual scroller (e.g., svelte-virtual if needed).
*   Defer heavy work to Rust via IPC and keep UI state minimal/derived.
*   Batch state updates in Svelte stores; avoid frequent top-level store writes in tight loops.

### PRE-COMMIT HOOKS
*   Implement pre-commit hooks to automate code cleanup and standardization before commits.
*   Run the directives in `PRE_COMMIT_CLEANUP.md` to create `.env.example`, update `.gitignore`, create missing docs, and move specified files into `docs/`. This file should then be removed.
