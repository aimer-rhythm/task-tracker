import { defineConfig } from "vitest/config";
import vue from "@vitejs/plugin-vue";

export default defineConfig({
  plugins: [vue()],
  test: {
    environment: "happy-dom",
    globals: true,
    alias: {
      "@/": new URL("./src/", import.meta.url).pathname,
      "@tauri-apps/api/core": new URL("./src/__mocks__/tauri-api.ts", import.meta.url).pathname,
    },
  },
});
