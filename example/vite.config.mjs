import { defineConfig } from "vite";
import topLevelAwait from "vite-plugin-top-level-await";
import wasm from "vite-plugin-wasm";
import vue from '@vitejs/plugin-vue'

export default defineConfig({
  plugins: [
    wasm(),
    topLevelAwait(),
    vue(),
  ]
})