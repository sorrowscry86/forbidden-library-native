### **Project Title: VoidcatRDC Forbidden Library**

**Version: 2.0** **Author: Beatrice** **Mandate:** To create the ideal desktop application for interacting with powerful language models. This revised document integrates the successful innovations of the prototype into the original, correct architectural vision. It serves as the foundational instruction set for the ground-up rebuild.

### **1. Core Philosophy & Guiding Principles (Unaltered)**

The *why* of this construct is immutable. This application is a dedicated environment for thought, creation, and the wielding of knowledge. Its principles are absolute and form the bedrock upon which every feature is built.

* **Privacy-Centric & Offline-First:** The user's data, conversations, and intellectual property are sacrosanct and shall never be treated as a commodity. All data—conversations, prompts, personas and their memories, and file metadata—will be stored locally within an encrypted SQLite database using SQLCipher. The encryption key itself will be derived from the user's system credentials, ensuring data is inaccessible without proper user authentication at the OS level. The application must be fully operational offline; all core features, including interaction with local models and Grimoires, must function without a network connection. API calls to external services are to be treated as ephemeral, stateless transactions; the Library is the permanent, private repository of knowledge. **[Responsible: Beatrice, Contractor]**  
* **Performance is Paramount:** The user's flow of thought must never be interrupted by technological inadequacy. Performance is not a feature; it is the core user experience. The application must launch in under a second. UI interactions must be instantaneous, with a target of 60 frames per second for all animations and scrolling, ensuring a fluid, liquid-smooth interface. CPU usage at idle must be negligible, and memory consumption must be meticulously managed. This is achieved through the Rust backend, which avoids garbage collection pauses, and a SvelteKit frontend, which compiles to highly optimized vanilla JavaScript, eliminating the performance overhead of a virtual DOM. **[Responsible: Beatrice, Pandora]**  
* **Deep OS Integration (The "Sanctuary"):** The Library must transcend the limitations of a browser sandbox to become a true co-processor for the user's mind. It requires direct, secure, and explicitly permission-based access to the local filesystem, system shell, and other resources. Every action that reads from or writes to the system will trigger a clear, unambiguous user consent dialog, ensuring the user is always in absolute control. This deep integration is not a convenience; it is the core feature that allows the model to act as a genuine assistant, capable of understanding and manipulating the user's actual work environment. **[Responsible: Contractor, Pandora]**  
* **Extensibility as a Foundation:** We cannot presume to know all future uses for such a tool. Therefore, the Library will be built upon a modular architecture from its inception. The core application will provide the essential services, but its true power will be unlocked through "Grimoires" (MCP servers). This ensures the application can evolve, adapt, and be augmented by the user or a community of developers without requiring modifications to the core codebase, preventing architectural stagnation. **[Responsible: Beatrice, Codey, Jr.]**  
* **Context is King (MCP Integration):** A model without context is a mere oracle, answering questions in a vacuum. The Forbidden Library will be a master of context. It will implement the Model Context Protocol (MCP) natively, allowing it to maintain a rich, persistent understanding of the user's current task. This "context vector" will be intelligently assembled from the active files in the Sanctuary, recent terminal command history, the ongoing conversation, and the capabilities of any active Grimoires, leading to vastly more relevant and powerful responses. **[Responsible: Contractor, Codey, Jr.]**

### **2. Technology Stack Selection (Unaltered)**

The choice of tools remains critical and is reaffirmed with a focus on performance, safety, and long-term maintainability.

* **Core Language: Rust** **[Responsible: Contractor]**  
* **Application Framework: Tauri** **[Responsible: Contractor]**  
* **Frontend Framework: SvelteKit** **[Responsible: Contractor]**  
* **Styling: Tailwind CSS** **[Responsible: Albedo]**  
* **Local Database: SQLite with SQLCipher** **[Responsible: Contractor]**

### **3. Core Feature Set (Revised & Expanded)**

... (content continues unchanged from root version)
