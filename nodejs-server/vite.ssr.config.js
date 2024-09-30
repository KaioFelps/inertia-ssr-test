import { defineConfig } from 'vite'
import laravel from 'laravel-vite-plugin';

export default defineConfig({
  plugins: [
    laravel({
      input: ["www/app.tsx"],
      ssr: ["www/ssr.tsx"],
      publicDirectory: "dist",
      buildDirectory: "server",
      refresh: false
    })
  ],
  build: {
    ssr: true,
    outDir: 'dist/server',
    rollupOptions: {
      input: "www/ssr.tsx",
      output: {
        entryFileNames: "[name].js",
        chunkFileNames: "[name].js",
        assetFileNames: "[name][extname]",
        manualChunks: undefined // disable automatic chunk splitting
      },
    },
  },
})

