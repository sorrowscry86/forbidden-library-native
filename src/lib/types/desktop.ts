/**
 * TypeScript type definitions for desktop-specific features
 * These types match the Rust command return structures
 */

export interface SystemInfo {
  os: string;
  arch: string;
  family: string;
  version: string;
  tauri_version: string;
  platform: string;
}

export interface WindowState {
  width: number;
  height: number;
  x: number;
  y: number;
  maximized: boolean;
  minimized: boolean;
  fullscreen: boolean;
}

export interface UpdateInfo {
  available: boolean;
  current_version: string;
  latest_version: string;
  download_url: string | null;
}
