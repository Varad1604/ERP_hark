import { reactRouter } from "@react-router/dev/vite";
import { defineConfig } from "vite";

export default defineConfig({
  plugins: [reactRouter()],
  server: {
    port: 5173,
    // Same-origin API in dev so the HttpOnly session cookie works without CORS.
    proxy: {
      "/api": { target: process.env.SUTRA_API_URL ?? "http://localhost:8080", changeOrigin: false },
      "/health": { target: process.env.SUTRA_API_URL ?? "http://localhost:8080" },
    },
  },
});
