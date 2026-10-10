/// <reference types="vite/client" />
interface ImportMetaEnv {
  readonly VITE_AUTH_MODE?: "mock" | "api";
  readonly VITE_API_BASE?: string;
  readonly VITE_DEV_PASSWORD?: string;
}
interface ImportMeta {
  readonly env: ImportMetaEnv;
}
