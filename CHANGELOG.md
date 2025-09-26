# Changelog

All notable changes to Forbidden Library will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

- Initial project structure with Tauri and SvelteKit
- Basic conversation UI with markdown rendering
- Local database integration with SQLite/SQLCipher
- File system access with secure permissions
- Cross-platform support (Windows, macOS, Linux)

### Changed

### Deprecated

### Removed

### Fixed

### Security

- Implemented secure database encryption with SQLCipher
- Added strict CSP policies in Tauri configuration

## [2.0.0] - 2025-08-22

### Added

- Complete rewrite using Rust/Tauri for backend and SvelteKit for frontend
- Privacy-first architecture with local-only processing
- Rich text rendering with Markdown, LaTeX, and syntax highlighting
- Secure file system integration with permission controls
- Multiple AI model provider support
- Voice interaction capabilities
- Encrypted local storage
- System tray integration
- Global shortcuts

### Changed

- Migrated from Electron to Tauri for improved performance and security
- Switched from React to SvelteKit for frontend
- Improved conversation management with thread organization

### Security

- Implemented end-to-end encryption for all stored data
- Added comprehensive permission system for file access
- Integrated Sentry for privacy-respecting error reporting

[Unreleased]: https://github.com/sorrowscry86/forbidden-library-native/compare/v2.0.0...HEAD
[2.0.0]: https://github.com/sorrowscry86/forbidden-library-native/releases/tag/v2.0.0
