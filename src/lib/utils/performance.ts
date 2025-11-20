/**
 * Frontend Performance Utilities
 *
 * Provides utilities for optimizing frontend performance including:
 * - Lazy loading components and modules
 * - Image optimization
 * - Debouncing and throttling
 * - Performance monitoring
 */

/**
 * Lazy load a Svelte component
 *
 * @param loader - Function that returns a promise of the component
 * @returns Promise that resolves to the component
 *
 * @example
 * ```typescript
 * const HeavyComponent = lazyLoad(() => import('./HeavyComponent.svelte'));
 * ```
 */
export function lazyLoad<T>(loader: () => Promise<{ default: T }>): () => Promise<T> {
  return async () => {
    const module = await loader();
    return module.default;
  };
}

/**
 * Preload a module or component
 *
 * @param loader - Function that returns a promise of the module
 *
 * @example
 * ```typescript
 * // Preload on hover
 * <button on:mouseenter={() => preload(() => import('./Modal.svelte'))}>
 *   Open Modal
 * </button>
 * ```
 */
export function preload<T>(loader: () => Promise<T>): void {
  // Start loading but don't wait for it
  loader().catch(error => {
    console.warn('Preload failed:', error);
  });
}

/**
 * Debounce a function call
 *
 * @param fn - Function to debounce
 * @param delay - Delay in milliseconds
 * @returns Debounced function
 *
 * @example
 * ```typescript
 * const handleSearch = debounce((query: string) => {
 *   performSearch(query);
 * }, 300);
 * ```
 */
export function debounce<T extends (...args: unknown[]) => unknown>(
  fn: T,
  delay: number
): (...args: Parameters<T>) => void {
  let timeoutId: ReturnType<typeof setTimeout>;

  return function(this: unknown, ...args: Parameters<T>) {
    clearTimeout(timeoutId);
    timeoutId = setTimeout(() => fn.apply(this, args), delay);
  };
}

/**
 * Throttle a function call
 *
 * @param fn - Function to throttle
 * @param delay - Delay in milliseconds
 * @returns Throttled function
 *
 * @example
 * ```typescript
 * const handleScroll = throttle(() => {
 *   updateScrollPosition();
 * }, 100);
 * ```
 */
export function throttle<T extends (...args: unknown[]) => unknown>(
  fn: T,
  delay: number
): (...args: Parameters<T>) => void {
  let lastCall = 0;

  return function(this: unknown, ...args: Parameters<T>) {
    const now = Date.now();

    if (now - lastCall >= delay) {
      lastCall = now;
      fn.apply(this, args);
    }
  };
}

/**
 * Intersection Observer utility for lazy loading elements
 *
 * @param element - Element to observe
 * @param callback - Callback when element intersects
 * @param options - Intersection observer options
 * @returns Cleanup function
 *
 * @example
 * ```typescript
 * const cleanup = observeIntersection(imageEl, (entry) => {
 *   if (entry.isIntersecting) {
 *     loadImage(imageEl);
 *   }
 * });
 * ```
 */
export function observeIntersection(
  element: Element,
  callback: (entry: IntersectionObserverEntry) => void,
  options?: IntersectionObserverInit
): () => void {
  const observer = new IntersectionObserver((entries) => {
    entries.forEach(callback);
  }, options);

  observer.observe(element);

  return () => observer.disconnect();
}

/**
 * Request idle callback wrapper
 *
 * @param callback - Function to call when browser is idle
 * @param options - Request idle callback options
 *
 * @example
 * ```typescript
 * whenIdle(() => {
 *   performExpensiveOperation();
 * });
 * ```
 */
export function whenIdle(
  callback: () => void,
  options?: IdleRequestOptions
): void {
  if ('requestIdleCallback' in window) {
    window.requestIdleCallback(callback, options);
  } else {
    // Fallback for browsers that don't support requestIdleCallback
    setTimeout(callback, 1);
  }
}

/**
 * Performance measurement utility
 */
export class PerformanceMeasure {
  private startMark: string;
  private endMark: string;
  private measureName: string;

  constructor(name: string) {
    this.startMark = `${name}-start`;
    this.endMark = `${name}-end`;
    this.measureName = name;
  }

  /**
   * Start the performance measurement
   */
  start(): void {
    if (typeof performance !== 'undefined' && performance.mark) {
      performance.mark(this.startMark);
    }
  }

  /**
   * End the performance measurement and return the duration
   */
  end(): number | null {
    if (typeof performance === 'undefined' || !performance.mark || !performance.measure) {
      return null;
    }

    try {
      performance.mark(this.endMark);
      performance.measure(this.measureName, this.startMark, this.endMark);

      const measure = performance.getEntriesByName(this.measureName)[0];
      const duration = measure.duration;

      // Clean up marks and measures
      performance.clearMarks(this.startMark);
      performance.clearMarks(this.endMark);
      performance.clearMeasures(this.measureName);

      return duration;
    } catch (error) {
      console.warn('Performance measurement failed:', error);
      return null;
    }
  }
}

/**
 * Measure the execution time of a function
 *
 * @param name - Name for the measurement
 * @param fn - Function to measure
 * @returns Result of the function
 *
 * @example
 * ```typescript
 * const result = await measurePerformance('data-fetch', async () => {
 *   return await fetchData();
 * });
 * ```
 */
export async function measurePerformance<T>(
  name: string,
  fn: () => T | Promise<T>
): Promise<T> {
  const measure = new PerformanceMeasure(name);

  measure.start();
  const result = await fn();
  const duration = measure.end();

  if (duration !== null) {
    console.debug(`[Performance] ${name}: ${duration.toFixed(2)}ms`);
  }

  return result;
}

/**
 * Batch DOM reads and writes for better performance
 */
export class BatchScheduler {
  private readQueue: Array<() => void> = [];
  private writeQueue: Array<() => void> = [];
  private scheduled = false;

  /**
   * Schedule a DOM read operation
   */
  read(callback: () => void): void {
    this.readQueue.push(callback);
    this.schedule();
  }

  /**
   * Schedule a DOM write operation
   */
  write(callback: () => void): void {
    this.writeQueue.push(callback);
    this.schedule();
  }

  /**
   * Schedule the flush
   */
  private schedule(): void {
    if (this.scheduled) return;

    this.scheduled = true;
    requestAnimationFrame(() => this.flush());
  }

  /**
   * Flush all queued operations
   */
  private flush(): void {
    // Execute all reads first
    while (this.readQueue.length > 0) {
      const read = this.readQueue.shift();
      read?.();
    }

    // Then execute all writes
    while (this.writeQueue.length > 0) {
      const write = this.writeQueue.shift();
      write?.();
    }

    this.scheduled = false;
  }
}

/**
 * Create a singleton batch scheduler
 */
export const batchScheduler = new BatchScheduler();

/**
 * Memoize a function's results
 *
 * @param fn - Function to memoize
 * @returns Memoized function
 *
 * @example
 * ```typescript
 * const expensiveCalc = memoize((n: number) => {
 *   return fibonacci(n);
 * });
 * ```
 */
export function memoize<T extends (...args: unknown[]) => unknown>(
  fn: T
): T {
  const cache = new Map<string, ReturnType<T>>();

  return ((...args: Parameters<T>) => {
    const key = JSON.stringify(args);

    if (cache.has(key)) {
      return cache.get(key);
    }

    const result = fn(...args) as ReturnType<T>;
    cache.set(key, result);

    return result;
  }) as T;
}

/**
 * Virtual scrolling utility for large lists
 */
export interface VirtualScrollConfig {
  itemHeight: number;
  containerHeight: number;
  items: unknown[];
  overscan?: number;
}

export interface VirtualScrollResult {
  startIndex: number;
  endIndex: number;
  offsetY: number;
  visibleItems: unknown[];
}

export function calculateVirtualScroll(
  scrollTop: number,
  config: VirtualScrollConfig
): VirtualScrollResult {
  const { itemHeight, containerHeight, items, overscan = 3 } = config;

  const startIndex = Math.max(0, Math.floor(scrollTop / itemHeight) - overscan);
  const visibleCount = Math.ceil(containerHeight / itemHeight) + overscan * 2;
  const endIndex = Math.min(items.length, startIndex + visibleCount);

  return {
    startIndex,
    endIndex,
    offsetY: startIndex * itemHeight,
    visibleItems: items.slice(startIndex, endIndex),
  };
}

/**
 * Image lazy loading utility
 *
 * @param img - Image element
 * @param src - Image source URL
 *
 * @example
 * ```typescript
 * <img data-src="/path/to/image.jpg" use:lazyLoadImage />
 * ```
 */
export function lazyLoadImage(img: HTMLImageElement): { destroy: () => void } {
  const src = img.dataset.src;

  if (!src) {
    return { destroy: () => {} };
  }

  const cleanup = observeIntersection(
    img,
    (entry) => {
      if (entry.isIntersecting && src) {
        img.src = src;
        img.removeAttribute('data-src');
        cleanup();
      }
    },
    { rootMargin: '50px' }
  );

  return { destroy: cleanup };
}

/**
 * Monitor memory usage (Chrome DevTools)
 */
export function getMemoryUsage(): { usedJSHeapSize: number; totalJSHeapSize: number; limit: number } | null {
  if ('memory' in performance && (performance as { memory?: {
    usedJSHeapSize: number;
    totalJSHeapSize: number;
    jsHeapSizeLimit: number;
  } }).memory) {
    const memory = (performance as { memory: {
      usedJSHeapSize: number;
      totalJSHeapSize: number;
      jsHeapSizeLimit: number;
    } }).memory;

    return {
      usedJSHeapSize: memory.usedJSHeapSize,
      totalJSHeapSize: memory.totalJSHeapSize,
      limit: memory.jsHeapSizeLimit,
    };
  }

  return null;
}

/**
 * Log performance metrics
 */
export function logPerformanceMetrics(): void {
  if (typeof performance === 'undefined') return;

  console.group('Performance Metrics');

  // Navigation timing
  const navigation = performance.getEntriesByType('navigation')[0] as PerformanceNavigationTiming;
  if (navigation) {
    console.log('DOM Content Loaded:', navigation.domContentLoadedEventEnd - navigation.fetchStart, 'ms');
    console.log('Load Complete:', navigation.loadEventEnd - navigation.fetchStart, 'ms');
  }

  // Memory usage
  const memory = getMemoryUsage();
  if (memory) {
    console.log('Memory Used:', (memory.usedJSHeapSize / 1024 / 1024).toFixed(2), 'MB');
    console.log('Memory Total:', (memory.totalJSHeapSize / 1024 / 1024).toFixed(2), 'MB');
    console.log('Memory Limit:', (memory.limit / 1024 / 1024).toFixed(2), 'MB');
  }

  // Resource timing
  const resources = performance.getEntriesByType('resource');
  console.log('Total Resources:', resources.length);

  const resourcesByType = resources.reduce((acc, resource) => {
    const type = (resource as PerformanceResourceTiming).initiatorType;
    acc[type] = (acc[type] || 0) + 1;
    return acc;
  }, {} as Record<string, number>);

  console.log('Resources by Type:', resourcesByType);

  console.groupEnd();
}
