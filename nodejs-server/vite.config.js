import react from '@vitejs/plugin-react';
import laravel from 'laravel-vite-plugin';
import { defineConfig } from 'vite';

export default defineConfig({
    plugins: [    
        laravel({
            input: 'www/app.tsx',
            publicDirectory: 'dist',
            buildDirectory: 'build',
            refresh: 'www/**',
            ssrOutputDirectory: "dist/ssr",
            ssr: "www/ssr.tsx",
        }),
        react(),
    ],
});

