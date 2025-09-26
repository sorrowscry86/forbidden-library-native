---
description: Repository Information Overview
alwaysApply: true
---

# Forbidden Library Information

## Summary

Privacy-first, high-performance desktop application for interacting with powerful language models. Built with Rust/Tauri for the backend and SvelteKit for the frontend, providing unparalleled performance, security, and deep OS integration. Currently at version 2.0.0.

## Structure

- **src/**: SvelteKit frontend source with components, routes, and utilities
- **src-tauri/**: Rust backend with database, services, and Tauri integration
- **static/**: Static assets for the application
- **docs/**: Project documentation including security policies and release notes
- **.github/**: GitHub Actions workflows for CI/CD, testing, and releases
- **.agentic-tools-mcp/**: MCP integration for AI agent tools
- **.specstory/**: Project history and AI communication logs
- **build/**: Production build output directory

## Language & Runtime

**Languages**: Rust, TypeScript, JavaScript
**Rust Version**: 1.70+
**Node.js Version**: 18+
**Package Manager**: pnpm 8.15.0+
**Build System**: Tauri 1.5+, Vite 5.0.3+

## Dependencies

### Frontend Dependencies

**Main Dependencies**:

- SvelteKit 2.0.0+ with adapter-static
- Tauri API 1.5.1+ for native integration
- Marked 11.1.1 (Markdown rendering)
- KaTeX 0.16.9 (LaTeX rendering)
- Highlight.js 11.9.0 (Code syntax highlighting)
- Lucide-svelte 0.303.0 (Icon library)
- Tailwind CSS 3.3.0+ with typography and forms plugins

**Development Dependencies**:

- TypeScript 5.0.0+ with ESLint integration
- Prettier 3.1.1+ with Svelte plugin
- Vitest 1.2.0+ for frontend testing

### Backend Dependencies

**Main Dependencies**:

- Tauri 1.5+ with extensive feature set (file system, dialogs, clipboard)
- Tokio 1.35+ with full async runtime features
- Rusqlite 0.30+ with bundled SQLite and chrono support
- Rusqlcipher 0.14+ for database encryption
- Reqwest 0.11+ with JSON and streaming support
- Ring 0.17+ for cryptographic operations
- Tracing ecosystem for structured logging

**Development Dependencies**:

- Mockall 0.12+ for test mocking
- Criterion 0.5+ for performance benchmarking
- Proptest 1.3+ for property-based testing

## Build & Installation

```bash
# Install dependencies
pnpm install

# Development mode
pnpm tauri dev
# OR
cargo tauri dev

# Production build
pnpm tauri build
# OR
cargo tauri build

# Version bumping
pnpm version:bump
```

## Testing

**Frontend Framework**: Vitest
**Test Location**: src/**/\*.{test,spec}.{js,ts}
**Configuration**: vitest.config.js
**Backend Testing\*\*:

- Unit tests in each module
- Integration tests in src-tauri/tests/
- Security audit tests in src-tauri/tests/security_audit.rs
- Performance benchmarks in src-tauri/benches/
  **Run Command**:

```bash
# Frontend tests
pnpm test
pnpm test:watch
pnpm test:ui

# Backend tests
cargo test
cargo test --test integration_tests
cargo test --test security_audit
cargo bench
```

## CI/CD Pipeline

**Workflows**:

- CI/CD Pipeline (.github/workflows/ci.yml)
- Comprehensive Testing (.github/workflows/test.yml)
- Release Automation (.github/workflows/release.yml)

**Quality Checks**:

- Rust formatting (cargo fmt)
- Rust linting (cargo clippy)
- TypeScript checking (svelte-check)
- ESLint and Prettier validation
- Security audits (cargo audit, npm audit)

**Testing Matrix**:

- Multiple OS: Windows, macOS, Linux
- Multiple Rust versions: stable, 1.70
- Performance benchmarks with criterion
- Code coverage with cargo-llvm-cov

## Project Components

### Frontend (SvelteKit)

- **Configuration**: svelte.config.js, vite.config.js, tailwind.config.js
- **Routing**: File-based routing in src/routes/
- **Components**: Organized in src/lib/components/
- **Services**: API communication in src/lib/services/
- **Type Definitions**: TypeScript types in src/lib/types/

### Backend (Rust/Tauri)

- **Entry Point**: src-tauri/src/main.rs and src-tauri/src/lib.rs
- **Modules**:
  - commands.rs: Tauri IPC commands
  - database/: SQLite database management with encryption
  - models.rs: Core data structures
  - services/: Business logic and external integrations
- **Configuration**: tauri.conf.json with strict security policies
- **Build Configuration**: src-tauri/build.rs

### Security Features

- CSP policies defined in tauri.conf.json
- Explicit filesystem permission scoping
- SQLCipher encryption for database
- Comprehensive security testing
- Secure IPC between frontend and backend
- No telemetry or data collection

### Application Features

- Conversational interface with rich text rendering
- File system integration with secure access controls
- MCP (Model Context Protocol) integration
- Multiple AI model provider support
- Voice interaction capabilities
- Encrypted local storage
- System tray integration
- Global shortcuts
