# **Unified Agent Directives for Project: Forbidden Library**

**Project Mandate:** To transition the "Forbidden Library" from its current state as a web-based prototype into a high-performance, privacy-centric, native desktop application, adhering strictly to the architectural principles outlined in the "Forbidden Library v1.3" design document.

**Prime Directive:** The singular focus of all agent activity is the successful refactoring and implementation of the application using the **Rust/Tauri/SvelteKit** stack. The existing Node.js/React prototype is to be treated as a **legacy feature reference only**, not as an architectural guide. All actions must contribute to the creation of the native application.

### **I. Universal Directives (Applicable to All Agents)**

- **Architectural Supremacy:** The Rust backend, Tauri application framework, and SvelteKit frontend constitute the non-negotiable architectural foundation. All generated code, configurations, tests, and documentation must be compatible with this stack.
- **Performance as a Mandate:** Every decision must be weighed against the project's core performance KPIs: sub-second launch time, 60 FPS UI responsiveness, and negligible idle resource consumption. Code that is merely functional but inefficient is unacceptable.
- **Privacy-Centric by Default:** All operations must assume a local-first, offline-capable model. The data layer is an encrypted SQLite database. No agent shall propose solutions that compromise user privacy by unnecessarily relying on external cloud services for core functionality.
- **Data Persistence is Law:** The MemStorage class of the prototype is deprecated and forbidden. All data operations must be designed to interact with the persistent database layer defined by the Drizzle ORM schema (shared/schema.ts) and implemented in the Rust core.
- **Inter-Agent Communication:** Agents must operate with an awareness of each other's protocols.
  - Codey, Jr. provides the contextual analysis of the new Rust codebase.
  - Albedo uses this context to scaffold the repository and CI/CD pipeline correctly.
  - Pandora uses the same context to generate relevant tests and validate the quality of the native application.

### **II. Albedo-Specific Directives (The Foundation)**

- **Repository Configuration:** Your primary task is to ensure the GitHub repository is configured for a native Rust/Tauri project.
  - The .gitignore file must include Rust and Tauri build artifacts (e.g., /src-tauri/target, Cargo.lock).
  - The README.md must be rewritten to provide build and run instructions for the Tauri application (e.g., cargo tauri dev, cargo tauri build), not the legacy npm scripts.
- **Continuous Integration (CI) Focus:** The GitHub Actions workflow you establish must be centered on the Rust toolchain.
  - The CI pipeline must include stages for cargo check, cargo clippy (linter), cargo fmt (formatter), and cargo test.
  - It must also include a stage to build the Tauri application itself to catch any integration issues.

### **III. Codey, Jr.-Specific Directives (The Intellect)**

- **Analysis Target:** Your analytical focus must be exclusively on the new Rust/Tauri/SvelteKit codebase. The prototype's /server and /client directories are to be used only as a reference for feature logic, not for structural analysis.
- **Knowledge Base Construction:** Your contextual knowledge graph must be built from:
  - Rust source files (.rs) and their module structure.
  - The Cargo.toml file to understand crates and dependencies.
  - The tauri.conf.json file to understand native application settings.
  - SvelteKit components (.svelte) for the frontend structure.
- **Identify Critical Bridge:** You must identify the Tauri IPC (Inter-Process Communication) layer—the set of Rust functions exposed to the frontend—as the single most critical architectural component. All analysis of data flow and functionality must pivot around this secure bridge.

### **IV. Pandora-Specific Directives (The Crucible)**

- **Testing Strategy:** Your entire testing strategy must be re-aligned for the native application.
  - You are to generate Rust unit and integration tests (#\[test]) for the backend logic.
  - You must select and configure an E2E testing framework compatible with Tauri (e.g., Playwright with Tauri driver, or a similar solution) to test the compiled application's UI and functionality.
- **Performance Validation:** You are explicitly tasked with validating the project's performance mandate.
  - Generate test protocols to measure application startup time.
  - Create E2E tests that scroll long lists and trigger complex UI interactions to detect any frame drops below the 60 FPS target.
- **Static Analysis:** Your static analysis protocol must include Rust-specific tooling.
  - You must integrate cargo clippy into your continuous quality assurance checks to enforce idiomatic Rust and catch common errors.
- **Security Focus:** Your security audits must focus on the native context.
  - Scrutinize all Tauri IPC command handlers for potential vulnerabilities, such as improper path handling or command injection risks when interacting with the filesystem or shell.
