# Forbidden Library Troubleshooting Guide

Comprehensive troubleshooting guide for common issues and their solutions.

## Table of Contents

1. [Quick Diagnostics](#quick-diagnostics)
2. [Installation Issues](#installation-issues)
3. [Database Problems](#database-problems)
4. [Keychain/Credential Storage Issues](#keychaincreden

tial-storage-issues)
5. [Frontend Build Errors](#frontend-build-errors)
6. [Backend Compilation Errors](#backend-compilation-errors)
7. [Runtime Errors](#runtime-errors)
8. [Performance Issues](#performance-issues)
9. [Platform-Specific Issues](#platform-specific-issues)
10. [Development Environment](#development-environment)

---

## Quick Diagnostics

### Run Health Check

```bash
# Check all systems
./scripts/health-check.sh

# Or manually:
cd src-tauri
cargo test --all
cd ..
pnpm test
```

### Common Quick Fixes

Try these first for most issues:

```bash
# Clean and rebuild
pnpm clean
pnpm install
cd src-tauri && cargo clean && cargo build
cd ..

# Reset database
rm -rf ~/.local/share/forbidden-library/  # Linux
rm -rf ~/Library/Application\ Support/forbidden-library/  # macOS
rm -rf %APPDATA%\forbidden-library\  # Windows

# Clear node_modules
rm -rf node_modules .svelte-kit
pnpm install
```

---

## Installation Issues

### Problem: `pnpm install` fails

**Symptoms:**
```
ERROR  Unable to find compatible versions
```

**Solutions:**

1. **Check Node.js version:**
```bash
node --version  # Should be >= 18.0.0
```

If outdated:
```bash
# Install nvm (Node Version Manager)
curl -o- https://raw.githubusercontent.com/nvm-sh/nvm/v0.39.0/install.sh | bash

# Install latest LTS
nvm install --lts
nvm use --lts
```

2. **Clear pnpm cache:**
```bash
pnpm store prune
rm -rf node_modules pnpm-lock.yaml
pnpm install
```

3. **Check for conflicting global packages:**
```bash
pnpm list -g --depth 0
# Remove conflicting packages if needed
```

### Problem: Rust toolchain not found

**Symptoms:**
```
error: rustc not found
```

**Solutions:**

```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Update Rust
rustup update

# Verify installation
rustc --version
cargo --version
```

### Problem: Missing system dependencies (Linux)

**Symptoms:**
```
error: failed to run custom build command for `libsqlite3-sys`
```

**Solutions:**

**Ubuntu/Debian:**
```bash
sudo apt update
sudo apt install -y \
  libwebkit2gtk-4.0-dev \
  build-essential \
  curl \
  wget \
  file \
  libssl-dev \
  libgtk-3-dev \
  libayatana-appindicator3-dev \
  librsvg2-dev \
  libsqlite3-dev \
  libsecret-1-dev
```

**Fedora:**
```bash
sudo dnf install \
  webkit2gtk4.0-devel \
  openssl-devel \
  curl \
  wget \
  file \
  libappindicator-gtk3-devel \
  librsvg2-devel \
  sqlite-devel \
  libsecret-devel
```

**Arch Linux:**
```bash
sudo pacman -S \
  webkit2gtk \
  base-devel \
  curl \
  wget \
  file \
  openssl \
  appmenu-gtk-module \
  gtk3 \
  libappindicator-gtk3 \
  librsvg \
  sqlite \
  libsecret
```

---

## Database Problems

### Problem: Database locked

**Symptoms:**
```
Database error: database is locked
```

**Causes:**
- Another instance of the app is running
- Previous crash left connection open
- File system issues

**Solutions:**

1. **Close all instances:**
```bash
# Linux/macOS
pkill -f forbidden-library

# Windows
taskkill /F /IM forbidden-library.exe
```

2. **Remove lock file:**
```bash
# Find database location
# Linux: ~/.local/share/forbidden-library/
# macOS: ~/Library/Application Support/forbidden-library/
# Windows: %APPDATA%\forbidden-library\

rm forbidden_library.db-shm forbidden_library.db-wal
```

3. **Check file permissions:**
```bash
chmod 644 forbidden_library.db
```

### Problem: Database corruption

**Symptoms:**
```
Database error: file is not a database
Database error: malformed database
```

**Solutions:**

1. **Restore from backup:**
```bash
cp forbidden_library.db forbidden_library.db.corrupt
cp backups/forbidden_library_YYYY-MM-DD.db forbidden_library.db
```

2. **Attempt recovery with SQLite:**
```bash
sqlite3 forbidden_library.db ".recover" | sqlite3 forbidden_library_recovered.db
mv forbidden_library.db forbidden_library.db.corrupt
mv forbidden_library_recovered.db forbidden_library.db
```

3. **Start fresh (LAST RESORT - loses data):**
```bash
rm -rf ~/.local/share/forbidden-library/
# App will create new database on next launch
```

### Problem: Migration failures

**Symptoms:**
```
Database error: no such table: conversations
```

**Solutions:**

1. **Check database version:**
```bash
sqlite3 forbidden_library.db "PRAGMA user_version;"
```

2. **Manually run migrations:**
```bash
cd src-tauri
cargo run --bin migrate
```

3. **Rebuild database schema:**
```rust
// In src-tauri/src/main.rs
let db = DatabaseManager::new(&app_handle)?;
db.initialize_schema()?;  // Force re-initialization
```

---

## Keychain/Credential Storage Issues

### Problem: Keychain access denied (macOS)

**Symptoms:**
```
Keychain error: User interaction is not allowed
Keychain error: The specified item could not be found in the keychain
```

**Solutions:**

1. **Grant keychain access:**
```bash
# Open Keychain Access
open /Applications/Utilities/Keychain\ Access.app

# Find "forbidden-library" items
# Click "Always Allow" when prompted
```

2. **Reset keychain permissions:**
```bash
security delete-generic-password -s "com.voidcat.forbidden-library" || true
# Restart app and re-enter credentials
```

3. **Check codesigning:**
```bash
codesign --verify --verbose /Applications/Forbidden\ Library.app
```

### Problem: Credential Manager issues (Windows)

**Symptoms:**
```
Keychain error: Failed to access Windows Credential Manager
```

**Solutions:**

1. **Verify Credential Manager service:**
```powershell
Get-Service -Name "VaultSvc"
# If not running:
Start-Service -Name "VaultSvc"
```

2. **Clear existing credentials:**
```powershell
cmdkey /list | findstr "forbidden-library"
# For each found:
cmdkey /delete:TargetName=forbidden-library:provider_name
```

3. **Run as administrator:**
- Right-click app → "Run as administrator"

### Problem: Secret Service unavailable (Linux)

**Symptoms:**
```
Keychain error: Cannot connect to secret service
```

**Solutions:**

1. **Install secret service:**
```bash
# Ubuntu/Debian
sudo apt install gnome-keyring libsecret-tools

# Fedora
sudo dnf install gnome-keyring libsecret

# Arch
sudo pacman -S gnome-keyring libsecret
```

2. **Start secret service:**
```bash
# Check if running
ps aux | grep gnome-keyring

# If not running (in a graphical session):
gnome-keyring-daemon --start --components=secrets
```

3. **Set up D-Bus session:**
```bash
export $(dbus-launch)
```

---

## Frontend Build Errors

### Problem: SvelteKit build fails

**Symptoms:**
```
Error: Cannot find module '@sveltejs/kit'
```

**Solutions:**

1. **Reinstall dependencies:**
```bash
rm -rf node_modules .svelte-kit
pnpm install
```

2. **Sync SvelteKit:**
```bash
pnpm exec svelte-kit sync
```

3. **Check for TypeScript errors:**
```bash
pnpm check
```

### Problem: Vite build errors

**Symptoms:**
```
Error: Build failed with 1 error
Transform failed with 1 error
```

**Solutions:**

1. **Clear Vite cache:**
```bash
rm -rf node_modules/.vite
pnpm dev
```

2. **Check for import errors:**
```bash
# Look for missing imports
pnpm check
```

3. **Update Vite:**
```bash
pnpm update vite @sveltejs/vite-plugin-svelte
```

### Problem: TypeScript compilation errors

**Symptoms:**
```
error TS2307: Cannot find module
error TS2339: Property does not exist
```

**Solutions:**

1. **Regenerate TypeScript config:**
```bash
pnpm exec svelte-kit sync
```

2. **Check tsconfig.json:**
```json
{
  "extends": "./.svelte-kit/tsconfig.json",
  "compilerOptions": {
    "strict": true,
    "moduleResolution": "bundler"
  }
}
```

3. **Install missing type definitions:**
```bash
pnpm add -D @types/node @tauri-apps/api
```

---

## Backend Compilation Errors

### Problem: Cargo build fails

**Symptoms:**
```
error: could not compile `forbidden-library-native`
```

**Solutions:**

1. **Clean build artifacts:**
```bash
cd src-tauri
cargo clean
cargo build
```

2. **Update dependencies:**
```bash
cargo update
```

3. **Check Rust version:**
```bash
rustc --version  # Should be >= 1.70.0
rustup update
```

### Problem: Linker errors

**Symptoms:**
```
error: linking with `cc` failed
ld: library not found
```

**Solutions:**

**macOS:**
```bash
xcode-select --install
```

**Linux:**
```bash
sudo apt install build-essential  # Ubuntu/Debian
sudo dnf groupinstall "Development Tools"  # Fedora
```

**Windows:**
```powershell
# Install Visual Studio Build Tools
# https://visualstudio.microsoft.com/downloads/
```

### Problem: Dependency compilation errors

**Symptoms:**
```
error: failed to compile `rusqlite v0.30.0`
```

**Solutions:**

1. **Install system dependencies:**
```bash
# See "Missing system dependencies" section above
```

2. **Feature flags:**
```bash
# Check Cargo.toml for required features
cargo build --features bundled  # For rusqlite
```

---

## Runtime Errors

### Problem: App crashes on startup

**Symptoms:**
- App window opens then immediately closes
- No error message visible

**Solutions:**

1. **Check logs:**

**Linux:**
```bash
journalctl --user -f | grep forbidden-library
```

**macOS:**
```bash
log stream --predicate 'process == "Forbidden Library"'
```

**Windows:**
```powershell
# Check Event Viewer → Application logs
```

2. **Run from terminal:**
```bash
# macOS
/Applications/Forbidden\ Library.app/Contents/MacOS/forbidden-library

# Linux
./target/release/forbidden-library

# Windows
.\target\release\forbidden-library.exe
```

3. **Check for missing dependencies:**
```bash
# Linux
ldd target/release/forbidden-library | grep "not found"

# macOS
otool -L target/release/forbidden-library
```

### Problem: Tauri IPC commands fail

**Symptoms:**
```
Error: Failed to invoke command: greet
```

**Solutions:**

1. **Check command registration:**
```rust
// In src-tauri/src/main.rs
fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            greet,  // Make sure command is listed
            create_conversation,
            // ... all other commands
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
```

2. **Verify command signature:**
```rust
// Must have #[tauri::command] attribute
#[tauri::command]
pub async fn my_command(param: String) -> Result<String, String> {
    Ok(param)
}
```

3. **Check frontend invocation:**
```typescript
// Correct
await invoke('my_command', { param: 'value' });

// Incorrect
await invoke('myCommand', { param: 'value' }); // Wrong case
await invoke('my_command', 'value'); // Wrong format
```

### Problem: State access errors

**Symptoms:**
```
Error: Failed to get AppState
thread 'main' panicked at 'Failed to manage state'
```

**Solutions:**

1. **Initialize state in main.rs:**
```rust
use crate::commands::AppState;
use crate::database::DatabaseManager;

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            let db = DatabaseManager::new(app.handle())?;
            let services = Arc::new(Services::new(Arc::new(db)));

            app.manage(AppState { services });
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![...])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
```

2. **Access state in commands:**
```rust
#[tauri::command]
pub async fn my_command(state: State<'_, AppState>) -> Result<String, String> {
    let services = &state.services;
    // Use services...
    Ok("success".to_string())
}
```

---

## Performance Issues

### Problem: Slow database queries

**Symptoms:**
- App freezes during data operations
- Long load times for conversations

**Solutions:**

1. **Add indices:**
```sql
CREATE INDEX IF NOT EXISTS idx_messages_conversation
ON messages(conversation_id);

CREATE INDEX IF NOT EXISTS idx_conversations_updated
ON conversations(updated_at DESC);
```

2. **Optimize connection pool:**
```rust
let config = DatabaseConfig {
    pool_config: PoolConfig {
        max_size: 20,  // Increase from default 10
        min_idle: Some(5),
        timeout_seconds: 60,
    },
    ..Default::default()
};
```

3. **Vacuum database:**
```bash
sqlite3 forbidden_library.db "VACUUM;"
```

### Problem: High memory usage

**Symptoms:**
- App uses excessive RAM
- System becomes slow

**Solutions:**

1. **Limit message loading:**
```typescript
// Load in batches instead of all at once
const messages = await invoke('get_messages', {
  conversationId,
  limit: 50,
  offset: 0
});
```

2. **Clear unused data:**
```rust
// Implement cleanup routine
pub fn cleanup_old_data(&self) -> AppResult<()> {
    self.with_transaction(|tx| {
        // Delete old temp data
        tx.execute("DELETE FROM temp_data WHERE created_at < datetime('now', '-7 days')", [])?;
        Ok(())
    })
}
```

3. **Monitor memory:**
```bash
# Linux
ps aux | grep forbidden-library

# macOS
top -pid $(pgrep "Forbidden Library")

# Windows
tasklist /FI "IMAGENAME eq forbidden-library.exe"
```

---

## Platform-Specific Issues

### macOS

#### Problem: Gatekeeper blocks app

**Symptoms:**
```
"Forbidden Library.app" cannot be opened because the developer cannot be verified
```

**Solutions:**

1. **Allow in System Preferences:**
```bash
# Right-click app → Open
# Or in terminal:
xattr -d com.apple.quarantine /Applications/Forbidden\ Library.app
```

2. **Sign the app (for distribution):**
```bash
codesign --force --deep --sign "Developer ID Application: Your Name" \
  target/release/bundle/macos/Forbidden\ Library.app
```

#### Problem: Retina display issues

**Symptoms:**
- Blurry UI on Retina displays

**Solutions:**

Update `tauri.conf.json`:
```json
{
  "tauri": {
    "macOSPrivateApi": true,
    "bundle": {
      "macOS": {
        "minimumSystemVersion": "10.13"
      }
    }
  }
}
```

### Windows

#### Problem: Antivirus blocks app

**Symptoms:**
- App deleted/quarantined by Windows Defender

**Solutions:**

1. **Add exclusion:**
```powershell
# Run as Administrator
Add-MpPreference -ExclusionPath "C:\Path\To\forbidden-library.exe"
```

2. **Sign executable (for distribution):**
- Obtain code signing certificate
- Sign with signtool.exe

#### Problem: WebView2 not found

**Symptoms:**
```
error: WebView2 runtime not found
```

**Solutions:**

1. **Install WebView2:**
```powershell
# Download and run:
# https://go.microsoft.com/fwlink/p/?LinkId=2124703
```

2. **Bundle WebView2 with app:**
```json
// tauri.conf.json
{
  "tauri": {
    "bundle": {
      "windows": {
        "webviewInstallMode": {
          "type": "downloadBootstrapper"
        }
      }
    }
  }
}
```

### Linux

#### Problem: App won't start on Wayland

**Symptoms:**
```
error: cannot open display
```

**Solutions:**

1. **Force X11:**
```bash
GDK_BACKEND=x11 ./forbidden-library
```

2. **Install Xwayland:**
```bash
sudo apt install xwayland  # Ubuntu/Debian
```

#### Problem: Missing tray icon

**Symptoms:**
- System tray icon doesn't appear

**Solutions:**

1. **Install libappindicator:**
```bash
sudo apt install libayatana-appindicator3-1
```

2. **Check desktop environment support:**
- GNOME: Install gnome-shell-extension-appindicator
- KDE: Built-in support
- XFCE: Built-in support

---

## Development Environment

### Problem: Hot reload not working

**Symptoms:**
- Changes don't appear without restart

**Solutions:**

1. **Check Vite config:**
```typescript
// vite.config.ts
export default defineConfig({
  server: {
    watch: {
      usePolling: true  // For some file systems
    }
  }
});
```

2. **Restart dev server:**
```bash
# Kill existing process
pkill -f "pnpm dev"

# Restart
pnpm dev
```

### Problem: Rust-analyzer issues

**Symptoms:**
- VS Code shows errors that don't exist
- Auto-complete not working

**Solutions:**

1. **Restart rust-analyzer:**
```
Cmd/Ctrl + Shift + P → "Rust Analyzer: Restart Server"
```

2. **Clear cache:**
```bash
rm -rf ~/.cache/rust-analyzer
```

3. **Update rust-analyzer:**
```bash
rustup component add rust-analyzer
```

### Problem: Tests failing locally but passing in CI

**Symptoms:**
- CI builds succeed but local tests fail

**Solutions:**

1. **Check Node.js version:**
```bash
node --version  # Match CI version
nvm use 18  # Or whatever CI uses
```

2. **Clean install:**
```bash
rm -rf node_modules pnpm-lock.yaml
pnpm install
```

3. **Check environment variables:**
```bash
# Compare local vs CI environment
env | grep NODE
env | grep RUST
```

---

## Getting Help

If issues persist after trying these solutions:

1. **Search existing issues:**
   - GitHub: https://github.com/your-org/forbidden-library/issues

2. **Gather diagnostic information:**
```bash
# System info
uname -a  # Linux/macOS
systeminfo  # Windows

# App version
./forbidden-library --version

# Rust toolchain
rustc --version
cargo --version

# Node.js
node --version
pnpm --version

# Logs location
# Linux: ~/.local/share/forbidden-library/logs/
# macOS: ~/Library/Logs/forbidden-library/
# Windows: %APPDATA%\forbidden-library\logs\
```

3. **Create a bug report:**
   - Include error messages
   - Include logs
   - Include steps to reproduce
   - Include system information

4. **Community support:**
   - Discord: https://discord.gg/forbidden-library
   - Forum: https://forum.forbidden-library.dev

---

## Preventive Maintenance

### Regular Tasks

**Weekly:**
```bash
# Optimize database
sqlite3 ~/.local/share/forbidden-library/forbidden_library.db "VACUUM; ANALYZE;"

# Clear old logs
find ~/.local/share/forbidden-library/logs/ -mtime +30 -delete
```

**Monthly:**
```bash
# Backup database
cp forbidden_library.db backups/forbidden_library_$(date +%Y-%m-%d).db

# Update dependencies
pnpm update
cd src-tauri && cargo update
```

**Before major updates:**
```bash
# Full backup
tar -czf forbidden-library-backup-$(date +%Y-%m-%d).tar.gz \
  ~/.local/share/forbidden-library/

# Test in development mode first
pnpm dev
```

---

## See Also

- [API Documentation](./API.md)
- [Usage Examples](./EXAMPLES.md)
- [Architecture Guide](./ARCHITECTURE.md)
- [Error Message Style Guide](../src-tauri/ERROR_MESSAGE_GUIDE.md)
