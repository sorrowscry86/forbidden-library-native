# Project: Forbidden Library

## Project Overview

The Forbidden Library is a privacy-first, high-performance native desktop application for interacting with powerful language models. It is built with a Rust/Tauri/SvelteKit architecture, providing a secure, offline-capable, and extensible environment for thought, creation, and knowledge management.

The core philosophy of the project is to be privacy-centric and offline-first, with all data stored locally in an encrypted SQLite database. The application is designed for performance, with sub-second launch times, a 60 FPS UI, and negligible idle resource consumption. It also features deep OS integration with secure filesystem and shell access, requiring explicit user consent.

The application is built with the following technology stack:

- **Core Language:** Rust
- **Application Framework:** Tauri
- **Frontend:** SvelteKit
- **Styling:** Tailwind CSS
- **Database:** SQLite with SQLCipher encryption
- **Protocol:** Model Context Protocol (MCP) for extensibility

## Building and Running

### Prerequisites

- **Rust** (1.70 or later)
- **Node.js** (18 or later)
- **pnpm**

### Installation & Development

1.  **Install dependencies:**

    ```bash
    pnpm install
    ```

2.  **Run in development mode:**

    ```bash
    pnpm tauri dev
    ```

3.  **Build for production:**
    ```bash
    pnpm tauri build
    ```

### Available Scripts

- `pnpm tauri dev`: Start development server with hot reload
- `pnpm tauri build`: Build production application bundles
- `pnpm dev`: Run SvelteKit dev server only (for frontend development)
- `pnpm build`: Build SvelteKit frontend only
- `cargo test`: Run Rust unit tests
- `cargo clippy`: Run Rust linter
- `cargo fmt`: Format Rust code

## Development Conventions

The project follows the official Rust, Svelte, and Tauri conventions. All code should be formatted with `cargo fmt` and `prettier`. The project also uses `clippy` for Rust linting and `eslint` for TypeScript/JavaScript linting.

Contributions are welcome. Please read the `CONTRIBUTING.md` file for more details.

## Project Structure

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

## Key Features

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

## Configuration

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

## Project Status

The project is currently in **Phase 6: Cross-Platform Testing & Release Preparation**. The backend and frontend are complete, and the application is functional. The next steps are to conduct comprehensive cross-platform testing and prepare for release.

## Security & Privacy

- **Local-First Architecture**: All user data remains on device
- **SQLCipher Encryption**: Database encrypted with system-derived keys
- **Explicit Permissions**: All system access requires user consent
- **Secure IPC**: Tauri's secure Inter-Process Communication layer
- **No Telemetry**: Zero data collection or tracking

## Performance Benchmarks

The Forbidden Library is designed for exceptional performance:

- **Launch Time**: < 1 second (target)
- **UI Responsiveness**: 60 FPS target for all interactions
- **Memory Usage**: Optimized for minimal idle consumption
- **Database Operations**: Sub-millisecond query response times

## Roadmap

- [ ] Multi-language support and localization
- [ ] Advanced plugin ecosystem expansion
- [ ] Cross-platform mobile companion app
- [ ] Distributed model inference capabilities
- [ ] Advanced vector database integration

## Contributing

We welcome contributions from the community! Please read our [Contributing Guidelines](./CONTRIBUTING.md) before submitting pull requests.

### Development Workflow

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Make your changes following our coding standards
4. Run tests and linting (`cargo test`, `cargo clippy`, `cargo fmt`)
5. Commit your changes (`git commit -m 'Add amazing feature'`)
6. Push to your branch (`git push origin feature/amazing-feature`)
7. Open a Pull Request

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Troubleshooting

- **`pnpm install` fails:**
  - Ensure you have Node.js and pnpm installed and that their versions meet the requirements in `package.json`.
  - Try deleting `pnpm-lock.yaml` and `node_modules` and running `pnpm install` again.
- **`pnpm tauri dev` fails:**
  - Make sure you have the Rust toolchain installed and properly configured.
  - Check that the `beforeDevCommand` in `src-tauri/tauri.conf.json` is correct.
- **Rust compilation errors:**
  - Run `cargo check` to get more detailed error messages.
  - Ensure that all dependencies in `Cargo.toml` are compatible.
- **Frontend issues:**
  - Check the browser's developer console for any errors.
  - Use the Svelte DevTools to inspect the component hierarchy and state.
