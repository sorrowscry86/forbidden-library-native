import { render, screen } from '@testing-library/svelte';
import { describe, it, expect } from 'vitest';
import MessageBubble from './MessageBubble.svelte';
import type { Message } from '$lib/types/models';

describe('MessageBubble', () => {
  const baseMessage: Message = {
    id: 1,
    conversation_id: 1,
    role: 'user',
    content: 'Hello, world!',
    created_at: '2025-01-01T12:00:00Z',
    tokens_used: 10,
    model_used: 'gpt-4'
  };

  describe('rendering', () => {
    it('should render user message with correct content', () => {
      render(MessageBubble, { props: { message: baseMessage } });
      expect(screen.getByText('Hello, world!')).toBeInTheDocument();
    });

    it('should render assistant message with correct styling', () => {
      const assistantMessage: Message = {
        ...baseMessage,
        role: 'assistant'
      };
      const { container } = render(MessageBubble, { props: { message: assistantMessage } });

      // Check for assistant-specific styling (gray background)
      const bubble = container.querySelector('.bg-gray-700');
      expect(bubble).toBeInTheDocument();
    });

    it('should render system message with correct styling', () => {
      const systemMessage: Message = {
        ...baseMessage,
        role: 'system',
        content: 'System initialized'
      };
      const { container } = render(MessageBubble, { props: { message: systemMessage } });

      // Check for system-specific styling (red background)
      const bubble = container.querySelector('.bg-red-600');
      expect(bubble).toBeInTheDocument();
      expect(screen.getByText('System initialized')).toBeInTheDocument();
    });

    it('should display timestamp in correct format', () => {
      const message: Message = {
        ...baseMessage,
        created_at: '2025-01-01T14:30:00Z'
      };
      render(MessageBubble, { props: { message } });

      // The timestamp format depends on locale, so we just check it exists
      const timeElement = screen.getByText(/\d{1,2}:\d{2}/);
      expect(timeElement).toBeInTheDocument();
    });

    it('should display token count when provided', () => {
      const message: Message = {
        ...baseMessage,
        tokens_used: 150
      };
      render(MessageBubble, { props: { message } });

      expect(screen.getByText('150 tokens')).toBeInTheDocument();
    });

    it('should display model name when provided', () => {
      const message: Message = {
        ...baseMessage,
        model_used: 'claude-3-opus'
      };
      render(MessageBubble, { props: { message } });

      expect(screen.getByText('claude-3-opus')).toBeInTheDocument();
    });

    it('should not display mock-model name', () => {
      const message: Message = {
        ...baseMessage,
        model_used: 'mock-model'
      };
      render(MessageBubble, { props: { message } });

      expect(screen.queryByText('mock-model')).not.toBeInTheDocument();
    });

    it('should render multi-line content correctly', () => {
      const message: Message = {
        ...baseMessage,
        content: 'Line 1\nLine 2\nLine 3'
      };
      render(MessageBubble, { props: { message } });

      const contentElement = screen.getByText(/Line 1/);
      expect(contentElement).toBeInTheDocument();
      expect(contentElement.className).toContain('whitespace-pre-wrap');
    });
  });

  describe('role handling', () => {
    it('should align user messages to the right', () => {
      const { container } = render(MessageBubble, { props: { message: baseMessage } });

      const wrapper = container.querySelector('.justify-end');
      expect(wrapper).toBeInTheDocument();
    });

    it('should align assistant messages to the left', () => {
      const assistantMessage: Message = {
        ...baseMessage,
        role: 'assistant'
      };
      const { container } = render(MessageBubble, { props: { message: assistantMessage } });

      const wrapper = container.querySelector('.justify-start');
      expect(wrapper).toBeInTheDocument();
    });

    it('should handle case-insensitive role names', () => {
      const message: Message = {
        ...baseMessage,
        role: 'USER'
      };
      const { container } = render(MessageBubble, { props: { message } });

      // Should still render as user message (right-aligned)
      const wrapper = container.querySelector('.justify-end');
      expect(wrapper).toBeInTheDocument();
    });

    it('should sanitize invalid role to prevent XSS', () => {
      const message: Message = {
        ...baseMessage,
        role: '<script>alert("xss")</script>'
      };
      const { container } = render(MessageBubble, { props: { message } });

      // Invalid role doesn't match 'user', so it renders as non-user (left-aligned)
      const wrapper = container.querySelector('.justify-start');
      expect(wrapper).toBeInTheDocument();

      // Should not execute or render script tag
      expect(container.innerHTML).not.toContain('<script>');
    });
  });

  describe('metadata display', () => {
    it('should hide metadata when not provided', () => {
      const message: Message = {
        ...baseMessage,
        tokens_used: null,
        model_used: null
      };
      render(MessageBubble, { props: { message } });

      // Only timestamp should be visible
      expect(screen.queryByText(/tokens/)).not.toBeInTheDocument();
      expect(screen.queryByText(/gpt/)).not.toBeInTheDocument();
    });

    it('should display all metadata when available', () => {
      const message: Message = {
        ...baseMessage,
        tokens_used: 250,
        model_used: 'claude-sonnet'
      };
      render(MessageBubble, { props: { message } });

      expect(screen.getByText('250 tokens')).toBeInTheDocument();
      expect(screen.getByText('claude-sonnet')).toBeInTheDocument();
    });
  });

  describe('accessibility', () => {
    it('should have proper role icons for screen readers', () => {
      const { container } = render(MessageBubble, { props: { message: baseMessage } });

      // Check that SVG icons are present
      const svg = container.querySelector('svg');
      expect(svg).toBeInTheDocument();
    });

    it('should maintain readable text contrast', () => {
      const { container } = render(MessageBubble, { props: { message: baseMessage } });

      // Check for text color classes that ensure readability
      const bubble = container.querySelector('.text-white');
      expect(bubble).toBeInTheDocument();
    });
  });

  describe('edge cases', () => {
    it('should handle empty content gracefully', () => {
      const message: Message = {
        ...baseMessage,
        content: ''
      };
      const { container } = render(MessageBubble, { props: { message } });

      // Should still render the bubble structure
      const bubble = container.querySelector('.rounded-lg');
      expect(bubble).toBeInTheDocument();
    });

    it('should handle very long content without breaking layout', () => {
      const longContent = 'a'.repeat(1000);
      const message: Message = {
        ...baseMessage,
        content: longContent
      };
      const { container } = render(MessageBubble, { props: { message } });

      // Check for word break class
      const content = container.querySelector('.break-words');
      expect(content).toBeInTheDocument();
    });

    it('should handle zero tokens', () => {
      const message: Message = {
        ...baseMessage,
        tokens_used: 0
      };
      render(MessageBubble, { props: { message } });

      // 0 is falsy, so tokens won't be displayed (current behavior)
      // This could be improved in the future by checking !== null/undefined instead
      expect(screen.queryByText('0 tokens')).not.toBeInTheDocument();
    });
  });
});
