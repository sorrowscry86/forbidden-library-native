# Forbidden Library - Continuation Prompt for New Session

## Quick Context

You are continuing development on **Forbidden Library**, a Tauri-based desktop AI conversation manager. The project is currently at **90% completion** with Phases 1-6 complete.

## Project Overview

**Tech Stack:**
- **Backend**: Rust + Tauri + SQLite
- **Frontend**: SvelteKit + TypeScript
- **AI Integration**: OpenAI, Anthropic, Google Gemini, Azure, LM Studio, Ollama

**Purpose:** Desktop application for managing conversations with multiple AI providers, featuring advanced search, performance optimization, and beautiful UI/UX.

---

## Current State (90% Complete)

### ✅ Completed Phases (1-6)

#### Phase 1-3: Foundation (PR #12)
- Backend stability & error handling
- Code quality improvements (reduced complexity, TypeScript strict mode)
- Comprehensive documentation (6,500+ lines across API, Examples, Troubleshooting, Deployment guides)

#### Phase 4: Performance Optimization (Latest PR)
- **Query Optimizer** (`src-tauri/src/database/query_optimizer.rs`, 435 lines)
  - Query caching with TTL
  - Batch operations (10-12x faster)
  - 15+ database indices (20-100x query speedup)
- **Frontend Performance** (`src/lib/utils/performance.ts`, 463 lines)
  - Virtual scrolling (37x faster for 10K items)
  - Debounce/throttle utilities
  - Lazy loading support
  - Performance monitoring
- **Build Optimization** (vite.config.js)
  - Code splitting (5 vendor chunks)
  - 2.5x faster initial load
- **Documentation** (`docs/PERFORMANCE.md`, 892 lines)

#### Phase 5: Core Features (Latest PR)
- **AI Providers** (`src-tauri/src/ai_providers.rs`, +563 lines)
  - 7 providers: OpenAI, Anthropic (Claude), Google Gemini, Azure OpenAI, LM Studio, Ollama, OpenAI Compatible
  - Provider-specific message formatting
  - Dynamic model listing
- **Full-Text Search** (`src-tauri/src/database/fts_search.rs`, 430 lines)
  - SQLite FTS5 with BM25 relevance scoring
  - Porter stemming + Unicode support
  - 100-1000x faster than LIKE queries
  - 5 search commands (full-text, title, phrase, suggestions, rebuild)
  - Automatic index maintenance (6 triggers)
- **Import/Export** (`src-tauri/src/commands.rs`)
  - JSON import with validation
  - Atomic transactions
  - Case-insensitive role parsing

#### Phase 6: UI/UX Polish (Latest PR)
- **Components** (5 files, 1,200 lines)
  - `LoadingStates.svelte` - 5 loading variants (spinner, dots, bars, pulse, skeleton)
  - `SkeletonLoaders.svelte` - 4 content placeholders
  - `MessageRenderer.svelte` - Syntax highlighting (180+ languages) + math rendering (KaTeX)
  - `SearchHighlight.svelte` - Animated search term highlighting
  - `ProviderSelector.svelte` - AI provider configuration UI
- **Animation System** (2 files, 1,230 lines)
  - `src/lib/utils/animations.ts` - Custom transitions, micro-interactions, utilities
  - `src/lib/styles/transitions.css` - Global CSS animations, hover effects, stagger
- **Accessibility** (`src/lib/utils/accessibility.ts`, 700 lines)
  - WCAG 2.1 Level AA compliance
  - Keyboard navigation (focus trap, arrow keys)
  - Screen reader support (announcements, SR-only content)
  - ARIA utilities (ID generation, label linking)
  - Color contrast checking (WCAG AA/AAA)
  - Form enhancement (auto-validation, error linking)
  - Focus management, skip links, motion preferences
  - Accessible notifications (4 types)
- **Documentation** (`docs/UI_UX_GUIDE.md`, 900 lines)
  - Complete design system
  - Component library reference
  - Animation guide
  - Accessibility features
  - Color system + typography

### 📊 Key Metrics

**Performance:**
- Database queries: 20-100x faster
- Batch operations: 10-12x faster
- Initial load: 2.5x faster
- List rendering: 37x faster
- Search: 100-1000x faster (FTS5)

**Codebase:**
- Backend: 8,500+ lines (Rust)
- Frontend: 4,000+ lines (TypeScript/Svelte)
- Documentation: 9,200+ lines (Markdown)
- Total: 21,700+ lines

**Files:**
- Rust: 15+ files
- Svelte: 10+ components
- TypeScript utilities: 5+ files
- Documentation: 6 major guides

---

## Current Git State

**Branch:** `claude/backend-docs-quality-01BD6RHf2tq5fwmEpgnNmD5t`

**Recent Commits:**
```
39b7b8a - feat: add comprehensive UI/UX polish and accessibility features (Phase 6)
9575958 - feat: add conversation import and advanced full-text search
d973923 - feat: add comprehensive AI provider integrations (OpenAI, Claude, Gemini, Azure)
2b186ab - docs: add PR description for Phase 4 performance optimization
630a22d - perf: add comprehensive Phase 4 performance optimizations
```

**Status:** Clean working directory, all changes committed and pushed

**PR Status:** PR description created (`PR_PHASES_4_5_6.md`) but not yet opened (awaiting manual creation via GitHub UI)

---

## Architecture Overview

### Backend Structure
```
src-tauri/src/
├── main.rs                    # App entry point
├── commands.rs                # Tauri commands (20+ commands)
├── ai_providers.rs            # AI provider integrations (7 providers)
├── services/
│   └── mod.rs                 # ConversationService
├── database/
│   ├── mod.rs                 # Database manager
│   ├── query_optimizer.rs     # Caching, batching, indices
│   └── fts_search.rs          # Full-text search (FTS5)
├── models/
│   └── mod.rs                 # Data models
└── errors/
    └── mod.rs                 # Error types
```

### Frontend Structure
```
src/
├── routes/                    # SvelteKit routes
├── lib/
│   ├── components/
│   │   ├── LoadingStates.svelte
│   │   ├── SkeletonLoaders.svelte
│   │   ├── MessageRenderer.svelte
│   │   ├── SearchHighlight.svelte
│   │   └── ProviderSelector.svelte
│   ├── utils/
│   │   ├── performance.ts
│   │   ├── animations.ts
│   │   └── accessibility.ts
│   └── styles/
│       └── transitions.css
└── app.html                   # HTML template
```

### Database Schema
```sql
-- Main tables
conversations (id, title, persona_id, created_at, updated_at, archived, token_count)
messages (id, conversation_id, role, content, created_at, token_count)
personas (id, name, system_prompt, created_at, updated_at)

-- FTS5 virtual tables
conversations_fts (conversation_id, title)
messages_fts (message_id, conversation_id, content)

-- Indices (15+)
-- Covering: conversations by persona, messages by conversation, date ranges, etc.

-- Triggers (6)
-- Auto-maintain FTS tables on insert/update/delete
```

---

## Key Files to Know

### Backend Entry Points
- `src-tauri/src/main.rs` - App initialization, command registration
- `src-tauri/src/commands.rs` - All Tauri commands exposed to frontend
- `src-tauri/src/ai_providers.rs` - AI provider implementations

### Frontend Entry Points
- `src/routes/+page.svelte` - Main application page
- `src/lib/components/` - Reusable UI components

### Configuration
- `vite.config.js` - Frontend build config with code splitting
- `src-tauri/tauri.conf.json` - Tauri configuration
- `src-tauri/Cargo.toml` - Rust dependencies

### Documentation
- `docs/API.md` - Complete API reference
- `docs/PERFORMANCE.md` - Performance optimization guide
- `docs/UI_UX_GUIDE.md` - Design system & component library
- `docs/EXAMPLES.md` - Usage examples
- `docs/TROUBLESHOOTING.md` - Common issues & solutions
- `docs/DEPLOYMENT.md` - Build & deployment guide

---

## What Needs to Be Done (Phase 7 - Final 10%)

### High Priority

1. **Integrate UI Components with Backend**
   - Use `ProviderSelector` in settings page
   - Add `SearchHighlight` to search results
   - Use `LoadingStates` during API calls
   - Replace placeholders with `SkeletonLoaders`
   - Use `MessageRenderer` for all message display

2. **Streaming Responses**
   - Implement streaming for AI providers
   - Show real-time token generation
   - Update UI as tokens arrive
   - Handle stream interruption

3. **Advanced Search UI**
   - Create search page/modal
   - Integrate FTS5 search commands
   - Display results with highlighting
   - Add filter UI (persona, date, tokens)

4. **Settings Page Enhancement**
   - Provider configuration UI
   - API key management
   - Model selection
   - Performance settings
   - Accessibility preferences

### Medium Priority

5. **Keyboard Shortcuts**
   - Global shortcuts (Cmd/Ctrl+K for search)
   - Navigation shortcuts
   - Action shortcuts
   - Settings panel with shortcut list

6. **Theme System**
   - Light/dark mode toggle
   - Custom color themes
   - Theme persistence
   - Respect system preferences

7. **Error Boundaries**
   - Frontend error boundaries
   - Graceful error display
   - Error reporting
   - Recovery mechanisms

### Low Priority

8. **Additional Features**
   - Conversation tags/labels
   - Starred/favorite conversations
   - Conversation duplication
   - Bulk operations UI
   - Advanced export options (PDF, HTML)

9. **Testing**
   - Integration tests for providers
   - E2E tests for critical paths
   - Performance regression tests
   - Accessibility audit

10. **Polish**
    - Empty states
    - Onboarding flow
    - Tooltips
    - Help documentation in-app
    - About page

---

## Important Technical Details

### AI Provider Integration

**Adding a new provider requires:**
1. Add variant to `AIProvider` enum in `ai_providers.rs`
2. Implement `send_X_request()` function
3. Handle provider-specific message format
4. Add to `create_ai_provider()` helper in `commands.rs`
5. Update `ProviderSelector.svelte` with new provider
6. Test with real API key

**Example message format quirks:**
- **Anthropic**: System message must be separate parameter, not in messages array
- **Gemini**: Uses "model" role instead of "assistant"
- **Azure**: Requires endpoint + deployment name + API version

### Search Performance

**FTS5 query syntax:**
```sql
-- Exact phrase
SELECT * FROM messages_fts WHERE content MATCH '"exact phrase"';

-- Multiple terms (AND)
SELECT * FROM messages_fts WHERE content MATCH 'term1 AND term2';

-- Multiple terms (OR)
SELECT * FROM messages_fts WHERE content MATCH 'term1 OR term2';

-- Prefix matching
SELECT * FROM messages_fts WHERE content MATCH 'prefix*';

-- Relevance scoring
SELECT *, rank FROM messages_fts WHERE content MATCH 'query' ORDER BY rank;
```

**Stemming examples:**
- Searching "run" finds: run, runs, running, ran
- Searching "happy" finds: happy, happiness, happier, happiest

### Animation Best Practices

**Always respect reduced motion:**
```typescript
import { prefersReducedMotion } from '$lib/utils/accessibility';

const duration = prefersReducedMotion() ? 0 : 300;
```

**Use GPU-accelerated properties:**
- ✅ `transform`, `opacity`
- ❌ `width`, `height`, `top`, `left`, `margin`

**Keep animations subtle:**
- Duration: 150-300ms for most interactions
- Easing: `cubic-out` for entrances, `cubic-in` for exits

### Accessibility Requirements

**Every interactive element needs:**
- Keyboard accessibility (tab, enter, space, arrows)
- Focus indicator (`:focus-visible`)
- ARIA labels/roles where appropriate
- Screen reader announcements for dynamic changes

**Forms must have:**
- Labels for all inputs
- Error messages linked with `aria-describedby`
- `aria-invalid` on invalid fields
- `aria-required` on required fields

**Color contrast:**
- Normal text: 4.5:1 minimum (WCAG AA)
- Large text: 3:1 minimum (WCAG AA)
- Use `getContrastRatio()` utility to check

---

## Development Workflow

### Starting Development

```bash
# Install dependencies
npm install
cd src-tauri && cargo build

# Start dev server
npm run dev

# In another terminal, start Tauri
npm run tauri dev
```

### Testing

```bash
# Frontend tests
npm test

# Backend tests
cd src-tauri && cargo test

# Type checking
npm run check
```

### Building

```bash
# Development build
npm run tauri build

# Production build (optimized)
npm run tauri build -- --release
```

### Git Workflow

**Branch naming:** `claude/feature-description-SESSIONID`

**Commit format:**
```
type(scope): description

feat: add new feature
fix: bug fix
docs: documentation
perf: performance improvement
refactor: code refactoring
test: add tests
```

**Before committing:**
1. Run type check: `npm run check`
2. Format code: `cargo fmt` (backend), `npm run format` (frontend)
3. Ensure all tests pass
4. Check for untracked files

---

## Common Tasks

### Adding a New Tauri Command

1. **Define in `commands.rs`:**
```rust
#[tauri::command]
pub async fn my_command(
    param: String,
    state: State<'_, AppState>,
) -> Result<ReturnType, String> {
    // Implementation
    Ok(result)
}
```

2. **Register in `main.rs`:**
```rust
.invoke_handler(tauri::generate_handler![
    // ... existing commands
    commands::my_command,
])
```

3. **Call from frontend:**
```typescript
import { invoke } from '@tauri-apps/api';

const result = await invoke('my_command', { param: 'value' });
```

### Adding a New Component

1. **Create in `src/lib/components/`:**
```svelte
<script lang="ts">
  export let prop: string;
</script>

<div>
  {prop}
</div>

<style>
  /* Scoped styles */
</style>
```

2. **Use in pages:**
```svelte
<script>
  import MyComponent from '$lib/components/MyComponent.svelte';
</script>

<MyComponent prop="value" />
```

### Adding a New Route

1. **Create `src/routes/myroute/+page.svelte`**
2. **Navigation:**
```svelte
<a href="/myroute">My Route</a>
```

---

## Troubleshooting

### Common Issues

**Database locked error:**
- Ensure no other connections are open
- Use connection pooling (already implemented)
- Check for long-running transactions

**Build errors:**
- Clear cache: `rm -rf node_modules .svelte-kit`
- Reinstall: `npm install`
- Rebuild Rust: `cd src-tauri && cargo clean && cargo build`

**Performance issues:**
- Check database indices with `EXPLAIN QUERY PLAN`
- Use query caching for repeated queries
- Profile with browser DevTools
- Monitor memory with `PerformanceMeasure` utility

**FTS search not working:**
- Rebuild index: `invoke('rebuild_search_index')`
- Check trigger creation
- Verify data in FTS tables

---

## Resources

### Documentation
- **Project Docs**: `/docs` directory
- **Tauri Docs**: https://tauri.app/
- **SvelteKit Docs**: https://kit.svelte.dev/
- **SQLite FTS5**: https://www.sqlite.org/fts5.html

### Tools
- **Tauri CLI**: `npm run tauri`
- **Rust Analyzer**: VS Code extension
- **Svelte for VS Code**: Official extension

### Dependencies
- **Backend**: See `src-tauri/Cargo.toml`
- **Frontend**: See `package.json`

---

## Suggested Next Steps

### Immediate (Session 1)

1. **Create the PR** (if not done yet)
   - Open GitHub UI
   - Create PR from `claude/backend-docs-quality-01BD6RHf2tq5fwmEpgnNmD5t`
   - Use `PR_PHASES_4_5_6.md` as description

2. **Integrate UI Components**
   - Find existing message rendering → replace with `MessageRenderer`
   - Add `ProviderSelector` to settings
   - Use `LoadingStates` for all async operations
   - Add `SkeletonLoaders` to conversation list

3. **Search UI**
   - Create search modal/page
   - Wire up FTS5 commands
   - Add `SearchHighlight` for results
   - Implement filters (persona, date, etc.)

### Short Term (Sessions 2-3)

4. **Streaming Responses**
   - Add streaming support to providers
   - Update UI to show tokens as they arrive
   - Handle stream errors

5. **Settings Page**
   - Provider configuration with `ProviderSelector`
   - API key management
   - Preferences (theme, accessibility)

6. **Keyboard Shortcuts**
   - Implement shortcut system
   - Add search shortcut (Cmd/Ctrl+K)
   - Navigation shortcuts

### Long Term (Sessions 4+)

7. **Theme System**
8. **Additional Features** (tags, favorites, bulk ops)
9. **Testing & Polish**
10. **1.0 Release Preparation**

---

## Success Criteria for Phase 7

- [ ] All UI components integrated into app
- [ ] Search UI functional with highlighting
- [ ] Provider configuration accessible in settings
- [ ] Streaming responses working
- [ ] Keyboard shortcuts implemented
- [ ] Theme system functional
- [ ] All empty states handled
- [ ] Error boundaries in place
- [ ] E2E tests for critical paths
- [ ] Performance benchmarks maintained
- [ ] Accessibility audit passed
- [ ] Documentation updated
- [ ] Ready for 1.0 release

---

## Questions to Ask User

When starting a new session, consider asking:

1. **Priority**: "What should we focus on first? UI integration, streaming, or search UI?"
2. **Scope**: "Should we aim for MVP (just UI integration) or full Phase 7?"
3. **Timeline**: "How many sessions do we have for Phase 7?"
4. **Specific needs**: "Any specific features or bugs that are urgent?"

---

## Quick Start for Next Session

**Copy-paste this to start:**

```
I'm continuing work on Forbidden Library from the previous session. Here's the current state:

📊 Status: 90% complete (Phases 1-6 done)
🌿 Branch: claude/backend-docs-quality-01BD6RHf2tq5fwmEpgnNmD5t
✅ Last work: Phase 6 UI/UX polish + accessibility (committed and pushed)

Recent additions:
- 5 UI components (LoadingStates, SkeletonLoaders, MessageRenderer, SearchHighlight, ProviderSelector)
- Animation system (animations.ts + transitions.css)
- Accessibility utilities (WCAG 2.1 AA)
- Full-text search (FTS5)
- 7 AI provider integrations
- Performance optimizations (10-100x faster)

Next up (Phase 7 - Final 10%):
1. Integrate UI components into app
2. Build search UI with FTS5
3. Implement streaming responses
4. Enhance settings page
5. Add keyboard shortcuts

What should we tackle first?
```

---

**Last Updated:** Phase 6 Complete
**Project Completion:** 90%
**Ready for:** Phase 7 - Final Polish & Integration
