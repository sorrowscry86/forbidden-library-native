# Forbidden Library v2.0.0 Release Notes

## Highlights

- **Complete Native Rewrite**: The Forbidden Library has been rebuilt from the ground up using a Rust/Tauri/SvelteKit stack, replacing the original Node.js/React prototype. This provides a massive leap in performance, security, and stability.
- **Sub-Second Performance**: Experience a fluid, responsive UI with a sub-second application launch time and a consistent 60 FPS, even with large amounts of data.
- **Privacy-First Architecture**: All your data is stored locally in an encrypted SQLite database. There is no telemetry, and all OS-level interactions require explicit user consent.
- **Comprehensive Feature Set**: Includes all the features from the original prototype, now implemented natively:
  - **Conversational Interface**: Full Markdown, LaTeX, and code highlighting.
  - **The Sanctuary**: Integrated file system explorer and terminal.
  - **Extensible Architecture**: Support for Grimoires (MCP servers) and Personas.
  - **Project Management**: Built-in tools for tracking tasks and projects.
- **Sentry Monitoring**: Integrated error and performance monitoring with Sentry to ensure stability and catch issues early (opt-in for crash reporting).

## System Requirements

- Windows 10+ (x64)
- macOS 12+ (Apple Silicon/Intel)
- Linux (Ubuntu 22.04 LTS or compatible)

## Install & Uninstall

- Windows: Run the signed `.msi` or `.exe` installer.
- macOS: Open the signed & notarized `.dmg`, drag the app to your Applications folder.
- Linux: Install the `.deb` package with `sudo apt install ./ForbiddenLibrary-v2.0.0-linux-x64.deb` or run the AppImage.

## Integrity Verification

- Verify SHA256 checksum:

```bash
# Windows (PowerShell)
Get-FileHash .\ForbiddenLibrary-v2.0.0-windows-x64.msi -Algorithm SHA256

# macOS / Linux
shasum -a 256 ForbiddenLibrary-v2.0.0-macos-universal.dmg
sha256sum ForbiddenLibrary-v2.0.0-linux-x64.deb
```

## Known Issues

- No known issues at this time.

## Changelog Summary

- **Added**: Complete Rust/Tauri backend with all core services.
- **Added**: SvelteKit frontend with a focus on performance and UX.
- **Added**: Encrypted SQLite database with SQLCipher.
- **Added**: Comprehensive testing suite with unit, integration, and performance tests.
- **Added**: CI/CD pipeline for automated builds and quality checks across all platforms.
- **Added**: Sentry integration for backend and frontend monitoring.
- **Fixed**: Numerous performance and stability issues from the original prototype.
- **Changed**: Migrated from a web-based architecture to a native desktop application.
