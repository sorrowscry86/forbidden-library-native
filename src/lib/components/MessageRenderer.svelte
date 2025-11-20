<script lang="ts">
  import { onMount } from 'svelte';
  import { marked } from 'marked';
  import hljs from 'highlight.js';
  import katex from 'katex';

  export let content: string;
  export let role: 'user' | 'assistant' | 'system' = 'user';
  export let enableMath: boolean = true;
  export let enableCodeHighlight: boolean = true;
  export let enableMermaid: boolean = false;

  let renderedContent: string = '';
  let messageElement: HTMLElement;

  // Configure marked options
  const markedOptions: marked.MarkedOptions = {
    breaks: true,
    gfm: true,
    highlight: function(code: string, lang: string) {
      if (!enableCodeHighlight) return code;

      const language = hljs.getLanguage(lang) ? lang : 'plaintext';
      try {
        const highlighted = hljs.highlight(code, { language }).value;
        return `<div class="code-block-wrapper">
          <div class="code-block-header">
            <span class="code-block-language">${language}</span>
            <button class="code-block-copy" data-code="${escapeHtml(code)}" aria-label="Copy code">
              <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                <rect x="9" y="9" width="13" height="13" rx="2" ry="2"></rect>
                <path d="M5 15H4a2 2 0 0 1-2-2V4a2 2 0 0 1 2-2h9a2 2 0 0 1 2 2v1"></path>
              </svg>
            </button>
          </div>
          <pre><code class="hljs language-${language}">${highlighted}</code></pre>
        </div>`;
      } catch (e) {
        return code;
      }
    }
  };

  marked.setOptions(markedOptions);

  function escapeHtml(text: string): string {
    const map: { [key: string]: string } = {
      '&': '&amp;',
      '<': '&lt;',
      '>': '&gt;',
      '"': '&quot;',
      "'": '&#039;'
    };
    return text.replace(/[&<>"']/g, (m) => map[m]);
  }

  function renderMath(text: string): string {
    if (!enableMath) return text;

    // Render inline math: $...$
    text = text.replace(/\$([^\$]+)\$/g, (match, math) => {
      try {
        return katex.renderToString(math, { throwOnError: false, displayMode: false });
      } catch (e) {
        return match;
      }
    });

    // Render block math: $$...$$
    text = text.replace(/\$\$([^\$]+)\$\$/g, (match, math) => {
      try {
        return katex.renderToString(math, { throwOnError: false, displayMode: true });
      } catch (e) {
        return match;
      }
    });

    return text;
  }

  function processContent(text: string): string {
    let processed = text;

    // First, render math equations
    processed = renderMath(processed);

    // Then, render markdown
    processed = marked.parse(processed) as string;

    return processed;
  }

  function handleCopyCode(event: Event) {
    const target = event.target as HTMLElement;
    const button = target.closest('.code-block-copy') as HTMLButtonElement;

    if (button) {
      const code = button.getAttribute('data-code');
      if (code) {
        navigator.clipboard.writeText(code).then(() => {
          // Visual feedback
          const originalHTML = button.innerHTML;
          button.innerHTML = `
            <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
              <polyline points="20 6 9 17 4 12"></polyline>
            </svg>
          `;
          button.classList.add('copied');

          setTimeout(() => {
            button.innerHTML = originalHTML;
            button.classList.remove('copied');
          }, 2000);
        }).catch(err => {
          console.error('Failed to copy code:', err);
        });
      }
    }
  }

  onMount(() => {
    renderedContent = processContent(content);

    // Add click handlers for copy buttons
    if (messageElement) {
      messageElement.addEventListener('click', handleCopyCode);
    }

    return () => {
      if (messageElement) {
        messageElement.removeEventListener('click', handleCopyCode);
      }
    };
  });

  $: if (content) {
    renderedContent = processContent(content);
  }
</script>

<div
  class="message-content {role}"
  bind:this={messageElement}
  role="article"
  aria-label="{role} message"
>
  {@html renderedContent}
</div>

<style global>
  .message-content {
    line-height: 1.7;
    color: var(--text-color, #1f2937);
  }

  .message-content.user {
    background: var(--user-message-bg, #f3f4f6);
    padding: 1rem;
    border-radius: 0.5rem;
  }

  .message-content.assistant {
    background: transparent;
  }

  .message-content.system {
    background: var(--system-message-bg, #fef3c7);
    padding: 0.75rem;
    border-radius: 0.5rem;
    border-left: 4px solid var(--warning-color, #f59e0b);
  }

  /* Markdown Styles */
  .message-content h1,
  .message-content h2,
  .message-content h3,
  .message-content h4,
  .message-content h5,
  .message-content h6 {
    margin-top: 1.5em;
    margin-bottom: 0.5em;
    font-weight: 600;
    line-height: 1.25;
  }

  .message-content h1 { font-size: 1.875rem; }
  .message-content h2 { font-size: 1.5rem; }
  .message-content h3 { font-size: 1.25rem; }
  .message-content h4 { font-size: 1.125rem; }
  .message-content h5 { font-size: 1rem; }
  .message-content h6 { font-size: 0.875rem; }

  .message-content p {
    margin-bottom: 1em;
  }

  .message-content ul,
  .message-content ol {
    margin-bottom: 1em;
    padding-left: 2em;
  }

  .message-content li {
    margin-bottom: 0.5em;
  }

  .message-content a {
    color: var(--link-color, #3b82f6);
    text-decoration: underline;
    transition: color 0.2s ease;
  }

  .message-content a:hover {
    color: var(--link-hover-color, #2563eb);
  }

  .message-content blockquote {
    border-left: 4px solid var(--border-color, #e5e7eb);
    padding-left: 1rem;
    margin: 1em 0;
    color: var(--muted-color, #6b7280);
    font-style: italic;
  }

  .message-content table {
    border-collapse: collapse;
    width: 100%;
    margin: 1em 0;
  }

  .message-content th,
  .message-content td {
    border: 1px solid var(--border-color, #e5e7eb);
    padding: 0.5rem 1rem;
    text-align: left;
  }

  .message-content th {
    background: var(--table-header-bg, #f9fafb);
    font-weight: 600;
  }

  .message-content tr:nth-child(even) {
    background: var(--table-row-bg, #f9fafb);
  }

  /* Code Block Styles */
  .code-block-wrapper {
    position: relative;
    margin: 1em 0;
    border-radius: 0.5rem;
    overflow: hidden;
    background: var(--code-bg, #1e293b);
  }

  .code-block-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 0.5rem 1rem;
    background: var(--code-header-bg, #0f172a);
    border-bottom: 1px solid var(--code-border, #334155);
  }

  .code-block-language {
    font-size: 0.75rem;
    font-weight: 600;
    text-transform: uppercase;
    color: var(--code-language-color, #94a3b8);
    letter-spacing: 0.05em;
  }

  .code-block-copy {
    background: transparent;
    border: none;
    color: var(--code-copy-color, #94a3b8);
    cursor: pointer;
    padding: 0.25rem;
    display: flex;
    align-items: center;
    justify-content: center;
    transition: all 0.2s ease;
    border-radius: 0.25rem;
  }

  .code-block-copy:hover {
    background: var(--code-copy-hover-bg, #334155);
    color: var(--code-copy-hover-color, #e2e8f0);
  }

  .code-block-copy.copied {
    color: var(--success-color, #10b981);
  }

  .code-block-wrapper pre {
    margin: 0;
    padding: 1rem;
    overflow-x: auto;
    background: var(--code-bg, #1e293b);
  }

  .code-block-wrapper code {
    font-family: 'Fira Code', 'Consolas', 'Monaco', 'Courier New', monospace;
    font-size: 0.875rem;
    line-height: 1.7;
    color: var(--code-text-color, #e2e8f0);
  }

  /* Inline Code */
  .message-content :not(pre) > code {
    background: var(--inline-code-bg, #f1f5f9);
    color: var(--inline-code-color, #db2777);
    padding: 0.125rem 0.375rem;
    border-radius: 0.25rem;
    font-family: 'Fira Code', 'Consolas', 'Monaco', 'Courier New', monospace;
    font-size: 0.875em;
  }

  /* Math Rendering */
  .message-content .katex {
    font-size: 1.1em;
  }

  .message-content .katex-display {
    margin: 1em 0;
    overflow-x: auto;
    overflow-y: hidden;
  }

  /* Syntax Highlighting Theme (One Dark Pro inspired) */
  .hljs {
    background: #1e293b !important;
    color: #e2e8f0;
  }

  .hljs-comment,
  .hljs-quote {
    color: #64748b;
    font-style: italic;
  }

  .hljs-keyword,
  .hljs-selector-tag,
  .hljs-literal,
  .hljs-type {
    color: #c792ea;
  }

  .hljs-string,
  .hljs-title,
  .hljs-section {
    color: #a5d6ff;
  }

  .hljs-name,
  .hljs-attribute {
    color: #f9826c;
  }

  .hljs-variable,
  .hljs-template-variable {
    color: #f78c6c;
  }

  .hljs-number {
    color: #f78c6c;
  }

  .hljs-built_in,
  .hljs-builtin-name,
  .hljs-selector-id,
  .hljs-selector-attr,
  .hljs-selector-pseudo {
    color: #82aaff;
  }

  .hljs-function {
    color: #82aaff;
  }

  .hljs-tag {
    color: #c792ea;
  }

  .hljs-meta {
    color: #7fdbca;
  }

  .hljs-deletion {
    background: #ef4444;
  }

  .hljs-addition {
    background: #10b981;
  }

  .hljs-emphasis {
    font-style: italic;
  }

  .hljs-strong {
    font-weight: bold;
  }
</style>
