/**
 * Animation Utilities for Forbidden Library
 *
 * Provides smooth transitions, micro-interactions, and animation helpers
 * for a polished user experience.
 */

import { cubicOut, cubicIn, cubicInOut } from 'svelte/easing';
import type { TransitionConfig } from 'svelte/transition';

// ============================================================================
// Transition Configurations
// ============================================================================

export interface FadeParams {
  delay?: number;
  duration?: number;
  easing?: (t: number) => number;
}

export interface SlideParams extends FadeParams {
  axis?: 'x' | 'y';
  distance?: number;
}

export interface ScaleParams extends FadeParams {
  start?: number;
  opacity?: number;
}

// ============================================================================
// Custom Transitions
// ============================================================================

/**
 * Enhanced fade transition with customizable easing
 */
export function fade(
  node: Element,
  { delay = 0, duration = 200, easing = cubicOut }: FadeParams = {}
): TransitionConfig {
  const o = +getComputedStyle(node).opacity;

  return {
    delay,
    duration,
    easing,
    css: (t) => `opacity: ${t * o}`
  };
}

/**
 * Slide transition with direction control
 */
export function slide(
  node: Element,
  { delay = 0, duration = 300, easing = cubicOut, axis = 'y', distance = 100 }: SlideParams = {}
): TransitionConfig {
  const style = getComputedStyle(node);
  const opacity = +style.opacity;
  const transform = axis === 'x'
    ? `translateX(${distance}px)`
    : `translateY(${distance}px)`;

  return {
    delay,
    duration,
    easing,
    css: (t, u) => `
      opacity: ${t * opacity};
      transform: ${axis === 'x' ? `translateX(${u * distance}px)` : `translateY(${u * distance}px)`};
    `
  };
}

/**
 * Scale transition with fade
 */
export function scale(
  node: Element,
  { delay = 0, duration = 200, easing = cubicOut, start = 0.95, opacity = 0 }: ScaleParams = {}
): TransitionConfig {
  const o = +getComputedStyle(node).opacity;

  return {
    delay,
    duration,
    easing,
    css: (t) => `
      opacity: ${(t * (o - opacity)) + opacity};
      transform: scale(${(t * (1 - start)) + start});
    `
  };
}

/**
 * Fly-in transition from a direction
 */
export function fly(
  node: Element,
  { delay = 0, duration = 300, easing = cubicOut, x = 0, y = 0, opacity = 0 }: {
    delay?: number;
    duration?: number;
    easing?: (t: number) => number;
    x?: number;
    y?: number;
    opacity?: number;
  } = {}
): TransitionConfig {
  const style = getComputedStyle(node);
  const target_opacity = +style.opacity;
  const transform = style.transform === 'none' ? '' : style.transform;

  return {
    delay,
    duration,
    easing,
    css: (t, u) => `
      transform: ${transform} translate(${u * x}px, ${u * y}px);
      opacity: ${(t * (target_opacity - opacity)) + opacity}
    `
  };
}

/**
 * Expand/collapse transition
 */
export function expand(
  node: Element,
  { delay = 0, duration = 300, easing = cubicOut }: FadeParams = {}
): TransitionConfig {
  const style = getComputedStyle(node);
  const opacity = +style.opacity;
  const height = parseFloat(style.height);
  const paddingTop = parseFloat(style.paddingTop);
  const paddingBottom = parseFloat(style.paddingBottom);
  const marginTop = parseFloat(style.marginTop);
  const marginBottom = parseFloat(style.marginBottom);
  const borderTopWidth = parseFloat(style.borderTopWidth);
  const borderBottomWidth = parseFloat(style.borderBottomWidth);

  return {
    delay,
    duration,
    easing,
    css: (t) => `
      overflow: hidden;
      opacity: ${t * opacity};
      height: ${t * height}px;
      padding-top: ${t * paddingTop}px;
      padding-bottom: ${t * paddingBottom}px;
      margin-top: ${t * marginTop}px;
      margin-bottom: ${t * marginBottom}px;
      border-top-width: ${t * borderTopWidth}px;
      border-bottom-width: ${t * borderBottomWidth}px;
    `
  };
}

/**
 * Blur transition
 */
export function blur(
  node: Element,
  { delay = 0, duration = 300, easing = cubicOut, amount = 5, opacity = 0 }: {
    delay?: number;
    duration?: number;
    easing?: (t: number) => number;
    amount?: number;
    opacity?: number;
  } = {}
): TransitionConfig {
  const style = getComputedStyle(node);
  const target_opacity = +style.opacity;

  return {
    delay,
    duration,
    easing,
    css: (t, u) => `
      opacity: ${(t * (target_opacity - opacity)) + opacity};
      filter: blur(${u * amount}px);
    `
  };
}

// ============================================================================
// Animation Utilities
// ============================================================================

/**
 * Create a stagger delay for list animations
 */
export function staggerDelay(index: number, baseDelay: number = 0, increment: number = 50): number {
  return baseDelay + (index * increment);
}

/**
 * Animate element on scroll into view
 */
export function animateOnScroll(
  element: Element,
  options: {
    threshold?: number;
    rootMargin?: string;
    animationClass?: string;
    once?: boolean;
  } = {}
): () => void {
  const {
    threshold = 0.1,
    rootMargin = '0px',
    animationClass = 'animate-in',
    once = true
  } = options;

  const observer = new IntersectionObserver(
    (entries) => {
      entries.forEach((entry) => {
        if (entry.isIntersecting) {
          entry.target.classList.add(animationClass);
          if (once) {
            observer.unobserve(entry.target);
          }
        } else if (!once) {
          entry.target.classList.remove(animationClass);
        }
      });
    },
    { threshold, rootMargin }
  );

  observer.observe(element);

  return () => observer.disconnect();
}

/**
 * Spring animation helper
 */
export function springAnimation(
  from: number,
  to: number,
  options: {
    stiffness?: number;
    damping?: number;
    mass?: number;
    onUpdate?: (value: number) => void;
    onComplete?: () => void;
  } = {}
): () => void {
  const {
    stiffness = 0.15,
    damping = 0.8,
    mass = 1,
    onUpdate = () => {},
    onComplete = () => {}
  } = options;

  let current = from;
  let velocity = 0;
  let frame: number;

  const animate = () => {
    const force = (to - current) * stiffness;
    velocity += force / mass;
    velocity *= damping;
    current += velocity;

    onUpdate(current);

    if (Math.abs(velocity) > 0.001 || Math.abs(to - current) > 0.001) {
      frame = requestAnimationFrame(animate);
    } else {
      onUpdate(to);
      onComplete();
    }
  };

  frame = requestAnimationFrame(animate);

  return () => cancelAnimationFrame(frame);
}

/**
 * Sequence multiple animations
 */
export async function sequenceAnimations(
  animations: Array<() => Promise<void> | void>,
  delay: number = 0
): Promise<void> {
  for (const animation of animations) {
    await animation();
    if (delay > 0) {
      await new Promise(resolve => setTimeout(resolve, delay));
    }
  }
}

/**
 * Parallax scroll effect
 */
export function parallaxScroll(
  element: HTMLElement,
  speed: number = 0.5
): () => void {
  const handleScroll = () => {
    const scrolled = window.pageYOffset;
    const rate = scrolled * speed;
    element.style.transform = `translateY(${rate}px)`;
  };

  window.addEventListener('scroll', handleScroll, { passive: true });
  return () => window.removeEventListener('scroll', handleScroll);
}

// ============================================================================
// Micro-interactions
// ============================================================================

/**
 * Ripple effect for button clicks
 */
export function rippleEffect(
  event: MouseEvent,
  element: HTMLElement,
  color: string = 'rgba(255, 255, 255, 0.5)'
): void {
  const rect = element.getBoundingClientRect();
  const x = event.clientX - rect.left;
  const y = event.clientY - rect.top;

  const ripple = document.createElement('span');
  ripple.style.cssText = `
    position: absolute;
    border-radius: 50%;
    background: ${color};
    width: 20px;
    height: 20px;
    left: ${x}px;
    top: ${y}px;
    transform: translate(-50%, -50%);
    pointer-events: none;
    animation: ripple 0.6s ease-out;
  `;

  element.style.position = 'relative';
  element.style.overflow = 'hidden';
  element.appendChild(ripple);

  setTimeout(() => ripple.remove(), 600);
}

/**
 * Shake animation for errors
 */
export function shakeElement(element: HTMLElement, duration: number = 500): void {
  element.style.animation = `shake ${duration}ms ease-in-out`;
  setTimeout(() => {
    element.style.animation = '';
  }, duration);
}

/**
 * Pulse animation for attention
 */
export function pulseElement(element: HTMLElement, times: number = 3): void {
  element.style.animation = `pulse 0.5s ease-in-out ${times}`;
  setTimeout(() => {
    element.style.animation = '';
  }, 500 * times);
}

/**
 * Highlight flash effect
 */
export function flashHighlight(
  element: HTMLElement,
  color: string = '#fef3c7',
  duration: number = 1000
): void {
  const originalBg = element.style.backgroundColor;
  element.style.transition = `background-color ${duration}ms ease-out`;
  element.style.backgroundColor = color;

  setTimeout(() => {
    element.style.backgroundColor = originalBg;
    setTimeout(() => {
      element.style.transition = '';
    }, duration);
  }, 50);
}

// ============================================================================
// CSS Animation Classes
// ============================================================================

export const animationClasses = {
  // Fade animations
  fadeIn: 'animate-fade-in',
  fadeOut: 'animate-fade-out',

  // Slide animations
  slideInLeft: 'animate-slide-in-left',
  slideInRight: 'animate-slide-in-right',
  slideInUp: 'animate-slide-in-up',
  slideInDown: 'animate-slide-in-down',

  // Scale animations
  scaleIn: 'animate-scale-in',
  scaleOut: 'animate-scale-out',

  // Rotate animations
  rotate: 'animate-rotate',

  // Bounce animations
  bounce: 'animate-bounce',

  // Shake animations
  shake: 'animate-shake',

  // Pulse animations
  pulse: 'animate-pulse'
};

// ============================================================================
// Keyframe Definitions (for injection into CSS)
// ============================================================================

export const keyframeDefinitions = `
@keyframes ripple {
  to {
    transform: translate(-50%, -50%) scale(4);
    opacity: 0;
  }
}

@keyframes shake {
  0%, 100% { transform: translateX(0); }
  10%, 30%, 50%, 70%, 90% { transform: translateX(-10px); }
  20%, 40%, 60%, 80% { transform: translateX(10px); }
}

@keyframes pulse {
  0%, 100% { opacity: 1; }
  50% { opacity: 0.5; }
}

@keyframes fade-in {
  from { opacity: 0; }
  to { opacity: 1; }
}

@keyframes fade-out {
  from { opacity: 1; }
  to { opacity: 0; }
}

@keyframes slide-in-left {
  from {
    transform: translateX(-100%);
    opacity: 0;
  }
  to {
    transform: translateX(0);
    opacity: 1;
  }
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

@keyframes slide-in-up {
  from {
    transform: translateY(100%);
    opacity: 0;
  }
  to {
    transform: translateY(0);
    opacity: 1;
  }
}

@keyframes slide-in-down {
  from {
    transform: translateY(-100%);
    opacity: 0;
  }
  to {
    transform: translateY(0);
    opacity: 1;
  }
}

@keyframes scale-in {
  from {
    transform: scale(0);
    opacity: 0;
  }
  to {
    transform: scale(1);
    opacity: 1;
  }
}

@keyframes scale-out {
  from {
    transform: scale(1);
    opacity: 1;
  }
  to {
    transform: scale(0);
    opacity: 0;
  }
}

@keyframes rotate {
  from { transform: rotate(0deg); }
  to { transform: rotate(360deg); }
}

@keyframes bounce {
  0%, 100% {
    transform: translateY(-25%);
    animation-timing-function: cubic-bezier(0.8, 0, 1, 1);
  }
  50% {
    transform: translateY(0);
    animation-timing-function: cubic-bezier(0, 0, 0.2, 1);
  }
}

.animate-fade-in {
  animation: fade-in 0.3s ease-out;
}

.animate-fade-out {
  animation: fade-out 0.3s ease-out;
}

.animate-slide-in-left {
  animation: slide-in-left 0.3s ease-out;
}

.animate-slide-in-right {
  animation: slide-in-right 0.3s ease-out;
}

.animate-slide-in-up {
  animation: slide-in-up 0.3s ease-out;
}

.animate-slide-in-down {
  animation: slide-in-down 0.3s ease-out;
}

.animate-scale-in {
  animation: scale-in 0.2s ease-out;
}

.animate-scale-out {
  animation: scale-out 0.2s ease-out;
}

.animate-rotate {
  animation: rotate 1s linear infinite;
}

.animate-bounce {
  animation: bounce 1s infinite;
}

.animate-shake {
  animation: shake 0.5s ease-in-out;
}

.animate-pulse {
  animation: pulse 2s cubic-bezier(0.4, 0, 0.6, 1) infinite;
}
`;
