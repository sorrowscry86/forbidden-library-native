<script lang="ts">
  /**
   * SearchHighlight Component
   *
   * Highlights search terms in text with customizable styling and animations
   */

  import { onMount } from 'svelte';

  export let text: string;
  export let searchTerm: string = '';
  export let highlightClass: string = 'search-highlight';
  export let caseSensitive: boolean = false;
  export let wholeWord: boolean = false;
  export let maxLength: number | null = null;
  export let truncate: boolean = false;

  let highlightedHTML: string = '';

  function escapeRegExp(string: string): string {
    return string.replace(/[.*+?^${}()|[\]\\]/g, '\\$&');
  }

  function highlightText(content: string, term: string): string {
    if (!term || !content) {
      return escapeHTML(content);
    }

    const escapedTerm = escapeRegExp(term);
    const flags = caseSensitive ? 'g' : 'gi';
    const pattern = wholeWord ? `\\b${escapedTerm}\\b` : escapedTerm;
    const regex = new RegExp(pattern, flags);

    // Escape HTML first
    const escaped = escapeHTML(content);

    // Then highlight
    return escaped.replace(regex, (match) => {
      return `<mark class="${highlightClass}">${match}</mark>`;
    });
  }

  function escapeHTML(str: string): string {
    const div = document.createElement('div');
    div.textContent = str;
    return div.innerHTML;
  }

  function truncateText(content: string, maxLen: number): string {
    if (content.length <= maxLen) {
      return content;
    }

    // If search term exists, try to center around it
    if (searchTerm) {
      const index = content.toLowerCase().indexOf(searchTerm.toLowerCase());
      if (index !== -1) {
        const start = Math.max(0, index - Math.floor(maxLen / 2));
        const end = Math.min(content.length, start + maxLen);
        const truncated = content.substring(start, end);
        return (start > 0 ? '...' : '') + truncated + (end < content.length ? '...' : '');
      }
    }

    // Otherwise truncate from start
    return content.substring(0, maxLen) + '...';
  }

  $: {
    let processedText = text;

    // Truncate if needed
    if (truncate && maxLength) {
      processedText = truncateText(processedText, maxLength);
    }

    // Highlight search terms
    highlightedHTML = highlightText(processedText, searchTerm);
  }
</script>

<span class="search-highlight-container">
  {@html highlightedHTML}
</span>

<style>
  .search-highlight-container {
    display: inline;
  }

  :global(.search-highlight) {
    background: linear-gradient(120deg, #fef3c7 0%, #fde68a 100%);
    padding: 0.125rem 0.25rem;
    border-radius: 0.25rem;
    font-weight: 600;
    color: #92400e;
    animation: highlight-pulse 0.6s ease-out;
  }

  @keyframes highlight-pulse {
    0% {
      background: #fef3c7;
    }
    50% {
      background: #fcd34d;
    }
    100% {
      background: linear-gradient(120deg, #fef3c7 0%, #fde68a 100%);
    }
  }

  :global(.search-highlight-primary) {
    background: linear-gradient(120deg, #dbeafe 0%, #bfdbfe 100%);
    color: #1e40af;
  }

  :global(.search-highlight-success) {
    background: linear-gradient(120deg, #d1fae5 0%, #a7f3d0 100%);
    color: #065f46;
  }

  :global(.search-highlight-danger) {
    background: linear-gradient(120deg, #fee2e2 0%, #fecaca 100%);
    color: #991b1b;
  }
</style>
