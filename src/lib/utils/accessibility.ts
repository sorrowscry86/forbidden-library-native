/**
 * Accessibility Utilities for Forbidden Library
 *
 * WCAG 2.1 Level AA compliance utilities and helpers
 */

// ============================================================================
// Keyboard Navigation
// ============================================================================

/**
 * Trap focus within a container (for modals and dialogs)
 */
export function trapFocus(container: HTMLElement): () => void {
  const focusableElements = container.querySelectorAll<HTMLElement>(
    'button, [href], input, select, textarea, [tabindex]:not([tabindex="-1"])'
  );

  const firstFocusable = focusableElements[0];
  const lastFocusable = focusableElements[focusableElements.length - 1];

  const handleKeyDown = (e: KeyboardEvent) => {
    if (e.key !== 'Tab') return;

    if (e.shiftKey) {
      // Shift + Tab
      if (document.activeElement === firstFocusable) {
        lastFocusable?.focus();
        e.preventDefault();
      }
    } else {
      // Tab
      if (document.activeElement === lastFocusable) {
        firstFocusable?.focus();
        e.preventDefault();
      }
    }
  };

  container.addEventListener('keydown', handleKeyDown);

  // Focus first element
  firstFocusable?.focus();

  return () => container.removeEventListener('keydown', handleKeyDown);
}

/**
 * Handle keyboard navigation for custom components
 */
export function handleArrowKeys(
  event: KeyboardEvent,
  items: NodeListOf<HTMLElement> | HTMLElement[],
  currentIndex: number,
  options: {
    loop?: boolean;
    horizontal?: boolean;
    onSelect?: (index: number) => void;
  } = {}
): number | null {
  const { loop = true, horizontal = false, onSelect } = options;
  const itemsArray = Array.from(items);
  let newIndex = currentIndex;

  const nextKey = horizontal ? 'ArrowRight' : 'ArrowDown';
  const prevKey = horizontal ? 'ArrowLeft' : 'ArrowUp';

  switch (event.key) {
    case nextKey:
      event.preventDefault();
      newIndex = currentIndex + 1;
      if (newIndex >= itemsArray.length) {
        newIndex = loop ? 0 : currentIndex;
      }
      break;

    case prevKey:
      event.preventDefault();
      newIndex = currentIndex - 1;
      if (newIndex < 0) {
        newIndex = loop ? itemsArray.length - 1 : currentIndex;
      }
      break;

    case 'Home':
      event.preventDefault();
      newIndex = 0;
      break;

    case 'End':
      event.preventDefault();
      newIndex = itemsArray.length - 1;
      break;

    case 'Enter':
    case ' ':
      event.preventDefault();
      onSelect?.(currentIndex);
      return currentIndex;

    default:
      return null;
  }

  if (newIndex !== currentIndex) {
    itemsArray[newIndex]?.focus();
    return newIndex;
  }

  return null;
}

// ============================================================================
// Screen Reader Utilities
// ============================================================================

/**
 * Announce message to screen readers
 */
export function announceToScreenReader(
  message: string,
  priority: 'polite' | 'assertive' = 'polite'
): void {
  const announcement = document.createElement('div');
  announcement.setAttribute('role', 'status');
  announcement.setAttribute('aria-live', priority);
  announcement.setAttribute('aria-atomic', 'true');
  announcement.className = 'sr-only';
  announcement.textContent = message;

  document.body.appendChild(announcement);

  // Remove after announcement
  setTimeout(() => {
    document.body.removeChild(announcement);
  }, 1000);
}

/**
 * Create visually hidden element for screen readers
 */
export function createScreenReaderOnly(text: string): HTMLElement {
  const element = document.createElement('span');
  element.className = 'sr-only';
  element.textContent = text;
  return element;
}

// ============================================================================
// ARIA Utilities
// ============================================================================

/**
 * Generate unique ID for ARIA relationships
 */
let idCounter = 0;
export function generateId(prefix: string = 'a11y'): string {
  return `${prefix}-${Date.now()}-${++idCounter}`;
}

/**
 * Set up ARIA label relationships
 */
export function linkLabelToControl(
  label: HTMLElement,
  control: HTMLElement
): void {
  const labelId = label.id || generateId('label');
  label.id = labelId;
  control.setAttribute('aria-labelledby', labelId);
}

/**
 * Set up ARIA description relationships
 */
export function linkDescriptionToControl(
  description: HTMLElement,
  control: HTMLElement
): void {
  const descId = description.id || generateId('desc');
  description.id = descId;
  control.setAttribute('aria-describedby', descId);
}

/**
 * Manage expanded state for disclosure widgets
 */
export function toggleExpanded(trigger: HTMLElement, content: HTMLElement): void {
  const isExpanded = trigger.getAttribute('aria-expanded') === 'true';
  trigger.setAttribute('aria-expanded', String(!isExpanded));
  content.hidden = isExpanded;
}

// ============================================================================
// Color Contrast Utilities
// ============================================================================

interface RGB {
  r: number;
  g: number;
  b: number;
}

/**
 * Convert hex color to RGB
 */
function hexToRgb(hex: string): RGB | null {
  const result = /^#?([a-f\d]{2})([a-f\d]{2})([a-f\d]{2})$/i.exec(hex);
  return result
    ? {
        r: parseInt(result[1], 16),
        g: parseInt(result[2], 16),
        b: parseInt(result[3], 16),
      }
    : null;
}

/**
 * Calculate relative luminance
 */
function getLuminance(rgb: RGB): number {
  const { r, g, b } = rgb;
  const [rs, gs, bs] = [r, g, b].map((c) => {
    const srgb = c / 255;
    return srgb <= 0.03928 ? srgb / 12.92 : Math.pow((srgb + 0.055) / 1.055, 2.4);
  });
  return 0.2126 * rs + 0.7152 * gs + 0.0722 * bs;
}

/**
 * Calculate contrast ratio between two colors
 */
export function getContrastRatio(color1: string, color2: string): number | null {
  const rgb1 = hexToRgb(color1);
  const rgb2 = hexToRgb(color2);

  if (!rgb1 || !rgb2) return null;

  const lum1 = getLuminance(rgb1);
  const lum2 = getLuminance(rgb2);

  const lighter = Math.max(lum1, lum2);
  const darker = Math.min(lum1, lum2);

  return (lighter + 0.05) / (darker + 0.05);
}

/**
 * Check if contrast ratio meets WCAG AA standard
 */
export function meetsWCAGAA(
  foreground: string,
  background: string,
  isLargeText: boolean = false
): boolean {
  const ratio = getContrastRatio(foreground, background);
  if (!ratio) return false;

  // WCAG AA requires 4.5:1 for normal text, 3:1 for large text
  return isLargeText ? ratio >= 3 : ratio >= 4.5;
}

/**
 * Check if contrast ratio meets WCAG AAA standard
 */
export function meetsWCAGAAA(
  foreground: string,
  background: string,
  isLargeText: boolean = false
): boolean {
  const ratio = getContrastRatio(foreground, background);
  if (!ratio) return false;

  // WCAG AAA requires 7:1 for normal text, 4.5:1 for large text
  return isLargeText ? ratio >= 4.5 : ratio >= 7;
}

// ============================================================================
// Form Accessibility
// ============================================================================

/**
 * Validate and enhance form accessibility
 */
export function enhanceFormAccessibility(form: HTMLFormElement): void {
  const inputs = form.querySelectorAll('input, select, textarea');

  inputs.forEach((input) => {
    const inputElement = input as HTMLInputElement;

    // Ensure every input has a label
    const label = form.querySelector(`label[for="${inputElement.id}"]`);
    if (!label && !inputElement.getAttribute('aria-label')) {
      console.warn(
        `Input ${inputElement.name || inputElement.id} is missing a label`
      );
    }

    // Add required indicator for required fields
    if (inputElement.required) {
      inputElement.setAttribute('aria-required', 'true');

      const labelElement = label as HTMLElement;
      if (labelElement && !labelElement.querySelector('.required-indicator')) {
        const indicator = document.createElement('span');
        indicator.className = 'required-indicator';
        indicator.setAttribute('aria-hidden', 'true');
        indicator.textContent = '*';
        labelElement.appendChild(indicator);
      }
    }

    // Add error handling
    inputElement.addEventListener('invalid', (e) => {
      e.preventDefault();
      const errorId = `${inputElement.id}-error`;
      let errorElement = form.querySelector(`#${errorId}`);

      if (!errorElement) {
        errorElement = document.createElement('div');
        errorElement.id = errorId;
        errorElement.className = 'error-message';
        errorElement.setAttribute('role', 'alert');
        inputElement.parentElement?.appendChild(errorElement);
      }

      errorElement.textContent = inputElement.validationMessage;
      inputElement.setAttribute('aria-describedby', errorId);
      inputElement.setAttribute('aria-invalid', 'true');
    });

    // Clear errors on input
    inputElement.addEventListener('input', () => {
      if (inputElement.validity.valid) {
        const errorId = `${inputElement.id}-error`;
        const errorElement = form.querySelector(`#${errorId}`);
        if (errorElement) {
          errorElement.textContent = '';
        }
        inputElement.removeAttribute('aria-invalid');
      }
    });
  });
}

// ============================================================================
// Focus Management
// ============================================================================

/**
 * Manage focus restoration
 */
export class FocusManager {
  private previousFocus: HTMLElement | null = null;

  save(): void {
    this.previousFocus = document.activeElement as HTMLElement;
  }

  restore(): void {
    if (this.previousFocus) {
      this.previousFocus.focus();
      this.previousFocus = null;
    }
  }

  moveTo(element: HTMLElement): void {
    this.save();
    element.focus();
  }
}

/**
 * Skip to main content link
 */
export function createSkipLink(targetId: string, text: string = 'Skip to main content'): HTMLAnchorElement {
  const skipLink = document.createElement('a');
  skipLink.href = `#${targetId}`;
  skipLink.className = 'skip-link';
  skipLink.textContent = text;

  skipLink.addEventListener('click', (e) => {
    e.preventDefault();
    const target = document.getElementById(targetId);
    if (target) {
      target.tabIndex = -1;
      target.focus();
      target.addEventListener('blur', () => {
        target.removeAttribute('tabindex');
      }, { once: true });
    }
  });

  return skipLink;
}

// ============================================================================
// Motion & Animation Preferences
// ============================================================================

/**
 * Check if user prefers reduced motion
 */
export function prefersReducedMotion(): boolean {
  return window.matchMedia('(prefers-reduced-motion: reduce)').matches;
}

/**
 * Respect motion preferences for animations
 */
export function respectMotionPreference<T>(
  withMotion: T,
  withoutMotion: T
): T {
  return prefersReducedMotion() ? withoutMotion : withMotion;
}

// ============================================================================
// Landmark Utilities
// ============================================================================

/**
 * Ensure proper landmark structure
 */
export function validateLandmarks(): void {
  const landmarks = {
    banner: document.querySelector('[role="banner"], header'),
    main: document.querySelector('[role="main"], main'),
    contentinfo: document.querySelector('[role="contentinfo"], footer'),
  };

  if (!landmarks.main) {
    console.warn('Page is missing a main landmark');
  }

  // Ensure main is unique
  const mains = document.querySelectorAll('[role="main"], main');
  if (mains.length > 1) {
    console.warn('Page has multiple main landmarks');
  }
}

// ============================================================================
// Accessible Notifications
// ============================================================================

export interface NotificationOptions {
  type?: 'info' | 'success' | 'warning' | 'error';
  duration?: number;
  action?: {
    label: string;
    onClick: () => void;
  };
}

/**
 * Show accessible notification
 */
export function showNotification(
  message: string,
  options: NotificationOptions = {}
): () => void {
  const { type = 'info', duration = 5000, action } = options;

  const notification = document.createElement('div');
  notification.className = `notification notification-${type}`;
  notification.setAttribute('role', 'status');
  notification.setAttribute('aria-live', type === 'error' ? 'assertive' : 'polite');
  notification.setAttribute('aria-atomic', 'true');

  const messageElement = document.createElement('p');
  messageElement.textContent = message;
  notification.appendChild(messageElement);

  if (action) {
    const actionButton = document.createElement('button');
    actionButton.textContent = action.label;
    actionButton.onclick = () => {
      action.onClick();
      dismiss();
    };
    notification.appendChild(actionButton);
  }

  const closeButton = document.createElement('button');
  closeButton.className = 'notification-close';
  closeButton.setAttribute('aria-label', 'Close notification');
  closeButton.textContent = '×';
  closeButton.onclick = dismiss;
  notification.appendChild(closeButton);

  document.body.appendChild(notification);

  let timeoutId: number | null = null;
  if (duration > 0) {
    timeoutId = window.setTimeout(dismiss, duration);
  }

  function dismiss() {
    if (timeoutId) clearTimeout(timeoutId);
    notification.classList.add('notification-dismissing');
    setTimeout(() => {
      document.body.removeChild(notification);
    }, 300);
  }

  return dismiss;
}

// ============================================================================
// Export CSS for screen reader only class
// ============================================================================

export const accessibilityStyles = `
.sr-only {
  position: absolute;
  width: 1px;
  height: 1px;
  padding: 0;
  margin: -1px;
  overflow: hidden;
  clip: rect(0, 0, 0, 0);
  white-space: nowrap;
  border-width: 0;
}

.skip-link {
  position: absolute;
  top: -40px;
  left: 0;
  background: #000;
  color: #fff;
  padding: 8px;
  text-decoration: none;
  z-index: 100;
}

.skip-link:focus {
  top: 0;
}

.required-indicator {
  color: #ef4444;
  margin-left: 4px;
}

.error-message {
  color: #ef4444;
  font-size: 0.875rem;
  margin-top: 4px;
}

.notification {
  position: fixed;
  bottom: 20px;
  right: 20px;
  padding: 16px;
  border-radius: 8px;
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.15);
  max-width: 400px;
  animation: slide-in-right 0.3s ease-out;
  z-index: 1000;
}

.notification-dismissing {
  animation: slide-out-right 0.3s ease-out;
}

.notification-info {
  background: #dbeafe;
  color: #1e40af;
}

.notification-success {
  background: #d1fae5;
  color: #065f46;
}

.notification-warning {
  background: #fef3c7;
  color: #92400e;
}

.notification-error {
  background: #fee2e2;
  color: #991b1b;
}

.notification-close {
  position: absolute;
  top: 8px;
  right: 8px;
  background: transparent;
  border: none;
  font-size: 1.5rem;
  cursor: pointer;
  opacity: 0.7;
}

.notification-close:hover {
  opacity: 1;
}

@keyframes slide-in-right {
  from {
    transform: translateX(100%);
    opacity: 0;
  }
  to {
    transform: translateX(0);
    opacity: 1;
  }
}

@keyframes slide-out-right {
  from {
    transform: translateX(0);
    opacity: 1;
  }
  to {
    transform: translateX(100%);
    opacity: 0;
  }
}
`;
