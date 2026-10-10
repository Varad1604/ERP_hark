import type { Config } from "@react-router/dev/config";

// ADR-L5-0015: React Router framework mode, SPA-first, runtime SSR disabled. The build emits a
// static artifact (build/client) served by any static host with SPA fallback to index.html.
export default {
  appDirectory: "app",
  ssr: false,
} satisfies Config;
