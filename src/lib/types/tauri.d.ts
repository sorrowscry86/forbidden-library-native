// Type definitions for Tauri global object
interface Window {
  __TAURI__?: {
    invoke: <T>(cmd: string, args?: any) => Promise<T>;
    [key: string]: any;
  };
}
