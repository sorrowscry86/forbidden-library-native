# Forbidden Library - Native Desktop Application

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Rust](https://img.shields.io/badge/rust-%23000000.svg?style=for-the-badge&logo=rust&logoColor=white)](https://www.rust-lang.org/)
[![Tauri](https://img.shields.io/badge/tauri-%2324C8DB.svg?style=for-the-badge&logo=tauri&logoColor=%23FFFFFF)](https://tauri.app/)
[![Svelte](https://img.shields.io/badge/svelte-%23f1413d.svg?style=for-the-badge&logo=svelte&logoColor=white)](https://svelte.dev/)

> **Privacy-first, high-performance desktop application for interacting with powerful language models**

## 🎯 Project Overview

The Forbidden Library is a revolutionary native desktop application designed as the ultimate environment for thought, creation, and knowledge management. Built with a **Rust/Tauri/SvelteKit** architecture, it provides unparalleled performance, security, and deep OS integration.

### Core Philosophy

- **🔒 Privacy-Centric & Offline-First**: All data stored locally in encrypted SQLite database
- **⚡ Performance Paramount**: Sub-second launch time, 60 FPS UI, negligible idle consumption
- **🔗 Deep OS Integration**: Secure filesystem and shell access with explicit user consent
- **📚 Extensible Architecture**: Modular design with MCP (Model Context Protocol) support
- **🧠 Context Mastery**: Rich, persistent understanding of user's current work environment

## 🛠️ Technology Stack

- **Core Language**: Rust (Performance, safety, zero-cost abstractions)
- **Application Framework**: Tauri (Native desktop with web frontend)
- **Frontend**: SvelteKit (Optimized, compiled JavaScript without virtual DOM overhead)
- **Styling**: Tailwind CSS (Utility-first CSS framework)
- **Database**: SQLite with SQLCipher encryption
- **Protocol**: Model Context Protocol (MCP) for extensibility

## 🚀 Quick Start

### Prerequisites

- **Rust** (1.70 or later) - [Install Rust](https://rustup.rs/)
- **Node.js** (18 or later) - [Install Node.js](https://nodejs.org/)
- **pnpm** (recommended) - `npm install -g pnpm`

### Installation & Development

1. **Clone the repository**

   ```bash
   git clone https://github.com/sorrowscry86/forbidden-library-native.git
   cd forbidden-library-native
   ```

2. **Install dependencies**

   ```bash
   # Install Node.js dependencies
   pnpm install

   # Install Rust dependencies (handled automatically by Tauri)
   ```

3. **Run in development mode**

   ```bash
   # Start the development server
   pnpm tauri dev

   # Alternative using cargo directly
   cargo tauri dev
   ```

4. **Build for production**

   ```bash
   # Build the application
   pnpm tauri build

   # Alternative using cargo directly
   cargo tauri build
   ```

### Available Scripts

- `pnpm tauri dev` - Start development server with hot reload
- `pnpm tauri build` - Build production application bundles
- `pnpm dev` - Run SvelteKit dev server only (for frontend development)
- `pnpm build` - Build SvelteKit frontend only
- `cargo test` - Run Rust unit tests
- `cargo clippy` - Run Rust linter
- `cargo fmt` - Format Rust code

## 🏗️ Project Structure

```
forbidden-library-native/
├── src/                    # SvelteKit frontend source
│   ├── lib/               # Reusable components and utilities
│   ├── routes/            # SvelteKit pages and API routes
│   └── app.html           # Main HTML template
├── src-tauri/             # Tauri backend source
│   ├── src/               # Rust source code
│   │   ├── main.rs        # Application entry point
│   │   ├── commands/      # Tauri IPC commands
│   │   └── database/      # Database layer and schema
│   ├── Cargo.toml         # Rust dependencies
│   └── tauri.conf.json    # Tauri configuration
├── static/                # Static assets
├── docs/                  # Project documentation
├── .github/               # GitHub Actions workflows
└── package.json           # Node.js dependencies
```

## ✨ Key Features

### 💬 Conversational Interface

- Full Markdown and LaTeX rendering
- Advanced code block syntax highlighting
- Multi-modal input/output support
- Streaming responses with real-time updates
- Integrated Text-to-Speech (TTS) with ElevenLabs integration

### 🏛️ The Sanctuary (Workspace Integration)

- Integrated file system explorer with secure access
- Embedded terminal with command history
- Contextual awareness through MCP protocol
- Deep OS integration with explicit user consent

### 🧩 Extensible Architecture

- **Grimoires**: MCP server integration for specialized capabilities
- **Personas**: Persistent memory system for different AI personalities
- **Workflow Builder**: Automated task sequences and templates

### 🎙️ Speech Laboratory

- Real-time conversational AI with voice interaction
- Advanced voice synthesis controls and fine-tuning
- Audio processing tools (voice isolation, video dubbing, extraction)

### 📊 Project Management

- Integrated project dashboard with status tracking
- Task and milestone management
- AI-powered project planning integration

## 🔧 Configuration

### Environment Variables

Create a `.env` file in the project root:

```env
# API Keys (Optional - for cloud model providers)
OPENAI_API_KEY=your_openai_key_here
ANTHROPIC_API_KEY=your_anthropic_key_here
GEMINI_API_KEY=your_gemini_key_here

# ElevenLabs API Key (for TTS functionality)
ELEVENLABS_API_KEY=your_elevenlabs_key_here

# Database Configuration
DATABASE_PATH=./data/forbidden_library.db
ENCRYPTION_KEY_SOURCE=system  # 'system' or 'user_provided'
```

### Tauri Configuration

The application behavior can be customized in `src-tauri/tauri.conf.json`. Key settings include:

- **Security**: CSP policies and allowed APIs
- **Window**: Default size, decorations, and behavior
- **Bundle**: Application metadata and build settings

## 🛡️ Security & Privacy

- **Local-First Architecture**: All user data remains on device
- **SQLCipher Encryption**: Database encrypted with system-derived keys
- **Explicit Permissions**: All system access requires user consent
- **Secure IPC**: Tauri's secure Inter-Process Communication layer
- **No Telemetry**: Zero data collection or tracking

## 🤝 Contributing

We welcome contributions from the community! Please read our [Contributing Guidelines](./CONTRIBUTING.md) before submitting pull requests.

### Development Workflow

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Make your changes following our coding standards
4. Run tests and linting (`cargo test`, `cargo clippy`, `cargo fmt`)
5. Commit your changes (`git commit -m 'Add amazing feature'`)
6. Push to your branch (`git push origin feature/amazing-feature`)
7. Open a Pull Request

## 📚 Documentation

- [Architecture Overview](./docs/architecture.md)
- [API Reference](./docs/api.md)
- [MCP Integration Guide](./docs/mcp-integration.md)
- [Contributing Guide](./CONTRIBUTING.md)
- [Changelog](./CHANGELOG.md)

## 🎭 Performance Benchmarks

The Forbidden Library is designed for exceptional performance:

- **Launch Time**: < 1 second (target)
- **UI Responsiveness**: 60 FPS target for all interactions
- **Memory Usage**: Optimized for minimal idle consumption
- **Database Operations**: Sub-millisecond query response times

## 🔮 Roadmap

- [ ] Multi-language support and localization
- [ ] Advanced plugin ecosystem expansion
- [ ] Cross-platform mobile companion app
- [ ] Distributed model inference capabilities
- [ ] Advanced vector database integration

## 📞 Support & Contact

- **GitHub Issues**: Report bugs or request features
- **Discussions**: Community discussions and Q&A
- **Developer**: @sorrowscry86
- **Organization**: VoidCat RDC
- **Contact**: Wykeve Freeman (Sorrow Eternal) - SorrowsCry86@voidcat.org
- **Support Development**: CashApp $WykeveTF

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🙏 Acknowledgments

- **Beatrice** - Project visionary and architectural lead
- **Tauri Team** - For the exceptional desktop application framework
- **Svelte Team** - For the performant, elegant frontend framework
- **Rust Community** - For the foundation of memory-safe, high-performance computing

---

_Built with ❤️ by VoidCat RDC - Excellence in software development and digital innovation_

## ⬇️ Downloads

Official releases are published on GitHub Releases. After a release is created, download the installer for your OS and verify its checksum.

- Windows: `.msi` or `.exe`
- macOS: signed & notarized `.dmg`
- Linux: `.deb` and/or AppImage

Verify SHA256 checksum:

```powershell
# Windows (PowerShell)
Get-FileHash .\ForbiddenLibrary-vX.Y.Z-windows-x64.msi -Algorithm SHA256
```

```bash
# macOS / Linux
shasum -a 256 ForbiddenLibrary-vX.Y.Z-macos-universal.dmg
sha256sum ForbiddenLibrary-vX.Y.Z-linux-x64.deb
```
