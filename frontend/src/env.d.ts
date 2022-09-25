/// <reference types="vite/client" />

interface ImportMetaEnv {
  readonly vite_API_BASE_URL: string;
}

interface ImportMeta {
  readonly env: ImportMetaEnv;
}
