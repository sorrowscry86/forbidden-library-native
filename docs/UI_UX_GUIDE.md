# UI/UX Design Guide - Forbidden Library

## Overview

This guide documents the user interface and user experience design patterns, components, and best practices for the Forbidden Library application. Our design philosophy focuses on:

- **Clarity**: Clear visual hierarchy and intuitive interactions
- **Accessibility**: WCAG 2.1 Level AA compliance
- **Performance**: Smooth animations and responsive interactions
- **Consistency**: Unified design language across all components
- **Delight**: Micro-interactions and polished details

---

## Table of Contents

1. [Design Principles](#design-principles)
2. [Component Library](#component-library)
3. [Animation System](#animation-system)
4. [Accessibility Features](#accessibility-features)
5. [Color System](#color-system)
6. [Typography](#typography)
7. [Best Practices](#best-practices)

---

## Design Principles

### 1. Progressive Disclosure

Show only essential information by default, revealing additional details on demand:
- Collapsed sidebar navigation
- Expandable conversation details
- Contextual actions on hover

### 2. Feedback & Responsiveness

Every user action receives immediate visual feedback:
- Button states (hover, active, disabled)
- Loading states with animations
- Success/error notifications
- Progress indicators

### 3. Consistency

Maintain predictable patterns:
- Standard spacing scale (4px base unit)
- Consistent icon usage
- Unified interaction patterns
- Coherent color usage

### 4. Efficiency

Optimize for power users:
- Keyboard shortcuts
- Quick actions
- Search functionality
- Batch operations

---

## Component Library

### Loading States

**File**: `src/lib/components/LoadingStates.svelte`

Provides multiple loading state patterns for different contexts:

#### Variants

1. **Spinner** - Default rotating loader
```svelte
<LoadingStates type="spinner" size="md" />
```

2. **Dots** - Bouncing dots animation
```svelte
<LoadingStates type="dots" size="sm" text="Loading messages..." />
```

3. **Bars** - Pulsing bars (music visualizer style)
```svelte
<LoadingStates type="bars" size="lg" />
```

4. **Pulse** - Simple pulsing circle
```svelte
<LoadingStates type="pulse" size="md" />
```

5. **Skeleton** - Content placeholder
```svelte
<LoadingStates type="skeleton" />
```

#### Full-Screen Loading
```svelte
<LoadingStates type="spinner" fullScreen={true} text="Initializing..." />
```

#### Sizes
- `sm` - 16px (small inline elements)
- `md` - 32px (default, most use cases)
- `lg` - 48px (cards, sections)
- `xl` - 64px (full-page loading)

---

### Skeleton Loaders

**File**: `src/lib/components/SkeletonLoaders.svelte`

Content placeholders that match actual content structure:

#### Variants

1. **Conversation** - For conversation list items
```svelte
<SkeletonLoaders variant="conversation" count={5} />
```

2. **Message** - For chat messages
```svelte
<SkeletonLoaders variant="message" count={3} />
```

3. **Sidebar** - For sidebar navigation items
```svelte
<SkeletonLoaders variant="sidebar" count={8} />
```

4. **Settings** - For settings sections
```svelte
<SkeletonLoaders variant="settings" count={2} />
```

#### Best Practices
- Use skeletons for initial page loads
- Match skeleton structure to actual content
- Keep skeleton count realistic (3-5 items max)
- Remove skeletons immediately when data loads

---

### Message Renderer

**File**: `src/lib/components/MessageRenderer.svelte`

Advanced markdown and code rendering with syntax highlighting:

#### Features

1. **Markdown Support**
   - GitHub Flavored Markdown (GFM)
   - Headings, lists, tables
   - Blockquotes and links
   - Automatic line breaks

2. **Syntax Highlighting**
   - 180+ programming languages
   - One Dark Pro theme
   - Copy code button
   - Language badge

3. **Math Rendering**
   - Inline math: `$E = mc^2$`
   - Block math: `$$\int_0^\infty e^{-x^2} dx$$`
   - KaTeX rendering engine

#### Usage

```svelte
<MessageRenderer
  content={message.content}
  role="assistant"
  enableMath={true}
  enableCodeHighlight={true}
/>
```

#### Supported Languages

Auto-detected from code fence:
````markdown
```python
def hello_world():
    print("Hello, World!")
```
````

#### Code Block Features

- **Automatic language detection**
- **Copy to clipboard** - Click copy button in header
- **Syntax highlighting** - Color-coded syntax
- **Line numbers** - Optional via configuration
- **Horizontal scroll** - For long lines

---

### Search Highlight

**File**: `src/lib/components/SearchHighlight.svelte`

Highlights search terms in text with animations:

#### Basic Usage

```svelte
<SearchHighlight
  text="The quick brown fox jumps over the lazy dog"
  searchTerm="fox"
/>
```

#### Advanced Options

```svelte
<SearchHighlight
  text={content}
  searchTerm={query}
  highlightClass="search-highlight-primary"
  caseSensitive={false}
  wholeWord={true}
  truncate={true}
  maxLength={200}
/>
```

#### Highlight Variants

- `search-highlight` - Default yellow gradient
- `search-highlight-primary` - Blue gradient
- `search-highlight-success` - Green gradient
- `search-highlight-danger` - Red gradient

#### Features

- **Regex support** - Advanced pattern matching
- **Whole word matching** - Optional boundary detection
- **Case sensitivity** - Toggle via prop
- **Text truncation** - Auto-center around match
- **Animation** - Pulse effect on render

---

### Provider Selector

**File**: `src/lib/components/ProviderSelector.svelte`

AI provider selection and configuration UI:

#### Supported Providers

1. **OpenAI** 🤖
   - GPT-4 Turbo, GPT-4, GPT-3.5 Turbo
   - Requires API key
   - Optional organization ID

2. **Anthropic (Claude)** 🎭
   - Claude 3.5 Sonnet, Opus, Haiku
   - Requires API key

3. **Google Gemini** 💎
   - Gemini 1.5 Pro, Flash
   - Requires API key

4. **Azure OpenAI** ☁️
   - Custom deployment
   - Requires endpoint + API key

5. **LM Studio** 🖥️
   - Local models
   - Free (no API key)

6. **Ollama** 🦙
   - Local models (Llama, Mistral, etc.)
   - Free (no API key)

7. **OpenAI Compatible** 🔌
   - Any compatible API
   - Optional API key

#### Usage

```svelte
<ProviderSelector
  bind:selectedProvider
  bind:apiKey
  bind:selectedModel
  bind:showConfig
  on:providerSelected={handleProviderChange}
  on:modelSelected={handleModelChange}
  on:configSaved={handleSaveConfig}
/>
```

#### Events

- `providerSelected` - Fires when provider is clicked
- `modelSelected` - Fires when model is chosen
- `configSaved` - Fires when configuration is saved

#### Validation

- Required fields marked with asterisk (*)
- Real-time validation feedback
- Disabled save until valid

---

## Animation System

### Transition Utilities

**File**: `src/lib/utils/animations.ts`

Comprehensive animation toolkit built on Svelte transitions:

#### Custom Transitions

```typescript
import { fade, slide, scale, fly, expand, blur } from '$lib/utils/animations';
```

##### Fade
```svelte
<div transition:fade={{ duration: 200 }}>Content</div>
```

##### Slide
```svelte
<div transition:slide={{ axis: 'x', distance: 100 }}>Content</div>
```

##### Scale
```svelte
<div transition:scale={{ start: 0.95, opacity: 0 }}>Content</div>
```

##### Fly
```svelte
<div transition:fly={{ x: -100, y: 0 }}>Content</div>
```

##### Expand/Collapse
```svelte
<div transition:expand={{ duration: 300 }}>Content</div>
```

##### Blur
```svelte
<div transition:blur={{ amount: 5 }}>Content</div>
```

#### Animation Helpers

**Stagger Delay** - For list animations
```typescript
import { staggerDelay } from '$lib/utils/animations';

{#each items as item, i}
  <div style="animation-delay: {staggerDelay(i, 0, 50)}ms">
    {item}
  </div>
{/each}
```

**Animate on Scroll** - Intersection Observer based
```typescript
import { animateOnScroll } from '$lib/utils/animations';

onMount(() => {
  const cleanup = animateOnScroll(element, {
    threshold: 0.1,
    animationClass: 'fade-in',
    once: true
  });

  return cleanup;
});
```

**Spring Animation** - Physics-based motion
```typescript
import { springAnimation } from '$lib/utils/animations';

const cancel = springAnimation(0, 100, {
  stiffness: 0.15,
  damping: 0.8,
  onUpdate: (value) => {
    element.style.transform = `translateX(${value}px)`;
  }
});
```

**Sequence Animations** - Chain animations
```typescript
import { sequenceAnimations } from '$lib/utils/animations';

await sequenceAnimations([
  async () => fadeIn(),
  async () => slideUp(),
  async () => scaleIn()
], 100); // 100ms delay between each
```

#### Micro-Interactions

**Ripple Effect** - Material Design ripple
```typescript
import { rippleEffect } from '$lib/utils/animations';

function handleClick(event: MouseEvent) {
  rippleEffect(event, buttonElement);
}
```

**Shake Animation** - Error feedback
```typescript
import { shakeElement } from '$lib/utils/animations';

if (error) {
  shakeElement(formElement);
}
```

**Pulse Animation** - Draw attention
```typescript
import { pulseElement } from '$lib/utils/animations';

pulseElement(notificationElement, 3); // Pulse 3 times
```

**Flash Highlight** - Highlight new content
```typescript
import { flashHighlight } from '$lib/utils/animations';

flashHighlight(newMessageElement, '#fef3c7', 1000);
```

---

### Global Transitions

**File**: `src/lib/styles/transitions.css`

Pre-built CSS animations and transitions:

#### Hover Effects

```html
<button class="hover-lift">Lift on Hover</button>
<button class="hover-scale">Scale on Hover</button>
<button class="hover-grow">Grow on Hover</button>
<button class="hover-glow">Glow on Hover</button>
<a class="hover-underline">Underline on Hover</a>
```

#### Animation Classes

```html
<!-- Fade -->
<div class="fade-in">Fades in</div>
<div class="fade-out">Fades out</div>

<!-- Slide -->
<div class="slide-in-left">Slides from left</div>
<div class="slide-in-right">Slides from right</div>
<div class="slide-in-up">Slides up</div>
<div class="slide-in-down">Slides down</div>

<!-- Scale -->
<div class="scale-in">Scales in</div>
<div class="pop-in">Pops in with bounce</div>

<!-- Motion -->
<div class="shake">Shakes</div>
<div class="wiggle">Wiggles</div>
<div class="spin">Spins continuously</div>
<div class="pulse">Pulses opacity</div>
<div class="bounce">Bounces vertically</div>
```

#### Stagger Children

```html
<div class="stagger-children">
  <div>Item 1 (0ms delay)</div>
  <div>Item 2 (50ms delay)</div>
  <div>Item 3 (100ms delay)</div>
</div>
```

#### Loading States

```html
<div class="loading">Button text hidden, spinner shown</div>
<div class="skeleton">Animated skeleton placeholder</div>
```

#### Card Interactions

```html
<div class="card-interactive">
  Interactive card with lift and shadow
</div>
```

---

## Accessibility Features

**File**: `src/lib/utils/accessibility.ts`

WCAG 2.1 Level AA compliance utilities:

### Keyboard Navigation

#### Focus Trap
```typescript
import { trapFocus } from '$lib/utils/accessibility';

const cleanup = trapFocus(modalElement);
// User can only tab within modal
// Call cleanup() when modal closes
```

#### Arrow Key Navigation
```typescript
import { handleArrowKeys } from '$lib/utils/accessibility';

let currentIndex = 0;

function onKeyDown(e: KeyboardEvent) {
  const newIndex = handleArrowKeys(e, menuItems, currentIndex, {
    loop: true,
    horizontal: false,
    onSelect: (index) => selectItem(index)
  });

  if (newIndex !== null) {
    currentIndex = newIndex;
  }
}
```

### Screen Reader Support

#### Announce Messages
```typescript
import { announceToScreenReader } from '$lib/utils/accessibility';

// Polite announcement (default)
announceToScreenReader('3 new messages');

// Assertive announcement (interrupts)
announceToScreenReader('Error: Failed to save', 'assertive');
```

#### Screen Reader Only Content
```typescript
import { createScreenReaderOnly } from '$lib/utils/accessibility';

const srText = createScreenReaderOnly('Opens in new window');
linkElement.appendChild(srText);
```

### ARIA Utilities

#### Generate IDs
```typescript
import { generateId } from '$lib/utils/accessibility';

const labelId = generateId('label'); // "label-1638123456789-1"
```

#### Link Label to Control
```typescript
import { linkLabelToControl } from '$lib/utils/accessibility';

linkLabelToControl(labelElement, inputElement);
// Sets label.id and input.aria-labelledby
```

#### Link Description
```typescript
import { linkDescriptionToControl } from '$lib/utils/accessibility';

linkDescriptionToControl(helpTextElement, inputElement);
// Sets description.id and input.aria-describedby
```

#### Toggle Expanded
```typescript
import { toggleExpanded } from '$lib/utils/accessibility';

toggleExpanded(accordionTrigger, accordionContent);
// Toggles aria-expanded and content.hidden
```

### Color Contrast

#### Check Contrast Ratio
```typescript
import { getContrastRatio, meetsWCAGAA, meetsWCAGAAA } from '$lib/utils/accessibility';

const ratio = getContrastRatio('#000000', '#FFFFFF');
// Returns: 21 (perfect contrast)

const isAACompliant = meetsWCAGAA('#333333', '#FFFFFF', false);
// Returns: true (7.7:1 ratio for normal text)

const isAAACompliant = meetsWCAGAAA('#333333', '#FFFFFF', false);
// Returns: true (7.7:1 ratio exceeds AAA requirement)
```

#### WCAG Standards

**Normal Text** (< 18pt or < 14pt bold):
- AA: 4.5:1 minimum
- AAA: 7:1 minimum

**Large Text** (≥ 18pt or ≥ 14pt bold):
- AA: 3:1 minimum
- AAA: 4.5:1 minimum

### Form Accessibility

#### Enhance Form
```typescript
import { enhanceFormAccessibility } from '$lib/utils/accessibility';

enhanceFormAccessibility(formElement);
```

**Features added:**
- Validates all inputs have labels
- Adds required indicators
- Sets up error messages with ARIA
- Links errors to inputs
- Adds aria-required to required fields

### Focus Management

```typescript
import { FocusManager } from '$lib/utils/accessibility';

const focusManager = new FocusManager();

// Save current focus
focusManager.save();

// Move focus somewhere else
focusManager.moveTo(dialogElement);

// Restore previous focus (e.g., when dialog closes)
focusManager.restore();
```

### Skip Links

```typescript
import { createSkipLink } from '$lib/utils/accessibility';

const skipLink = createSkipLink('main-content', 'Skip to main content');
document.body.insertBefore(skipLink, document.body.firstChild);
```

### Motion Preferences

```typescript
import { prefersReducedMotion, respectMotionPreference } from '$lib/utils/accessibility';

if (prefersReducedMotion()) {
  // Use instant transitions
}

const duration = respectMotionPreference(300, 0);
// Returns 300ms if motion OK, 0ms if reduced motion
```

### Notifications

```typescript
import { showNotification } from '$lib/utils/accessibility';

const dismiss = showNotification('Message sent successfully!', {
  type: 'success',
  duration: 5000,
  action: {
    label: 'Undo',
    onClick: () => undoSend()
  }
});

// Manually dismiss
dismiss();
```

**Notification Types:**
- `info` - Blue (aria-live="polite")
- `success` - Green (aria-live="polite")
- `warning` - Yellow (aria-live="polite")
- `error` - Red (aria-live="assertive")

---

## Color System

### Primary Palette

```css
:root {
  /* Primary - Blue */
  --primary-50: #eff6ff;
  --primary-100: #dbeafe;
  --primary-200: #bfdbfe;
  --primary-300: #93c5fd;
  --primary-400: #60a5fa;
  --primary-500: #3b82f6; /* Main brand color */
  --primary-600: #2563eb;
  --primary-700: #1d4ed8;
  --primary-800: #1e40af;
  --primary-900: #1e3a8a;
}
```

### Semantic Colors

```css
:root {
  /* Success - Green */
  --success: #10b981;
  --success-bg: #d1fae5;
  --success-text: #065f46;

  /* Warning - Yellow */
  --warning: #f59e0b;
  --warning-bg: #fef3c7;
  --warning-text: #92400e;

  /* Error - Red */
  --error: #ef4444;
  --error-bg: #fee2e2;
  --error-text: #991b1b;

  /* Info - Blue */
  --info: #3b82f6;
  --info-bg: #dbeafe;
  --info-text: #1e40af;
}
```

### Neutral Palette

```css
:root {
  --gray-50: #f9fafb;
  --gray-100: #f3f4f6;
  --gray-200: #e5e7eb;
  --gray-300: #d1d5db;
  --gray-400: #9ca3af;
  --gray-500: #6b7280;
  --gray-600: #4b5563;
  --gray-700: #374151;
  --gray-800: #1f2937;
  --gray-900: #111827;
}
```

### Usage Guidelines

- **Primary**: Main actions, links, selected states
- **Success**: Confirmations, success messages
- **Warning**: Warnings, non-critical alerts
- **Error**: Errors, destructive actions
- **Gray**: Text, borders, backgrounds

---

## Typography

### Font Stack

```css
:root {
  --font-sans: -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, "Helvetica Neue", Arial, sans-serif;
  --font-mono: "Fira Code", "Consolas", "Monaco", "Courier New", monospace;
}
```

### Type Scale

```css
:root {
  --text-xs: 0.75rem;    /* 12px */
  --text-sm: 0.875rem;   /* 14px */
  --text-base: 1rem;     /* 16px */
  --text-lg: 1.125rem;   /* 18px */
  --text-xl: 1.25rem;    /* 20px */
  --text-2xl: 1.5rem;    /* 24px */
  --text-3xl: 1.875rem;  /* 30px */
  --text-4xl: 2.25rem;   /* 36px */
}
```

### Font Weights

```css
:root {
  --font-normal: 400;
  --font-medium: 500;
  --font-semibold: 600;
  --font-bold: 700;
}
```

### Line Heights

```css
:root {
  --leading-tight: 1.25;
  --leading-normal: 1.5;
  --leading-relaxed: 1.7;
}
```

### Usage

```css
.heading-1 {
  font-size: var(--text-4xl);
  font-weight: var(--font-bold);
  line-height: var(--leading-tight);
}

.body-text {
  font-size: var(--text-base);
  font-weight: var(--font-normal);
  line-height: var(--leading-relaxed);
}

.code-text {
  font-family: var(--font-mono);
  font-size: var(--text-sm);
}
```

---

## Best Practices

### Component Design

1. **Single Responsibility**
   - Each component does one thing well
   - Composable and reusable
   - Clear props interface

2. **Prop Validation**
   - TypeScript types for all props
   - Sensible defaults
   - Optional vs required clearly marked

3. **Event Handling**
   - Use Svelte event dispatcher
   - Descriptive event names
   - Include relevant data in event detail

4. **Styling**
   - Scoped styles by default
   - Use CSS custom properties for themes
   - Mobile-first responsive design

### Animation Guidelines

1. **Duration**
   - **Fast** (150ms): Micro-interactions, tooltips
   - **Base** (200ms): Hover states, simple transitions
   - **Slow** (300ms): Page transitions, complex animations
   - **Slower** (500ms): Full-page overlays

2. **Easing**
   - **ease-out**: Entrances (feels snappy)
   - **ease-in**: Exits (feels smooth)
   - **ease-in-out**: Bidirectional (feels balanced)
   - **ease-bounce**: Playful interactions

3. **Performance**
   - Use `transform` and `opacity` for animations
   - Avoid animating `width`, `height`, `top`, `left`
   - Use `will-change` sparingly
   - Test on low-end devices

4. **Respect Preferences**
   - Always check `prefers-reduced-motion`
   - Provide instant alternatives
   - Keep animations subtle

### Accessibility Checklist

- [ ] All interactive elements are keyboard accessible
- [ ] Focus indicators are visible and clear
- [ ] Color is not the only indicator
- [ ] All images have alt text
- [ ] Headings are in logical order (h1 → h2 → h3)
- [ ] Forms have proper labels
- [ ] ARIA attributes used correctly
- [ ] Semantic HTML elements used
- [ ] Contrast ratios meet WCAG AA
- [ ] Screen reader tested

### Performance Optimization

1. **Loading States**
   - Show skeleton loaders immediately
   - Replace with real content when ready
   - Avoid layout shifts (reserve space)

2. **Code Splitting**
   - Lazy load heavy components
   - Preload on hover for instant feel
   - Bundle critical CSS inline

3. **Virtual Scrolling**
   - Use for lists > 100 items
   - Render only visible + overscan
   - Maintain scroll position

4. **Debounce & Throttle**
   - Debounce search inputs (300ms)
   - Throttle scroll handlers (100ms)
   - Throttle resize handlers (100ms)

---

## Examples

### Complete Modal Example

```svelte
<script lang="ts">
  import { trapFocus, FocusManager } from '$lib/utils/accessibility';
  import { scale, fade } from '$lib/utils/animations';
  import { onMount } from 'svelte';

  export let open = false;
  export let title = '';

  let modalElement: HTMLElement;
  const focusManager = new FocusManager();

  $: if (open) {
    focusManager.save();
  }

  function close() {
    open = false;
    focusManager.restore();
  }

  onMount(() => {
    if (open && modalElement) {
      const cleanup = trapFocus(modalElement);
      return cleanup;
    }
  });
</script>

{#if open}
  <div class="modal-backdrop" transition:fade on:click={close}>
    <div
      class="modal"
      bind:this={modalElement}
      transition:scale={{ duration: 200 }}
      on:click|stopPropagation
      role="dialog"
      aria-modal="true"
      aria-labelledby="modal-title"
    >
      <h2 id="modal-title">{title}</h2>
      <slot />
      <button on:click={close} aria-label="Close dialog">Close</button>
    </div>
  </div>
{/if}
```

---

## Changelog

### Version 1.0.0 (Phase 6)

**Added:**
- Loading state components (5 variants)
- Skeleton loaders (4 variants)
- Message renderer with syntax highlighting
- Search highlighting component
- Provider selector UI
- Animation utility library
- Global transition styles
- Accessibility utilities (WCAG 2.1 AA)
- Color contrast checking
- Focus management
- Keyboard navigation helpers
- Screen reader utilities
- Complete design system documentation

**Design System:**
- Color palette (primary, semantic, neutral)
- Typography scale
- Spacing system
- Animation timing standards

---

## Resources

### External Links

- [WCAG 2.1 Guidelines](https://www.w3.org/WAI/WCAG21/quickref/)
- [Svelte Transitions](https://svelte.dev/docs#run-time-svelte-transition)
- [Highlight.js Languages](https://highlightjs.org/static/demo/)
- [KaTeX Documentation](https://katex.org/docs/supported.html)
- [ARIA Authoring Practices](https://www.w3.org/WAI/ARIA/apg/)

### Internal Documentation

- [API Documentation](./API.md)
- [Performance Guide](./PERFORMANCE.md)
- [Deployment Guide](./DEPLOYMENT.md)
- [Troubleshooting Guide](./TROUBLESHOOTING.md)

---

**Last Updated**: Phase 6 - Final Polish
**Maintained By**: Forbidden Library Team
