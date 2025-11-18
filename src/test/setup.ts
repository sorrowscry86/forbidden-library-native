import '@testing-library/jest-dom';
import { vi } from 'vitest';

// Mock Tauri API
const mockInvoke = vi.fn();
const mockListen = vi.fn();

vi.mock('@tauri-apps/api/tauri', () => ({
  invoke: mockInvoke
}));

vi.mock('@tauri-apps/api/event', () => ({
  listen: mockListen,
  emit: vi.fn()
}));

// Mock window.__TAURI__
global.window = Object.create(window);
Object.defineProperty(window, '__TAURI__', {
  value: {
    invoke: mockInvoke,
    event: {
      listen: mockListen,
      emit: vi.fn()
    }
  },
  writable: true
});

// Reset mocks before each test
beforeEach(() => {
  mockInvoke.mockClear();
  mockListen.mockClear();
});
