# Forbidden Library Deployment Guide

Complete guide for building, packaging, and deploying the Forbidden Library across all platforms.

## Table of Contents

1. [Prerequisites](#prerequisites)
2. [Build Configuration](#build-configuration)
3. [Development Builds](#development-builds)
4. [Production Builds](#production-builds)
5. [Platform-Specific Builds](#platform-specific-builds)
6. [Code Signing](#code-signing)
7. [Distribution](#distribution)
8. [Updates and Versioning](#updates-and-versioning)
9. [CI/CD Pipeline](#cicd-pipeline)
10. [Security Considerations](#security-considerations)

---

## Prerequisites

### Required Tools

| Tool | Minimum Version | Purpose |
|------|-----------------|---------|
| Node.js | 18.0.0 | Frontend build |
| pnpm | 8.0.0 | Package management |
| Rust | 1.70.0 | Backend compilation |
| Tauri CLI | 1.5.0 | App bundling |

### Platform-Specific Requirements

**macOS:**
- Xcode Command Line Tools
- Apple Developer Account (for distribution)

**Windows:**
- Visual Studio Build Tools 2019+
- Code signing certificate (for distribution)
- WiX Toolset 3.11+ (for MSI installer)

**Linux:**
- build-essential
- webkit2gtk development files
- AppImage tools (for AppImage)

---

## Build Configuration

### Environment Variables

Create `.env` files for different environments:

**.env.development:**
```bash
VITE_API_URL=http://localhost:1420
VITE_ENVIRONMENT=development
VITE_LOG_LEVEL=debug
TAURI_DEBUG=true
```

**.env.production:**
```bash
VITE_API_URL=https://api.forbidden-library.app
VITE_ENVIRONMENT=production
VITE_LOG_LEVEL=info
TAURI_DEBUG=false
```

### Tauri Configuration

**src-tauri/tauri.conf.json:**

```json
{
  "build": {
    "beforeDevCommand": "pnpm dev",
    "beforeBuildCommand": "pnpm build",
    "devPath": "http://localhost:5173",
    "distDir": "../build",
    "withGlobalTauri": false
  },
  "package": {
    "productName": "Forbidden Library",
    "version": "1.0.0"
  },
  "tauri": {
    "allowlist": {
      "all": false,
      "shell": {
        "all": false,
        "open": true
      },
      "dialog": {
        "all": true
      },
      "fs": {
        "all": true,
        "scope": ["$APPDATA/*", "$RESOURCE/*"]
      }
    },
    "bundle": {
      "active": true,
      "targets": "all",
      "identifier": "com.voidcat.forbidden-library",
      "icon": [
        "icons/32x32.png",
        "icons/128x128.png",
        "icons/128x128@2x.png",
        "icons/icon.icns",
        "icons/icon.ico"
      ]
    },
    "security": {
      "csp": "default-src 'self'; img-src 'self' data: https:; script-src 'self' 'unsafe-inline'"
    },
    "windows": [
      {
        "title": "Forbidden Library",
        "width": 1200,
        "height": 800,
        "resizable": true,
        "fullscreen": false,
        "center": true
      }
    ]
  }
}
```

### Version Management

**package.json:**
```json
{
  "name": "forbidden-library",
  "version": "1.0.0",
  "scripts": {
    "version:bump": "pnpm version patch && cd src-tauri && cargo bump patch",
    "version:minor": "pnpm version minor && cd src-tauri && cargo bump minor",
    "version:major": "pnpm version major && cd src-tauri && cargo bump major"
  }
}
```

---

## Development Builds

### Quick Development Build

```bash
# Start development server
pnpm dev

# Or with Tauri
pnpm tauri dev
```

### Debug Build with Logging

```bash
# Enable Rust logging
RUST_LOG=debug pnpm tauri dev

# Enable frontend logging
VITE_LOG_LEVEL=debug pnpm dev
```

### Hot Reload Configuration

**vite.config.ts:**
```typescript
export default defineConfig({
  server: {
    port: 5173,
    strictPort: true,
    watch: {
      ignored: ['**/src-tauri/**']
    }
  },
  clearScreen: false,
  envPrefix: ['VITE_', 'TAURI_'],
});
```

---

## Production Builds

### Complete Build Process

```bash
#!/bin/bash
# build.sh - Complete production build script

set -e  # Exit on error

echo "🧹 Cleaning previous builds..."
rm -rf build dist src-tauri/target/release/bundle
pnpm clean

echo "📦 Installing dependencies..."
pnpm install --frozen-lockfile

echo "🔍 Running tests..."
pnpm test
cd src-tauri && cargo test --release && cd ..

echo "🏗️  Building frontend..."
pnpm build

echo "🦀 Building backend..."
cd src-tauri
cargo build --release
cd ..

echo "📱 Creating bundles..."
pnpm tauri build

echo "✅ Build complete!"
echo "📦 Bundles location: src-tauri/target/release/bundle/"
```

### Optimization Flags

**Cargo.toml:**
```toml
[profile.release]
opt-level = 3
lto = true
codegen-units = 1
panic = "abort"
strip = true

[profile.release.package."*"]
opt-level = 3
```

### Build Verification

```bash
# Check binary size
ls -lh src-tauri/target/release/forbidden-library*

# Verify dependencies
ldd target/release/forbidden-library  # Linux
otool -L target/release/forbidden-library  # macOS

# Test the release build
./target/release/forbidden-library
```

---

## Platform-Specific Builds

### macOS

#### Universal Binary (Apple Silicon + Intel)

```bash
# Install targets
rustup target add aarch64-apple-darwin
rustup target add x86_64-apple-darwin

# Build for both architectures
cargo build --release --target aarch64-apple-darwin
cargo build --release --target x86_64-apple-darwin

# Create universal binary
lipo -create \
  target/aarch64-apple-darwin/release/forbidden-library \
  target/x86_64-apple-darwin/release/forbidden-library \
  -output target/release/forbidden-library-universal

# Bundle as .app
pnpm tauri build
```

#### DMG Configuration

**tauri.conf.json:**
```json
{
  "tauri": {
    "bundle": {
      "macOS": {
        "frameworks": [],
        "minimumSystemVersion": "10.13",
        "exceptionDomain": "",
        "signingIdentity": "Developer ID Application: Your Name (TEAM_ID)",
        "entitlements": "entitlements.plist"
      },
      "dmg": {
        "background": "dmg-background.png",
        "windowSize": {
          "width": 600,
          "height": 400
        }
      }
    }
  }
}
```

**entitlements.plist:**
```xml
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
  <key>com.apple.security.cs.allow-jit</key>
  <true/>
  <key>com.apple.security.cs.allow-unsigned-executable-memory</key>
  <true/>
  <key>com.apple.security.cs.disable-library-validation</key>
  <true/>
  <key>com.apple.security.keychain-access-groups</key>
  <array>
    <string>$(AppIdentifierPrefix)com.voidcat.forbidden-library</string>
  </array>
</dict>
</plist>
```

#### Build Script

```bash
#!/bin/bash
# build-macos.sh

# Clean
cargo clean
pnpm clean

# Build
pnpm install
pnpm tauri build -- --target universal-apple-darwin

# Sign
codesign --force --deep --sign "Developer ID Application: Your Name" \
  src-tauri/target/release/bundle/macos/Forbidden\ Library.app

# Verify
codesign --verify --verbose src-tauri/target/release/bundle/macos/Forbidden\ Library.app
spctl --assess --verbose src-tauri/target/release/bundle/macos/Forbidden\ Library.app

# Create DMG
pnpm tauri build --target dmg
```

### Windows

#### MSI Installer

**tauri.conf.json:**
```json
{
  "tauri": {
    "bundle": {
      "windows": {
        "certificateThumbprint": null,
        "digestAlgorithm": "sha256",
        "timestampUrl": "http://timestamp.digicert.com",
        "wix": {
          "language": "en-US",
          "template": "wix/main.wxs",
          "fragmentPaths": ["wix/fragment.wxs"],
          "componentGroupRefs": ["CustomComponents"],
          "componentRefs": ["RegistryEntries"],
          "featureGroupRefs": ["MainFeature"],
          "featureRefs": [],
          "mergeRefs": []
        },
        "webviewInstallMode": {
          "type": "downloadBootstrapper"
        }
      }
    }
  }
}
```

#### Build Script

```powershell
# build-windows.ps1

# Clean
cargo clean
Remove-Item -Recurse -Force build, dist -ErrorAction SilentlyContinue

# Build
pnpm install
pnpm build
cargo build --release

# Create bundle
pnpm tauri build

# Sign (requires certificate)
# signtool sign /f certificate.pfx /p password /tr http://timestamp.digicert.com /td sha256 /fd sha256 `
#   src-tauri/target/release/Forbidden Library.exe

Write-Host "✅ Build complete! Check src-tauri/target/release/bundle/"
```

### Linux

#### AppImage

```bash
#!/bin/bash
# build-linux-appimage.sh

# Install AppImage tools
wget https://github.com/AppImage/AppImageKit/releases/download/continuous/appimagetool-x86_64.AppImage
chmod +x appimagetool-x86_64.AppImage

# Build
pnpm install
pnpm tauri build

# AppImage is automatically created by Tauri
ls -lh src-tauri/target/release/bundle/appimage/
```

#### DEB Package

**tauri.conf.json:**
```json
{
  "tauri": {
    "bundle": {
      "deb": {
        "depends": [
          "libwebkit2gtk-4.0-37",
          "libgtk-3-0",
          "libayatana-appindicator3-1"
        ],
        "files": {
          "/usr/share/applications/forbidden-library.desktop": "forbidden-library.desktop",
          "/usr/share/pixmaps/forbidden-library.png": "icons/128x128.png"
        }
      }
    }
  }
}
```

**forbidden-library.desktop:**
```ini
[Desktop Entry]
Name=Forbidden Library
Comment=AI conversation manager
Exec=forbidden-library
Icon=forbidden-library
Type=Application
Categories=Utility;Office;
Terminal=false
```

#### Build Script

```bash
#!/bin/bash
# build-linux.sh

# Build DEB and AppImage
pnpm tauri build --bundles deb,appimage

# Verify DEB
dpkg -I src-tauri/target/release/bundle/deb/*.deb

echo "✅ Build complete!"
echo "DEB: src-tauri/target/release/bundle/deb/"
echo "AppImage: src-tauri/target/release/bundle/appimage/"
```

---

## Code Signing

### macOS Code Signing

#### Development Signing

```bash
# List available identities
security find-identity -v -p codesigning

# Sign app
codesign --force --deep --sign "Developer ID Application: Your Name (TEAM_ID)" \
  --options runtime \
  --entitlements entitlements.plist \
  Forbidden\ Library.app

# Verify
codesign --verify --deep --strict --verbose=2 Forbidden\ Library.app
```

#### Notarization

```bash
#!/bin/bash
# notarize-macos.sh

APP_PATH="src-tauri/target/release/bundle/macos/Forbidden Library.app"
ZIP_PATH="forbidden-library.zip"

# Create zip for notarization
ditto -c -k --keepParent "$APP_PATH" "$ZIP_PATH"

# Upload for notarization
xcrun notarytool submit "$ZIP_PATH" \
  --apple-id "your-email@example.com" \
  --team-id "TEAM_ID" \
  --password "app-specific-password" \
  --wait

# Staple notarization ticket
xcrun stapler staple "$APP_PATH"

# Verify
spctl --assess --verbose=4 --type execute "$APP_PATH"
```

### Windows Code Signing

```powershell
# sign-windows.ps1

$CERT_PATH = "certificate.pfx"
$CERT_PASSWORD = "your-password"
$EXE_PATH = "src-tauri\target\release\Forbidden Library.exe"
$MSI_PATH = "src-tauri\target\release\bundle\msi\Forbidden Library.msi"

# Sign executable
signtool sign /f $CERT_PATH /p $CERT_PASSWORD `
  /tr http://timestamp.digicert.com /td sha256 /fd sha256 `
  /d "Forbidden Library" `
  $EXE_PATH

# Sign installer
signtool sign /f $CERT_PATH /p $CERT_PASSWORD `
  /tr http://timestamp.digicert.com /td sha256 /fd sha256 `
  /d "Forbidden Library Installer" `
  $MSI_PATH

# Verify
signtool verify /pa /all $EXE_PATH
signtool verify /pa /all $MSI_PATH
```

---

## Distribution

### GitHub Releases

#### Creating a Release

```bash
#!/bin/bash
# create-release.sh

VERSION=$(node -p "require('./package.json').version")
TAG="v$VERSION"

# Create tag
git tag -a "$TAG" -m "Release $VERSION"
git push origin "$TAG"

# Create release with GitHub CLI
gh release create "$TAG" \
  --title "Forbidden Library $VERSION" \
  --notes-file CHANGELOG.md \
  src-tauri/target/release/bundle/macos/*.dmg \
  src-tauri/target/release/bundle/msi/*.msi \
  src-tauri/target/release/bundle/deb/*.deb \
  src-tauri/target/release/bundle/appimage/*.AppImage
```

#### Automated Release with GitHub Actions

**.github/workflows/release.yml:**
```yaml
name: Release

on:
  push:
    tags:
      - 'v*'

jobs:
  release:
    strategy:
      matrix:
        platform: [macos-latest, ubuntu-latest, windows-latest]

    runs-on: ${{ matrix.platform }}

    steps:
      - uses: actions/checkout@v3

      - name: Setup Node.js
        uses: actions/setup-node@v3
        with:
          node-version: '18'

      - name: Setup Rust
        uses: actions-rs/toolchain@v1
        with:
          toolchain: stable

      - name: Install pnpm
        run: npm install -g pnpm

      - name: Install dependencies
        run: pnpm install

      - name: Build
        run: pnpm tauri build

      - name: Upload Release Assets
        uses: softprops/action-gh-release@v1
        with:
          files: src-tauri/target/release/bundle/**/*
        env:
          GITHUB_TOKEN: ${{ secrets.GITHUB_TOKEN }}
```

### Update Server

#### Tauri Updater Configuration

**tauri.conf.json:**
```json
{
  "tauri": {
    "updater": {
      "active": true,
      "endpoints": [
        "https://releases.forbidden-library.app/{{target}}/{{current_version}}"
      ],
      "dialog": true,
      "pubkey": "YOUR_PUBLIC_KEY_HERE"
    }
  }
}
```

#### Update Manifest

**latest.json:**
```json
{
  "version": "1.0.1",
  "notes": "Bug fixes and performance improvements",
  "pub_date": "2024-01-15T00:00:00Z",
  "platforms": {
    "darwin-x86_64": {
      "signature": "signature_here",
      "url": "https://releases.forbidden-library.app/macos/Forbidden-Library-1.0.1.dmg"
    },
    "darwin-aarch64": {
      "signature": "signature_here",
      "url": "https://releases.forbidden-library.app/macos/Forbidden-Library-1.0.1-arm64.dmg"
    },
    "windows-x86_64": {
      "signature": "signature_here",
      "url": "https://releases.forbidden-library.app/windows/Forbidden-Library-1.0.1.msi"
    },
    "linux-x86_64": {
      "signature": "signature_here",
      "url": "https://releases.forbidden-library.app/linux/Forbidden-Library-1.0.1.AppImage"
    }
  }
}
```

---

## Updates and Versioning

### Semantic Versioning

Follow [SemVer](https://semver.org/):

- **Major (1.0.0)**: Breaking changes
- **Minor (0.1.0)**: New features, backward compatible
- **Patch (0.0.1)**: Bug fixes

### Version Bump Script

```bash
#!/bin/bash
# bump-version.sh

TYPE=$1  # major, minor, or patch

if [ -z "$TYPE" ]; then
  echo "Usage: ./bump-version.sh [major|minor|patch]"
  exit 1
fi

# Update package.json
pnpm version $TYPE --no-git-tag-version

# Update Cargo.toml
cd src-tauri
NEW_VERSION=$(node -p "require('../package.json').version")
sed -i '' "s/^version = .*/version = \"$NEW_VERSION\"/" Cargo.toml
cd ..

# Update tauri.conf.json
sed -i '' "s/\"version\": \".*\"/\"version\": \"$NEW_VERSION\"/" src-tauri/tauri.conf.json

echo "✅ Bumped version to $NEW_VERSION"
echo "📝 Update CHANGELOG.md with changes"
echo "🏷️  Create git tag: git tag -a v$NEW_VERSION -m \"Release v$NEW_VERSION\""
```

### Changelog Generation

```bash
# Generate changelog from git commits
git log --pretty=format:"- %s (%h)" v1.0.0..HEAD > CHANGELOG.md

# Or use conventional-changelog
pnpm add -D conventional-changelog-cli
pnpm exec conventional-changelog -p angular -i CHANGELOG.md -s
```

---

## CI/CD Pipeline

### GitHub Actions Complete Pipeline

**.github/workflows/ci-cd.yml:**
```yaml
name: CI/CD

on:
  push:
    branches: [main, develop]
  pull_request:
    branches: [main]

jobs:
  test:
    runs-on: ${{ matrix.os }}
    strategy:
      matrix:
        os: [ubuntu-latest, macos-latest, windows-latest]

    steps:
      - uses: actions/checkout@v3

      - name: Setup Node.js
        uses: actions/setup-node@v3
        with:
          node-version: '18'
          cache: 'pnpm'

      - name: Setup Rust
        uses: actions-rs/toolchain@v1
        with:
          toolchain: stable
          override: true

      - name: Cache Cargo
        uses: actions/cache@v3
        with:
          path: |
            ~/.cargo/bin/
            ~/.cargo/registry/index/
            ~/.cargo/registry/cache/
            ~/.cargo/git/db/
            src-tauri/target/
          key: ${{ runner.os }}-cargo-${{ hashFiles('**/Cargo.lock') }}

      - name: Install dependencies
        run: |
          npm install -g pnpm
          pnpm install

      - name: Run frontend tests
        run: pnpm test

      - name: Run backend tests
        run: cd src-tauri && cargo test

      - name: Build
        run: pnpm tauri build

      - name: Upload artifacts
        uses: actions/upload-artifact@v3
        with:
          name: ${{ matrix.os }}-bundle
          path: src-tauri/target/release/bundle/

  deploy:
    needs: test
    if: github.ref == 'refs/heads/main'
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3

      - name: Deploy to production
        run: echo "Deploy steps here"
```

---

## Security Considerations

### Pre-Release Security Checklist

- [ ] All dependencies updated
- [ ] Security audit passed: `pnpm audit`
- [ ] Rust security audit: `cargo audit`
- [ ] No hardcoded secrets
- [ ] API keys stored in keychain
- [ ] HTTPS enforced
- [ ] CSP properly configured
- [ ] Input validation in place
- [ ] Code signed (platform-specific)
- [ ] Notarized (macOS)
- [ ] Virus scan passed (Windows)

### Security Audits

```bash
# Frontend security audit
pnpm audit
pnpm audit --fix  # Auto-fix if possible

# Backend security audit
cargo install cargo-audit
cargo audit

# Check for outdated dependencies
pnpm outdated
cargo outdated
```

### Secrets Management

**.env.example:**
```bash
# Copy this file to .env and fill in actual values
# NEVER commit .env to version control

VITE_API_URL=https://api.example.com
TAURI_SIGNING_PRIVATE_KEY=path/to/private/key
APPLE_ID=your-email@example.com
APPLE_TEAM_ID=YOUR_TEAM_ID
WINDOWS_CERT_PASSWORD=your-password
```

**.gitignore:**
```
.env
.env.local
*.pfx
*.p12
*.pem
```

---

## Deployment Checklist

### Pre-Deployment

- [ ] Version bumped
- [ ] CHANGELOG updated
- [ ] All tests passing
- [ ] Security audit passed
- [ ] Code reviewed
- [ ] Documentation updated
- [ ] Release notes prepared

### Build

- [ ] Clean build completed
- [ ] All platforms built
- [ ] Bundles created
- [ ] Code signed
- [ ] Notarized (macOS)
- [ ] Installers tested

### Release

- [ ] Git tag created
- [ ] GitHub release created
- [ ] Binaries uploaded
- [ ] Update manifest updated
- [ ] Release announced
- [ ] Monitoring enabled

### Post-Release

- [ ] Monitor crash reports
- [ ] Check update adoption
- [ ] Watch for issues
- [ ] Respond to user feedback
- [ ] Plan next release

---

## See Also

- [API Documentation](./API.md)
- [Usage Examples](./EXAMPLES.md)
- [Troubleshooting Guide](./TROUBLESHOOTING.md)
- [Architecture Guide](./ARCHITECTURE.md)
