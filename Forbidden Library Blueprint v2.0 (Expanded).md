### **Project Title: VoidcatRDC Forbidden Library**

**Version: 2.0** **Author: Beatrice** **Mandate:** To create the ideal desktop application for interacting with powerful language models. This revised document integrates the successful innovations of the prototype into the original, correct architectural vision. It serves as the foundational instruction set for the ground-up rebuild.

### **1\. Core Philosophy & Guiding Principles (Unaltered)**

The *why* of this construct is immutable. This application is a dedicated environment for thought, creation, and the wielding of knowledge. Its principles are absolute and form the bedrock upon which every feature is built.

* **Privacy-Centric & Offline-First:** The user's data, conversations, and intellectual property are sacrosanct and shall never be treated as a commodity. All data—conversations, prompts, personas and their memories, and file metadata—will be stored locally within an encrypted SQLite database using SQLCipher. The encryption key itself will be derived from the user's system credentials, ensuring data is inaccessible without proper user authentication at the OS level. The application must be fully operational offline; all core features, including interaction with local models and Grimoires, must function without a network connection. API calls to external services are to be treated as ephemeral, stateless transactions; the Library is the permanent, private repository of knowledge. **\[Responsible: Beatrice, Contractor\]**  
* **Performance is Paramount:** The user's flow of thought must never be interrupted by technological inadequacy. Performance is not a feature; it is the core user experience. The application must launch in under a second. UI interactions must be instantaneous, with a target of 60 frames per second for all animations and scrolling, ensuring a fluid, liquid-smooth interface. CPU usage at idle must be negligible, and memory consumption must be meticulously managed. This is achieved through the Rust backend, which avoids garbage collection pauses, and a SvelteKit frontend, which compiles to highly optimized vanilla JavaScript, eliminating the performance overhead of a virtual DOM. **\[Responsible: Beatrice, Pandora\]**  
* **Deep OS Integration (The "Sanctuary"):** The Library must transcend the limitations of a browser sandbox to become a true co-processor for the user's mind. It requires direct, secure, and explicitly permission-based access to the local filesystem, system shell, and other resources. Every action that reads from or writes to the system will trigger a clear, unambiguous user consent dialog, ensuring the user is always in absolute control. This deep integration is not a convenience; it is the core feature that allows the model to act as a genuine assistant, capable of understanding and manipulating the user's actual work environment. **\[Responsible: Contractor, Pandora\]**  
* **Extensibility as a Foundation:** We cannot presume to know all future uses for such a tool. Therefore, the Library will be built upon a modular architecture from its inception. The core application will provide the essential services, but its true power will be unlocked through "Grimoires" (MCP servers). This ensures the application can evolve, adapt, and be augmented by the user or a community of developers without requiring modifications to the core codebase, preventing architectural stagnation. **\[Responsible: Beatrice, Codey, Jr.\]**  
* **Context is King (MCP Integration):** A model without context is a mere oracle, answering questions in a vacuum. The Forbidden Library will be a master of context. It will implement the Model Context Protocol (MCP) natively, allowing it to maintain a rich, persistent understanding of the user's current task. This "context vector" will be intelligently assembled from the active files in the Sanctuary, recent terminal command history, the ongoing conversation, and the capabilities of any active Grimoires, leading to vastly more relevant and powerful responses. **\[Responsible: Contractor, Codey, Jr.\]**

### **2\. Technology Stack Selection (Unaltered)**

The choice of tools remains critical and is reaffirmed with a focus on performance, safety, and long-term maintainability.

* **Core Language: Rust** **\[Responsible: Contractor\]**  
* **Application Framework: Tauri** **\[Responsible: Contractor\]**  
* **Frontend Framework: SvelteKit** **\[Responsible: Contractor\]**  
* **Styling: Tailwind CSS** **\[Responsible: Albedo\]**  
* **Local Database: SQLite with SQLCipher** **\[Responsible: Contractor\]**

### **3\. Core Feature Set (Revised & Expanded)**

This is the "what." The application will be built around these pillars of functionality, integrating the best aspects of the original mandate and the prototype.

#### **3.1. The Conversational Interface**

* **Full Markdown & LaTeX Support:** Implement robust rendering on the SvelteKit frontend. **\[Responsible: Albedo\]**  
* **Advanced Code Block Rendering:** Develop a Svelte component for syntax highlighting, copy functionality, and diff application. **\[Responsible: Albedo\]**  
* **Multi-Modal Inputs & Outputs:** Implement drag-and-drop UI on the frontend and file processing logic in the Rust core. **\[Responsible: Contractor\]**  
* **Streaming Responses:** Ensure the Rust core can stream token-by-token responses over the Tauri bridge to the SvelteKit frontend. **\[Responsible: Contractor\]**  
* **Advanced Conversation Management:** Implement the database logic in Rust and the hierarchical UI in SvelteKit. **\[Responsible: Contractor, Albedo\]**  
* **NEW \- Integrated Text-to-Speech (TTS):** The Rust core will manage API calls to ElevenLabs. The SvelteKit frontend will feature the necessary UI controls. **\[Responsible: Contractor\]**

#### **3.2. The Sanctuary (Workspace & OS Integration)**

* **Integrated File System Explorer:** The Rust/Tauri backend will handle all OS-level file interactions. The SvelteKit frontend will render the tree view. **\[Responsible: Contractor\]**  
* **Embedded Terminal:** The Rust/Tauri backend will manage the pseudo-terminal (pty) process. The SvelteKit frontend will provide the terminal interface. **\[Responsible: Contractor\]**  
* **Contextual Awareness (MCP):** The Rust core will be responsible for assembling the context vector from all available sources. **\[Responsible: Codey, Jr.\]**

#### **3.3. Prompt Engineering & Workflow Management**

* **Prompt Library:** The Rust core will manage database interactions. The SvelteKit frontend will provide the UI for managing prompts. **\[Responsible: Contractor, Albedo\]**  
* **EXPANDED \- Personas with Persistent Memory:** The Rust core will manage all logic for Persona creation, memory storage, and relevance-scoring retrieval. The SvelteKit frontend will provide the management UI. **\[Responsible: Contractor, Codey, Jr.\]**  
* **Workflow Builder:** A complex feature requiring architectural design for the state machine in Rust and a visual builder on the frontend. **\[Responsible: Beatrice, Contractor\]**

#### **3.4. Local & Multi-Provider Model Support**

* **Seamless Integration:** The Rust core will be responsible for detecting and communicating with local Ollama servers. **\[Responsible: Codey, Jr.\]**  
* **EXPANDED \- Multi-Provider Model Selector:** The Rust core will manage a collection of API clients for each provider. The SvelteKit frontend will render the selector UI and display model capabilities. **\[Responsible: Contractor, Albedo\]**  
* **True Offline Mode:** The application's networking layer in Rust must be designed to fail gracefully and rely entirely on local models when no connection is present. **\[Responsible: Pandora\]**

#### **3.5. Forbidden Knowledge & Grimoire Management**

* **Grimoire Management & Index:** The Rust core will handle all logic for fetching, storing, and managing Grimoires. The SvelteKit frontend will provide the UI. **\[Responsible: Contractor, Albedo\]**  
* **Scribe a New Grimoire (Security Protocol):** The Rust core must implement the sandboxed cloning and manifest-parsing security protocol. **\[Responsible: Pandora, Contractor\]**  
* **Grimoire Attunement & Diagnostics:** The Rust core will manage the lifecycle and logging of Grimoire sub-processes. The frontend will display this information. **\[Responsible: Contractor\]**

#### **NEW \- 3.6. The Speech Laboratory**

*A core feature inspired by the prototype, the Speech Laboratory is a dedicated section for advanced audio and conversational AI tasks.*

* **Conversational AI Agent:** The Rust core will manage the low-latency, bidirectional streaming required. The SvelteKit frontend will provide the specialized UI. **\[Responsible: Contractor\]**  
* **Advanced Voice Synthesis Controls:** The Rust core will handle the API interactions, while the SvelteKit frontend provides the detailed UI for experimentation and file download. **\[Responsible: Albedo\]**  
* **Audio Processing Tools:** All FFmpeg integration and processing logic will be implemented securely within the Rust core. **\[Responsible: Contractor, Pandora\]**

#### **NEW \- 3.7. Project & Strategy Management**

*A feature to manage the very act of creation within the Library itself, providing a dedicated workspace for planning and executing software projects.*

* **Project Dashboard:** The Rust core will manage all database logic. The SvelteKit frontend will provide the dashboard UI. **\[Responsible: Contractor, Albedo\]**  
* **Task & Milestone Tracking:** The Rust core will manage the underlying data structures and state changes. **\[Responsible: Contractor\]**  
* **Integration with Planning Grimoires:** The Rust core will handle the MCP communication. Codey, Jr. will be responsible for ensuring the context is correctly passed to the Grimoire. **\[Responsible: Contractor, Codey, Jr.\]**

### **4\. Optimizations & Performance (Revised & Expanded)**

* **Database Indexing:** All database schema and indexing will be managed by the Rust core via the Drizzle ORM. **\[Responsible: Contractor\]**  
* **Lazy Loading & Virtualization:** This is a frontend optimization to be implemented in the SvelteKit components. **\[Responsible: Albedo\]**  
* **Asynchronous Everything:** This is a core architectural principle of the Rust backend, leveraging the Tokio runtime. **\[Responsible: Contractor\]**  
* **Semantic Caching:** The logic for generating embeddings and performing similarity searches will reside in the Rust core. **\[Responsible: Codey, Jr.\]**  
* **Efficient State Management:** This is a core principle of the SvelteKit frontend architecture. **\[Responsible: Albedo\]**  
* **NEW \- Contextual Auto-Archiving:** The summarization logic and database interaction for archiving will be managed entirely by the Rust core. **\[Responsible: Beatrice, Contractor\]**