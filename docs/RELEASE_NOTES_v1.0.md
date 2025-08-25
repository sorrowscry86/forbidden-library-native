# Forbidden Library v1.0.0 Release Notes

## Highlights

- First native release built with Rust/Tauri/SvelteKit
- Encrypted local storage with SQLCipher
- Sanctuary workspace integration and Personas/Grimoires foundation

## System Requirements

- Windows 10+ (x64)
- macOS 12+ (Apple Silicon/Intel)
- Linux (Ubuntu 22.04 LTS or compatible)

## Install & Uninstall

- Windows: Run the signed `.msi` or `.exe` installer
- macOS: Open the signed & notarized `.dmg`, drag app to Applications
- Linux: Install `.deb` with `sudo apt install ./ForbiddenLibrary-v1.0.0-linux-x64.deb` or run AppImage

## Integrity Verification

Verify SHA256 checksum:

```powershell
# Windows (PowerShell)
Get-FileHash .\ForbiddenLibrary-v1.0.0-windows-x64.msi -Algorithm SHA256
```

```bash
# macOS / Linux
shasum -a 256 ForbiddenLibrary-v1.0.0-macos-universal.dmg
sha256sum ForbiddenLibrary-v1.0.0-linux-x64.deb
```

## Known Issues

- Initial indexing on first launch may take a few seconds

## Changelog Summary

- Migrate to native architecture per mandate
- Implement baseline secure IPC and storage
